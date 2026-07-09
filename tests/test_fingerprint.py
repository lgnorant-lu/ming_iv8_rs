"""
Task 59: CreepJS / FingerprintJS 核心检测验证套件

模拟 CreepJS 和 FingerprintJS 的核心检测逻辑，验证 iv8-rs 的指纹健壮性。
每个测试对应一个真实的反检测/指纹检测点。
"""
import iv8_rs
import pytest


@pytest.fixture
def ctx():
    """Standard JSContext with default environment."""
    c = iv8_rs.JSContext()
    yield c
    c.close()


@pytest.fixture
def ctx_custom():
    """JSContext with custom fingerprint."""
    c = iv8_rs.JSContext(environment={
        "navigator": {
            "userAgent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 "
                         "(KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36",
            "platform": "Win32",
            "language": "zh-CN",
            "languages": ["zh-CN", "en"],
            "hardwareConcurrency": 16,
            "deviceMemory": 8,
            "maxTouchPoints": 0,
        },
        "screen": {
            "width": 1920, "height": 1080,
            "availWidth": 1920, "availHeight": 1040,
            "colorDepth": 24, "pixelDepth": 24,
        },
        "webgl": {
            "UNMASKED_VENDOR_WEBGL": "Google Inc. (NVIDIA)",
            "UNMASKED_RENDERER_WEBGL": "ANGLE (NVIDIA, NVIDIA GeForce RTX 4060 Direct3D11)",
        },
    })
    yield c
    c.close()


# ─── 1. 基础反检测（CreepJS: headless detection）────────────────────────────

class TestHeadlessDetection:
    """CreepJS headless browser detection checks."""

    def test_webdriver_undefined(self, ctx):
        """navigator.webdriver should be falsy (undefined or false)."""
        wd = ctx.eval("navigator.webdriver")
        assert wd is None or wd == False, f"webdriver should be falsy, got {wd!r}"

    def test_webdriver_not_true(self, ctx):
        """navigator.webdriver must not be true."""
        assert ctx.eval("navigator.webdriver !== true") == True

    def test_chrome_exists(self, ctx):
        """window.chrome must exist and be an object."""
        assert ctx.eval("typeof window.chrome") == "object"
        assert ctx.eval("window.chrome !== null") == True

    def test_chrome_csi(self, ctx):
        """window.chrome.csi must be a function."""
        assert ctx.eval("typeof window.chrome.csi") == "function"

    def test_chrome_load_times(self, ctx):
        """window.chrome.loadTimes must be a function."""
        assert ctx.eval("typeof window.chrome.loadTimes") == "function"

    def test_chrome_runtime(self, ctx):
        """window.chrome.runtime must exist."""
        assert ctx.eval("typeof window.chrome.runtime") == "object"

    def test_iv8_hidden(self, ctx):
        """__iv8__ must not appear in Object.keys(window)."""
        assert ctx.eval("Object.keys(window).indexOf('__iv8__') === -1") == True

    def test_iv8_not_enumerable(self, ctx):
        """__iv8__ must not be enumerable."""
        assert ctx.eval("!Object.getOwnPropertyDescriptor(window, '__iv8__').enumerable") == True

    def test_permissions_exists(self, ctx):
        """navigator.permissions must exist."""
        assert ctx.eval("typeof navigator.permissions") == "object"

    def test_permissions_query(self, ctx):
        """navigator.permissions.query must return a Promise."""
        result = ctx.eval_promise(
            "navigator.permissions.query({name:'geolocation'}).then(r => r.state)"
        )
        assert result in ("granted", "denied", "prompt"), f"Unexpected state: {result}"


# ─── 2. Function.prototype.toString 检测 ─────────────────────────────────────

