//! Web Worker execution environment.
//!
//! Each Worker spawns a dedicated OS thread that creates its own V8 isolate,
//! installs WorkerGlobalScope, runs the worker script, and processes messages
//! via an mpsc channel. Communication uses V8 structured clone serialization.

use std::sync::mpsc::{Receiver, Sender};
use std::thread::JoinHandle;

use crate::shims::browser_profile::BrowserProfile;
use crate::shims::structured_clone::{deserialize_value, serialize_value};
use crate::v8_init::ensure_v8_initialized;

const WORKER_BOOTSTRAP_JS: &str = include_str!("worker_bootstrap.js");
const WORKER_JS_STUB: &str = include_str!("worker_js_stub.js");

pub enum WorkerMessage {
    PostMessage(Vec<u8>),
    Terminate,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_worker_message_post_message_variant() {
        let msg = WorkerMessage::PostMessage(vec![1, 2, 3]);
        match msg {
            WorkerMessage::PostMessage(data) => assert_eq!(data, vec![1, 2, 3]),
            WorkerMessage::Terminate => panic!("expected PostMessage"),
        }
    }

    #[test]
    fn test_worker_message_terminate_variant() {
        let msg = WorkerMessage::Terminate;
        match msg {
            WorkerMessage::Terminate => {}
            _ => panic!("expected Terminate"),
        }
    }

    #[test]
    fn test_worker_message_channel_send_receive() {
        let (tx, rx) = std::sync::mpsc::channel();
        tx.send(WorkerMessage::PostMessage(vec![42])).unwrap();
        let msg = rx.recv().unwrap();
        match msg {
            WorkerMessage::PostMessage(data) => assert_eq!(data, vec![42]),
            _ => panic!("expected PostMessage"),
        }
    }

    #[test]
    fn test_worker_message_channel_terminate() {
        let (tx, rx) = std::sync::mpsc::channel();
        tx.send(WorkerMessage::Terminate).unwrap();
        let msg = rx.recv().unwrap();
        matches!(msg, WorkerMessage::Terminate);
    }

    #[test]
    fn test_worker_message_channel_closed_returns_error() {
        let (tx, rx) = std::sync::mpsc::channel::<WorkerMessage>();
        drop(tx);
        assert!(rx.recv().is_err());
    }

    #[test]
    fn test_worker_bootstrap_js_is_non_empty() {
        assert!(!WORKER_BOOTSTRAP_JS.is_empty());
        assert!(WORKER_BOOTSTRAP_JS.contains("function"));
    }

    #[test]
    fn test_worker_js_stub_is_non_empty() {
        assert!(!WORKER_JS_STUB.is_empty());
    }
}

struct WorkerSlot {
    main_tx: Sender<Vec<u8>>,
    closed: std::cell::Cell<bool>,
}

impl WorkerSlot {
    fn get(isolate: &v8::Isolate) -> &Self {
        isolate
            .get_slot::<Self>()
            .expect("WorkerSlot not installed on worker isolate")
    }
}

pub struct WorkerHandle {
    pub thread: Option<JoinHandle<()>>,
    pub tx: Sender<WorkerMessage>,
    pub isolate_handle: v8::IsolateHandle,
    pub rx: Receiver<Vec<u8>>,
    pub worker_id: u64,
}

impl WorkerHandle {
    pub fn terminate(&mut self) {
        let _ = self.tx.send(WorkerMessage::Terminate);
        let _ = self.isolate_handle.terminate_execution();
        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
    }
}

impl Drop for WorkerHandle {
    fn drop(&mut self) {
        let _ = self.tx.send(WorkerMessage::Terminate);
        let _ = self.isolate_handle.terminate_execution();
        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
    }
}

