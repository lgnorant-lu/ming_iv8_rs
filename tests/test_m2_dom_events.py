"""
M2 DOM manipulation + event propagation tests.

Migrated from the historical ad-hoc audit scripts (audit_deep/audit_extended)
into proper pytest assertions. Covers DOM methods and event behaviors that were
NOT previously in the tracked test suite:
  - removeEventListener / once listener / stopPropagation / capture+bubble order
  - insertBefore / replaceChild / removeChild / cloneNode
  - removeAttribute / insertAdjacentHTML / innerHTML setter
  - classList add/remove / nextElementSibling / closest / matches
"""
import pytest
import iv8_rs


@pytest.fixture
def ctx():
    c = iv8_rs.JSContext()
    yield c
    c.close()


# ============================================================
# Event listener lifecycle
# ============================================================
class TestEventListeners:
    def test_remove_event_listener(self, ctx):
        ctx.page_load('<html><body><div id="t"></div></body></html>')
        ctx.eval("""
            var fired = 0;
            var handler = function() { fired++; };
            var el = document.getElementById('t');
            el.addEventListener('click', handler);
            el.dispatchEvent({type:'click', bubbles:true});
            el.removeEventListener('click', handler);
            el.dispatchEvent({type:'click', bubbles:true});
        """)
        assert ctx.eval("fired") == 1

    def test_once_listener_fires_once(self, ctx):
        ctx.page_load('<html><body><div id="t"></div></body></html>')
        ctx.eval("""
            var count = 0;
            var el = document.getElementById('t');
            el.addEventListener('click', function() { count++; }, {once: true});
            el.dispatchEvent({type:'click', bubbles:false});
            el.dispatchEvent({type:'click', bubbles:false});
        """)
        assert ctx.eval("count") == 1

    def test_capture_then_target_then_bubble_order(self, ctx):
        ctx.page_load('<html><body><div id="outer"><div id="inner"></div></div></body></html>')
        ctx.eval("""
            var log = [];
            document.getElementById('outer').addEventListener('click', function() { log.push('outer-bubble'); });
            document.getElementById('outer').addEventListener('click', function() { log.push('outer-capture'); }, true);
            document.getElementById('inner').addEventListener('click', function() { log.push('inner'); });
            document.getElementById('inner').dispatchEvent({type:'click', bubbles:true});
        """)
        assert ctx.eval("log.join(',')") == 'outer-capture,inner,outer-bubble'

    def test_stop_propagation_prevents_bubble(self, ctx):
        ctx.page_load('<html><body><div id="outer"><div id="inner"></div></div></body></html>')
        ctx.eval("""
            var outerFired = 0;
            document.getElementById('outer').addEventListener('click', function() { outerFired++; });
            document.getElementById('inner').addEventListener('click', function(e) { e.stopPropagation(); });
            document.getElementById('inner').dispatchEvent({type:'click', bubbles:true});
        """)
        assert ctx.eval("outerFired") == 0


