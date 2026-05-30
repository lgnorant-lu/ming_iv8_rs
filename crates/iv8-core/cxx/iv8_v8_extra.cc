// Copyright 2026 lgnorant-lu. All rights reserved. MIT license.
//
// iv8-rs extra V8 bindings.
//
// Adds two ObjectTemplate methods that are missing from upstream rusty_v8
// (denoland/rusty_v8) but exist in V8's C++ API:
//
//   - MarkAsUndetectable: enables [[IsHTMLDDA]] semantics (typeof === 'undefined')
//   - SetCallAsFunctionHandler: enables calling object instances as functions
//
// These wrappers use the same naming convention as upstream binding.cc
// (v8__ObjectTemplate__*) and the same ptr_to_local idiom for ABI compatibility
// with rusty_v8's existing Rust extern "C" declarations.
//
// The actual V8 implementations of MarkAsUndetectable and SetCallAsFunctionHandler
// are already compiled into the prebuilt libv8 static library shipped by upstream
// rusty_v8 (they are part of V8's source tree). We only need to provide the
// extern "C" wrappers.

#include "v8.h"

namespace {

// Mirrors `support::ptr_to_local` from rusty_v8's src/support.h.
//
// rusty_v8 represents `v8::Local<T>` on the Rust side as a `*const T` raw
// pointer. The C++ `v8::Local<T>` is layout-compatible with `T*` (it is
// effectively a wrapper around a single pointer). To convert between them
// we take the address of the pointer variable and reinterpret it as a
// pointer to `v8::Local<T>`, then dereference.
template <class T>
inline v8::Local<T> ptr_to_local(const T* ptr) {
  static_assert(sizeof(v8::Local<T>) == sizeof(T*),
                "v8::Local<T> must be layout-compatible with T*");
  return *reinterpret_cast<const v8::Local<T>*>(&ptr);
}

}  // namespace

extern "C" {

void v8__ObjectTemplate__MarkAsUndetectable(const v8::ObjectTemplate& self) {
  ptr_to_local(&self)->MarkAsUndetectable();
}

void v8__ObjectTemplate__SetCallAsFunctionHandler(
    const v8::ObjectTemplate& self,
    v8::FunctionCallback callback,
    const v8::Value* data_or_null) {
  ptr_to_local(&self)->SetCallAsFunctionHandler(
      callback, ptr_to_local(data_or_null));
}

}  // extern "C"
