//! XMLHttpRequest stub: synchronous XHR that reads from ResourceBundle.
//!
//! In v0.1 (offline mode), XHR only supports synchronous requests against
//! pre-registered resources. Async XHR is stubbed (fires onload after eventLoop advance).
//!
//! Installed as a JS class via eval shim that delegates to native __xhr_send__.

use crate::state::RuntimeState;

/// Install XMLHttpRequest as a JS class via shim.
pub fn install_xhr(scope: &v8::PinScope<'_, '_>, global: v8::Local<v8::Object>) {
    // Install native __xhr_send__(method, url) → {status, responseText} or null
    let send_tmpl = v8::FunctionTemplate::builder_raw(xhr_send_callback).build(scope);
    let send_fn = send_tmpl.get_function(scope).expect("fn");
    let key = v8::String::new(scope, "__xhr_send__").expect("key");
    global.define_own_property(scope, key.into(), send_fn.into(), v8::PropertyAttribute::DONT_ENUM);
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
        this.onreadystatechange = null;
        this.onload = null;
        this.onerror = null;
        this._method = 'GET';
        this._url = '';
        this._async = true;
        this._headers = {};
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

        function doSend() {
            var result = globalThis.__xhr_send__(self._method, self._url);
            if (result) {
                self.status = result.status;
                self.statusText = result.status === 200 ? 'OK' : '';
                self.responseText = result.responseText;
                self.response = result.responseText;
                self._responseHeaders = result.headers || {};
                self.readyState = 4; // DONE
                if (self.onreadystatechange) self.onreadystatechange();
                if (self.onload) self.onload();
            } else {
                // Network error
                self.status = 0;
                self.readyState = 4;
                if (self.onreadystatechange) self.onreadystatechange();
                if (self.onerror) self.onerror(new Error('NetworkError'));
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
        this.readyState = 0;
    };

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
            handler_result.map(|(status, body)| crate::network::bundle::Resource::new(body, status, None))
        } else {
            resource_ref
        };

        match resource {
            Some(res) => {
                let obj = v8::Object::new(scope);

                let status_key = v8::String::new(scope, "status").expect("key");
                obj.set(scope, status_key.into(), v8::Integer::new(scope, res.status as i32).into());

                let text_key = v8::String::new(scope, "responseText").expect("key");
                let body_str = String::from_utf8_lossy(&res.body);
                let text_val = v8::String::new(scope, &body_str).expect("val");
                obj.set(scope, text_key.into(), text_val.into());

                // Headers as object
                let headers_obj = v8::Object::new(scope);
                for (k, v) in &res.headers {
                    if let (Some(hk), Some(hv)) = (v8::String::new(scope, k), v8::String::new(scope, v)) {
                        headers_obj.set(scope, hk.into(), hv.into());
                    }
                }
                let headers_key = v8::String::new(scope, "headers").expect("key");
                obj.set(scope, headers_key.into(), headers_obj.into());

                rv.set(obj.into());
            }
            None => {
                rv.set(v8::null(scope).into());
            }
        }
    }));
}
