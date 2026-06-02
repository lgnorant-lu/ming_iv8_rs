//! EventTarget: addEventListener / removeEventListener / dispatchEvent.
//!
//! Stores listeners in a central registry keyed by NodeId.
//! Dispatch follows the DOM three-phase model: capture → target → bubble.

use std::cell::RefCell;
use std::collections::HashMap;

use crate::dom::NodeId;

/// A registered event listener.
#[derive(Debug)]
pub struct EventListener {
    /// The V8 callback function.
    pub callback: v8::Global<v8::Function>,
    /// Whether this listener is for the capture phase.
    pub capture: bool,
    /// Whether this is a one-shot listener (once: true).
    pub once: bool,
}

/// Central registry of event listeners, keyed by (NodeId, event_type).
pub struct EventListenerRegistry {
    /// Map: NodeId → event_type → Vec<EventListener>
    listeners: HashMap<NodeId, HashMap<String, Vec<EventListener>>>,
}

impl EventListenerRegistry {
    pub fn new() -> Self {
        Self {
            listeners: HashMap::new(),
        }
    }

    /// Add an event listener for a node.
    pub fn add(
        &mut self,
        node_id: NodeId,
        event_type: &str,
        callback: v8::Global<v8::Function>,
        capture: bool,
        once: bool,
    ) {
        self.listeners
            .entry(node_id)
            .or_default()
            .entry(event_type.to_string())
            .or_default()
            .push(EventListener {
                callback,
                capture,
                once,
            });
    }

    /// Remove an event listener by callback identity.
    /// Uses V8 strict equality to find the matching listener.
    pub fn remove_by_callback(
        &mut self,
        scope: &v8::PinScope<'_, '_>,
        node_id: NodeId,
        event_type: &str,
        callback: v8::Local<v8::Function>,
        capture: bool,
    ) {
        if let Some(type_map) = self.listeners.get_mut(&node_id) {
            if let Some(listeners) = type_map.get_mut(event_type) {
                // Find the listener with matching callback (strict equality) and capture phase
                let pos = listeners.iter().position(|l| {
                    if l.capture != capture { return false; }
                    let stored = v8::Local::new(scope, &l.callback);
                    // Use strict_equals for function identity comparison
                    stored.strict_equals(callback.into())
                });
                if let Some(idx) = pos {
                    listeners.remove(idx);
                }
            }
        }
    }

    /// Remove an event listener (legacy: removes last added for the type).
    pub fn remove(
        &mut self,
        node_id: NodeId,
        event_type: &str,
        _capture: bool,
    ) {
        if let Some(type_map) = self.listeners.get_mut(&node_id) {
            if let Some(listeners) = type_map.get_mut(event_type) {
                listeners.pop();
            }
        }
    }

