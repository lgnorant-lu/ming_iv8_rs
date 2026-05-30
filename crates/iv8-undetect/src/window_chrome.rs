//! window.chrome object installation.
//!
//! Installs chrome.app, chrome.csi, chrome.loadTimes, chrome.runtime
//! per docs/research/window-chrome-object.md field table.

/// JS shim that installs the complete window.chrome object.
/// Must be called AFTER wrapNative is installed (uses __iv8__.wrapNative internally).
pub const WINDOW_CHROME_SHIM: &str = r#"
(function(wrapNative) {
    'use strict';

    // Property descriptor matching real Chrome
    Object.defineProperty(window, 'chrome', {
        writable: true,
        enumerable: true,
        configurable: false,
        value: {}
    });

    // --- chrome.app ---
    chrome.app = {
        isInstalled: false,
        InstallState: {
            DISABLED: "disabled",
            INSTALLED: "installed",
            NOT_INSTALLED: "not_installed"
        },
        RunningState: {
            CANNOT_RUN: "cannot_run",
            READY_TO_RUN: "ready_to_run",
            RUNNING: "running"
        },
        getDetails: wrapNative(function getDetails() { return null; }, 'getDetails'),
        getIsInstalled: wrapNative(function getIsInstalled() { return false; }, 'getIsInstalled'),
        runningState: wrapNative(function runningState() { return "cannot_run"; }, 'runningState')
    };

    // --- chrome.csi ---
    var _navigationStart = Date.now();
    chrome.csi = wrapNative(function csi() {
        return {
            onloadT: _navigationStart + 300,
            startE: _navigationStart,
            pageT: Date.now() - _navigationStart,
            tran: 15
        };
    }, 'csi');

    // --- chrome.loadTimes ---
    var _requestTime = _navigationStart / 1000;
    chrome.loadTimes = wrapNative(function loadTimes() {
        return {
            requestTime: _requestTime,
            startLoadTime: _requestTime,
            commitLoadTime: _requestTime + 0.1,
            finishDocumentLoadTime: _requestTime + 0.3,
            finishLoadTime: _requestTime + 0.5,
            firstPaintTime: _requestTime + 0.2,
            firstPaintAfterLoadTime: 0,
            navigationType: "Other",
            wasFetchedViaSpdy: true,
            wasNpnNegotiated: true,
            npnNegotiatedProtocol: "h2",
            wasAlternateProtocolAvailable: false,
            connectionInfo: "h2"
        };
    }, 'loadTimes');

    // --- chrome.runtime ---
    chrome.runtime = {
        OnInstalledReason: {
            CHROME_UPDATE: "chrome_update",
            INSTALL: "install",
            SHARED_MODULE_UPDATE: "shared_module_update",
            UPDATE: "update"
        },
        OnRestartRequiredReason: {
            APP_UPDATE: "app_update",
            OS_UPDATE: "os_update",
            PERIODIC: "periodic"
        },
        PlatformArch: {
            ARM: "arm",
            ARM64: "arm64",
            MIPS: "mips",
            MIPS64: "mips64",
            X86_32: "x86-32",
            X86_64: "x86-64"
        },
        PlatformNaclArch: {
            ARM: "arm",
            ARM64: "arm64",
            MIPS: "mips",
            MIPS64: "mips64",
            X86_32: "x86-32",
            X86_64: "x86-64"
        },
        PlatformOs: {
            ANDROID: "android",
            CROS: "cros",
            LINUX: "linux",
            MAC: "mac",
            OPENBSD: "openbsd",
            WIN: "win"
        },
        RequestUpdateCheckStatus: {
            NO_UPDATE: "no_update",
            THROTTLED: "throttled",
            UPDATE_AVAILABLE: "update_available"
        },
        get id() { return undefined; }
    };

    // --- chrome.runtime.connect ---
    var _isValidExtensionID = function(str) {
        return typeof str === 'string' && str.length === 32 && /^[a-p]+$/.test(str.toLowerCase());
    };

    chrome.runtime.connect = wrapNative(function connect() {
        var args = Array.prototype.slice.call(arguments);
        var extensionId = args[0];
        var preamble = 'Error in invocation of runtime.connect(optional string extensionId, optional object connectInfo): ';

        if (args.length === 0 || typeof extensionId !== 'string') {
            throw new TypeError(preamble + 'chrome.runtime.connect() called from a webpage must specify an Extension ID (string) for its first argument.');
        }
        if (!_isValidExtensionID(extensionId)) {
            throw new TypeError(preamble + "Invalid extension id: '" + extensionId + "'");
        }
        // Valid extension ID — return a disconnected Port-like object
        return { name: '', disconnect: function(){}, onDisconnect: { addListener: function(){} }, onMessage: { addListener: function(){} }, postMessage: function(){} };
    }, 'connect');

    // --- chrome.runtime.sendMessage ---
    chrome.runtime.sendMessage = wrapNative(function sendMessage() {
        var args = Array.prototype.slice.call(arguments);
        var preamble = 'Error in invocation of runtime.sendMessage(optional string extensionId, any message, optional object options, optional function responseCallback): ';

        if (args.length === 0 || args.length > 4) {
            throw new TypeError(preamble + 'No matching signature.');
        }
        var extensionId = args[0];
        if (typeof extensionId !== 'string') {
            throw new TypeError(preamble + 'chrome.runtime.sendMessage() called from a webpage must specify an Extension ID (string) for its first argument.');
        }
        if (!_isValidExtensionID(extensionId)) {
            throw new TypeError(preamble + "Invalid extension id: '" + extensionId + "'");
        }
        return undefined;
    }, 'sendMessage');
})
"#;

