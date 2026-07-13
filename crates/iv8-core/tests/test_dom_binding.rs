#![allow(
    clippy::expect_used,
    clippy::panic,
    clippy::unwrap_used,
    unused_imports,
    unused_variables
)]
mod common;

// Integration tests for DOM V8 bindings (Task 27).
// Tests document.getElementById, querySelector, querySelectorAll, getElementsByTagName.

use iv8_core::RustValue;

#[test]
fn get_element_by_id_found() {
    let mut kernel = common::make_kernel_with_doc("<div id=\"main\">hello</div>");
    let result = kernel.eval_to_rust_value("document.getElementById('main').tagName");
    assert_eq!(result, RustValue::String("DIV".into()));
}

#[test]
fn get_element_by_id_text_content() {
    let mut kernel = common::make_kernel_with_doc("<p id=\"msg\">Hello World</p>");
    let result = kernel.eval_to_rust_value("document.getElementById('msg').textContent");
    assert_eq!(result, RustValue::String("Hello World".into()));
}

#[test]
fn get_element_by_id_not_found() {
    let mut kernel = common::make_kernel_with_doc("<div>no id</div>");
    let result = kernel.eval_to_rust_value("document.getElementById('missing')");
    assert_eq!(result, RustValue::Null);
}

#[test]
fn query_selector_by_tag() {
    let mut kernel = common::make_kernel_with_doc("<div><p>first</p><p>second</p></div>");
    let result = kernel.eval_to_rust_value("document.querySelector('p').textContent");
    assert_eq!(result, RustValue::String("first".into()));
}

#[test]
fn query_selector_by_class() {
    let mut kernel = common::make_kernel_with_doc("<span class=\"highlight\">text</span>");
    let result = kernel.eval_to_rust_value("document.querySelector('.highlight').tagName");
    assert_eq!(result, RustValue::String("SPAN".into()));
}

#[test]
fn query_selector_not_found() {
    let mut kernel = common::make_kernel_with_doc("<div>hello</div>");
    let result = kernel.eval_to_rust_value("document.querySelector('.missing')");
    assert_eq!(result, RustValue::Null);
}

#[test]
fn query_selector_all_count() {
    let mut kernel = common::make_kernel_with_doc("<ul><li>1</li><li>2</li><li>3</li></ul>");
    let result = kernel.eval_to_rust_value("document.querySelectorAll('li').length");
    assert_eq!(result, RustValue::Int(3));
}

#[test]
fn query_selector_all_access_items() {
    let mut kernel = common::make_kernel_with_doc("<div><p>a</p><p>b</p></div>");
    let result = kernel.eval_to_rust_value("document.querySelectorAll('p')[1].textContent");
    assert_eq!(result, RustValue::String("b".into()));
}

#[test]
fn get_elements_by_tag_name() {
    let mut kernel = common::make_kernel_with_doc("<div><span>1</span><span>2</span></div>");
    let result = kernel.eval_to_rust_value("document.getElementsByTagName('span').length");
    assert_eq!(result, RustValue::Int(2));
}

#[test]
fn get_elements_by_class_name() {
    let mut kernel = common::make_kernel_with_doc(
        "<div class=\"a\"><p class=\"b\">1</p><p class=\"b\">2</p></div>",
    );
    let result = kernel.eval_to_rust_value("document.getElementsByClassName('b').length");
    assert_eq!(result, RustValue::Int(2));
}

#[test]
fn get_attribute() {
    let mut kernel =
        common::make_kernel_with_doc("<a href=\"https://example.com\" target=\"_blank\">link</a>");
    let result = kernel.eval_to_rust_value("document.querySelector('a').getAttribute('href')");
    assert_eq!(result, RustValue::String("https://example.com".into()));
}

#[test]
fn get_attribute_missing() {
    let mut kernel = common::make_kernel_with_doc("<div>no attrs</div>");
    let result = kernel.eval_to_rust_value("document.querySelector('div').getAttribute('data-x')");
    assert_eq!(result, RustValue::Null);
}

#[test]
fn element_id_property() {
    let mut kernel = common::make_kernel_with_doc("<div id=\"test\">content</div>");
    let result = kernel.eval_to_rust_value("document.getElementById('test').id");
    assert_eq!(result, RustValue::String("test".into()));
}

