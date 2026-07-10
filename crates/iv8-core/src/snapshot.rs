use std::sync::OnceLock;

static SNAPSHOT_BLOB: OnceLock<Vec<u8>> = OnceLock::new();

pub fn get_snapshot() -> &'static [u8] {
    SNAPSHOT_BLOB.get().map(|v| v.as_slice()).unwrap_or(&[])
}

pub fn set_snapshot(data: &[u8]) {
    let _ = SNAPSHOT_BLOB.set(data.to_vec());
}

pub fn create_snapshot_and_get() -> Vec<u8> {
    create_snapshot_impl().unwrap_or_default()
}

fn create_snapshot_impl() -> Option<Vec<u8>> {
    let isolate_ptr: *mut v8::OwnedIsolate = Box::into_raw(Box::new(
        v8::Isolate::snapshot_creator(None, None)
    ));
    let isolate = unsafe { &mut *isolate_ptr };
    isolate.set_microtasks_policy(v8::MicrotasksPolicy::Explicit);

    let state = crate::state::RuntimeState::new(
        false,
        crate::state::TimeMode::System,
        "snapshot".to_string(),
        std::sync::Arc::new(crate::config::EnvironmentMap::defaults()),
        None,
        None,
    );
    crate::state::RuntimeState::install(isolate, state);

    let context_global = {
        v8::scope!(scope, isolate);
        let context = v8::Context::new(scope, Default::default());
        v8::scope_with_context!(scope, scope, context);
        let global = context.global(scope);

        let callbacks = iv8_surface::BehaviorCallbackRegistry::new();
        if let Ok(_registry) = iv8_surface::install_browser_surface(scope, global, &callbacks, false) {
            let codegen_protos =
                crate::dom::template::capture_codegen_prototypes(scope, global);
            let dom_templates = crate::dom::template::build_dom_templates(scope);
            crate::dom::template::install_dom_constructors(scope, global, &dom_templates, false);
            crate::dom::template::chain_dom_prototypes(scope, global, &codegen_protos);
        }
        v8::Global::new(scope, context)
    };

    {
        let iso_ref = unsafe { &mut *isolate_ptr };
        v8::scope!(scope, iso_ref);
        let context = v8::Local::new(scope, &context_global);
        unsafe { (*isolate_ptr).set_default_context(context); }
    }

    let owned = *unsafe { Box::from_raw(isolate_ptr) };
    owned.create_blob(v8::FunctionCodeHandling::Keep)
        .map(|data| data.to_vec())
}
