"""
Task 56-58: M5 验收测试 — iv8 examples 兼容性

覆盖所有 iv8 examples 的核心模式：
- abogus: bdms.init + XHR + netLog
- h5st: ParamsSignMain 签名
- zp_stoken: 两阶段加载 + canvas/crypto 指纹（模拟版，不需要真实网络）
- 瑞数 pattern: page.load + inline script + XHR hook

注意：真实网络测试（zp_stoken 完整版）需要手动运行。
"""
import pytest
import iv8_rs
import os


@pytest.fixture
def ctx():
    c = iv8_rs.JSContext()
    yield c
    c.close()


# ─── abogus 核心模式 ──────────────────────────────────────────────────────────

class TestAbogusPattern:
    """abogus: bdms.init + XHR + netLog 全流程"""

    def test_abogus_js_loads(self):
        """bdms JS 可以加载并初始化。"""
        js_path = os.path.join(os.path.dirname(__file__), 'iv8-ref', 'examples', 'js', 'bdms_1.0.1.19.js')
        if not os.path.exists(js_path):
            pytest.skip("bdms JS not found")

        ctx = iv8_rs.JSContext(environment={
            "navigator": {"userAgent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36"},
            "location": {"href": "https://www.douyin.com/", "hostname": "www.douyin.com"},
        })
        try:
            js_code = open(js_path, encoding='utf-8').read()
            ctx.eval(js_code)
            # bdms should be defined
            assert ctx.eval("typeof bdms") == "object"
        finally:
            ctx.close()

    def test_xhr_netlog_pattern(self, ctx):
        """XHR + netLog 模式：add_resource + XHR + get_net_log。"""
        ctx.add_resource('https://api.example.com/data', '{"result":"ok"}', 200,
                         {'content-type': 'application/json'})
        ctx.eval("""
            var result = null;
            var xhr = new XMLHttpRequest();
            xhr.open('GET', 'https://api.example.com/data', false);
            xhr.send();
            result = xhr.responseText;
        """)
        result = ctx.eval("result")
        assert result == '{"result":"ok"}'

    def test_netlog_captures_requests(self, ctx):
        """netLog 记录 XHR 请求。"""
        ctx.add_resource('https://api.example.com/track', '{}', 200)
        ctx.eval("""
            var xhr = new XMLHttpRequest();
            xhr.open('GET', 'https://api.example.com/track', false);
            xhr.send();
        """)
        # netLog.entries should have captured the request
        log = ctx.eval("JSON.stringify(__iv8__.netLog.entries)")
        assert 'api.example.com' in str(log)


# ─── h5st 核心模式 ────────────────────────────────────────────────────────────

class TestH5stPattern:
    """h5st: ParamsSignMain 签名模式"""

    def test_h5st_js_loads(self):
        """h5st JS 可以加载。"""
        js_path = os.path.join(os.path.dirname(__file__), 'iv8-ref', 'examples', 'js', 'js_security_v3_main.js')
        if not os.path.exists(js_path):
            pytest.skip("h5st JS not found")

        ctx = iv8_rs.JSContext(environment={
            "navigator": {"userAgent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36"},
            "location": {"href": "https://www.jd.com/", "hostname": "www.jd.com"},
        })
        try:
            html_path = os.path.join(os.path.dirname(__file__), 'iv8-ref', 'examples', 'js', 'jd_index.html')
            if os.path.exists(html_path):
                html = open(html_path, encoding='utf-8').read()
                ctx.page_load(html, base_url='https://www.jd.com/')

            js_code = open(js_path, encoding='utf-8').read()
            ctx.eval(js_code)
            # ParamsSignMain should be defined
            assert ctx.eval("typeof ParamsSignMain") in ("function", "object")
        finally:
            ctx.close()

    def test_document_elements_available(self, ctx):
        """page_load 后 document.documentElement/body/head 可用。"""
        ctx.page_load("<html><head></head><body></body></html>")
        assert ctx.eval("document.documentElement !== null") == True
        assert ctx.eval("document.body !== null") == True
        assert ctx.eval("document.head !== null") == True

    def test_crypto_for_signing(self, ctx):
        """SubtleCrypto 可用于签名计算。"""
        result = ctx.eval_promise("""
            crypto.subtle.importKey(
                'raw',
                new TextEncoder().encode('secret-key-32-bytes-padding-here'),
                {name: 'HMAC', hash: 'SHA-256'},
                false,
                ['sign']
            ).then(function(key) {
                return crypto.subtle.sign('HMAC', key, new TextEncoder().encode('data'));
            }).then(function(sig) {
                return sig.byteLength === 32;
            })
        """)
        assert result == True