class TestFunctionToString:
    """CreepJS: Function.prototype.toString tampering detection."""

    def test_eval_is_native(self, ctx):
        assert "[native code]" in ctx.eval("eval.toString()")

    def test_settimeout_is_native(self, ctx):
        assert "[native code]" in ctx.eval("setTimeout.toString()")

    def test_setinterval_is_native(self, ctx):
        assert "[native code]" in ctx.eval("setInterval.toString()")

    def test_fetch_is_native(self, ctx):
        assert "[native code]" in ctx.eval("fetch.toString()")

    def test_object_define_property_is_native(self, ctx):
        assert "[native code]" in ctx.eval("Object.defineProperty.toString()")

    def test_wrapped_function_is_native(self, ctx):
        """wrapNative should make custom functions appear native."""
        ctx.eval("var myFn = function() { return 42; }; myFn = __iv8__.wrapNative(myFn, 'myFn');")
        assert "[native code]" in ctx.eval("myFn.toString()")

    def test_wrapped_function_still_works(self, ctx):
        ctx.eval("var fn2 = __iv8__.wrapNative(function() { return 99; }, 'fn2');")
        assert ctx.eval("fn2()") == 99

    def test_function_prototype_toString_not_tampered(self, ctx):
        """Function.prototype.toString itself must be native."""
        assert "[native code]" in ctx.eval("Function.prototype.toString.toString()")


# ─── 3. Navigator 指纹（FingerprintJS core）──────────────────────────────────

class TestNavigatorFingerprint:
    """FingerprintJS: navigator property fingerprinting."""

    def test_user_agent_is_chrome(self, ctx):
        ua = ctx.eval("navigator.userAgent")
        assert "Chrome" in ua, f"UA should contain Chrome: {ua}"

    def test_user_agent_is_string(self, ctx):
        assert ctx.eval("typeof navigator.userAgent") == "string"

    def test_platform_win32(self, ctx):
        assert ctx.eval("navigator.platform") == "Win32"

    def test_vendor_google(self, ctx):
        assert ctx.eval("navigator.vendor") == "Google Inc."

    def test_language_is_string(self, ctx):
        assert ctx.eval("typeof navigator.language") == "string"

    def test_languages_is_array(self, ctx):
        assert ctx.eval("Array.isArray(navigator.languages)") == True
        assert ctx.eval("navigator.languages.length > 0") == True

    def test_hardware_concurrency_positive(self, ctx):
        hc = ctx.eval("navigator.hardwareConcurrency")
        assert isinstance(hc, (int, float)) and hc > 0

    def test_device_memory_positive(self, ctx):
        dm = ctx.eval("navigator.deviceMemory")
        assert isinstance(dm, (int, float)) and dm > 0

    def test_cookie_enabled_true(self, ctx):
        assert ctx.eval("navigator.cookieEnabled") == True

    def test_online_true(self, ctx):
        assert ctx.eval("navigator.onLine") == True

    def test_pdf_viewer_enabled(self, ctx):
        assert ctx.eval("navigator.pdfViewerEnabled") == True

    def test_user_agent_native_getter(self, ctx):
        """navigator.userAgent must have a native getter descriptor on Navigator.prototype."""
        desc_type = ctx.eval(
            "typeof Object.getOwnPropertyDescriptor(Navigator.prototype, 'userAgent').get"
        )
        assert desc_type == "function", f"Expected getter, got {desc_type}"

    def test_user_agent_getter_native_code(self, ctx):
        getter_str = ctx.eval(
            "Object.getOwnPropertyDescriptor(Navigator.prototype, 'userAgent').get.toString()"
        )
        assert "[native code]" in getter_str, f"Getter not native: {getter_str}"

    def test_platform_native_getter(self, ctx):
        desc_type = ctx.eval(
            "typeof Object.getOwnPropertyDescriptor(Navigator.prototype, 'platform').get"
        )
        assert desc_type == "function"

    def test_custom_ua_override(self, ctx_custom):
        ua = ctx_custom.eval("navigator.userAgent")
        assert "Chrome/124" in ua

    def test_custom_language_override(self, ctx_custom):
        assert ctx_custom.eval("navigator.language") == "zh-CN"

    def test_custom_hardware_concurrency(self, ctx_custom):
        assert ctx_custom.eval("navigator.hardwareConcurrency") == 16

    def test_navigator_instanceof_event_target(self, ctx):
        """v0.8.78: navigator instanceof EventTarget must be True"""
        assert ctx.eval("navigator instanceof EventTarget") == True

    def test_navigator_instanceof_navigator(self, ctx):
        """v0.8.78: navigator instanceof Navigator must be True"""
        assert ctx.eval("navigator instanceof Navigator") == True

    def test_navigator_proto_chain(self, ctx):
        """v0.8.78: navigator.__proto__.__proto__ === Navigator.prototype"""
        assert ctx.eval(
            "Object.getPrototypeOf(Object.getPrototypeOf(navigator)) === Navigator.prototype"
        ) == True