fn build_profile_json(profile: &BrowserProfile, worker_url: &str) -> String {
    let langs: Vec<String> = profile
        .languages
        .iter()
        .map(|s| format!("\"{}\"", s))
        .collect();
    let user_agent_data = format!(
        "{{\"brands\":{}, \"mobile\":{}, \"platform\":\"{}\"}}",
        profile.ua_brands_json, profile.ua_mobile, profile.ua_platform
    );
    let (origin, protocol, host, hostname, port, pathname) = parse_worker_url(worker_url);
    format!(
        "{{\
            \"userAgent\": {:?},\
            \"platform\": {:?},\
            \"language\": {:?},\
            \"languages\": [{}],\
            \"hardwareConcurrency\": {},\
            \"deviceMemory\": {},\
            \"vendor\": {:?},\
            \"product\": {:?},\
            \"appName\": {:?},\
            \"appVersion\": {:?},\
            \"appCodeName\": {:?},\
            \"cookieEnabled\": {},\
            \"pdfViewerEnabled\": {},\
            \"onLine\": {},\
            \"userAgentData\": {},\
            \"workerUrl\": {:?},\
            \"workerOrigin\": {:?},\
            \"workerProtocol\": {:?},\
            \"workerHost\": {:?},\
            \"workerHostname\": {:?},\
            \"workerPort\": {:?},\
            \"workerPathname\": {:?},\
            \"name\": \"\"\
        }}",
        profile.user_agent,
        profile.platform,
        profile.language,
        langs.join(","),
        profile.hardware_concurrency as u64,
        profile.device_memory as u64,
        profile.vendor,
        profile.product,
        profile.app_name,
        profile.app_version,
        profile.app_code_name,
        profile.cookie_enabled,
        profile.pdf_viewer_enabled,
        profile.on_line,
        user_agent_data,
        worker_url,
        origin,
        protocol,
        host,
        hostname,
        port,
        pathname,
    )
}

fn parse_worker_url(url: &str) -> (String, String, String, String, String, String) {
    if url.starts_with("blob:") || url.starts_with("data:") {
        return (
            "null".to_string(),
            ":".to_string(),
            String::new(),
            String::new(),
            String::new(),
            url.to_string(),
        );
    }
    let protocol = if let Some(idx) = url.find("://") {
        url[..idx].to_string()
    } else {
        "https".to_string()
    };
    let rest = if let Some(idx) = url.find("://") {
        &url[idx + 3..]
    } else {
        url
    };
    let (host_part, path_part) = if let Some(idx) = rest.find('/') {
        (&rest[..idx], &rest[idx..])
    } else {
        (rest, "")
    };
    let (hostname, port) = if let Some(idx) = host_part.find(':') {
        (&host_part[..idx], &host_part[idx + 1..])
    } else {
        (host_part, "")
    };
    let host = host_part.to_string();
    let origin = format!("{}://{}", protocol, host);
    (
        origin,
        format!("{}:", protocol),
        host,
        hostname.to_string(),
        port.to_string(),
        path_part.to_string(),
    )
}

pub fn spawn_worker(
    script_source: String,
    script_url: String,
    profile: &'static BrowserProfile,
    worker_id: u64,
) -> WorkerHandle {
    let (main_tx, main_rx) = std::sync::mpsc::channel::<Vec<u8>>();
    let (worker_tx, worker_rx) = std::sync::mpsc::channel::<WorkerMessage>();
    let profile_json = build_profile_json(profile, &script_url);

    let isolate_handle_tx = worker_tx.clone();
    let (handle_tx, handle_rx) = std::sync::mpsc::channel::<v8::IsolateHandle>();

    let builder = std::thread::Builder::new()
        .name(format!("iv8-worker-{}", worker_id))
        .stack_size(64 * 1024 * 1024);

    let thread = builder
        .spawn(move || {
            worker_thread_main(
                script_source,
                script_url,
                profile_json,
                isolate_handle_tx,
                worker_rx,
                main_tx,
                handle_tx,
            );
        })
        .expect("failed to spawn worker thread");

    let isolate_handle = handle_rx
        .recv()
        .expect("worker thread failed to provide isolate handle");

    WorkerHandle {
        thread: Some(thread),
        tx: worker_tx,
        isolate_handle,
        rx: main_rx,
        worker_id,
    }
}