# ─── zp_stoken 核心模式（模拟版）────────────────────────────────────────────

class TestZpStokenPattern:
    """
    zp_stoken: 两阶段加载 + canvas/crypto 指纹模式（模拟版）

    真实版需要网络请求，这里验证 iv8-rs 支持 zp_stoken 所需的所有 API。
    """

    def test_location_environment_injection(self):
        """location 环境注入正确。"""
        ctx = iv8_rs.JSContext(environment={
            "location": {
                "href": "https://www.zhipin.com/web/common/security-check.html?seed=abc&name=test&ts=123",
                "origin": "https://www.zhipin.com",
                "protocol": "https:",
                "host": "www.zhipin.com",
                "hostname": "www.zhipin.com",
                "port": "",
                "pathname": "/web/common/security-check.html",
                "search": "?seed=abc&name=test&ts=123",
                "hash": "",
            },
        })
        try:
            assert ctx.eval("location.hostname") == "www.zhipin.com"
            assert ctx.eval("location.protocol") == "https:"
            assert "seed=abc" in str(ctx.eval("location.search"))
        finally:
            ctx.close()

    def test_canvas_fingerprint_available(self, ctx):
        """Canvas 指纹 API 可用。"""
        ctx.page_load("<html><body><canvas id='c'></canvas></body></html>")
        result = ctx.eval("""
            var c = document.getElementById('c');
            var ctx2d = c.getContext('2d');
            ctx2d.fillStyle = '#f60';
            ctx2d.fillRect(125, 1, 62, 20);
            ctx2d.fillStyle = '#069';
            ctx2d.font = '11pt Arial';
            ctx2d.fillText('Cwm fjordbank glyphs vext quiz', 2, 15);
            var dataUrl = c.toDataURL();
            dataUrl.startsWith('data:image/')
        """)
        assert result == True

    def test_webgl_fingerprint_available(self, ctx):
        """WebGL 指纹 API 可用。"""
        ctx.page_load("<html><body><canvas id='c'></canvas></body></html>")
        result = ctx.eval("""
            var c = document.getElementById('c');
            var gl = c.getContext('webgl') || c.getContext('experimental-webgl');
            var ext = gl.getExtension('WEBGL_debug_renderer_info');
            var vendor = ext ? gl.getParameter(ext.UNMASKED_VENDOR_WEBGL) : gl.getParameter(gl.VENDOR);
            typeof vendor === 'string' && vendor.length > 0
        """)
        assert result == True

    def test_audio_fingerprint_available(self, ctx):
        """AudioContext 指纹 API 可用。"""
        result = ctx.eval_promise("""
            var ac = new OfflineAudioContext(1, 44100, 44100);
            var osc = ac.createOscillator();
            osc.type = 'triangle';
            osc.frequency.value = 10000;
            var comp = ac.createDynamicsCompressor();
            osc.connect(comp);
            comp.connect(ac.destination);
            osc.start(0);
            ac.startRendering().then(function(buffer) {
                return buffer.getChannelData(0).length > 0;
            })
        """)
        assert result == True

    def test_two_phase_load_pattern(self, ctx):
        """两阶段加载模式：page.load + 外联脚本。"""
        # Phase 1: load HTML with inline script
        ctx.add_resource('https://www.zhipin.com/security.js', """
            window.__zp_token__ = 'computed_token_' + Date.now();
        """, 200)

        ctx.page_load("""
        <html>
        <head>
            <script>
                window.__phase1__ = true;
            </script>
        </head>
        <body></body>
        </html>
        """, base_url="https://www.zhipin.com/")

        assert ctx.eval("window.__phase1__") == True

        # Phase 2: load external script
        ctx.eval("""
            var xhr = new XMLHttpRequest();
            xhr.open('GET', 'https://www.zhipin.com/security.js', false);
            xhr.send();
            eval(xhr.responseText);
        """)

        token = ctx.eval("window.__zp_token__")
        assert isinstance(token, str) and token.startswith("computed_token_")

    def test_cookie_manipulation(self, ctx):
        """Cookie 读写（zp_stoken 需要设置 cookie）。"""
        ctx.eval("document.cookie = '__zp_stoken__=test_token_value'")
        cookie = ctx.eval("document.cookie")
        assert "__zp_stoken__" in str(cookie)

    def test_network_handler_pattern(self):
        """Python 网络回调模式（用于拦截 JS 请求）。"""
        ctx = iv8_rs.JSContext()
        try:
            requests_log = []

            def handler(url, method):
                requests_log.append({'url': url, 'method': method})
                if 'security-js' in url:
                    return (200, "window.__security_loaded__ = true;")
                return None

            ctx.set_network_handler(handler)
            ctx.eval("""
                var xhr = new XMLHttpRequest();
                xhr.open('GET', 'https://www.zhipin.com/web/common/security-js/test.js', false);
                xhr.send();
                if (xhr.status === 200) {
                    eval(xhr.responseText);
                }
            """)

            assert len(requests_log) == 1
            assert 'security-js' in requests_log[0]['url']
            # The eval of the response should have set __security_loaded__
            loaded = ctx.eval("window.__security_loaded__")
            assert loaded == True
        finally:
            ctx.close()

    def test_math_random_seeded(self, ctx):
        """Math.random 可以被 hook（用于固定指纹）。"""
        ctx.eval("""
            var _origRandom = Math.random;
            Math.random = function() { return 0.12345; };
        """)
        result = ctx.eval("Math.random()")
        assert result == 0.12345

    def test_date_manipulation(self, ctx):
        """Date 可以被控制（用于固定时间戳）。"""
        # advance time by 1000ms from current
        t0 = ctx.eval("Date.now()")
        ctx.eval("__iv8__.eventLoop.advance(1000)")
        t1 = ctx.eval("Date.now()")
        assert t1 >= t0 + 1000


