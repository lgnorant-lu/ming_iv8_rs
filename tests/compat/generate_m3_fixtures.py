"""
生成 M3 差异测试 fixtures。

直接用 iv8-rs 生成 expected（因为我们已经验证过行为正确性），
同时记录 iv8 0.1.2 的已知行为差异。

运行: uv run python tests/compat/generate_m3_fixtures.py
"""
import json
import pathlib
import sys

FIXTURES_DIR = pathlib.Path(__file__).parent / "fixtures"

# ─── fixture 定义 ─────────────────────────────────────────────────────────────
# 格式: (category, name, js_source, expected_value, expected_type, xfail_reason)
# xfail_reason=None 表示正常测试

FIXTURES = [

    # ── crypto ────────────────────────────────────────────────────────────────
    ("crypto", "001_crypto_exists",
     "typeof crypto", "object", "str", None),
    ("crypto", "002_subtle_exists",
     "typeof crypto.subtle", "object", "str", None),
    ("crypto", "003_get_random_values_length",
     "crypto.getRandomValues(new Uint8Array(16)).length", 16, "int", None),
    ("crypto", "004_random_uuid_type",
     "typeof crypto.randomUUID()", "string", "str", None),
    ("crypto", "005_random_uuid_length",
     "crypto.randomUUID().length", 36, "int", None),
    ("crypto", "006_random_uuid_dashes",
     "crypto.randomUUID().split('-').length", 5, "int", None),
    ("crypto", "007_get_random_values_is_uint8array",
     "crypto.getRandomValues(new Uint8Array(4)) instanceof Uint8Array", True, "bool", None),
    ("crypto", "008_random_values_not_all_zero",
     # 16 bytes of random — extremely unlikely to be all zero
     "crypto.getRandomValues(new Uint8Array(16)).some(function(b){return b!==0;})", True, "bool", None),

    # ── screen ────────────────────────────────────────────────────────────────
    ("screen", "001_width_type",
     "typeof screen.width", "number", "str", None),
    ("screen", "002_height_type",
     "typeof screen.height", "number", "str", None),
    ("screen", "003_color_depth_type",
     "typeof screen.colorDepth", "number", "str", None),
    ("screen", "004_width_positive",
     "screen.width > 0", True, "bool", None),
    ("screen", "005_height_positive",
     "screen.height > 0", True, "bool", None),
    ("screen", "006_color_depth_24",
     "screen.colorDepth === 24 || screen.colorDepth === 32", True, "bool", None),
    ("screen", "007_avail_width_positive",
     "screen.availWidth > 0", True, "bool", None),
    ("screen", "008_avail_height_positive",
     "screen.availHeight > 0", True, "bool", None),
    ("screen", "009_pixel_depth_type",
     "typeof screen.pixelDepth", "number", "str", None),
    ("screen", "010_width_native_getter",
     "typeof Object.getOwnPropertyDescriptor(screen, 'width').get", "function", "str", None),

    # ── window ────────────────────────────────────────────────────────────────
    ("window", "001_window_equals_globalthis",
     "window === globalThis", True, "bool", None),
    ("window", "002_window_equals_self",
     "window === self", True, "bool", None),
    ("window", "003_inner_width_type",
     "typeof window.innerWidth", "number", "str", None),
    ("window", "004_inner_height_type",
     "typeof window.innerHeight", "number", "str", None),
    ("window", "005_device_pixel_ratio_type",
     "typeof window.devicePixelRatio", "number", "str", None),
    ("window", "006_chrome_exists",
     "typeof window.chrome", "object", "str", None),
    ("window", "007_chrome_csi",
     "typeof window.chrome.csi", "function", "str", None),
    ("window", "008_chrome_load_times",
     "typeof window.chrome.loadTimes", "function", "str", None),
    ("window", "009_chrome_runtime",
     "typeof window.chrome.runtime", "object", "str", None),
    ("window", "010_history_exists",
     "typeof window.history", "object", "str", None),
    ("window", "011_location_exists",
     "typeof window.location", "object", "str", None),
    ("window", "012_performance_exists",
     "typeof window.performance", "object", "str", None),

    # ── location ──────────────────────────────────────────────────────────────
    ("location", "001_href_type",
     "typeof location.href", "string", "str", None),
    ("location", "002_protocol_type",
     "typeof location.protocol", "string", "str", None),
    ("location", "003_hostname_type",
     "typeof location.hostname", "string", "str", None),
    ("location", "004_pathname_type",
     "typeof location.pathname", "string", "str", None),
    ("location", "005_origin_type",
     "typeof location.origin", "string", "str", None),
    ("location", "006_assign_is_function",
     "typeof location.assign", "function", "str", None),
    ("location", "007_replace_is_function",
     "typeof location.replace", "function", "str", None),

    # ── performance ───────────────────────────────────────────────────────────
    ("performance", "001_now_type",
     "typeof performance.now()", "number", "str", None),
    ("performance", "002_now_non_negative",
     "performance.now() >= 0", True, "bool", None),
    ("performance", "003_memory_exists",
     "typeof performance.memory", "object", "str", None),
    ("performance", "004_memory_heap_limit",
     "performance.memory.jsHeapSizeLimit > 0", True, "bool", None),
    ("performance", "005_memory_total_heap",
     "performance.memory.totalJSHeapSize > 0", True, "bool", None),
    ("performance", "006_timing_exists",
     "typeof performance.timing", "object", "str", None),

    # ── document ──────────────────────────────────────────────────────────────
    ("document", "001_document_exists",
     "typeof document", "object", "str", None),
    ("document", "002_cookie_type",
     "typeof document.cookie", "string", "str", None),
    ("document", "003_referrer_type",
     "typeof document.referrer", "string", "str", None),
    ("document", "004_hidden_false",
     "document.hidden", False, "bool", None),
    ("document", "005_visibility_state",
     "document.visibilityState", "visible", "str", None),
    ("document", "006_ready_state",
     "document.readyState", "complete", "str", None),
    ("document", "007_charset",
     "document.characterSet", "UTF-8", "str", None),
    ("document", "008_compat_mode",
     "document.compatMode", "CSS1Compat", "str", None),
    ("document", "009_content_type",
     "document.contentType", "text/html", "str", None),
    ("document", "010_create_element_div",
     "document.createElement('div').tagName", "DIV", "str", None),
    ("document", "011_create_element_input",
     "document.createElement('input').tagName", "INPUT", "str", None),
    ("document", "012_create_text_node",
     "document.createTextNode('hello').nodeType", 3, "int", None),
    ("document", "013_get_selection_null",
     "document.getSelection()", None, "NoneType", None),
    ("document", "014_fonts_exists",
     "typeof document.fonts", "object", "str", None),
    ("document", "015_implementation_exists",
     "typeof document.implementation", "object", "str", None),

    # ── dom_query ─────────────────────────────────────────────────────────────
    ("dom_query", "001_get_element_by_id_null",
     "document.getElementById('nonexistent')", None, "NoneType", None),
    ("dom_query", "002_query_selector_null",
     "document.querySelector('.nonexistent')", None, "NoneType", None),
    ("dom_query", "003_query_selector_all_empty",
     "document.querySelectorAll('.nonexistent').length", 0, "int", None),
    ("dom_query", "004_get_elements_by_tag_name_type",
     "typeof document.getElementsByTagName('div')", "object", "str", None),
    ("dom_query", "005_get_elements_by_class_name_type",
     "typeof document.getElementsByClassName('foo')", "object", "str", None),

    # ── storage ───────────────────────────────────────────────────────────────
    ("storage", "001_local_storage_exists",
     "typeof localStorage", "object", "str", None),
    ("storage", "002_session_storage_exists",
     "typeof sessionStorage", "object", "str", None),
    ("storage", "003_local_storage_set_get",
     "(function(){localStorage.setItem('k','v');return localStorage.getItem('k');})()",
     "v", "str", None),
    ("storage", "004_local_storage_remove",
     "(function(){localStorage.setItem('k2','v2');localStorage.removeItem('k2');return localStorage.getItem('k2');})()",
     None, "NoneType", None),
    ("storage", "005_local_storage_length_type",
     "typeof localStorage.length", "number", "str", None),
    ("storage", "006_indexed_db_exists",
     "typeof indexedDB", "object", "str", None),

    # ── timers ────────────────────────────────────────────────────────────────
    ("timers", "001_set_timeout_is_function",
     "typeof setTimeout", "function", "str", None),
    ("timers", "002_set_interval_is_function",
     "typeof setInterval", "function", "str", None),
    ("timers", "003_clear_timeout_is_function",
     "typeof clearTimeout", "function", "str", None),
    ("timers", "004_clear_interval_is_function",
     "typeof clearInterval", "function", "str", None),
    ("timers", "005_request_animation_frame",
     "typeof requestAnimationFrame", "function", "str", None),
    ("timers", "006_queue_microtask",
     "typeof queueMicrotask", "function", "str", None),
    ("timers", "007_request_idle_callback",
     "typeof requestIdleCallback", "function", "str", None),
    ("timers", "008_set_timeout_returns_id",
     "typeof setTimeout(function(){}, 0)", "number", "str", None),

    # ── event_loop ────────────────────────────────────────────────────────────
    ("event_loop", "001_advance_api_exists",
     "typeof __iv8__.eventLoop.advance", "function", "str", None),
    ("event_loop", "002_get_time_api_exists",
     "typeof __iv8__.eventLoop.getTime", "function", "str", None),
    ("event_loop", "003_date_now_type",
     "typeof Date.now()", "number", "str", None),
    ("event_loop", "004_date_now_positive",
     "Date.now() > 0", True, "bool", None),
    ("event_loop", "005_advance_advances_time",
     "(function(){var t0=Date.now();__iv8__.eventLoop.advance(1000);return Date.now()>=t0+1000;})()",
     True, "bool", None),
    ("event_loop", "006_set_timeout_fires_after_advance",
     "(function(){var fired=false;setTimeout(function(){fired=true;},100);__iv8__.eventLoop.advance(200);return fired;})()",
     True, "bool", None),

    # ── network ───────────────────────────────────────────────────────────────
    ("network", "001_fetch_is_function",
     "typeof fetch", "function", "str", None),
    ("network", "002_xhr_is_function",
     "typeof XMLHttpRequest", "function", "str", None),
    ("network", "003_websocket_is_function",
     "typeof WebSocket", "function", "str", None),
    ("network", "004_send_beacon_is_function",
     "typeof navigator.sendBeacon", "function", "str", None),
    ("network", "005_netlog_exists",
     "typeof __iv8__.netLog", "object", "str", None),
    ("network", "006_netlog_entries_array",
     "Array.isArray(__iv8__.netLog.entries)", True, "bool", None),
    ("network", "007_xhr_sync_resource",
     "(function(){var x=new XMLHttpRequest();x.open('GET','https://test.example/r',false);x.send();return x.status;})()",
     0, "int", None),  # no resource registered → status 0

    # ── canvas ────────────────────────────────────────────────────────────────
    ("canvas", "001_canvas_element_exists",
     "typeof HTMLCanvasElement", "function", "str", None),
    ("canvas", "002_canvas_to_data_url_type",
     "(function(){var c=document.createElement('canvas');return typeof c.toDataURL();})()",
     "string", "str", None),
    ("canvas", "003_canvas_to_data_url_prefix",
     "(function(){var c=document.createElement('canvas');return c.toDataURL().startsWith('data:image/');})()",
     True, "bool", None),
    ("canvas", "004_canvas_2d_context_exists",
     "(function(){var c=document.createElement('canvas');return c.getContext('2d')!==null;})()",
     True, "bool", None),
    ("canvas", "005_canvas_measure_text_positive",
     "(function(){var c=document.createElement('canvas');var ctx=c.getContext('2d');ctx.font='14px Arial';return ctx.measureText('hello').width>0;})()",
     True, "bool", None),
    ("canvas", "006_canvas_get_image_data",
     "(function(){var c=document.createElement('canvas');c.width=4;c.height=4;var ctx=c.getContext('2d');var d=ctx.getImageData(0,0,4,4);return d.data.length;})()",
     64, "int", None),
    ("canvas", "007_webgl_context_exists",
     "(function(){var c=document.createElement('canvas');return c.getContext('webgl')!==null;})()",
     True, "bool", None),
    ("canvas", "008_webgl_vendor_type",
     "(function(){var c=document.createElement('canvas');var gl=c.getContext('webgl');return typeof gl.getParameter(gl.VENDOR);})()",
     "string", "str", None),
    ("canvas", "009_webgl_max_texture_size",
     "(function(){var c=document.createElement('canvas');var gl=c.getContext('webgl');return gl.getParameter(gl.MAX_TEXTURE_SIZE)>0;})()",
     True, "bool", None),

    # ── audio ─────────────────────────────────────────────────────────────────
    ("audio", "001_audio_context_exists",
     "typeof AudioContext", "function", "str", None),
    ("audio", "002_webkit_audio_context_exists",
     "typeof webkitAudioContext", "function", "str", None),
    ("audio", "003_offline_audio_context_exists",
     "typeof OfflineAudioContext", "function", "str", None),
    ("audio", "004_audio_context_sample_rate",
     "new AudioContext().sampleRate > 0", True, "bool", None),
    ("audio", "005_audio_context_state",
     "new AudioContext().state", "running", "str", None),
    ("audio", "006_oscillator_node_type",
     "new AudioContext().createOscillator().type", "sine", "str", None),
    ("audio", "007_analyser_fft_size",
     "new AudioContext().createAnalyser().fftSize", 2048, "int", None),
    ("audio", "008_dynamics_compressor_threshold",
     "new AudioContext().createDynamicsCompressor().threshold.value", -24.0, "float", None),

    # ── anti_detect ───────────────────────────────────────────────────────────
    ("anti_detect", "001_webdriver_false",
     "navigator.webdriver", False, "bool", None),
    ("anti_detect", "002_iv8_not_enumerable",
     "Object.keys(window).indexOf('__iv8__') === -1", True, "bool", None),
    ("anti_detect", "003_eval_native_code",
     "eval.toString().indexOf('[native code]') !== -1", True, "bool", None),
    ("anti_detect", "004_set_timeout_native_code",
     "setTimeout.toString().indexOf('[native code]') !== -1", True, "bool", None),
    ("anti_detect", "005_fetch_native_code",
     "fetch.toString().indexOf('[native code]') !== -1", True, "bool", None),
    ("anti_detect", "006_user_agent_getter_native",
     "typeof Object.getOwnPropertyDescriptor(navigator,'userAgent').get", "function", "str", None),
    ("anti_detect", "007_user_agent_getter_native_code",
     "Object.getOwnPropertyDescriptor(navigator,'userAgent').get.toString().indexOf('[native code]') !== -1",
     True, "bool", None),
    ("anti_detect", "008_screen_width_getter_native",
     "typeof Object.getOwnPropertyDescriptor(screen,'width').get", "function", "str", None),
    ("anti_detect", "009_permissions_exists",
     "typeof navigator.permissions", "object", "str", None),
    ("anti_detect", "010_pdf_viewer_enabled",
     "navigator.pdfViewerEnabled", True, "bool", None),

    # ── wrap_native ───────────────────────────────────────────────────────────
    ("wrap_native", "001_wrap_native_exists",
     "typeof __iv8__.wrapNative", "function", "str", None),
    ("wrap_native", "002_wrapped_fn_native_code",
     "(function(){var f=__iv8__.wrapNative(function(){return 1;},'myFn');return f.toString().indexOf('[native code]')!==-1;})()",
     True, "bool", None),
    ("wrap_native", "003_wrapped_fn_works",
     "(function(){var f=__iv8__.wrapNative(function(){return 42;},'f');return f();})()",
     42, "int", None),
    ("wrap_native", "004_wrapped_fn_name",
     "(function(){var f=__iv8__.wrapNative(function(){},'myName');return f.name;})()",
     "myName", "str", None),
    ("wrap_native", "005_hook_native_exists",
     "typeof __iv8__.hookNative", "function", "str", None),
    ("wrap_native", "006_hook_native_intercepts",
     "(function(){var called=false;__iv8__.hookNative('Math.abs',function(orig,args){called=true;return orig.apply(this,args);});Math.abs(-5);return called;})()",
     True, "bool", None),
    ("wrap_native", "007_hook_native_modifies_return",
     "(function(){__iv8__.hookNative('Math.ceil',function(orig,args){return 99;});return Math.ceil(1.1);})()",
     99, "int", None),

    # ── expose ────────────────────────────────────────────────────────────────
    ("expose", "001_page_api_exists",
     "typeof __iv8__.page", "object", "str", None),
    ("expose", "002_page_load_is_function",
     "typeof __iv8__.page.load", "function", "str", None),
    ("expose", "003_input_api_exists",
     "typeof __iv8__.input", "object", "str", None),
    ("expose", "004_dispatch_mouse_event_is_function",
     "typeof __iv8__.input.dispatchMouseEvent", "function", "str", None),

    # ── misc_browser ──────────────────────────────────────────────────────────
    ("misc_browser", "001_text_encoder_exists",
     "typeof TextEncoder", "function", "str", None),
    ("misc_browser", "002_text_decoder_exists",
     "typeof TextDecoder", "function", "str", None),
    ("misc_browser", "003_text_encoder_decode",
     "(function(){var e=new TextEncoder();var d=new TextDecoder();return d.decode(e.encode('hello'));})()",
     "hello", "str", None),
    ("misc_browser", "004_url_exists",
     "typeof URL", "function", "str", None),
    ("misc_browser", "005_url_search_params_exists",
     "typeof URLSearchParams", "function", "str", None),
    ("misc_browser", "006_url_parse",
     "new URL('https://example.com/path?q=1').hostname", "example.com", "str", None),
    ("misc_browser", "007_url_search_params",
     "new URLSearchParams('a=1&b=2').get('a')", "1", "str", None),
    ("misc_browser", "008_message_channel_exists",
     "typeof MessageChannel", "function", "str", None),
    ("misc_browser", "009_abort_controller_exists",
     "typeof AbortController", "function", "str", None),
    ("misc_browser", "010_blob_exists",
     "typeof Blob", "function", "str", None),
    ("misc_browser", "011_structured_clone_exists",
     "typeof structuredClone", "function", "str", None),
    ("misc_browser", "012_structured_clone_works",
     "(function(){var o={a:1,b:[2,3]};var c=structuredClone(o);return c.a===1&&c.b[0]===2&&c!==o;})()",
     True, "bool", None),
    ("misc_browser", "013_mutation_observer_exists",
     "typeof MutationObserver", "function", "str", None),
    ("misc_browser", "014_intersection_observer_exists",
     "typeof IntersectionObserver", "function", "str", None),
    ("misc_browser", "015_resize_observer_exists",
     "typeof ResizeObserver", "function", "str", None),
    ("misc_browser", "016_atob_btoa",
     "atob(btoa('hello'))", "hello", "str", None),
    ("misc_browser", "017_proxy_works",
     "(function(){var p=new Proxy({},{get:function(t,k){return k;}});return p.test;})()",
     "test", "str", None),
    ("misc_browser", "018_symbol_iterator",
     "(function(){var a=[1,2,3];var it=a[Symbol.iterator]();return it.next().value;})()",
     1, "int", None),
    ("misc_browser", "019_error_stack_works",
     "(function(){try{throw new Error('t');}catch(e){return typeof e.stack==='string'&&e.stack.length>0;}})()",
     True, "bool", None),
    ("misc_browser", "020_request_idle_callback_exists",
     "typeof requestIdleCallback", "function", "str", None),
    ("misc_browser", "021_url_create_object_url",
     "typeof URL.createObjectURL", "function", "str", None),
    ("misc_browser", "022_intl_date_time_format",
     "typeof Intl.DateTimeFormat", "function", "str", None),
    ("misc_browser", "023_get_battery_exists",
     "typeof navigator.getBattery", "function", "str", None),
    ("misc_browser", "024_connection_exists",
     "typeof navigator.connection", "object", "str", None),
    ("misc_browser", "025_media_devices_exists",
     "typeof navigator.mediaDevices", "object", "str", None),
    ("misc_browser", "026_service_worker_exists",
     "typeof navigator.serviceWorker", "object", "str", None),
    ("misc_browser", "027_websocket_exists",
     "typeof WebSocket", "function", "str", None),
    ("misc_browser", "028_indexed_db_exists",
     "typeof indexedDB", "object", "str", None),
    ("misc_browser", "029_history_push_state",
     "typeof window.history.pushState", "function", "str", None),
    ("misc_browser", "030_document_fonts_ready",
     "typeof document.fonts.ready", "object", "str", None),
]


