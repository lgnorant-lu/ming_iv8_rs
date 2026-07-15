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

// Chromium/d8 path: load ICU from an external icudtl.dat file before
// V8::Initialize. Returns 1 on success, 0 on failure.
// See v8::V8::InitializeICU in v8-initialization.h.
int iv8__V8__InitializeICU(const char* icu_data_file) {
  if (icu_data_file == nullptr || icu_data_file[0] == '\0') {
    return v8::V8::InitializeICU(nullptr) ? 1 : 0;
  }
  return v8::V8::InitializeICU(icu_data_file) ? 1 : 0;
}

// Match Chromium icu_util.cc after udata_setCommonData:
//   udata_setFileAccess(UDATA_ONLY_PACKAGES, &err);
// Without this, ICU may still search filesystem and fail lookups even when
// common data was registered. Returns 1 if err==U_ZERO_ERROR.
//
// UDataFileAccess enum (unicode/udata.h):
//   UDATA_FILES_FIRST=0, UDATA_ONLY_PACKAGES=1, UDATA_PACKAGES_FIRST=2,
//   UDATA_NO_FILES=3, UDATA_DEFAULT_ACCESS=0
int iv8__ICU__SetFileAccessOnlyPackages() {
  // rusty_v8 / ICU 77 exports versioned symbol udata_setFileAccess_77
  // (see rusty_v8.lib dumpbin). Chromium icu_util.cc calls the unversioned
  // name via headers; we must use the versioned export that is actually linked.
  // UDataFileAccess: UDATA_ONLY_PACKAGES = 1
  // UDATA_NO_FILES=3 is stronger than ONLY_PACKAGES=1 and matches
  // "never try to load ICU data from files" intent in Chromium after package load.
  extern void udata_setFileAccess_77(int access, int* status);
  int err = 0;
  // UDATA_ONLY_PACKAGES=1 (Chromium icu_util.cc). UDATA_NO_FILES=3 can hang
  // when package is incomplete; ONLY_PACKAGES matches Chromium after load.
  udata_setFileAccess_77(1, &err);
  return err == 0 ? 1 : 0;
}

}  // extern "C"