#[allow(clippy::too_many_arguments)]
fn worker_thread_main(
    script_source: String,
    script_url: String,
    profile_json: String,
    worker_tx: Sender<WorkerMessage>,
    worker_rx: Receiver<WorkerMessage>,
    main_tx: Sender<Vec<u8>>,
    handle_tx: Sender<v8::IsolateHandle>,
) {
    ensure_v8_initialized();

    let mut isolate = v8::Isolate::new(
        v8::CreateParams::default().heap_limits(0, 2048 * 1024 * 1024),
    );
    isolate.set_microtasks_policy(v8::MicrotasksPolicy::Explicit);

    let isolate_handle = isolate.thread_safe_handle();
    let _ = handle_tx.send(isolate_handle);

    isolate.set_slot(WorkerSlot {
        main_tx,
        closed: std::cell::Cell::new(false),
    });

    let context = {
        v8::scope!(scope, &mut isolate);
        let context = v8::Context::new(scope, Default::default());
        v8::Global::new(scope, context)
    };

    // Install RuntimeState for Worker isolate.
    // NOTE: codegen IDL template installation (install_browser_surface) causes
    // V8 GC "IsOnCentralStack" crash on Worker thread. Root cause: V8 shared
    // ReadOnlySpace (V8_SHARED_RO_HEAP, compile-time macro) is shared across
    // isolates. GC accessing shared RO heap must run on the "central stack"
    // thread (main thread).
    // Tested approaches that do NOT work:
    //   - --single-threaded flag: still crashes, 2 regression in WPT
    //   - --no-shared-readonly-heap: flag not recognized in V8 147
    //   - 4GB heap limit: GC still triggered during FunctionTemplate creation
    //   - new_unprotected_default_platform: does not fix shared RO heap GC
    // Only viable solution: V8 snapshot (SnapshotCreator)
    //   - Pre-install FunctionTemplates in snapshot blob on main thread
    //   - Worker isolate creates from snapshot, no runtime FunctionTemplate GC
    //   - Deno validates this approach (JsRuntimeForSnapshot)
    // This is the "通道架构重构" blocker (D-116 §9.8).
    let state = crate::state::RuntimeState::new(
        false,
        crate::state::TimeMode::System,
        "worker".to_string(),
        std::sync::Arc::new(crate::config::EnvironmentMap::defaults()),
        None,
        None,
    );
    crate::state::RuntimeState::install(&mut isolate, state);

    {
        v8::scope!(scope, &mut isolate);
        let context = v8::Local::new(scope, &context);
        v8::scope_with_context!(scope, scope, context);
        let global = context.global(scope);
        // Install Worker interface stubs via pure JS eval first so
        // WorkerNavigator exists before bootstrap wires navigator proto.
        // (FunctionTemplate path crashes Worker thread GC — D-116.)
        let stub_str = crate::v8_utils::v8_string(scope, WORKER_JS_STUB);
        let stub_result = v8::Script::compile(scope, stub_str, None).and_then(|s| s.run(scope));
        if stub_result.is_none() {
            crate::telemetry::worker_script_error("JS stub eval failed");
        }

        install_worker_globals(scope, global, &profile_json);
        install_worker_callbacks(scope, global);
    }

    {
        v8::scope!(scope, &mut isolate);
        let context = v8::Local::new(scope, &context);
        v8::scope_with_context!(scope, scope, context);

        let source_str = match v8::String::new(scope, &script_source) {
            Some(s) => s,
            None => return,
        };

        let origin = v8::ScriptOrigin::new(
            scope,
            v8::String::new(scope, &script_url)
                .unwrap_or_else(|| v8::String::empty(scope))
                .into(),
            0,
            0,
            false,
            0,
            None,
            false,
            false,
            false,
            None,
        );

        v8::tc_scope!(tc, scope);
        if let Some(script) = v8::Script::compile(tc, source_str, Some(&origin)) {
            let _ = script.run(tc);
        }
        if tc.has_caught() {
            if let Some(exc) = tc.exception() {
                let msg = exc.to_rust_string_lossy(tc);
                crate::telemetry::worker_script_error(&msg);
            }
        }
    }

    isolate.perform_microtask_checkpoint();

    worker_message_loop(&mut isolate, &context, &worker_rx);

    drop(isolate);
}

fn install_worker_globals<'s>(
    scope: &v8::PinScope<'s, '_>,
    global: v8::Local<'s, v8::Object>,
    profile_json: &str,
) {
    let self_key = crate::v8_utils::v8_string(scope, "self");
    let _ = global.set(scope, self_key.into(), global.into());

    let profile_key = crate::v8_utils::v8_string(scope, "__iv8WorkerProfile");
    let profile_json_str = crate::v8_utils::v8_string(scope, profile_json);
    let profile_val = match v8::json::parse(scope, profile_json_str) {
        Some(v) => v,
        None => return,
    };
    let _ = global.define_own_property(
        scope,
        profile_key.into(),
        profile_val,
        v8::PropertyAttribute::DONT_ENUM,
    );

    let bootstrap_src = crate::v8_utils::v8_string(scope, WORKER_BOOTSTRAP_JS);
    let bootstrap_script = v8::Script::compile(scope, bootstrap_src, None);
    if let Some(script) = bootstrap_script {
        let _ = script.run(scope);
    }
}