/// Get the full install script for window.chrome.
/// Requires `__iv8__.wrapNative` to already be installed.
pub fn get_install_script(iv8_obj_name: &str) -> String {
    format!("{}({}.wrapNative)", WINDOW_CHROME_SHIM, iv8_obj_name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use iv8_core::{EmbeddedV8Kernel, EvalOpts, KernelConfig, RustValue};

    fn make_kernel_with_chrome() -> EmbeddedV8Kernel {
        let mut kernel = EmbeddedV8Kernel::new(KernelConfig::default()).unwrap();
        // Install __iv8__ + wrapNative first
        kernel.eval("var __iv8__ = {}", EvalOpts::default()).unwrap();
        let wrap_script = crate::wrap_native::get_install_script("__iv8__");
        kernel.eval(&wrap_script, EvalOpts::default()).unwrap();
        // Install window.chrome
        let chrome_script = get_install_script("__iv8__");
        kernel.eval(&chrome_script, EvalOpts::default()).unwrap();
        kernel
    }

    #[test]
    fn chrome_exists() {
        let mut k = make_kernel_with_chrome();
        assert_eq!(k.eval_to_rust_value("typeof window.chrome"), RustValue::String("object".into()));
    }

    #[test]
    fn chrome_runtime_exists() {
        let mut k = make_kernel_with_chrome();
        assert_eq!(k.eval_to_rust_value("typeof chrome.runtime"), RustValue::String("object".into()));
    }

    #[test]
    fn chrome_runtime_id_undefined() {
        let mut k = make_kernel_with_chrome();
        assert_eq!(k.eval_to_rust_value("chrome.runtime.id"), RustValue::Null);
    }

    #[test]
    fn chrome_runtime_on_installed_reason() {
        let mut k = make_kernel_with_chrome();
        assert_eq!(
            k.eval_to_rust_value("chrome.runtime.OnInstalledReason.INSTALL"),
            RustValue::String("install".into())
        );
    }

    #[test]
    fn chrome_load_times_callable() {
        let mut k = make_kernel_with_chrome();
        let result = k.eval_to_rust_value("typeof chrome.loadTimes()");
        assert_eq!(result, RustValue::String("object".into()));
    }

    #[test]
    fn chrome_load_times_has_13_keys() {
        let mut k = make_kernel_with_chrome();
        let result = k.eval_to_rust_value("Object.keys(chrome.loadTimes()).length");
        assert_eq!(result, RustValue::Int(13));
    }

    #[test]
    fn chrome_load_times_request_time_positive() {
        let mut k = make_kernel_with_chrome();
        let result = k.eval_to_rust_value("chrome.loadTimes().requestTime > 0");
        assert_eq!(result, RustValue::Bool(true));
    }

    #[test]
    fn chrome_csi_callable() {
        let mut k = make_kernel_with_chrome();
        let result = k.eval_to_rust_value("typeof chrome.csi()");
        assert_eq!(result, RustValue::String("object".into()));
    }

    #[test]
    fn chrome_csi_has_4_keys() {
        let mut k = make_kernel_with_chrome();
        let result = k.eval_to_rust_value("Object.keys(chrome.csi()).length");
        assert_eq!(result, RustValue::Int(4));
    }

    #[test]
    fn chrome_app_is_installed_false() {
        let mut k = make_kernel_with_chrome();
        assert_eq!(k.eval_to_rust_value("chrome.app.isInstalled"), RustValue::Bool(false));
    }

    #[test]
    fn chrome_load_times_tostring_native() {
        let mut k = make_kernel_with_chrome();
        let result = k.eval_to_rust_value("chrome.loadTimes.toString()");
        assert_eq!(result, RustValue::String("function loadTimes() { [native code] }".into()));
    }

    #[test]
    fn chrome_csi_tostring_native() {
        let mut k = make_kernel_with_chrome();
        let result = k.eval_to_rust_value("chrome.csi.toString()");
        assert_eq!(result, RustValue::String("function csi() { [native code] }".into()));
    }

    #[test]
    fn chrome_runtime_connect_no_arg_throws() {
        let mut k = make_kernel_with_chrome();
        let err = k.eval("chrome.runtime.connect()", EvalOpts::default()).unwrap_err();
        match err {
            iv8_core::IV8Error::Js { name, message, .. } => {
                assert_eq!(name, "TypeError");
                assert!(message.contains("Extension ID"), "msg: {}", message);
            }
            other => panic!("expected TypeError, got: {:?}", other),
        }
    }

    #[test]
    fn chrome_runtime_connect_invalid_id_throws() {
        let mut k = make_kernel_with_chrome();
        let err = k.eval("chrome.runtime.connect('invalid')", EvalOpts::default()).unwrap_err();
        match err {
            iv8_core::IV8Error::Js { name, message, .. } => {
                assert_eq!(name, "TypeError");
                assert!(message.contains("Invalid extension id"), "msg: {}", message);
            }
            other => panic!("expected TypeError, got: {:?}", other),
        }
    }

    #[test]
    fn chrome_runtime_connect_valid_id_returns_port() {
        let mut k = make_kernel_with_chrome();
        // 32 chars [a-p]
        let result = k.eval_to_rust_value("typeof chrome.runtime.connect('aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa'.slice(0,32))");
        assert_eq!(result, RustValue::String("object".into()));
    }
}