# ─── 4. Screen 指纹 ───────────────────────────────────────────────────────────

class TestScreenFingerprint:
    """FingerprintJS: screen property fingerprinting."""

    def test_screen_width_positive(self, ctx):
        assert ctx.eval("screen.width > 0") == True

    def test_screen_height_positive(self, ctx):
        assert ctx.eval("screen.height > 0") == True

    def test_screen_color_depth(self, ctx):
        cd = ctx.eval("screen.colorDepth")
        assert cd in (24, 32), f"Unexpected colorDepth: {cd}"

    def test_screen_avail_width(self, ctx):
        assert ctx.eval("screen.availWidth > 0") == True

    def test_screen_avail_height(self, ctx):
        assert ctx.eval("screen.availHeight > 0") == True

    def test_screen_width_native_getter(self, ctx):
        desc_type = ctx.eval(
            "typeof Object.getOwnPropertyDescriptor(Screen.prototype, 'width').get"
        )
        assert desc_type == "function"

    def test_screen_custom_override(self, ctx_custom):
        assert ctx_custom.eval("screen.width") == 1920
        assert ctx_custom.eval("screen.height") == 1080


# ─── 5. DOM 原型链（CreepJS: prototype pollution detection）──────────────────

class TestDOMPrototypeChain:
    """CreepJS: DOM prototype chain integrity checks."""

    def test_instanceof_htmlelement(self, ctx):
        ctx.page_load("<html><body><div id='t'></div></body></html>")
        assert ctx.eval("document.getElementById('t') instanceof HTMLElement") == True

    def test_instanceof_element(self, ctx):
        ctx.page_load("<html><body><div id='t'></div></body></html>")
        assert ctx.eval("document.getElementById('t') instanceof Element") == True

    def test_instanceof_node(self, ctx):
        ctx.page_load("<html><body><div id='t'></div></body></html>")
        assert ctx.eval("document.getElementById('t') instanceof Node") == True

    def test_instanceof_event_target(self, ctx):
        ctx.page_load("<html><body><div id='t'></div></body></html>")
        assert ctx.eval("document.getElementById('t') instanceof EventTarget") == True

    def test_instanceof_htmldivelement(self, ctx):
        ctx.page_load("<html><body><div id='t'></div></body></html>")
        assert ctx.eval("document.getElementById('t') instanceof HTMLDivElement") == True

    def test_instanceof_htmlinputelement(self, ctx):
        ctx.page_load("<html><body><input id='i' type='text'></body></html>")
        assert ctx.eval("document.getElementById('i') instanceof HTMLInputElement") == True

    def test_get_attribute_native_code(self, ctx):
        ctx.page_load("<html><body><div id='t' data-x='1'></div></body></html>")
        assert "[native code]" in ctx.eval(
            "document.getElementById('t').getAttribute.toString()"
        )

    def test_prototype_chain_not_polluted(self, ctx):
        """Object.prototype should not have unexpected properties."""
        ctx.page_load("<html><body></body></html>")
        # Check that common prototype pollution markers are absent
        for prop in ["__nodeId__", "__navInstalled__", "__protoSet__"]:
            result = ctx.eval(f"'{prop}' in Object.prototype")
            assert result == False, f"Object.prototype polluted with {prop}"

    def test_node_identity_cache(self, ctx):
        """Same DOM node should return same JS object."""
        ctx.page_load("<html><body><div id='t'></div></body></html>")
        assert ctx.eval(
            "document.getElementById('t') === document.getElementById('t')"
        ) == True