fn install_worker_callbacks<'s>(scope: &v8::PinScope<'s, '_>, global: v8::Local<'s, v8::Object>) {
    let post_msg_tmpl = v8::FunctionTemplate::builder_raw(worker_post_message_cb).build(scope);
    let post_msg_fn = crate::v8_utils::v8_fn(scope, &post_msg_tmpl);
    let post_msg_key = crate::v8_utils::v8_string(scope, "__iv8WorkerPostMessage");
    let _ = global.define_own_property(
        scope,
        post_msg_key.into(),
        post_msg_fn.into(),
        v8::PropertyAttribute::DONT_ENUM,
    );

    let close_tmpl = v8::FunctionTemplate::builder_raw(worker_close_cb).build(scope);
    let close_fn = crate::v8_utils::v8_fn(scope, &close_tmpl);
    let close_key = crate::v8_utils::v8_string(scope, "__iv8WorkerClose");
    let _ = global.define_own_property(
        scope,
        close_key.into(),
        close_fn.into(),
        v8::PropertyAttribute::DONT_ENUM,
    );

    let import_tmpl = v8::FunctionTemplate::builder_raw(worker_import_script_cb).build(scope);
    let import_fn = crate::v8_utils::v8_fn(scope, &import_tmpl);
    let import_key = crate::v8_utils::v8_string(scope, "__iv8ImportScript");
    let _ = global.define_own_property(
        scope,
        import_key.into(),
        import_fn.into(),
        v8::PropertyAttribute::DONT_ENUM,
    );
}

unsafe extern "C" fn worker_post_message_cb(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let isolate: &v8::Isolate = &*scope;
        let slot = WorkerSlot::get(isolate);
        if slot.closed.get() {
            return;
        }

        let context = scope.get_current_context();
        if args.length() < 1 {
            let msg =
                crate::v8_utils::v8_string(scope, "postMessage requires a message argument");
            let exc = v8::Exception::type_error(scope, msg);
            scope.throw_exception(exc);
            return;
        }

        let arg = args.get(0);
        match serialize_value(scope, context, arg) {
            Ok(bytes) => {
                let _ = slot.main_tx.send(bytes);
            }
            Err(e) => {
                let msg = crate::v8_utils::v8_string(scope, &e);
                let exc = v8::Exception::type_error(scope, msg);
                scope.throw_exception(exc);
            }
        }
    }));
}

unsafe extern "C" fn worker_close_cb(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let isolate: &v8::Isolate = &*scope;
        let slot = WorkerSlot::get(isolate);
        slot.closed.set(true);
    }));
}

/// Resolve a potentially relative script URL against the worker's base URL.
/// Returns the resolved absolute URL string.
fn resolve_worker_url(base_url: &str, script_url: &str) -> String {
    if script_url.starts_with("http://")
        || script_url.starts_with("https://")
        || script_url.starts_with("blob:")
        || script_url.starts_with("data:")
    {
        return script_url.to_string();
    }

    if let Some(idx) = base_url.find("://") {
        let rest = &base_url[idx + 3..];
        let (host_part, path_part) = if let Some(slash) = rest.find('/') {
            (&rest[..slash], &rest[slash..])
        } else {
            (rest, "")
        };

        let origin = &base_url[..idx + 3 + host_part.len()];

        if script_url.starts_with('/') {
            return format!("{}{}", origin, script_url);
        }

        let dir = if let Some(last_slash) = path_part.rfind('/') {
            &path_part[..=last_slash]
        } else {
            "/"
        };

        return format!("{}{}{}", origin, dir, script_url);
    }

    script_url.to_string()
}

