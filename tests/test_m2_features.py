"""M2 feature tests: DOM, EventLoop, page.load, timers, DateInterceptor, add_resource."""
import pytest
import iv8_rs
import math


# ─── page.load ───────────────────────────────────────────────────────────────

class TestPageLoad:
    def test_basic_html(self):
        ctx = iv8_rs.JSContext()
        ctx.page_load("<div id='app'>Hello</div>")
        assert ctx.eval("document.getElementById('app').textContent") == "Hello"
        ctx.close()

    def test_inline_script_executes(self):
        ctx = iv8_rs.JSContext()
        ctx.page_load("<script>globalThis.x = 42;</script>")
        assert ctx.eval("globalThis.x") == 42
        ctx.close()

    def test_multiple_scripts_order(self):
        ctx = iv8_rs.JSContext()
        ctx.page_load("""
            <script>globalThis.order = [];</script>
            <script>globalThis.order.push('a');</script>
            <script>globalThis.order.push('b');</script>
        """)
        assert ctx.eval("globalThis.order") == ["a", "b"]
        ctx.close()

    def test_script_accesses_dom(self):
        ctx = iv8_rs.JSContext()
        ctx.page_load("""
            <p id="msg">world</p>
            <script>globalThis.greeting = 'hello ' + document.getElementById('msg').textContent;</script>
        """)
        assert ctx.eval("globalThis.greeting") == "hello world"
        ctx.close()

    def test_script_error_non_fatal(self):
        ctx = iv8_rs.JSContext()
        ctx.page_load("""
            <script>throw new Error('fail');</script>
            <script>globalThis.ok = true;</script>
        """)
        assert ctx.eval("globalThis.ok") is True
        ctx.close()


# ─── DOM Query ───────────────────────────────────────────────────────────────

class TestDOMQuery:
    def setup_method(self):
        self.ctx = iv8_rs.JSContext()
        self.ctx.page_load("""
            <div id="container" class="wrapper">
                <p class="item">First</p>
                <p class="item special">Second</p>
                <span class="item">Third</span>
            </div>
        """)

    def teardown_method(self):
        self.ctx.close()

    def test_get_element_by_id(self):
        assert self.ctx.eval("document.getElementById('container').tagName") == "DIV"

    def test_query_selector_tag(self):
        assert self.ctx.eval("document.querySelector('span').textContent") == "Third"

    def test_query_selector_class(self):
        assert self.ctx.eval("document.querySelector('.special').textContent") == "Second"

    def test_query_selector_all_count(self):
        assert self.ctx.eval("document.querySelectorAll('.item').length") == 3

    def test_get_elements_by_tag_name(self):
        assert self.ctx.eval("document.getElementsByTagName('p').length") == 2

    def test_get_elements_by_class_name(self):
        assert self.ctx.eval("document.getElementsByClassName('item').length") == 3

    def test_get_attribute(self):
        assert self.ctx.eval("document.getElementById('container').getAttribute('class')") == "wrapper"

    def test_query_selector_not_found(self):
        assert self.ctx.eval("document.querySelector('.nonexistent')") is None


# ─── DOM Mutation ────────────────────────────────────────────────────────────

class TestDOMMutation:
    def setup_method(self):
        self.ctx = iv8_rs.JSContext()
        self.ctx.page_load("<div id='root'></div>")

    def teardown_method(self):
        self.ctx.close()

    def test_create_element(self):
        assert self.ctx.eval("document.createElement('span').tagName") == "SPAN"

    def test_append_child(self):
        self.ctx.eval("""
            var root = document.getElementById('root');
            var child = document.createElement('p');
            root.appendChild(child);
        """)
        assert self.ctx.eval("document.querySelectorAll('p').length") == 1

    def test_set_attribute(self):
        self.ctx.eval("""
            var root = document.getElementById('root');
            root.setAttribute('data-x', 'hello');
        """)
        assert self.ctx.eval("document.getElementById('root').getAttribute('data-x')") == "hello"

    def test_remove_child(self):
        self.ctx.eval("""
            var root = document.getElementById('root');
            var child = document.createElement('div');
            root.appendChild(child);
            root.removeChild(child);
        """)
        assert self.ctx.eval("document.querySelectorAll('div').length") == 1  # only #root


# ─── EventLoop ───────────────────────────────────────────────────────────────

class TestEventLoop:
    def setup_method(self):
        self.ctx = iv8_rs.JSContext()

    def teardown_method(self):
        self.ctx.close()

    def test_get_time_initial(self):
        assert self.ctx.eval("__iv8__.eventLoop.getTime()") == 0

    def test_advance(self):
        self.ctx.eval("__iv8__.eventLoop.advance(100)")
        assert self.ctx.eval("__iv8__.eventLoop.getTime()") == 100

    def test_sleep(self):
        self.ctx.eval("__iv8__.eventLoop.sleep(50)")
        assert self.ctx.eval("__iv8__.eventLoop.getTime()") == 50

    def test_tick_default_step(self):
        self.ctx.eval("__iv8__.eventLoop.tick()")
        assert self.ctx.eval("__iv8__.eventLoop.getTime()") == 4  # 4ms default

    def test_tick_explicit(self):
        self.ctx.eval("__iv8__.eventLoop.tick(10)")
        assert self.ctx.eval("__iv8__.eventLoop.getTime()") == 10

    def test_reset(self):
        self.ctx.eval("__iv8__.eventLoop.advance(999)")
        self.ctx.eval("__iv8__.eventLoop.reset()")
        assert self.ctx.eval("__iv8__.eventLoop.getTime()") == 0

    def test_set_auto_advance_step(self):
        self.ctx.eval("__iv8__.eventLoop.setAutoAdvanceStep(2000)")  # 2ms
        self.ctx.eval("__iv8__.eventLoop.tick()")
        assert self.ctx.eval("__iv8__.eventLoop.getTime()") == 2