    /// Get listeners for a node + event type, filtered by phase.
    pub fn get_listeners(
        &self,
        node_id: NodeId,
        event_type: &str,
        capture_phase: bool,
    ) -> Vec<&EventListener> {
        self.listeners
            .get(&node_id)
            .and_then(|m| m.get(event_type))
            .map(|listeners| {
                listeners
                    .iter()
                    .filter(|l| l.capture == capture_phase)
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Remove once-listeners that have fired.
    pub fn remove_once_listeners(&mut self, node_id: NodeId, event_type: &str) {
        if let Some(type_map) = self.listeners.get_mut(&node_id) {
            if let Some(listeners) = type_map.get_mut(event_type) {
                listeners.retain(|l| !l.once);
            }
        }
    }
}

impl Default for EventListenerRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Dispatch an event through the three-phase model.
/// Returns true if default was NOT prevented.
pub fn dispatch_event(
    scope: &v8::PinScope<'_, '_>,
    registry: &RefCell<EventListenerRegistry>,
    document: &crate::dom::Document,
    target_id: NodeId,
    event_type: &str,
    bubbles: bool,
) -> bool {
    // Build the event path: root → ... → parent → target
    let path = build_event_path(document, target_id);

    // Create the event object
    let event_obj = create_event_object(scope, event_type, bubbles, target_id);
    let prevented = std::cell::Cell::new(false);
    let stopped = std::cell::Cell::new(false);

    // Phase 1: Capturing (root → target.parent, NOT including target)
    // path = [target, parent, ..., root], so skip(1) removes target, rev() gives root→parent
    for &node_id in path.iter().skip(1).rev() {
        if stopped.get() {
            break;
        }
        invoke_listeners(scope, registry, node_id, event_type, true, event_obj, &prevented, &stopped);
    }

    // Phase 2: At target
    if !stopped.get() {
        // At target, both capture and bubble listeners fire
        invoke_listeners(scope, registry, target_id, event_type, true, event_obj, &prevented, &stopped);
        if !stopped.get() {
            invoke_listeners(scope, registry, target_id, event_type, false, event_obj, &prevented, &stopped);
        }
    }

    // Phase 3: Bubbling (target.parent → root, NOT including target)
    // path = [target, parent, ..., root], so skip(1) removes target, giving parent→root
    if bubbles && !stopped.get() {
        for &node_id in path.iter().skip(1) {
            if stopped.get() {
                break;
            }
            invoke_listeners(scope, registry, node_id, event_type, false, event_obj, &prevented, &stopped);
        }
    }

    // Clean up once-listeners
    for &node_id in &path {
        registry.borrow_mut().remove_once_listeners(node_id, event_type);
    }

    !prevented.get()
}

/// Build the path from target up to root (target first, root last).
fn build_event_path(document: &crate::dom::Document, target_id: NodeId) -> Vec<NodeId> {
    let mut path = Vec::new();
    let mut current = Some(target_id);
    while let Some(id) = current {
        path.push(id);
        current = document.get(id).and_then(|n| n.parent()).map(|p| p.id());
    }
    path
}

/// Create a JS Event object with standard properties.
fn create_event_object<'s>(
    scope: &v8::PinScope<'s, '_>,
    event_type: &str,
    bubbles: bool,
    _target_id: NodeId,
) -> v8::Local<'s, v8::Object> {
    let obj = v8::Object::new(scope);

    let type_key = crate::v8_utils::v8_string(scope, "type");
    let type_val = crate::v8_utils::v8_string(scope, event_type);
    obj.set(scope, type_key.into(), type_val.into());

    let bubbles_key = crate::v8_utils::v8_string(scope, "bubbles");
    let bubbles_val = v8::Boolean::new(scope, bubbles);
    obj.set(scope, bubbles_key.into(), bubbles_val.into());

    let cancelable_key = crate::v8_utils::v8_string(scope, "cancelable");
    let cancelable_val = v8::Boolean::new(scope, true);
    obj.set(scope, cancelable_key.into(), cancelable_val.into());

    let prevented_key = crate::v8_utils::v8_string(scope, "defaultPrevented");
    let prevented_val = v8::Boolean::new(scope, false);
    obj.set(scope, prevented_key.into(), prevented_val.into());

    // Install stopPropagation and preventDefault as real methods
    // These set hidden flags on the event object that the dispatch loop checks
    let stop_key = crate::v8_utils::v8_string(scope, "__stopped__");
    obj.set(scope, stop_key.into(), v8::Boolean::new(scope, false).into());

    let prevent_key = crate::v8_utils::v8_string(scope, "__prevented__");
    obj.set(scope, prevent_key.into(), v8::Boolean::new(scope, false).into());

    // stopPropagation
    let sp_tmpl = v8::FunctionTemplate::builder_raw(stop_propagation_cb).build(scope);
    let sp_fn = crate::v8_utils::v8_fn(scope, &*sp_tmpl);
    let sp_key = crate::v8_utils::v8_string(scope, "stopPropagation");
    obj.set(scope, sp_key.into(), sp_fn.into());

    // stopImmediatePropagation
    let sip_tmpl = v8::FunctionTemplate::builder_raw(stop_propagation_cb).build(scope);
    let sip_fn = crate::v8_utils::v8_fn(scope, &*sip_tmpl);
    let sip_key = crate::v8_utils::v8_string(scope, "stopImmediatePropagation");
    obj.set(scope, sip_key.into(), sip_fn.into());

    // preventDefault
    let pd_tmpl = v8::FunctionTemplate::builder_raw(prevent_default_cb).build(scope);
    let pd_fn = crate::v8_utils::v8_fn(scope, &*pd_tmpl);
    let pd_key = crate::v8_utils::v8_string(scope, "preventDefault");
    obj.set(scope, pd_key.into(), pd_fn.into());

    obj
}

unsafe extern "C" fn stop_propagation_cb(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let this = args.this();
        let key = crate::v8_utils::v8_string(scope, "__stopped__");
        this.set(scope, key.into(), v8::Boolean::new(scope, true).into());
    }));
}

unsafe extern "C" fn prevent_default_cb(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let this = args.this();
        let key = crate::v8_utils::v8_string(scope, "__prevented__");
        this.set(scope, key.into(), v8::Boolean::new(scope, true).into());
        let dp_key = crate::v8_utils::v8_string(scope, "defaultPrevented");
        this.set(scope, dp_key.into(), v8::Boolean::new(scope, true).into());
    }));
}

/// Invoke listeners for a specific node/phase.
fn invoke_listeners(
    scope: &v8::PinScope<'_, '_>,
    registry: &RefCell<EventListenerRegistry>,
    node_id: NodeId,
    event_type: &str,
    capture_phase: bool,
    event_obj: v8::Local<v8::Object>,
    _prevented: &std::cell::Cell<bool>,
    stopped: &std::cell::Cell<bool>,
) {
    let reg = registry.borrow();
    let listeners = reg.get_listeners(node_id, event_type, capture_phase);

    for listener in listeners {
        if stopped.get() { break; }
        let func = v8::Local::new(scope, &listener.callback);
        let undefined = v8::undefined(scope);
        func.call(scope, undefined.into(), &[event_obj.into()]);

        // Check if stopPropagation was called
        let stop_key = crate::v8_utils::v8_string(scope, "__stopped__");
        if let Some(stopped_val) = event_obj.get(scope, stop_key.into()) {
            if stopped_val.is_true() {
                stopped.set(true);
            }
        }

        // Check if preventDefault was called
        let prevent_key = crate::v8_utils::v8_string(scope, "__prevented__");
        if let Some(prevented_val) = event_obj.get(scope, prevent_key.into()) {
            if prevented_val.is_true() {
                _prevented.set(true);
            }
        }
    }
}