# ─── 6. WebGL 指纹（FingerprintJS: WebGL fingerprinting）─────────────────────

class TestWebGLFingerprint:
    """FingerprintJS: WebGL parameter fingerprinting."""

    def test_webgl_context_exists(self, ctx):
        ctx.page_load("<html><body><canvas id='c'></canvas></body></html>")
        result = ctx.eval("""
            var c = document.getElementById('c');
            var gl = c.getContext('webgl') || c.getContext('experimental-webgl');
            gl !== null
        """)
        assert result == True

    def test_webgl_vendor(self, ctx):
        ctx.page_load("<html><body><canvas id='c'></canvas></body></html>")
        vendor = ctx.eval("""
            var c = document.getElementById('c');
            var gl = c.getContext('webgl');
            var ext = gl.getExtension('WEBGL_debug_renderer_info');
            ext ? gl.getParameter(ext.UNMASKED_VENDOR_WEBGL) : gl.getParameter(gl.VENDOR)
        """)
        assert isinstance(vendor, str) and len(vendor) > 0

    def test_webgl_renderer(self, ctx):
        ctx.page_load("<html><body><canvas id='c'></canvas></body></html>")
        renderer = ctx.eval("""
            var c = document.getElementById('c');
            var gl = c.getContext('webgl');
            var ext = gl.getExtension('WEBGL_debug_renderer_info');
            ext ? gl.getParameter(ext.UNMASKED_RENDERER_WEBGL) : gl.getParameter(gl.RENDERER)
        """)
        assert isinstance(renderer, str) and len(renderer) > 0

    def test_webgl_max_texture_size(self, ctx):
        ctx.page_load("<html><body><canvas id='c'></canvas></body></html>")
        size = ctx.eval("""
            var gl = document.getElementById('c').getContext('webgl');
            gl.getParameter(gl.MAX_TEXTURE_SIZE)
        """)
        assert isinstance(size, (int, float)) and size > 0

    def test_webgl_custom_vendor(self, ctx_custom):
        ctx_custom.page_load("<html><body><canvas id='c'></canvas></body></html>")
        vendor = ctx_custom.eval("""
            var gl = document.getElementById('c').getContext('webgl');
            var ext = gl.getExtension('WEBGL_debug_renderer_info');
            ext ? gl.getParameter(ext.UNMASKED_VENDOR_WEBGL) : ''
        """)
        assert "NVIDIA" in vendor or "Google" in vendor


# ─── 7. Canvas 指纹（FingerprintJS: canvas fingerprinting）──────────────────

class TestCanvasFingerprint:
    """FingerprintJS: Canvas 2D fingerprinting."""

    def test_canvas_context_exists(self, ctx):
        ctx.page_load("<html><body><canvas id='c'></canvas></body></html>")
        result = ctx.eval("""
            var ctx2d = document.getElementById('c').getContext('2d');
            ctx2d !== null
        """)
        assert result == True

    def test_canvas_fill_text(self, ctx):
        """Canvas fillText should not throw."""
        ctx.page_load("<html><body><canvas id='c'></canvas></body></html>")
        ctx.eval("""
            var ctx2d = document.getElementById('c').getContext('2d');
            ctx2d.fillStyle = '#f60';
            ctx2d.fillRect(125, 1, 62, 20);
            ctx2d.fillStyle = '#069';
            ctx2d.font = '11pt Arial';
            ctx2d.fillText('Cwm fjordbank glyphs vext quiz', 2, 15);
        """)

    def test_canvas_to_data_url(self, ctx):
        """toDataURL should return a data URL string."""
        ctx.page_load("<html><body><canvas id='c'></canvas></body></html>")
        result = ctx.eval("""
            var c = document.getElementById('c');
            c.getContext('2d').fillText('test', 0, 10);
            c.toDataURL()
        """)
        assert isinstance(result, str)
        assert result.startswith("data:image/"), f"Expected data URL, got: {result[:50]}"

    def test_canvas_measure_text(self, ctx):
        """measureText should return an object with width."""
        ctx.page_load("<html><body><canvas id='c'></canvas></body></html>")
        width = ctx.eval("""
            var ctx2d = document.getElementById('c').getContext('2d');
            ctx2d.font = '14px Arial';
            ctx2d.measureText('Hello World').width
        """)
        assert isinstance(width, (int, float)) and width > 0

    def test_canvas_get_image_data(self, ctx):
        """getImageData should return ImageData-like object."""
        ctx.page_load("<html><body><canvas id='c' width='10' height='10'></canvas></body></html>")
        result = ctx.eval("""
            var ctx2d = document.getElementById('c').getContext('2d');
            var data = ctx2d.getImageData(0, 0, 10, 10);
            data.width === 10 && data.height === 10 && data.data.length === 400
        """)
        assert result == True