#[test]
fn element_class_name_property() {
    let mut kernel = common::make_kernel_with_doc("<div class=\"foo bar\">content</div>");
    let result = kernel.eval_to_rust_value("document.querySelector('div').className");
    assert_eq!(result, RustValue::String("foo bar".into()));
}

#[test]
fn element_node_type() {
    let mut kernel = common::make_kernel_with_doc("<div>content</div>");
    let result = kernel.eval_to_rust_value("document.querySelector('div').nodeType");
    assert_eq!(result, RustValue::Int(1));
}

// ─── Task 28: DOM Mutation Tests ────────────────────────────────────────────

#[test]
fn create_element() {
    let mut kernel = common::make_kernel_with_doc("<body></body>");
    let result = kernel.eval_to_rust_value("document.createElement('div').tagName");
    assert_eq!(result, RustValue::String("DIV".into()));
}

#[test]
fn create_element_node_type() {
    let mut kernel = common::make_kernel_with_doc("<body></body>");
    let result = kernel.eval_to_rust_value("document.createElement('span').nodeType");
    assert_eq!(result, RustValue::Int(1));
}

#[test]
fn append_child_adds_to_dom() {
    let mut kernel = common::make_kernel_with_doc("<div id=\"container\"></div>");
    kernel.eval_to_rust_value(
        r#"
        var container = document.getElementById('container');
        var child = document.createElement('p');
        container.appendChild(child);
    "#,
    );
    // Now querySelectorAll should find the new p inside container
    let result = kernel.eval_to_rust_value("document.querySelectorAll('p').length");
    assert_eq!(result, RustValue::Int(1));
}

#[test]
fn remove_child_removes_from_dom() {
    let mut kernel =
        common::make_kernel_with_doc("<div id=\"parent\"><p id=\"child\">text</p></div>");
    kernel.eval_to_rust_value(
        r#"
        var parent = document.getElementById('parent');
        var child = document.getElementById('child');
        parent.removeChild(child);
    "#,
    );
    let result = kernel.eval_to_rust_value("document.querySelectorAll('p').length");
    assert_eq!(result, RustValue::Int(0));
}

#[test]
fn set_attribute_updates_dom() {
    let mut kernel = common::make_kernel_with_doc("<div id=\"target\"></div>");
    kernel.eval_to_rust_value(
        r#"
        var el = document.getElementById('target');
        el.setAttribute('data-value', 'hello');
    "#,
    );
    let result = kernel.eval_to_rust_value(
        r#"
        document.getElementById('target').getAttribute('data-value')
    "#,
    );
    assert_eq!(result, RustValue::String("hello".into()));
}

#[test]
fn set_attribute_id_updates_index() {
    let mut kernel = common::make_kernel_with_doc("<div id=\"old\"></div>");
    kernel.eval_to_rust_value(
        r#"
        var el = document.getElementById('old');
        el.setAttribute('id', 'new');
    "#,
    );
    // Should be findable by new id
    let result = kernel.eval_to_rust_value("document.getElementById('new').tagName");
    assert_eq!(result, RustValue::String("DIV".into()));
}

#[test]
fn append_child_returns_child() {
    let mut kernel = common::make_kernel_with_doc("<div id=\"parent\"></div>");
    let result = kernel.eval_to_rust_value(
        r#"
        var parent = document.getElementById('parent');
        var child = document.createElement('span');
        var returned = parent.appendChild(child);
        returned.tagName
    "#,
    );
    assert_eq!(result, RustValue::String("SPAN".into()));
}