unsafe extern "C" fn worker_import_script_cb(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);

        let empty = || v8::String::new(scope, "").unwrap_or_else(|| v8::String::empty(scope));
        if args.length() < 1 {
            rv.set(empty().into());
            return;
        }

        let raw_url = args.get(0).to_rust_string_lossy(scope);
        let isolate: &v8::Isolate = &*scope;
        let state = crate::state::RuntimeState::get(isolate);

        let resolved_url = if raw_url.starts_with("http://")
            || raw_url.starts_with("https://")
            || raw_url.starts_with("blob:")
            || raw_url.starts_with("data:")
        {
            raw_url.clone()
        } else {
            let profile_url = {
                let global = scope.get_current_context().global(scope);
                let profile_key = crate::v8_utils::v8_string(scope, "__iv8WorkerProfile");
                global
                    .get(scope, profile_key.into())
                    .and_then(|v| {
                        let obj: v8::Local<v8::Object> = v.try_into().ok()?;
                        let url_key = crate::v8_utils::v8_string(scope, "workerUrl");
                        obj.get(scope, url_key.into())
                            .map(|v2| v2.to_rust_string_lossy(scope))
                    })
                    .unwrap_or_default()
            };
            resolve_worker_url(&profile_url, &raw_url)
        };

        let resource_ref = {
            let bundle = state.resource_bundle.borrow();
            bundle.get(&resolved_url).cloned()
        };

        let resource = if resource_ref.is_none() {
            let handler_result = {
                let handler = state.network_handler.borrow();
                if let Some(ref h) = *handler {
                    h(&resolved_url, "GET")
                } else {
                    None
                }
            };
            handler_result
                .map(|(status, body)| crate::network::bundle::Resource::new(body, status, None))
        } else {
            resource_ref
        };

        match resource {
            Some(res) => {
                let body = res.body_text();
                match v8::String::new(scope, &body) {
                    Some(s) => rv.set(s.into()),
                    None => rv.set(empty().into()),
                }
            }
            None => {
                crate::telemetry::worker_import_script_not_found(&resolved_url);
                rv.set(empty().into());
            }
        }
    }));
}

fn worker_message_loop(
    isolate: &mut v8::OwnedIsolate,
    context: &v8::Global<v8::Context>,
    worker_rx: &Receiver<WorkerMessage>,
) {
    loop {
        let msg = match worker_rx.recv() {
            Ok(m) => m,
            Err(_) => break,
        };

        match msg {
            WorkerMessage::Terminate => break,
            WorkerMessage::PostMessage(bytes) => {
                dispatch_message_to_worker(isolate, context, &bytes);
                isolate.perform_microtask_checkpoint();
            }
        }

        let slot = isolate.get_slot::<WorkerSlot>();
        if let Some(s) = slot {
            if s.closed.get() {
                break;
            }
        }
    }
}

fn dispatch_message_to_worker(
    isolate: &mut v8::OwnedIsolate,
    context: &v8::Global<v8::Context>,
    bytes: &[u8],
) {
    v8::scope!(scope, isolate);
    let ctx = v8::Local::new(scope, context);
    v8::scope_with_context!(scope, scope, ctx);

    let data = match deserialize_value(scope, ctx, bytes) {
        Some(v) => v,
        None => return,
    };

    let event = create_message_event(scope, data);

    let global = ctx.global(scope);
    let onmsg_key = crate::v8_utils::v8_string(scope, "onmessage");
    if let Some(handler) = global.get(scope, onmsg_key.into()) {
        if handler.is_function() {
            let func: v8::Local<v8::Function> = unsafe { v8::Local::cast_unchecked(handler) };
            let _ = func.call(scope, global.into(), &[event.into()]);
        }
    }
}

fn create_message_event<'s>(
    scope: &v8::PinScope<'s, '_>,
    data: v8::Local<'s, v8::Value>,
) -> v8::Local<'s, v8::Object> {
    let event = v8::Object::new(scope);
    let data_key = crate::v8_utils::v8_string(scope, "data");
    let _ = event.set(scope, data_key.into(), data);

    let type_key = crate::v8_utils::v8_string(scope, "type");
    let type_val = crate::v8_utils::v8_string(scope, "message");
    let _ = event.set(scope, type_key.into(), type_val.into());

    let tag_sym = v8::Symbol::get_to_string_tag(scope);
    let tag_val = crate::v8_utils::v8_string(scope, "MessageEvent");
    let _ = event.set(scope, tag_sym.into(), tag_val.into());

    event
}