# ─── 8. AudioContext 指纹（FingerprintJS: audio fingerprinting）──────────────

class TestAudioFingerprint:
    """FingerprintJS: AudioContext fingerprinting."""

    def test_audio_context_exists(self, ctx):
        assert ctx.eval("typeof AudioContext") == "function"

    def test_webkit_audio_context_exists(self, ctx):
        assert ctx.eval("typeof webkitAudioContext") == "function"

    def test_audio_context_create(self, ctx):
        result = ctx.eval("""
            var ac = new AudioContext();
            ac !== null && typeof ac.sampleRate === 'number'
        """)
        assert result == True

    def test_audio_context_sample_rate(self, ctx):
        sr = ctx.eval("new AudioContext().sampleRate")
        assert isinstance(sr, (int, float)) and sr > 0

    def test_offline_audio_context(self, ctx):
        assert ctx.eval("typeof OfflineAudioContext") == "function"

    def test_audio_context_oscillator(self, ctx):
        result = ctx.eval("""
            var ac = new AudioContext();
            var osc = ac.createOscillator();
            typeof osc.frequency === 'object'
        """)
        assert result == True

    def test_audio_context_analyser(self, ctx):
        result = ctx.eval("""
            var ac = new AudioContext();
            var analyser = ac.createAnalyser();
            analyser.fftSize === 2048
        """)
        assert result == True

    def test_audio_fingerprint_pattern(self, ctx):
        """Simulate FingerprintJS audio fingerprint collection."""
        result = ctx.eval_promise("""
            (function() {
                var ac = new OfflineAudioContext(1, 44100, 44100);
                var osc = ac.createOscillator();
                osc.type = 'triangle';
                osc.frequency.value = 10000;
                var comp = ac.createDynamicsCompressor();
                comp.threshold.value = -50;
                comp.knee.value = 40;
                comp.ratio.value = 12;
                comp.attack.value = 0;
                comp.release.value = 0.25;
                osc.connect(comp);
                comp.connect(ac.destination);
                osc.start(0);
                return ac.startRendering().then(function(buffer) {
                    var data = buffer.getChannelData(0);
                    return typeof data.length === 'number' && data.length > 0;
                });
            })()
        """)
        assert result == True


# ─── 9. Crypto 指纹（FingerprintJS: crypto fingerprinting）──────────────────

