//! XMLHttpRequest: supports sync/async requests via ResourceBundle or
//! Python network_handler callback.
//!
//! - ResourceBundle hit: immediate response (fast path).
//! - network_handler callback: used when URL not in bundle.
//! - Neither: status=0, statusText='', readyState=4, fire onerror.
//! - Async mode: defers response via setTimeout(0) so onreadystatechange
//!   fires in correct order: 1 (OPENED) → 2 (HEADERS_RECEIVED) → 3 (LOADING) → 4 (DONE).
//! - Sync mode: resolves immediately.
//! - timeout: if set and exceeded, fires ontimeout.
//!
//! Installed as a JS class via eval shim that delegates to native __xhr_send__.

use crate::state::RuntimeState;

/// Install XMLHttpRequest as a JS class via shim.
/// Installs both the native __xhr_send__ callback and the XHR_SHIM_JS
/// that creates the JS-level XMLHttpRequest constructor.
pub fn install_xhr(scope: &v8::PinScope<'_, '_>, global: v8::Local<v8::Object>) {
    // Install native __xhr_send__(method, url) → {status, responseText} or null
    let send_tmpl = v8::FunctionTemplate::builder_raw(xhr_send_callback).build(scope);
    let send_fn = crate::v8_utils::v8_fn(scope, &send_tmpl);
    let key = crate::v8_utils::v8_string(scope, "__xhr_send__");
    global.define_own_property(
        scope,
        key.into(),
        send_fn.into(),
        v8::PropertyAttribute::DONT_ENUM,
    );

    // Eval XHR_SHIM_JS to create the JS-level XMLHttpRequest constructor.
    // This is the single source of truth for XHR_SHIM_JS eval —
    // install_undetect_shims skips XHR_SHIM_JS when skip_native_behaviors=true.
    let script = v8::Script::compile(
        scope,
        crate::v8_utils::v8_string(scope, XHR_SHIM_JS),
        None,
    )
    .unwrap();
    let _ = script.run(scope);
}