# ============================================================
# DOM tree manipulation
# ============================================================
class TestDomManipulation:
    def test_insert_before(self, ctx):
        ctx.page_load('<html><body><div id="t"></div></body></html>')
        ctx.eval("""
            var parent = document.getElementById('t');
            var c1 = document.createElement('span'); c1.setAttribute('id','c1');
            var c2 = document.createElement('span'); c2.setAttribute('id','c2');
            parent.appendChild(c1);
            parent.insertBefore(c2, c1);
        """)
        assert ctx.eval("document.getElementById('t').firstChild.id") == 'c2'

    def test_replace_child(self, ctx):
        ctx.page_load('<html><body><div id="parent"><span id="old">old</span></div></body></html>')
        ctx.eval("""
            var parent = document.getElementById('parent');
            var oldEl = document.getElementById('old');
            var newEl = document.createElement('p'); newEl.setAttribute('id','new');
            parent.replaceChild(newEl, oldEl);
        """)
        assert ctx.eval("document.getElementById('new') !== null") is True
        assert ctx.eval("document.getElementById('old')") is None

    def test_remove_child(self, ctx):
        ctx.page_load('<html><body><div id="parent"><span id="child">x</span></div></body></html>')
        ctx.eval("""
            var parent = document.getElementById('parent');
            parent.removeChild(document.getElementById('child'));
        """)
        assert ctx.eval("document.getElementById('child')") is None

    def test_clone_node_deep(self, ctx):
        ctx.page_load('<html><body><div id="t"><span id="child">text</span></div></body></html>')
        ctx.eval("""
            var orig = document.getElementById('t');
            var clone = orig.cloneNode(true);
            document.body.appendChild(clone);
        """)
        assert ctx.eval("document.getElementById('t') !== null") is True
        assert ctx.eval("clone.children.length") >= 1

    def test_remove_attribute(self, ctx):
        ctx.page_load('<html><body><div id="t" data-x="1"></div></body></html>')
        ctx.eval("document.getElementById('t').removeAttribute('data-x')")
        assert ctx.eval("document.getElementById('t').hasAttribute('data-x')") is False

    def test_insert_adjacent_html_beforeend(self, ctx):
        ctx.page_load('<html><body><div id="t"></div></body></html>')
        ctx.eval("document.getElementById('t').insertAdjacentHTML('beforeend', '<span id=\"adj\">a</span>')")
        assert ctx.eval("document.getElementById('adj') !== null") is True

    def test_insert_adjacent_html_afterbegin_order(self, ctx):
        ctx.page_load('<html><body><div id="t"><span id="existing">x</span></div></body></html>')
        ctx.eval("""
            var t = document.getElementById('t');
            t.insertAdjacentHTML('afterbegin', '<span id="ab">ab</span>');
        """)
        assert ctx.eval("document.getElementById('t').firstChild.id") == 'ab'

    def test_inner_html_setter(self, ctx):
        ctx.page_load('<html><body><div id="t"></div></body></html>')
        ctx.eval("document.getElementById('t').innerHTML = '<span id=\"s\">hi</span>'")
        assert ctx.eval("document.getElementById('s').textContent") == 'hi'


# ============================================================
# DOM traversal + classList
# ============================================================
class TestDomTraversal:
    def test_classlist_add_remove(self, ctx):
        ctx.page_load('<html><body><div id="t" class="foo bar"></div></body></html>')
        ctx.eval("document.getElementById('t').classList.add('baz')")
        assert ctx.eval("document.getElementById('t').classList.contains('baz')") is True
        ctx.eval("document.getElementById('t').classList.remove('foo')")
        assert ctx.eval("document.getElementById('t').classList.contains('foo')") is False

    def test_next_element_sibling(self, ctx):
        ctx.page_load('<html><body><ul><li>1</li><li>2</li></ul></body></html>')
        assert ctx.eval("document.querySelector('li').nextElementSibling !== null") is True

    def test_closest(self, ctx):
        ctx.page_load('<html><body><ul id="u"><li>1</li></ul></body></html>')
        assert ctx.eval("document.querySelector('li').closest('ul') !== null") is True

    def test_matches(self, ctx):
        ctx.page_load('<html><body><li class="x">1</li></body></html>')
        assert ctx.eval("document.querySelector('li').matches('.x')") is True

    def test_first_last_element_child(self, ctx):
        ctx.page_load('<html><body><ul id="u"><li>1</li><li>2</li><li>3</li></ul></body></html>')
        assert ctx.eval("document.getElementById('u').firstElementChild !== null") is True
        assert ctx.eval("document.getElementById('u').lastElementChild !== null") is True

    def test_contains(self, ctx):
        ctx.page_load('<html><body><ul id="u"><li>1</li></ul></body></html>')
        assert ctx.eval("document.getElementById('u').contains(document.querySelector('li'))") is True

    def test_node_type_and_name(self, ctx):
        ctx.page_load('<html><body><ul id="u"><li>1</li></ul></body></html>')
        assert ctx.eval("document.getElementById('u').nodeType") == 1
        assert ctx.eval("document.getElementById('u').nodeName") == 'UL'
        assert ctx.eval("document.querySelector('li').firstChild.nodeType") == 3