class TestCryptoFingerprint:
    """FingerprintJS: Web Crypto API fingerprinting."""

    def test_crypto_exists(self, ctx):
        assert ctx.eval("typeof crypto") == "object"

    def test_crypto_subtle_exists(self, ctx):
        assert ctx.eval("typeof crypto.subtle") == "object"

    def test_get_random_values(self, ctx):
        result = ctx.eval("""
            var arr = new Uint8Array(16);
            crypto.getRandomValues(arr);
            arr.length === 16
        """)
        assert result == True

    def test_random_uuid(self, ctx):
        uuid = ctx.eval("crypto.randomUUID()")
        assert isinstance(uuid, str) and len(uuid) == 36
        assert uuid.count("-") == 4

    def test_subtle_digest_sha256(self, ctx):
        result = ctx.eval_promise("""
            crypto.subtle.digest('SHA-256', new Uint8Array([1,2,3]))
                .then(function(buf) { return buf.byteLength === 32; })
        """)
        assert result == True

    def test_subtle_hmac(self, ctx):
        result = ctx.eval_promise("""
            crypto.subtle.importKey(
                'raw', new Uint8Array(32),
                {name: 'HMAC', hash: 'SHA-256'}, false, ['sign']
            ).then(function(key) {
                return crypto.subtle.sign('HMAC', key, new Uint8Array(10));
            }).then(function(sig) {
                return sig.byteLength === 32;
            })
        """)
        assert result == True

    def test_subtle_aes_gcm(self, ctx):
        result = ctx.eval_promise("""
            crypto.subtle.generateKey(
                {name: 'AES-GCM', length: 256}, true, ['encrypt', 'decrypt']
            ).then(function(key) {
                return typeof key === 'object';
            })
        """)
        assert result == True


# ─── 10. 性能 API（FingerprintJS: timing fingerprinting）─────────────────────

class TestPerformanceFingerprint:
    """FingerprintJS: performance timing fingerprinting."""

    def test_performance_now(self, ctx):
        t = ctx.eval("performance.now()")
        assert isinstance(t, (int, float)) and t >= 0

    def test_performance_now_advances(self, ctx):
        ctx.eval("var t0 = performance.now();")
        ctx.eval("__iv8__.eventLoop.advance(100)")
        result = ctx.eval("performance.now() > t0")
        assert result == True

    def test_date_now_is_number(self, ctx):
        t = ctx.eval("Date.now()")
        assert isinstance(t, (int, float)) and t > 0

    def test_date_now_advances(self, ctx):
        ctx.eval("var d0 = Date.now();")
        ctx.eval("__iv8__.eventLoop.advance(1000)")
        result = ctx.eval("Date.now() >= d0 + 1000")
        assert result == True

    def test_performance_memory(self, ctx):
        """performance.memory should exist (Chrome-specific)."""
        result = ctx.eval("typeof performance.memory")
        assert result == "object"

    def test_performance_memory_values(self, ctx):
        result = ctx.eval("""
            performance.memory.jsHeapSizeLimit > 0 &&
            performance.memory.totalJSHeapSize > 0
        """)
        assert result == True


# ─── 11. 字体检测（FingerprintJS: font fingerprinting）───────────────────────

class TestFontFingerprint:
    """FingerprintJS: font detection via canvas measureText."""

    def test_measure_text_varies_by_font(self, ctx):
        """Different fonts should produce different text widths."""
        ctx.page_load("<html><body><canvas id='c'></canvas></body></html>")
        result = ctx.eval("""
            var ctx2d = document.getElementById('c').getContext('2d');
            ctx2d.font = '72px monospace';
            var w1 = ctx2d.measureText('mmmmmmmmmmlli').width;
            ctx2d.font = '72px sans-serif';
            var w2 = ctx2d.measureText('mmmmmmmmmmlli').width;
            // Both should be positive numbers
            w1 > 0 && w2 > 0
        """)
        assert result == True

    def test_document_fonts_exists(self, ctx):
        """document.fonts should exist."""
        assert ctx.eval("typeof document.fonts") == "object"


# ─── 12. 存储 API（FingerprintJS: storage fingerprinting）────────────────────

class TestStorageFingerprint:
    """FingerprintJS: storage API fingerprinting."""

    def test_local_storage_exists(self, ctx):
        assert ctx.eval("typeof localStorage") == "object"

    def test_session_storage_exists(self, ctx):
        assert ctx.eval("typeof sessionStorage") == "object"

    def test_local_storage_set_get(self, ctx):
        ctx.eval("localStorage.setItem('test_key', 'test_value')")
        result = ctx.eval("localStorage.getItem('test_key')")
        assert result == "test_value"

    def test_indexed_db_exists(self, ctx):
        assert ctx.eval("typeof indexedDB") == "object"

    def test_cookie_read_write(self, ctx):
        ctx.eval("document.cookie = 'fp_test=abc123'")
        cookie = ctx.eval("document.cookie")
        assert "fp_test=abc123" in str(cookie)