/// JS shim that creates the XMLHttpRequest class.
pub const XHR_SHIM_JS: &str = r#"
(function() {
    function XMLHttpRequest() {
        this.readyState = 0; // UNSENT
        this.status = 0;
        this.statusText = '';
        this.responseText = '';
        this.response = '';
        this.responseType = '';
        this.responseURL = '';
        this.timeout = 0;
        this.withCredentials = false;
        this.onreadystatechange = null;
        this._method = 'GET';
        this._url = '';
        this._async = true;
        this._headers = {};
        this._responseHeaders = null;
        this._aborted = false;
        this._timedOut = false;
        this._timeoutTimer = undefined;
        // XMLHttpRequestUpload stub (real browser has EventTarget subclass)
        this.upload = {
            addEventListener: function() {},
            removeEventListener: function() {},
            dispatchEvent: function() { return true; },
        };
        // on* event handlers: use defineProperty to create own data properties
        // because codegen XMLHttpRequestEventTarget.prototype has accessor-only
        // getters (no setter), so `this.onload = null` silently fails.
        var onProps = ['onload','onerror','onloadstart','onprogress','onabort','ontimeout','onloadend'];
        for (var i = 0; i < onProps.length; i++) {
            Object.defineProperty(this, onProps[i], {value: null, writable: true, enumerable: true, configurable: true});
        }
    }

    XMLHttpRequest.UNSENT = 0;
    XMLHttpRequest.OPENED = 1;
    XMLHttpRequest.HEADERS_RECEIVED = 2;
    XMLHttpRequest.LOADING = 3;
    XMLHttpRequest.DONE = 4;

    XMLHttpRequest.prototype.open = function(method, url, async) {
        this._method = method || 'GET';
        this._url = url || '';
        this._async = async !== false; // default true
        this._aborted = false;
        this._timedOut = false;
        this._timeoutTimer = undefined;
        this.status = 0;
        this.statusText = '';
        this.responseText = '';
        this.response = '';
        this.responseURL = '';
        this._responseHeaders = null;
        this.readyState = 1; // OPENED
        if (this.onreadystatechange) this.onreadystatechange();
    };

    XMLHttpRequest.prototype.setRequestHeader = function(name, value) {
        this._headers[name] = value;
    };

    XMLHttpRequest.prototype.getResponseHeader = function(name) {
        return this._responseHeaders ? (this._responseHeaders[name.toLowerCase()] || null) : null;
    };

    XMLHttpRequest.prototype.getAllResponseHeaders = function() {
        if (!this._responseHeaders) return '';
        var result = '';
        for (var k in this._responseHeaders) {
            result += k + ': ' + this._responseHeaders[k] + '\r\n';
        }
        return result;
    };

    XMLHttpRequest.prototype.send = function(body) {
        var self = this;
        self._body = body || '';

        // Record in netLog (use 'in' check — __iv8__ is undetectable/falsy)
        if ('__iv8__' in globalThis && globalThis.__iv8__.netLog) {
            var headerPairs = [];
            for (var h in this._headers) {
                headerPairs.push([h.toLowerCase(), this._headers[h]]);
            }
            globalThis.__iv8__.netLog.entries.push({
                method: this._method,
                url: this._url,
                headers: headerPairs,
                body: body || '',
            });
        }

        function fireReadyState(rs) {
            self.readyState = rs;
            if (self.onreadystatechange) self.onreadystatechange();
        }

        function fireProgress(loaded, total) {
            var event = {
                type: 'progress',
                loaded: loaded,
                total: total || 0,
                lengthComputable: !!total,
            };
            if (self.onprogress) self.onprogress(event);
        }

        function done(success, errorEvent) {
            if (self._aborted) return;
            fireReadyState(4); // DONE
            if (success) {
                if (self.onload) self.onload();
            } else {
                if (self.onerror) self.onerror(errorEvent || new Error('NetworkError'));
            }
            if (self.onloadend) self.onloadend();
            // Clear timeout timer if we set one
            if (self._timeoutTimer !== undefined) {
                clearTimeout(self._timeoutTimer);
                self._timeoutTimer = undefined;
            }
        }

        function doSend() {
            if (self._aborted) return;

            if (self.onloadstart) self.onloadstart();

            // Set up timeout for async requests
            if (self._async && self.timeout > 0) {
                self._timeoutTimer = setTimeout(function() {
                    if (self.readyState >= 4) return; // already done
                    self._timedOut = true;
                    self.status = 0;
                    self.statusText = '';
                    fireReadyState(4);
                    if (self.ontimeout) self.ontimeout(new Error('Timeout'));
                    if (self.onloadend) self.onloadend();
                }, self.timeout);
            }

            var result = globalThis.__xhr_send__(self._method, self._url);
            if (self._timedOut) return; // timeout fired before we got result

            if (result) {
                self.status = result.status;
                self.statusText = result.status === 200 ? 'OK' : (result.statusText || '');
                self._responseHeaders = result.headers || {};
                self.responseURL = self._url;

                // Process Set-Cookie headers from response
                try {
                    var hdrs = self._responseHeaders;
                    for (var h in hdrs) {
                        if (h.toLowerCase() === 'set-cookie') {
                            var cookieStr = hdrs[h];
                            // Set-Cookie may be a single string or an array
                            var cookies = Array.isArray(cookieStr) ? cookieStr : [cookieStr];
                            for (var ci = 0; ci < cookies.length; ci++) {
                                document.cookie = cookies[ci];
                            }
                        }
                    }
                } catch(e) {}

                fireReadyState(2); // HEADERS_RECEIVED

                self.responseText = result.responseText;
                self.response = result.responseText;

                fireProgress(result.responseText.length, 0);
                fireReadyState(3); // LOADING

                done(true);
            } else {
                // Neither ResourceBundle nor network_handler produced content
                self.status = 0;
                self.statusText = '';
                self.responseText = '';
                self.response = '';
                done(false, new Error('NetworkError'));
            }
        }

        if (this._async) {
            // Async mode: schedule via setTimeout(0) — fires on eventLoop advance
            setTimeout(doSend, 0);
        } else {
            // Sync mode: execute immediately
            doSend();
        }
    };

    XMLHttpRequest.prototype.abort = function() {
        this._aborted = true;
        this.readyState = 0;
        if (this._timeoutTimer !== undefined) {
            clearTimeout(this._timeoutTimer);
            this._timeoutTimer = undefined;
        }
        if (this.onabort) this.onabort();
        if (this.onloadend) this.onloadend();
    };

    XMLHttpRequest.prototype.overrideMimeType = function(mime) {
        this._overrideMimeType = mime;
    };

    // Link prototype chain to codegen XMLHttpRequestEventTarget.prototype
    // so addEventListener/removeEventListener/dispatchEvent are inherited
    // via EventTarget. This is re-applied on every eval of XHR_SHIM_JS
    // (install_undetect_shims re-evals it, overwriting the constructor).
    if (typeof XMLHttpRequestEventTarget !== 'undefined' && XMLHttpRequestEventTarget.prototype) {
        Object.setPrototypeOf(XMLHttpRequest.prototype, XMLHttpRequestEventTarget.prototype);
    }

    // P0-BT fix: toString must return native code for all shim methods
    var _nativeToStr = function(name) {
        var fn = function() { return 'function ' + name + '() { [native code] }'; };
        fn.toString = function() { return 'function toString() { [native code] }'; };
        return fn;
    };
    var _xhrMethods = ['open','setRequestHeader','send','abort','getResponseHeader','getAllResponseHeaders','overrideMimeType'];
    for (var _i = 0; _i < _xhrMethods.length; _i++) {
        var _m = _xhrMethods[_i];
        if (XMLHttpRequest.prototype[_m]) {
            XMLHttpRequest.prototype[_m].toString = _nativeToStr(_m);
        }
    }

    globalThis.XMLHttpRequest = XMLHttpRequest;
})();
"#;