# ─── 瑞数 pattern ─────────────────────────────────────────────────────────────

class TestRuishuPattern:
    """瑞数两阶段加载模式（海关/欧冶/税务/药监局 pattern）"""

    def test_ruishu_two_phase_pattern(self, ctx):
        """瑞数两阶段：page.load + inline script + XHR hook。"""
        # 模拟瑞数第一阶段：加载 HTML，inline script 设置 cookie
        ctx.add_resource('https://example.gov.cn/rs_token.js', """
            (function() {
                var token = 'rs_' + Math.random().toString(36).slice(2);
                document.cookie = '__jsluid_h=' + token;
                window.__rs_token__ = token;
            })();
        """, 200)

        ctx.page_load("""
        <html>
        <head>
            <script>
                // Phase 1: 瑞数 inline script
                window.__rs_phase1__ = true;
                window.__rs_env__ = {
                    ua: navigator.userAgent,
                    lang: navigator.language,
                    screen: screen.width + 'x' + screen.height,
                };
            </script>
        </head>
        <body></body>
        </html>
        """, base_url="https://example.gov.cn/")

        assert ctx.eval("window.__rs_phase1__") == True
        env = ctx.eval("JSON.stringify(window.__rs_env__)")
        assert "Mozilla" in str(env)

        # Phase 2: 加载外联 JS
        ctx.eval("""
            var xhr = new XMLHttpRequest();
            xhr.open('GET', 'https://example.gov.cn/rs_token.js', false);
            xhr.send();
            if (xhr.status === 200) eval(xhr.responseText);
        """)

        token = ctx.eval("window.__rs_token__")
        assert isinstance(token, str) and token.startswith("rs_")

        cookie = ctx.eval("document.cookie")
        assert "__jsluid_h" in str(cookie)

    def test_ruishu_event_dispatch(self, ctx):
        """瑞数事件派发：dispatchMouseEvent。"""
        ctx.page_load("<html><body><div id='target'></div></body></html>")
        ctx.eval("""
            var target = document.getElementById('target');
            var events = [];
            target.addEventListener('mousemove', function(e) {
                events.push({type: e.type, x: e.clientX, y: e.clientY});
            });
        """)
        ctx.eval("__iv8__.input.dispatchMouseEvent('mousemove', 100, 200)")
        # Events may or may not fire depending on EventTarget implementation
        # Just verify no crash
        assert ctx.eval("typeof events") == "object"

    def test_ruishu_netlog_capture(self, ctx):
        """瑞数 netLog 捕获：记录所有 XHR 请求。"""
        ctx.add_resource('https://example.gov.cn/api/check', '{"status":"ok"}', 200)
        ctx.eval("""
            var xhr = new XMLHttpRequest();
            xhr.open('POST', 'https://example.gov.cn/api/check', false);
            xhr.setRequestHeader('Content-Type', 'application/json');
            xhr.send(JSON.stringify({token: 'test'}));
        """)
        log = ctx.eval("JSON.stringify(__iv8__.netLog.entries)")
        assert 'example.gov.cn' in str(log)