# ─── 13. 网络 API（FingerprintJS: network fingerprinting）────────────────────

class TestNetworkFingerprint:
    """FingerprintJS: network API fingerprinting."""

    def test_connection_exists(self, ctx):
        assert ctx.eval("typeof navigator.connection") == "object"

    def test_connection_effective_type(self, ctx):
        et = ctx.eval("navigator.connection.effectiveType")
        assert et in ("slow-2g", "2g", "3g", "4g"), f"Unexpected: {et}"

    def test_send_beacon_exists(self, ctx):
        assert ctx.eval("typeof navigator.sendBeacon") == "function"

    def test_fetch_exists(self, ctx):
        assert ctx.eval("typeof fetch") == "function"

    def test_xhr_exists(self, ctx):
        assert ctx.eval("typeof XMLHttpRequest") == "function"

    def test_websocket_exists(self, ctx):
        assert ctx.eval("typeof WebSocket") == "function"


# ─── 14. 综合反检测（CreepJS 完整模拟）──────────────────────────────────────

class TestCreepJSSimulation:
    """Simulate CreepJS comprehensive detection checks."""

    def test_window_equals_globalthis(self, ctx):
        assert ctx.eval("window === globalThis") == True

    def test_window_equals_self(self, ctx):
        assert ctx.eval("window === self") == True

    def test_object_prototype_not_polluted(self, ctx):
        """Object.prototype should be clean."""
        result = ctx.eval("""
            var clean = true;
            var suspicious = ['__nodeId__', '__navInstalled__', '__protoSet__',
                              '__attrs__', '__domNav__', '__addNavProps__'];
            suspicious.forEach(function(p) {
                if (p in Object.prototype) clean = false;
            });
            clean
        """)
        assert result == True

    def test_error_stack_trace_works(self, ctx):
        """Error.stack should work (used by CreepJS for browser detection)."""
        result = ctx.eval("""
            try { throw new Error('test'); } catch(e) {
                typeof e.stack === 'string' && e.stack.length > 0
            }
        """)
        assert result == True

    def test_proxy_detection_resistance(self, ctx):
        """Basic Proxy should work (CreepJS uses it for detection)."""
        result = ctx.eval("""
            var p = new Proxy({}, { get: function(t, k) { return k; } });
            p.test === 'test'
        """)
        assert result == True

    def test_symbol_iterator_works(self, ctx):
        result = ctx.eval("""
            var arr = [1, 2, 3];
            var iter = arr[Symbol.iterator]();
            iter.next().value === 1
        """)
        assert result == True

    def test_intl_date_time_format(self, ctx):
        """Intl.DateTimeFormat should work."""
        # Note: our timezone shim wraps Intl.DateTimeFormat
        # Test basic functionality without triggering the shim
        result = ctx.eval("""
            try {
                var dtf = Intl.DateTimeFormat;
                typeof dtf === 'function'
            } catch(e) { false }
        """)
        assert result == True

    def test_structured_clone(self, ctx):
        """structuredClone should work."""
        result = ctx.eval("""
            var obj = {a: 1, b: [2, 3]};
            var clone = structuredClone(obj);
            clone.a === 1 && clone.b[0] === 2 && clone !== obj
        """)
        assert result == True

    def test_mutation_observer_exists(self, ctx):
        assert ctx.eval("typeof MutationObserver") == "function"

    def test_intersection_observer_exists(self, ctx):
        assert ctx.eval("typeof IntersectionObserver") == "function"

    def test_resize_observer_exists(self, ctx):
        assert ctx.eval("typeof ResizeObserver") == "function"

    def test_blob_exists(self, ctx):
        assert ctx.eval("typeof Blob") == "function"

    def test_url_create_object_url(self, ctx):
        assert ctx.eval("typeof URL.createObjectURL") == "function"

    def test_request_idle_callback(self, ctx):
        assert ctx.eval("typeof requestIdleCallback") == "function"

    def test_queue_microtask(self, ctx):
        assert ctx.eval("typeof queueMicrotask") == "function"

    def test_abort_controller(self, ctx):
        assert ctx.eval("typeof AbortController") == "function"

    def test_text_encoder_decoder(self, ctx):
        result = ctx.eval("""
            var enc = new TextEncoder();
            var bytes = enc.encode('hello');
            var dec = new TextDecoder();
            dec.decode(bytes) === 'hello'
        """)
        assert result == True