/// __xhr_send__(method, url) → {status, responseText, headers} or null
unsafe extern "C" fn xhr_send_callback(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);

        if args.length() < 2 {
            rv.set(v8::null(scope).into());
            return;
        }

        let _method = args.get(0).to_rust_string_lossy(scope);
        let url = args.get(1).to_rust_string_lossy(scope);

        let isolate: &v8::Isolate = &*scope;
        let state = RuntimeState::get(isolate);
        let resource_ref = {
            let bundle = state.resource_bundle.borrow();
            bundle.get(&url).cloned()
        };

        // If not in bundle, try Python network handler
        let resource = if resource_ref.is_none() {
            let handler_result = {
                let handler = state.network_handler.borrow();
                if let Some(ref h) = *handler {
                    h(&url, &_method)
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
                let obj = v8::Object::new(scope);

                let status_key = crate::v8_utils::v8_string(scope, "status");
                obj.set(
                    scope,
                    status_key.into(),
                    v8::Integer::new(scope, res.status as i32).into(),
                );

                let text_key = crate::v8_utils::v8_string(scope, "responseText");
                let body_str = String::from_utf8_lossy(&res.body);
                let text_val = crate::v8_utils::v8_string(scope, &body_str);
                obj.set(scope, text_key.into(), text_val.into());

                // statusText: "OK" for 200, empty otherwise (network_handler
                // does not provide status text, so we derive it)
                let status_text = if res.status == 200 { "OK" } else { "" };
                let status_text_key = crate::v8_utils::v8_string(scope, "statusText");
                let status_text_val = crate::v8_utils::v8_string(scope, status_text);
                obj.set(scope, status_text_key.into(), status_text_val.into());

                // Headers as object
                let headers_obj = v8::Object::new(scope);
                for (k, v) in &res.headers {
                    if let (Some(hk), Some(hv)) =
                        (v8::String::new(scope, k), v8::String::new(scope, v))
                    {
                        headers_obj.set(scope, hk.into(), hv.into());
                    }
                }
                let headers_key = crate::v8_utils::v8_string(scope, "headers");
                obj.set(scope, headers_key.into(), headers_obj.into());

                rv.set(obj.into());
            }
            None => {
                rv.set(v8::null(scope).into());
            }
        }
    }));
}
