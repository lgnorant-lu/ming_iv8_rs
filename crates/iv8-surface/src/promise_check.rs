use v8::Isolate;

pub fn check_receiver(
    scope: &v8::PinScope<'_, '_>,
    info: &v8::FunctionCallbackInfo,
    iface_name: &str,
) -> bool {
    let args = v8::FunctionCallbackArguments::from_function_callback_info(info);
    let this = args.this();
    let ctx = scope.get_current_context();
    let global = ctx.global(scope);
    let iface_str = match v8::String::new(scope, iface_name) {
        Some(s) => s,
        None => {
            throw_illegal_invocation(scope);
            return false;
        }
    };
    let Some(ctor_val) = global.get(scope, iface_str.into()) else {
        throw_illegal_invocation(scope);
        return false;
    };
    if !ctor_val.is_function() {
        throw_illegal_invocation(scope);
        return false;
    }
    let ctor = unsafe { v8::Local::<v8::Function>::cast_unchecked(ctor_val) };
    let proto_key = match v8::String::new(scope, "prototype") {
        Some(s) => s,
        None => {
            throw_illegal_invocation(scope);
            return false;
        }
    };
    let Some(proto_val) = ctor.get(scope, proto_key.into()) else {
        throw_illegal_invocation(scope);
        return false;
    };
    if !proto_val.is_object() || proto_val.is_null_or_undefined() {
        throw_illegal_invocation(scope);
        return false;
    }
    let proto = unsafe { v8::Local::<v8::Object>::cast_unchecked(proto_val) };
    if this.strict_equals(proto.into()) {
        throw_illegal_invocation(scope);
        return false;
    }
    let mut current: v8::Local<v8::Value> = this.into();
    for _ in 0..20usize {
        let Some(cur_obj) = current.to_object(scope) else { break; };
        let Some(parent) = cur_obj.get_prototype(scope) else { break; };
        if parent.is_null_or_undefined() || !parent.is_object() {
            break;
        }
        if parent.strict_equals(proto.into()) {
            return true;
        }
        current = parent;
    }
    throw_illegal_invocation(scope);
    false
}

fn throw_illegal_invocation(scope: &v8::PinScope<'_, '_>) {
    let msg = v8::String::new(scope, "Illegal invocation").unwrap();
    let exc = v8::Exception::type_error(scope, msg);
    scope.throw_exception(exc);
}

pub fn check_receiver_promise(
    scope: &v8::PinScope<'_, '_>,
    info: *const v8::FunctionCallbackInfo,
    iface_name: &str,
) -> bool {
    let info_ref = unsafe { &*info };
    let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
    let this = args.this();
    let ctx = scope.get_current_context();
    let global = ctx.global(scope);
    let iface_str = match v8::String::new(scope, iface_name) {
        Some(s) => s,
        None => {
            reject_promise(scope, info_ref);
            return false;
        }
    };
    let Some(ctor_val) = global.get(scope, iface_str.into()) else {
        reject_promise(scope, info_ref);
        return false;
    };
    if !ctor_val.is_function() {
        reject_promise(scope, info_ref);
        return false;
    }
    let ctor = unsafe { v8::Local::<v8::Function>::cast_unchecked(ctor_val) };
    let proto_key = match v8::String::new(scope, "prototype") {
        Some(s) => s,
        None => {
            reject_promise(scope, info_ref);
            return false;
        }
    };
    let Some(proto_val) = ctor.get(scope, proto_key.into()) else {
        reject_promise(scope, info_ref);
        return false;
    };
    if !proto_val.is_object() || proto_val.is_null_or_undefined() {
        reject_promise(scope, info_ref);
        return false;
    }
    let proto = unsafe { v8::Local::<v8::Object>::cast_unchecked(proto_val) };
    if this.strict_equals(proto.into()) {
        reject_promise(scope, info_ref);
        return false;
    }
    let mut current: v8::Local<v8::Value> = this.into();
    let mut found = false;
    for _ in 0..20usize {
        let Some(cur_obj) = current.to_object(scope) else { break; };
        let Some(parent) = cur_obj.get_prototype(scope) else { break; };
        if parent.is_null_or_undefined() || !parent.is_object() { break; }
        if parent.strict_equals(proto.into()) { found = true; break; }
        current = parent;
    }
    if !found {
        reject_promise(scope, info_ref);
        return false;
    }
    true
}

fn reject_promise(scope: &v8::PinScope<'_, '_>, info_ref: &v8::FunctionCallbackInfo) {
    let msg = v8::String::new(scope, "Illegal invocation").unwrap();
    let exc = v8::Exception::type_error(scope, msg);
    if let Some(resolver) = v8::PromiseResolver::new(scope) {
        let _ = resolver.reject(scope, exc.into());
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        rv.set(resolver.get_promise(scope).into());
    }
}