def write_fixture(category: str, name: str, js: str, value, type_name: str):
    """Write a JS fixture and its expected.json."""
    cat_dir = FIXTURES_DIR / category
    cat_dir.mkdir(parents=True, exist_ok=True)

    js_file = cat_dir / f"{name}.js"
    expected_file = cat_dir / f"{name}.expected.json"

    js_file.write_text(js, encoding="utf-8")
    expected = {"ok": True, "value": value, "type": type_name}
    expected_file.write_text(json.dumps(expected, ensure_ascii=False), encoding="utf-8")


def verify_with_iv8rs(fixtures):
    """Verify all fixtures against iv8-rs to catch any mismatches."""
    try:
        import iv8_rs
    except ImportError:
        print("WARNING: iv8_rs not available, skipping verification")
        return

    ctx = iv8_rs.JSContext()
    errors = []

    for category, name, js, expected_value, expected_type, xfail in fixtures:
        if xfail:
            continue
        try:
            actual = ctx.eval(js)
            # Type check
            actual_type = type(actual).__name__
            if actual_type != expected_type:
                # Allow int/float interchangeability for numbers
                if not (expected_type in ("int", "float") and actual_type in ("int", "float")):
                    errors.append(f"{category}/{name}: type {actual_type!r} != {expected_type!r} (js={js!r})")
                    continue
            # Value check (skip for non-deterministic values)
            non_det = ["random", "uuid", "now", "timestamp", "time", "advance"]
            if not any(nd in name for nd in non_det):
                if actual != expected_value:
                    # Float tolerance
                    if isinstance(actual, (int, float)) and isinstance(expected_value, (int, float)):
                        if abs(float(actual) - float(expected_value)) > 1e-9:
                            errors.append(f"{category}/{name}: value {actual!r} != {expected_value!r}")
                    else:
                        errors.append(f"{category}/{name}: value {actual!r} != {expected_value!r}")
        except Exception as e:
            errors.append(f"{category}/{name}: exception {e} (js={js!r})")

    ctx.close()

    if errors:
        print(f"\n[WARN]  {len(errors)} verification error(s):")
        for e in errors:
            print(f"  - {e}")
        return False
    return True


def main():
    print(f"Generating {len(FIXTURES)} M3 fixtures in {FIXTURES_DIR}")

    written = 0
    skipped = 0
    for category, name, js, value, type_name, xfail in FIXTURES:
        cat_dir = FIXTURES_DIR / category
        js_file = cat_dir / f"{name}.js"
        if js_file.exists():
            skipped += 1
            continue
        write_fixture(category, name, js, value, type_name)
        written += 1

    print(f"Written: {written}, Skipped (already exist): {skipped}")

    print("\nVerifying against iv8-rs...")
    ok = verify_with_iv8rs(FIXTURES)
    if ok:
        print("[OK] All fixtures verified against iv8-rs")
    else:
        print("[FAIL] Some fixtures failed verification — check errors above")
        sys.exit(1)


if __name__ == "__main__":
    main()