# ─── Timers ──────────────────────────────────────────────────────────────────

class TestTimers:
    def setup_method(self):
        self.ctx = iv8_rs.JSContext()

    def teardown_method(self):
        self.ctx.close()

    def test_set_timeout_fires(self):
        self.ctx.eval("globalThis.fired = false; setTimeout(function(){ globalThis.fired = true; }, 100);")
        self.ctx.eval("__iv8__.eventLoop.advance(200)")
        assert self.ctx.eval("globalThis.fired") is True

    def test_set_timeout_not_before_deadline(self):
        self.ctx.eval("globalThis.fired = false; setTimeout(function(){ globalThis.fired = true; }, 100);")
        self.ctx.eval("__iv8__.eventLoop.advance(50)")
        assert self.ctx.eval("globalThis.fired") is False

    def test_set_interval_fires_multiple(self):
        self.ctx.eval("globalThis.count = 0; setInterval(function(){ globalThis.count++; }, 50);")
        self.ctx.eval("__iv8__.eventLoop.advance(200)")
        count = self.ctx.eval("globalThis.count")
        assert count >= 3

    def test_clear_timeout(self):
        self.ctx.eval("""
            globalThis.fired = false;
            var id = setTimeout(function(){ globalThis.fired = true; }, 100);
            clearTimeout(id);
        """)
        self.ctx.eval("__iv8__.eventLoop.advance(200)")
        assert self.ctx.eval("globalThis.fired") is False

    def test_request_animation_frame(self):
        self.ctx.eval("globalThis.rafFired = false; requestAnimationFrame(function(){ globalThis.rafFired = true; });")
        self.ctx.eval("__iv8__.eventLoop.advance(20)")
        assert self.ctx.eval("globalThis.rafFired") is True


# ─── DateInterceptor ─────────────────────────────────────────────────────────

class TestDateInterceptor:
    def setup_method(self):
        self.ctx = iv8_rs.JSContext()

    def teardown_method(self):
        self.ctx.close()

    def test_date_now_deterministic(self):
        assert self.ctx.eval("Date.now() === Date.now()") is True

    def test_date_now_advances(self):
        before = self.ctx.eval("Date.now()")
        self.ctx.eval("__iv8__.eventLoop.advance(1000)")
        after = self.ctx.eval("Date.now()")
        assert abs((after - before) - 1000) < 1

    def test_performance_now_starts_zero(self):
        result = self.ctx.eval("performance.now()")
        assert abs(result) < 1

    def test_performance_now_advances(self):
        self.ctx.eval("__iv8__.eventLoop.advance(500)")
        result = self.ctx.eval("performance.now()")
        assert abs(result - 500) < 1

    def test_new_date_uses_logical_time(self):
        t = self.ctx.eval("new Date().getTime()")
        # Should be epoch 2024-01-01
        assert abs(t - 1704067200000) < 1


# ─── add_resource ────────────────────────────────────────────────────────────

class TestAddResource:
    def test_add_resource_no_crash(self):
        ctx = iv8_rs.JSContext()
        ctx.add_resource("https://example.com/api", b'{"ok":true}', 200)
        ctx.close()

    def test_add_resource_string_body(self):
        ctx = iv8_rs.JSContext()
        ctx.add_resource("https://example.com/text", "hello world", 200)
        ctx.close()

    def test_add_resource_with_headers(self):
        ctx = iv8_rs.JSContext()
        ctx.add_resource(
            "https://example.com/json",
            b'{"data":1}',
            200,
            {"content-type": "application/json"},
        )
        ctx.close()


# ─── expose_module ───────────────────────────────────────────────────────────

class TestExposeModule:
    def test_expose_math(self):
        ctx = iv8_rs.JSContext()
        ctx.expose_module(math)
        assert ctx.eval("floor(3.7)") == 3
        assert ctx.eval("sqrt(16)") == 4
        ctx.close()

    def test_expose_custom_module(self):
        import types
        mod = types.ModuleType("mymod")
        mod.__all__ = ["greet"]
        mod.greet = lambda name: f"hi {name}"
        ctx = iv8_rs.JSContext()
        ctx.expose_module(mod)
        assert ctx.eval('greet("world")') == "hi world"
        ctx.close()


# ─── EventTarget ─────────────────────────────────────────────────────────────

class TestEventTarget:
    def test_add_event_listener_and_dispatch(self):
        ctx = iv8_rs.JSContext()
        ctx.page_load("<div id='btn'>click me</div>")
        ctx.eval("""
            globalThis.clicked = false;
            var el = document.getElementById('btn');
            el.addEventListener('click', function() { globalThis.clicked = true; });
            el.dispatchEvent({type: 'click', bubbles: false});
        """)
        assert ctx.eval("globalThis.clicked") is True
        ctx.close()

    def test_once_listener(self):
        ctx = iv8_rs.JSContext()
        ctx.page_load("<div id='target'></div>")
        ctx.eval("""
            globalThis.count = 0;
            var el = document.getElementById('target');
            el.addEventListener('ping', function() { globalThis.count++; }, {once: true});
            el.dispatchEvent({type: 'ping', bubbles: false});
            el.dispatchEvent({type: 'ping', bubbles: false});
        """)
        assert ctx.eval("globalThis.count") == 1
        ctx.close()
