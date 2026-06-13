use std::sync::Arc;

use iv8_profile::BehaviorConfig;
use v8::PinScope;

use iv8_surface::behavior::{BehaviorCallbackRegistry, BehaviorInstaller};

thread_local! {
    static SHARED_CONFIG: std::cell::RefCell<Option<Arc<BehaviorConfig>>> =
        const { std::cell::RefCell::new(None) };
}

/// Build a BehaviorCallbackRegistry from a BehaviorConfig.
///
/// This is a manual builder — no derive macros, no V8 closure fields in config.
/// Each installer closure captures a clone of the config Arc and reads its
/// relevant subsection at install time.
pub fn build_registry(config: Arc<BehaviorConfig>) -> BehaviorCallbackRegistry {
    let mut registry = BehaviorCallbackRegistry::new();

    // Clone the Arc before the move into thread-local.
    let cfg_native_env = Arc::clone(&config);
    let cfg_location = Arc::clone(&config);
    let cfg_webgl = Arc::clone(&config);
    let cfg_canvas = Arc::clone(&config);
    let cfg_crypto = Arc::clone(&config);
    let cfg_time = Arc::clone(&config);
    let cfg_timers = Arc::clone(&config);

    // Store one clone in thread-local for runtime access.
    SHARED_CONFIG.with(move |sc| {
        *sc.borrow_mut() = Some(Arc::clone(&config));
    });

    registry.install_native_env = make_installer("install_native_env", {
        let _cfg = cfg_native_env;
        move |scope, global| {
            crate::shims::native_env::install_native_env(scope, global);
        }
    });

    registry.install_location = make_installer("install_location", {
        let _cfg = cfg_location;
        move |scope, global| {
            crate::shims::location::install_location(scope, global);
        }
    });

    registry.install_webgl_stubs = make_installer("install_webgl_stubs", {
        let _cfg = cfg_webgl;
        move |scope, global| {
            crate::canvas::webgl::install_webgl_stubs(scope, global);
        }
    });

    registry.install_canvas_bindings = make_installer("install_canvas_bindings", {
        let _cfg = cfg_canvas;
        move |scope, global| {
            crate::canvas::binding::install_canvas_bindings(scope, global);
        }
    });

    registry.install_crypto_random = make_installer("install_crypto_random", {
        let _cfg = cfg_crypto;
        move |scope, global| {
            crate::crypto::random::install_crypto_random(scope, global);
        }
    });

    registry.install_date_interceptor = make_installer("install_date_interceptor", {
        let _cfg = cfg_time;
        move |scope, global| {
            crate::events::date_interceptor::install_date_interceptor(scope, global);
        }
    });

    registry.install_timers = make_installer("install_timers", {
        let _cfg = cfg_timers;
        move |scope, global| {
            crate::events::timers::install_timer_globals(scope, global);
        }
    });

    registry
}

/// Create a BehaviorInstaller closure wrapping a plain fn pointer.
fn make_installer<F>(_name: &'static str, f: F) -> BehaviorInstaller
where
    F: Fn(&PinScope<'_, '_>, v8::Local<'_, v8::Object>) + 'static,
{
    let boxed: Box<dyn for<'s> Fn(&PinScope<'s, '_>, v8::Local<'s, v8::Object>)> = Box::new(f);
    std::rc::Rc::new(std::cell::RefCell::new(Some(boxed)))
}

/// Return a cloned reference to the current thread's shared config, if set.
pub fn current_config() -> Option<Arc<BehaviorConfig>> {
    SHARED_CONFIG.with(|sc| sc.borrow().clone())
}

#[cfg(test)]
mod tests {
    use super::*;
    use iv8_profile::defaults::default_profile_source;

    #[test]
    fn bcr_builder_produces_registry() {
        let source = default_profile_source();
        let (matrix, _) = iv8_profile::ProfileMatrix::from_source(&source);
        let config = Arc::new(BehaviorConfig::from_matrix(&matrix));
        let registry = build_registry(config);
        // Verify the 7 targeted installer slots are populated
        assert!(
            registry.install_native_env.borrow().is_some(),
            "install_native_env should be set"
        );
        assert!(
            registry.install_location.borrow().is_some(),
            "install_location should be set"
        );
        assert!(
            registry.install_webgl_stubs.borrow().is_some(),
            "install_webgl_stubs should be set"
        );
        assert!(
            registry.install_canvas_bindings.borrow().is_some(),
            "install_canvas_bindings should be set"
        );
        assert!(
            registry.install_crypto_random.borrow().is_some(),
            "install_crypto_random should be set"
        );
        assert!(
            registry.install_date_interceptor.borrow().is_some(),
            "install_date_interceptor should be set"
        );
        assert!(
            registry.install_timers.borrow().is_some(),
            "install_timers should be set"
        );
        // Verify the other 8 installer slots remain unset (defer to defaults)
        assert!(
            registry.install_page_api.borrow().is_none(),
            "install_page_api should remain unset (deferred)"
        );
    }

    #[test]
    fn current_config_is_set_after_build() {
        let source = default_profile_source();
        let (matrix, _) = iv8_profile::ProfileMatrix::from_source(&source);
        let config = Arc::new(BehaviorConfig::from_matrix(&matrix));
        assert!(current_config().is_none());
        build_registry(config.clone());
        let retrieved = current_config().expect("should be set after build");
        assert_eq!(retrieved.identity.user_agent, config.identity.user_agent);
    }
}