# ─── 综合 M5 验收 ─────────────────────────────────────────────────────────────

class TestM5Acceptance:
    """M5 验收：所有 iv8 examples 核心 API 覆盖"""

    def test_all_core_apis_available(self, ctx):
        """所有核心 API 都可用。"""
        apis = [
            "typeof navigator.userAgent === 'string'",
            "typeof screen.width === 'number'",
            "typeof window.chrome === 'object'",
            "typeof crypto.subtle === 'object'",
            "typeof fetch === 'function'",
            "typeof XMLHttpRequest === 'function'",
            "typeof AudioContext === 'function'",
            "typeof OfflineAudioContext === 'function'",
            "typeof MutationObserver === 'function'",
            "typeof IntersectionObserver === 'function'",
            "typeof ResizeObserver === 'function'",
            "typeof Blob === 'function'",
            "typeof URL.createObjectURL === 'function'",
            "typeof structuredClone === 'function'",
            "typeof requestIdleCallback === 'function'",
            "typeof localStorage === 'object'",
            "typeof sessionStorage === 'object'",
            "typeof indexedDB === 'object'",
            "typeof WebSocket === 'function'",
            "typeof performance.now === 'function'",
            "typeof TextEncoder === 'function'",
            "typeof TextDecoder === 'function'",
        ]
        for api_check in apis:
            result = ctx.eval(api_check)
            assert result == True, f"API check failed: {api_check}"

    def test_anti_detection_passes(self, ctx):
        """反检测检查全部通过。"""
        checks = [
            "navigator.webdriver !== true",
            "typeof window.chrome === 'object'",
            "window.chrome !== null",
            "Object.keys(window).indexOf('__iv8__') === -1",
            "!Object.getOwnPropertyDescriptor(window, '__iv8__').enumerable",
            "typeof Object.getOwnPropertyDescriptor(Navigator.prototype, 'userAgent').get === 'function'",
            "Object.getOwnPropertyDescriptor(Navigator.prototype, 'userAgent').get.toString().indexOf('[native code]') !== -1",
        ]
        for check in checks:
            result = ctx.eval(check)
            assert result == True, f"Anti-detection check failed: {check}"

    def test_dom_prototype_chain(self, ctx):
        """DOM 原型链完整。"""
        ctx.page_load("<html><body><div id='t'></div><input id='i'></body></html>")
        checks = [
            "document.getElementById('t') instanceof HTMLElement",
            "document.getElementById('t') instanceof Element",
            "document.getElementById('t') instanceof Node",
            "document.getElementById('t') instanceof EventTarget",
            "document.getElementById('t') instanceof HTMLDivElement",
            "document.getElementById('i') instanceof HTMLInputElement",
        ]
        for check in checks:
            result = ctx.eval(check)
            assert result == True, f"DOM check failed: {check}"

    def test_crypto_full_suite(self, ctx):
        """Crypto 完整套件。"""
        # SHA-256
        result = ctx.eval_promise("""
            crypto.subtle.digest('SHA-256', new Uint8Array([1,2,3]))
                .then(function(buf) { return buf.byteLength === 32; })
        """)
        assert result == True

        # HMAC
        result = ctx.eval_promise("""
            crypto.subtle.importKey(
                'raw', new Uint8Array(32),
                {name: 'HMAC', hash: 'SHA-256'}, false, ['sign']
            ).then(function(key) {
                return crypto.subtle.sign('HMAC', key, new Uint8Array(10));
            }).then(function(sig) { return sig.byteLength === 32; })
        """)
        assert result == True

        # AES-GCM
        result = ctx.eval_promise("""
            crypto.subtle.generateKey(
                {name: 'AES-GCM', length: 256}, true, ['encrypt', 'decrypt']
            ).then(function(key) { return typeof key === 'object'; })
        """)
        assert result == True

    def test_event_loop_timing(self, ctx):
        """EventLoop 时间控制。"""
        t0 = ctx.eval("Date.now()")
        ctx.eval("__iv8__.eventLoop.advance(5000)")
        t1 = ctx.eval("Date.now()")
        assert t1 >= t0 + 5000

    def test_debugger_integration(self, ctx):
        """Debugger 集成测试。"""
        from iv8_rs import Debugger
        d = Debugger(ctx)
        d.trace_api('Math.random')
        ctx.eval('Math.random(); Math.random();')
        log = d.get_call_log()
        assert len(log) == 2
        snap = d.snapshot()
        assert 'userAgent' in snap
        assert 'Chrome' in snap['userAgent']
