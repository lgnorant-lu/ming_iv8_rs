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
    // Real Chrome webpage: chrome.runtime is non-enumerable; methods are own
    // data properties on the runtime object (not on Object.prototype).
    var _isValidExtensionID = function(str) {
        return typeof str === 'string' && str.length === 32 && /^[a-p]+$/.test(str.toLowerCase());
    };
    var _runtimeConnect = wrapNative(function connect() {
        var args = Array.prototype.slice.call(arguments);
        var extensionId = args[0];
        var preamble = 'Error in invocation of runtime.connect(optional string extensionId, optional object connectInfo): ';
        if (args.length === 0 || typeof extensionId !== 'string') {
            throw new TypeError(preamble + 'chrome.runtime.connect() called from a webpage must specify an Extension ID (string) for its first argument.');
        }
        if (!_isValidExtensionID(extensionId)) {
            throw new TypeError(preamble + "Invalid extension id: '" + extensionId + "'");
        }
        return { name: '', disconnect: function(){}, onDisconnect: { addListener: function(){} }, onMessage: { addListener: function(){} }, postMessage: function(){} };
    }, 'connect');
    var _runtimeSendMessage = wrapNative(function sendMessage() {
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
    var _runtime = {
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
        connect: _runtimeConnect,
        sendMessage: _runtimeSendMessage
    };
    Object.defineProperty(_runtime, 'id', {
        get: function() { return undefined; },
        enumerable: true,
        configurable: true
    });
    Object.defineProperty(chrome, 'runtime', {
        enumerable: false,
        configurable: true,
        value: _runtime
    });
})