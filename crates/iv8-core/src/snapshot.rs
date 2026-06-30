use std::sync::OnceLock;

static SNAPSHOT_BLOB: OnceLock<Vec<u8>> = OnceLock::new();

pub fn get_or_create_snapshot() -> &'static [u8] {
    SNAPSHOT_BLOB.get_or_init(|| {
        create_snapshot_impl().unwrap_or_default()
    })
}

fn create_snapshot_impl() -> Option<Vec<u8>> {
    // Create snapshot creator isolate on the main thread (GC-safe).
    // The isolate is created via snapshot_creator which sets it up for
    // serialization. After installing all IDL templates, we call
    // create_blob to produce a StartupData that Worker isolates can use.
    let isolate_ptr: *mut v8::OwnedIsolate = Box::into_raw(Box::new(
        v8::Isolate::snapshot_creator(None, None)
    ));
    // SAFETY: isolate_ptr is a valid pointer to a heap-allocated OwnedIsolate.
    // We keep it alive via the Box and dispose at the end.
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
        if let Ok(_registry) = iv8_surface::install_browser_surface(scope, global, &callbacks) {
            let codegen_protos =
                crate::dom::template::capture_codegen_prototypes(scope, global);
            let dom_templates = crate::dom::template::build_dom_templates(scope);
            crate::dom::template::install_dom_constructors(scope, global, &dom_templates);
            crate::dom::template::chain_dom_prototypes(scope, global, &codegen_protos);
        }
        v8::Global::new(scope, context)
    };

    // set_default_context: called on the isolate, needs a Local<Context>.
    // We use a raw pointer to bypass the borrow checker since v8::scope!
    // shadows the isolate binding.
    {
        let iso_ref = unsafe { &mut *isolate_ptr };
        v8::scope!(scope, iso_ref);
        let context = v8::Local::new(scope, &context_global);
        // SAFETY: isolate_ptr is valid and points to the same isolate.
        // set_default_context is safe to call here.
        unsafe { (*isolate_ptr).set_default_context(context); }
    }

    // create_blob consumes the isolate (takes self by value).
    let owned = *unsafe { Box::from_raw(isolate_ptr) };
    owned.create_blob(v8::FunctionCodeHandling::Keep)
        .map(|data| data.to_vec())
}