#[test]
fn replace_child_owned_by_node_prototype_only() {
    let mut kernel = common::make_kernel_with_doc("<div id=\"parent\"></div>");
    let result = kernel.eval_to_rust_value(
        r#"
        (function () {
          var parent = document.getElementById('parent');
          var a = document.createElement('span');
          var b = document.createElement('p');
          parent.appendChild(a);
          var returned = parent.replaceChild(b, a);
          var throwsType = false;
          try { parent.replaceChild(b, 'test'); } catch (e) {
            throwsType = e instanceof TypeError;
          }
          return JSON.stringify({
            hasOwnElement: Object.prototype.hasOwnProperty.call(Element.prototype, 'replaceChild'),
            hasOwnNode: Object.prototype.hasOwnProperty.call(Node.prototype, 'replaceChild'),
            same: parent.replaceChild === Node.prototype.replaceChild,
            length: Node.prototype.replaceChild.length,
            childTag: parent.firstChild && parent.firstChild.tagName,
            returned: returned && returned.tagName,
            throwsType: throwsType,
            hasOwnInsertBeforeEl: Object.prototype.hasOwnProperty.call(Element.prototype, 'insertBefore'),
            hasOwnCloneEl: Object.prototype.hasOwnProperty.call(Element.prototype, 'cloneNode'),
            hasOwnContainsEl: Object.prototype.hasOwnProperty.call(Element.prototype, 'contains')
          });
        })()
    "#,
    );
    let s = match result {
        RustValue::String(v) => v,
        other => panic!("expected string, got {other:?}"),
    };
    assert!(s.contains("\"hasOwnElement\":false"), "{s}");
    assert!(s.contains("\"hasOwnNode\":true"), "{s}");
    assert!(s.contains("\"same\":true"), "{s}");
    assert!(s.contains("\"length\":2"), "{s}");
    assert!(s.contains("\"childTag\":\"P\""), "{s}");
    assert!(s.contains("\"returned\":\"SPAN\""), "{s}");
    assert!(s.contains("\"throwsType\":true"), "{s}");
    assert!(s.contains("\"hasOwnInsertBeforeEl\":false"), "{s}");
    assert!(s.contains("\"hasOwnCloneEl\":false"), "{s}");
    assert!(s.contains("\"hasOwnContainsEl\":false"), "{s}");
}

#[test]
fn select_selected_index_and_option_value_reflect_tree() {
    let mut kernel = common::make_kernel();
    let s = kernel.eval_to_rust_value(
        r#"
        (function () {
          var s = document.createElement('select');
          var o1 = document.createElement('option'); o1.textContent = 'A';
          var o2 = document.createElement('option'); o2.textContent = 'B';
          o2.setAttribute('selected', '');
          s.appendChild(o1); s.appendChild(o2);
          var afterSetOk = false;
          s.selectedIndex = 0;
          afterSetOk = s.selectedIndex === 0 && o1.selected === true && o2.selected === false;
          return JSON.stringify({
            selectedIndex: s.selectedIndex === 0 ? 0 : s.selectedIndex,
            length: s.length,
            optionsLen: s.options.length,
            o1val: o1.value,
            o2val: o2.value,
            o1idx: o1.index,
            o2idx: o2.index,
            instanceofOption: o1 instanceof HTMLOptionElement,
            afterSetOk: afterSetOk
          });
        })()
    "#,
    );
    // re-run with fresh select for initial selectedIndex==1
    let s2 = kernel.eval_to_rust_value(
        r#"
        (function () {
          var s = document.createElement('select');
          var o1 = document.createElement('option'); o1.textContent = 'A';
          var o2 = document.createElement('option'); o2.textContent = 'B';
          o2.setAttribute('selected', '');
          s.appendChild(o1); s.appendChild(o2);
          return JSON.stringify({
            selectedIndex: s.selectedIndex,
            o1val: o1.value,
            o2val: o2.value,
            o2selected: o2.selected,
            length: s.length
          });
        })()
    "#,
    );
    let a = match s2 {
        RustValue::String(v) => v,
        other => panic!("expected string, got {other:?}"),
    };
    assert!(a.contains("\"selectedIndex\":1"), "{a}");
    assert!(a.contains("\"o1val\":\"A\""), "{a}");
    assert!(a.contains("\"o2val\":\"B\""), "{a}");
    assert!(a.contains("\"o2selected\":true"), "{a}");
    assert!(a.contains("\"length\":2"), "{a}");
    let b = match s {
        RustValue::String(v) => v,
        other => panic!("expected string, got {other:?}"),
    };
    assert!(b.contains("\"afterSetOk\":true"), "{b}");
    assert!(b.contains("\"instanceofOption\":true"), "{b}");
}