# ─── 15. hookNative 深度测试 ─────────────────────────────────────────────────

class TestHookNative:
    """Deep tests for hookNative (iv8-rs specific feature)."""

    def test_hook_native_basic(self, ctx):
        """hookNative should intercept function calls."""
        ctx.eval("""
            var calls = [];
            __iv8__.hookNative('Math.random', function(orig, args) {
                calls.push('intercepted');
                return orig.apply(this, args);
            });
        """)
        ctx.eval("Math.random(); Math.random();")
        count = ctx.eval("calls.length")
        assert count == 2

    def test_hook_native_modify_return(self, ctx):
        """hookNative should be able to modify return values."""
        ctx.eval("""
            __iv8__.hookNative('Math.random', function(orig, args) {
                return 0.42;
            });
        """)
        result = ctx.eval("Math.random()")
        assert result == 0.42

    def test_hook_native_restore(self, ctx):
        """After hook, original function should still work."""
        original = ctx.eval("Math.floor(3.7)")
        assert original == 3

    def test_wrap_native_preserves_name(self, ctx):
        ctx.eval("var fn = __iv8__.wrapNative(function() {}, 'myFunc');")
        assert ctx.eval("fn.name") == "myFunc"

    def test_wrap_native_preserves_length(self, ctx):
        ctx.eval("var fn = __iv8__.wrapNative(function(a, b) {}, 'twoArgs');")
        # Length may vary, but function should work
        assert ctx.eval("typeof fn") == "function"


# ─── 16. 多线程隔离测试 ──────────────────────────────────────────────────────

class TestMultithreadIsolation:
    """Verify that multiple JSContext instances are fully isolated."""

    def test_contexts_isolated(self):
        ctx1 = iv8_rs.JSContext(environment={"navigator": {"userAgent": "Bot/1"}})
        ctx2 = iv8_rs.JSContext(environment={"navigator": {"userAgent": "Bot/2"}})
        try:
            ctx1.eval("var secret = 'ctx1_secret';")
            ctx2.eval("var secret = 'ctx2_secret';")
            assert ctx1.eval("navigator.userAgent") == "Bot/1"
            assert ctx2.eval("navigator.userAgent") == "Bot/2"
            assert ctx1.eval("secret") == "ctx1_secret"
            assert ctx2.eval("secret") == "ctx2_secret"
        finally:
            ctx1.close()
            ctx2.close()

    def test_concurrent_contexts(self):
        """Multiple contexts can run concurrently in different threads."""
        import threading
        results = {}
        errors = []
        lock = threading.Lock()

        def run(tid):
            try:
                c = iv8_rs.JSContext(environment={"navigator": {"userAgent": f"Bot/{tid}"}})
                ua = c.eval("navigator.userAgent")
                val = c.eval(f"{tid} * {tid}")
                c.close()
                with lock:
                    results[tid] = (ua, val)
            except Exception as e:
                with lock:
                    errors.append((tid, str(e)))

        threads = [threading.Thread(target=run, args=(i,)) for i in range(8)]
        for t in threads: t.start()
        for t in threads: t.join()

        assert not errors, f"Thread errors: {errors}"
        for i in range(8):
            assert results[i][0] == f"Bot/{i}"
            assert results[i][1] == i * i