#[test]
fn tree_walker_next_node_preorder_under_root() {
    let mut kernel = common::make_kernel_with_doc(
        "<html><body><div id=\"r\"><span>a</span><p>b</p></div></body></html>",
    );
    let result = kernel.eval_to_rust_value(
        r#"
        (function () {
          var root = document.getElementById('r');
          var tw = document.createTreeWalker(root);
          var seq = [];
          var n;
          while ((n = tw.nextNode())) {
            seq.push(n.tagName || n.nodeName);
            if (seq.length > 10) break;
          }
          // SHOW_ELEMENT = 1 → no text nodes
          var twEl = document.createTreeWalker(root, 1);
          var seqEl = [];
          while ((n = twEl.nextNode())) {
            seqEl.push(n.nodeName);
            if (seqEl.length > 10) break;
          }
          return JSON.stringify({
            rootId: tw.root && tw.root.id,
            seq: seq,
            seqEl: seqEl,
            firstChildTag: (function () {
              var t2 = document.createTreeWalker(root);
              var c = t2.firstChild();
              return c && c.tagName;
            })()
          });
        })()
    "#,
    );
    let s = match result {
        RustValue::String(v) => v,
        other => panic!("expected string, got {other:?}"),
    };
    assert!(s.contains("\"rootId\":\"r\""), "{s}");
    assert!(s.contains("SPAN"), "{s}");
    assert!(s.contains("P"), "{s}");
    assert!(s.contains("\"firstChildTag\":\"SPAN\""), "{s}");
    // whatToShow=1 must drop #text
    assert!(s.contains("\"seqEl\":[\"SPAN\",\"P\"]") || s.contains("\"seqEl\":[\"SPAN\", \"P\"]"), "{s}");
    assert!(!s.contains("\"seqEl\":[\"SPAN\",\"#text\""), "{s}");
}

#[test]
fn xpath_evaluate_subset_returns_real_nodes() {
    let mut kernel = common::make_kernel_with_doc(
        "<html><body><div id=\"r\"><span id=\"s\">a</span><p id=\"p\">b</p></div></body></html>",
    );
    let result = kernel.eval_to_rust_value(
        r#"
        (function () {
          var xe = document.createExpression('//span');
          var res = xe.evaluate(document);
          var resId = document.evaluate("id('p')", document);
          var resAttr = document.evaluate('//*[@id="s"]', document);
          return JSON.stringify({
            snap: res.snapshotLength,
            item0: res.snapshotItem(0) && res.snapshotItem(0).id,
            iter: (function () { var n = res.iterateNext(); return n && n.id; })(),
            idSnap: resId.snapshotLength,
            idItem: resId.snapshotItem(0) && resId.snapshotItem(0).id,
            attrSnap: resAttr.snapshotLength,
            attrItem: resAttr.snapshotItem(0) && resAttr.snapshotItem(0).id
          });
        })()
    "#,
    );
    let s = match result {
        RustValue::String(v) => v,
        other => panic!("expected string, got {other:?}"),
    };
    assert!(s.contains("\"snap\":1"), "{s}");
    assert!(s.contains("\"item0\":\"s\""), "{s}");
    assert!(s.contains("\"iter\":\"s\""), "{s}");
    assert!(s.contains("\"idSnap\":1"), "{s}");
    assert!(s.contains("\"idItem\":\"p\""), "{s}");
    assert!(s.contains("\"attrSnap\":1"), "{s}");
    assert!(s.contains("\"attrItem\":\"s\""), "{s}");
}

#[test]
fn document_tree_ops_on_prototype_not_own_and_cookie_survives_set_document() {
    let mut kernel = common::make_kernel();
    assert_eq!(
        kernel.eval_to_rust_value("typeof Object.getOwnPropertyDescriptor(document, 'cookie')"),
        RustValue::String("object".into())
    );
    kernel.set_document(
        "<html><head><title>Hi</title></head><body></body></html>",
        None,
    );
    assert_eq!(
        kernel.eval_to_rust_value("typeof Object.getOwnPropertyDescriptor(document, 'cookie')"),
        RustValue::String("object".into())
    );
    assert_eq!(
        kernel.eval_to_rust_value("document.title"),
        RustValue::String("Hi".into())
    );
    assert_eq!(
        kernel.eval_to_rust_value("document.createElement('div').tagName"),
        RustValue::String("DIV".into())
    );
    assert_eq!(
        kernel.eval_to_rust_value("document.body.tagName"),
        RustValue::String("BODY".into())
    );
    assert_eq!(
        kernel.eval_to_rust_value(
            "Object.prototype.hasOwnProperty.call(document, 'createElement')"
        ),
        RustValue::Bool(false)
    );
    assert_eq!(
        kernel.eval_to_rust_value("Object.prototype.hasOwnProperty.call(document, 'title')"),
        RustValue::Bool(false)
    );
    assert_eq!(
        kernel.eval_to_rust_value(
            "Object.prototype.hasOwnProperty.call(Document.prototype, 'createElement')"
        ),
        RustValue::Bool(true)
    );
}
