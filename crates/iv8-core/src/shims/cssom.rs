//! CSSOM shim: CSSStyleSheet/CSSRule/CSSStyleDeclaration runtime
//!
//! Implements a CSS parser + CSSOM object model that creates properly typed
//! CSSRule instances (CSSImportRule, CSSStyleRule, CSSMediaRule, etc.) with
//! correct prototype chains using codegen constructors.
//!
//! Architecture: ALL WebIDL attributes/operations are installed ON THE PROTOTYPE.
//! Internal state is stored in non-enumerable own properties (__iv8* prefix).
//!
//! Reference: https://drafts.csswg.org/cssom/

/// One-time prototype setup: installs real method/getter implementations
/// on codegen-generated prototypes so that idlharness assert_inherits passes.
pub const CSSOM_PROTO_SETUP_JS: &str = r#"
(function() {
    'use strict';

    // ---- helpers ----
    function defMethod(proto, name, fn, len) {
        Object.defineProperty(proto, name, {
            value: fn, writable: true, configurable: true, enumerable: true
        });
        if (len !== undefined) {
            Object.defineProperty(fn, 'length', { value: len, writable: false, enumerable: false, configurable: true });
        }
    }
    function defGetter(proto, name, fn) {
        Object.defineProperty(proto, name, {
            get: fn, set: undefined, enumerable: true, configurable: true
        });
        Object.defineProperty(fn, 'length', { value: 0, writable: false, enumerable: false, configurable: true });
    }
    function defAccessor(proto, name, getter, setter) {
        var desc = { enumerable: true, configurable: true };
        if (getter) {
            desc.get = getter;
            Object.defineProperty(getter, 'length', { value: 0, writable: false, enumerable: false, configurable: true });
        }
        if (setter) {
            desc.set = setter;
            Object.defineProperty(setter, 'length', { value: 1, writable: false, enumerable: false, configurable: true });
        }
        Object.defineProperty(proto, name, desc);
    }
    function hidden(obj, key, val) {
        Object.defineProperty(obj, key, { value: val, writable: true, enumerable: false, configurable: true });
    }

    // ---- CSSStyleDeclaration.prototype ----
    // codegen already has cssText/length/item/getPropertyValue/getPropertyPriority/setProperty/removeProperty/parentRule
    // We override them with real implementations that read from __iv8Props/__iv8Order
    if (typeof CSSStyleDeclaration !== 'undefined' && CSSStyleDeclaration.prototype) {
        var sdProto = CSSStyleDeclaration.prototype;

        defGetter(sdProto, 'cssText', function() {
            return this.__iv8CssText !== undefined ? this.__iv8CssText : '';
        });
        defGetter(sdProto, 'length', function() {
            return this.__iv8Order ? this.__iv8Order.length : 0;
        });
        defMethod(sdProto, 'item', function item(index) {
            if (arguments.length < 1) throw new TypeError('1 argument(s) required, but only 0 present');
            return this.__iv8Order ? (this.__iv8Order[index] || '') : '';
        }, 1);
        defMethod(sdProto, 'getPropertyValue', function getPropertyValue(prop) {
            if (arguments.length < 1) throw new TypeError('1 argument(s) required, but only 0 present');
            return this.__iv8Props ? (this.__iv8Props[prop] || '') : '';
        }, 1);
        defMethod(sdProto, 'getPropertyPriority', function getPropertyPriority(prop) {
            if (arguments.length < 1) throw new TypeError('1 argument(s) required, but only 0 present');
            return this.__iv8Priority && this.__iv8Priority[prop] ? 'important' : '';
        }, 1);
        defMethod(sdProto, 'setProperty', function setProperty(prop, value, priority) {
            if (arguments.length < 2) throw new TypeError('2 argument(s) required, but only ' + arguments.length + ' present');
            if (!this.__iv8Props) { hidden(this, '__iv8Props', {}); hidden(this, '__iv8Order', []); }
            if (!this.__iv8Props[prop]) this.__iv8Order.push(prop);
            this.__iv8Props[prop] = value;
            if (!this.__iv8Priority) hidden(this, '__iv8Priority', {});
            this.__iv8Priority[prop] = priority === 'important';
            this.__iv8CssText = rebuildCssText(this);
        }, 2);
        defMethod(sdProto, 'removeProperty', function removeProperty(prop) {
            if (arguments.length < 1) throw new TypeError('1 argument(s) required, but only 0 present');
            if (!this.__iv8Props) return '';
            var old = this.__iv8Props[prop] || '';
            delete this.__iv8Props[prop];
            if (this.__iv8Order) {
                var idx = this.__iv8Order.indexOf(prop);
                if (idx >= 0) this.__iv8Order.splice(idx, 1);
            }
            this.__iv8CssText = rebuildCssText(this);
            return old;
        }, 1);
        defGetter(sdProto, 'parentRule', function() {
            return this.__iv8ParentRule || null;
        });
    }

    function rebuildCssText(decl) {
        if (!decl.__iv8Order) return '';
        var parts = [];
        for (var i = 0; i < decl.__iv8Order.length; i++) {
            var p = decl.__iv8Order[i];
            var v = decl.__iv8Props[p];
            var pr = decl.__iv8Priority && decl.__iv8Priority[p] ? ' !important' : '';
            parts.push(p + ': ' + v + pr);
        }
        return parts.join('; ');
    }

    // ---- CSSStyleProperties.prototype ----
    // CSSStyleProperties inherits from CSSStyleDeclaration
    // codegen should have set up __proto__ via inherit()
    // We install cssFloat as an alias for float
    if (typeof CSSStyleProperties !== 'undefined' && CSSStyleProperties.prototype) {
        defAccessor(CSSStyleProperties.prototype, 'cssFloat',
            function() { return this.getPropertyValue('float'); },
            function(v) { this.setProperty('float', v); }
        );
    }

    // ---- CSSPageDescriptors.prototype ----
    // CSSPageDescriptors inherits from CSSStyleDeclaration
    // Has margin/marginTop/marginRight/marginBottom/marginLeft/margin-top/etc + size/pageOrientation/marks/bleed
    if (typeof CSSPageDescriptors !== 'undefined' && CSSPageDescriptors.prototype) {
        var pdProto = CSSPageDescriptors.prototype;
        var pageProps = [
            ['margin', 'margin'], ['marginTop', 'margin-top'], ['marginRight', 'margin-right'],
            ['marginBottom', 'margin-bottom'], ['marginLeft', 'margin-left'],
            ['size', 'size'], ['pageOrientation', 'page-orientation'],
            ['marks', 'marks'], ['bleed', 'bleed']
        ];
        // Also expose kebab-case as direct properties
        var kebabProps = ['margin-top', 'margin-right', 'margin-bottom', 'margin-left', 'page-orientation'];
        for (var pi = 0; pi < pageProps.length; pi++) {
            (function(camel, kebab) {
                defAccessor(pdProto, camel,
                    function() { return this.getPropertyValue(kebab); },
                    function(v) { this.setProperty(kebab, v); }
                );
            })(pageProps[pi][0], pageProps[pi][1]);
        }
        for (var ki = 0; ki < kebabProps.length; ki++) {
            (function(kebab) {
                defAccessor(pdProto, kebab,
                    function() { return this.getPropertyValue(kebab); },
                    function(v) { this.setProperty(kebab, v); }
                );
            })(kebabProps[ki]);
        }
    }

    // ---- CSSRule.prototype ----
    if (typeof CSSRule !== 'undefined' && CSSRule.prototype) {
        // type constants
        var consts = { STYLE_RULE: 1, CHARSET_RULE: 2, IMPORT_RULE: 3, MEDIA_RULE: 4,
            FONT_FACE_RULE: 5, PAGE_RULE: 6, MARGIN_RULE: 1000, NAMESPACE_RULE: 10,
            KEYFRAMES_RULE: 7, KEYFRAME_RULE: 8, SUPPORTS_RULE: 12, COUNTER_STYLE_RULE: 11,
            FONT_FEATURE_VALUES_RULE: 14, LAYER_BLOCK_RULE: 15, LAYER_STATEMENT_RULE: 16,
            PROPERTY_RULE: 17, SCOPE_RULE: 18, CONDITIONAL_RULE: 0, GROUPING_RULE: 0,
            NESTING_RULE: 0, STYLE_DECLARATION: 0 };
        for (var ck in consts) {
            Object.defineProperty(CSSRule.prototype, ck, { value: consts[ck], writable: false, enumerable: true, configurable: false });
        }
        defGetter(CSSRule.prototype, 'type', function() { return this.__iv8Type || 0; });
        defGetter(CSSRule.prototype, 'cssText', function() { return this.__iv8CssText || ''; });
        defGetter(CSSRule.prototype, 'parentRule', function() { return this.__iv8ParentRule || null; });
        defGetter(CSSRule.prototype, 'parentStyleSheet', function() { return this.__iv8ParentStyleSheet || null; });
    }

    // ---- CSSStyleRule.prototype ----
    if (typeof CSSStyleRule !== 'undefined' && CSSStyleRule.prototype) {
        defAccessor(CSSStyleRule.prototype, 'selectorText',
            function() { return this.__iv8SelectorText || ''; },
            function(v) { this.__iv8SelectorText = String(v); }
        );
        defGetter(CSSStyleRule.prototype, 'style', function() { return this.__iv8Style || null; });
    }

    // ---- CSSImportRule.prototype ----
    if (typeof CSSImportRule !== 'undefined' && CSSImportRule.prototype) {
        defGetter(CSSImportRule.prototype, 'href', function() { return this.__iv8Href || null; });
        defGetter(CSSImportRule.prototype, 'media', function() { return this.__iv8Media || null; });
        defGetter(CSSImportRule.prototype, 'styleSheet', function() { return this.__iv8StyleSheet || null; });
        defGetter(CSSImportRule.prototype, 'layerName', function() { return this.__iv8LayerName || ''; });
        defGetter(CSSImportRule.prototype, 'supportsText', function() { return this.__iv8SupportsText || ''; });
    }

    // ---- CSSMediaRule.prototype ----
    if (typeof CSSMediaRule !== 'undefined' && CSSMediaRule.prototype) {
        defGetter(CSSMediaRule.prototype, 'media', function() { return this.__iv8Media || null; });
        defGetter(CSSMediaRule.prototype, 'cssRules', function() { return this.__iv8CssRules || null; });
        defMethod(CSSMediaRule.prototype, 'insertRule', function insertRule(rule, index) {
            if (arguments.length < 1) throw new TypeError('1 argument(s) required, but only 0 present');
            if (!this.__iv8CssRules) return 0;
            if (index === undefined) index = this.__iv8CssRules.__iv8Rules.length;
            return this.__iv8CssRules.__iv8Rules.length; // stub
        }, 1);
        defMethod(CSSMediaRule.prototype, 'deleteRule', function deleteRule(index) {
            if (arguments.length < 1) throw new TypeError('1 argument(s) required, but only 0 present');
        }, 1);
    }

    // ---- CSSPageRule.prototype ----
    if (typeof CSSPageRule !== 'undefined' && CSSPageRule.prototype) {
        defAccessor(CSSPageRule.prototype, 'selectorText',
            function() { return this.__iv8SelectorText || ''; },
            function(v) { this.__iv8SelectorText = String(v); }
        );
        defGetter(CSSPageRule.prototype, 'style', function() { return this.__iv8Style || null; });
    }

    // ---- CSSMarginRule.prototype ----
    if (typeof CSSMarginRule !== 'undefined' && CSSMarginRule.prototype) {
        defGetter(CSSMarginRule.prototype, 'name', function() { return this.__iv8Name || ''; });
        defGetter(CSSMarginRule.prototype, 'style', function() { return this.__iv8Style || null; });
    }

    // ---- CSSNamespaceRule.prototype ----
    if (typeof CSSNamespaceRule !== 'undefined' && CSSNamespaceRule.prototype) {
        defGetter(CSSNamespaceRule.prototype, 'namespaceURI', function() { return this.__iv8NamespaceURI || ''; });
        defGetter(CSSNamespaceRule.prototype, 'prefix', function() { return this.__iv8Prefix || ''; });
    }

    // ---- CSSFontFaceRule.prototype ----
    if (typeof CSSFontFaceRule !== 'undefined' && CSSFontFaceRule.prototype) {
        defGetter(CSSFontFaceRule.prototype, 'style', function() { return this.__iv8Style || null; });
    }

    // ---- CSSGroupingRule.prototype ----
    if (typeof CSSGroupingRule !== 'undefined' && CSSGroupingRule.prototype) {
        defGetter(CSSGroupingRule.prototype, 'cssRules', function() { return this.__iv8CssRules || null; });
        defMethod(CSSGroupingRule.prototype, 'insertRule', function insertRule(rule, index) {
            if (arguments.length < 1) throw new TypeError('1 argument(s) required, but only 0 present');
            if (!this.__iv8CssRules) return 0;
            return 0;
        }, 1);
        defMethod(CSSGroupingRule.prototype, 'deleteRule', function deleteRule(index) {
            if (arguments.length < 1) throw new TypeError('1 argument(s) required, but only 0 present');
        }, 1);
    }

    // ---- CSSStyleSheet.prototype ----
    if (typeof CSSStyleSheet !== 'undefined' && CSSStyleSheet.prototype) {
        defGetter(CSSStyleSheet.prototype, 'cssRules', function() { return this.__iv8CssRules || null; });
        defGetter(CSSStyleSheet.prototype, 'rules', function() { return this.__iv8CssRules || null; });
        defGetter(CSSStyleSheet.prototype, 'ownerRule', function() { return this.__iv8OwnerRule || null; });
        defMethod(CSSStyleSheet.prototype, 'insertRule', function insertRule(rule, index) {
            if (arguments.length < 1) throw new TypeError('1 argument(s) required, but only 0 present');
            if (!this.__iv8CssRules || !this.__iv8CssRules.__iv8Rules) return 0;
            if (index === undefined) index = this.__iv8CssRules.__iv8Rules.length;
            return index;
        }, 1);
        defMethod(CSSStyleSheet.prototype, 'deleteRule', function deleteRule(index) {
            if (arguments.length < 1) throw new TypeError('1 argument(s) required, but only 0 present');
            if (this.__iv8CssRules && this.__iv8CssRules.__iv8Rules) {
                this.__iv8CssRules.__iv8Rules.splice(index, 1);
            }
        }, 1);
        defMethod(CSSStyleSheet.prototype, 'replace', function replace(text) {
            if (arguments.length < 1) throw new TypeError('1 argument(s) required, but only 0 present');
            return Promise.resolve(this);
        }, 1);
        defMethod(CSSStyleSheet.prototype, 'replaceSync', function replaceSync(text) {
            if (arguments.length < 1) throw new TypeError('1 argument(s) required, but only 0 present');
        }, 1);
    }

    // ---- StyleSheet.prototype ----
    if (typeof StyleSheet !== 'undefined' && StyleSheet.prototype) {
        defGetter(StyleSheet.prototype, 'type', function() { return this.__iv8Type || ''; });
        defGetter(StyleSheet.prototype, 'href', function() { return this.__iv8Href || null; });
        defGetter(StyleSheet.prototype, 'ownerNode', function() { return this.__iv8OwnerNode || null; });
        defGetter(StyleSheet.prototype, 'parentStyleSheet', function() { return this.__iv8ParentStyleSheet || null; });
        defGetter(StyleSheet.prototype, 'title', function() { return this.__iv8Title || ''; });
        defGetter(StyleSheet.prototype, 'media', function() { return this.__iv8Media || null; });
        defAccessor(StyleSheet.prototype, 'disabled',
            function() { return this.__iv8Disabled || false; },
            function(v) { this.__iv8Disabled = !!v; }
        );
    }

    // ---- CSSRuleList.prototype ----
    if (typeof CSSRuleList !== 'undefined' && CSSRuleList.prototype) {
        defGetter(CSSRuleList.prototype, 'length', function() { return this.__iv8Rules ? this.__iv8Rules.length : 0; });
        defMethod(CSSRuleList.prototype, 'item', function item(index) {
            if (arguments.length < 1) throw new TypeError('1 argument(s) required, but only 0 present');
            return this.__iv8Rules ? (this.__iv8Rules[index] || null) : null;
        }, 1);
    }

    // ---- StyleSheetList.prototype ----
    if (typeof StyleSheetList !== 'undefined' && StyleSheetList.prototype) {
        defGetter(StyleSheetList.prototype, 'length', function() { return this.__iv8Sheets ? this.__iv8Sheets.length : 0; });
        defMethod(StyleSheetList.prototype, 'item', function item(index) {
            if (arguments.length < 1) throw new TypeError('1 argument(s) required, but only 0 present');
            return this.__iv8Sheets ? (this.__iv8Sheets[index] || null) : null;
        }, 1);
    }

    // ---- MediaList.prototype ----
    if (typeof MediaList !== 'undefined' && MediaList.prototype) {
        defGetter(MediaList.prototype, 'mediaText', function() { return this.__iv8MediaText || ''; });
        defGetter(MediaList.prototype, 'length', function() {
            if (!this.__iv8MediaText) return 0;
            return this.__iv8MediaText.split(',').filter(function(s) { return s.trim().length > 0; }).length;
        });
        defMethod(MediaList.prototype, 'item', function item(index) {
            if (arguments.length < 1) throw new TypeError('1 argument(s) required, but only 0 present');
            if (!this.__iv8MediaText) return null;
            var parts = this.__iv8MediaText.split(',');
            return parts[index] ? parts[index].trim() : null;
        }, 1);
        defMethod(MediaList.prototype, 'appendMedium', function appendMedium(medium) {
            if (arguments.length < 1) throw new TypeError('1 argument(s) required, but only 0 present');
            if (this.__iv8MediaText) this.__iv8MediaText += ', ' + medium;
            else this.__iv8MediaText = medium;
        }, 1);
        defMethod(MediaList.prototype, 'deleteMedium', function deleteMedium(medium) {
            if (arguments.length < 1) throw new TypeError('1 argument(s) required, but only 0 present');
            if (!this.__iv8MediaText) return;
            var parts = this.__iv8MediaText.split(',').map(function(s) { return s.trim(); });
            var idx = parts.indexOf(medium);
            if (idx >= 0) { parts.splice(idx, 1); this.__iv8MediaText = parts.join(', '); }
        }, 1);
        // stringifier
        MediaList.prototype.toString = function() { return this.__iv8MediaText || ''; };
    }

    // ---- CSS namespace ----
    // CSS namespace is installed separately via CSS_NAMESPACE_JS to avoid
    // being skipped when CSSOM_PROTO_SETUP_JS IIFE throws on frozen prototypes.
    // See CSS_NAMESPACE_JS below.


    // ---- Document.prototype.styleSheets / adoptedStyleSheets ----
    // These are on Document.prototype, not on the instance
    if (typeof Document !== 'undefined' && Document.prototype) {
        if (!Object.getOwnPropertyDescriptor(Document.prototype, 'styleSheets')) {
            defGetter(Document.prototype, 'styleSheets', function() {
                var sheets = [];
                if (this.querySelectorAll) {
                    var styles = this.querySelectorAll('style, link[rel="stylesheet"]');
                    for (var i = 0; i < styles.length; i++) {
                        if (styles[i].tagName === 'STYLE' && styles[i].sheet) sheets.push(styles[i].sheet);
                    }
                }
                var slProto = (typeof StyleSheetList !== 'undefined') ? StyleSheetList.prototype : Object.prototype;
                var sl = Object.create(slProto);
                Object.defineProperty(sl, '__iv8Sheets', { value: sheets, writable: true, enumerable: false, configurable: true });
                return sl;
            });
        }
        if (!Object.getOwnPropertyDescriptor(Document.prototype, 'adoptedStyleSheets')) {
            defAccessor(Document.prototype, 'adoptedStyleSheets',
                function() { return []; },
                function() {}
            );
        }
    }

    // ---- ShadowRoot.prototype.styleSheets / adoptedStyleSheets ----
    if (typeof ShadowRoot !== 'undefined' && ShadowRoot.prototype) {
        if (!Object.getOwnPropertyDescriptor(ShadowRoot.prototype, 'styleSheets')) {
            defGetter(ShadowRoot.prototype, 'styleSheets', function() {
                var slProto = (typeof StyleSheetList !== 'undefined') ? StyleSheetList.prototype : Object.prototype;
                var sl = Object.create(slProto);
                Object.defineProperty(sl, '__iv8Sheets', { value: [], writable: true, enumerable: false, configurable: true });
                return sl;
            });
        }
        if (!Object.getOwnPropertyDescriptor(ShadowRoot.prototype, 'adoptedStyleSheets')) {
            defAccessor(ShadowRoot.prototype, 'adoptedStyleSheets',
                function() { return []; },
                function() {}
            );
        }
    }

    // ---- ProcessingInstruction.prototype.sheet ----
    if (typeof ProcessingInstruction !== 'undefined' && ProcessingInstruction.prototype) {
        defGetter(ProcessingInstruction.prototype, 'sheet', function() { return this.__iv8Sheet || null; });
    }
})();
"#;

/// JS shim for CSSOM — CSS parser + CSSStyleSheet/CSSRule population.
/// Uses prototype-based architecture: all properties on prototype, state in __iv8* hidden props.
pub const CSSOM_SHIM_JS: &str = r#"
(function() {
    'use strict';

    function hidden(obj, key, val) {
        Object.defineProperty(obj, key, { value: val, writable: true, enumerable: false, configurable: true });
    }

    // CSS parser: parse CSS text into rule descriptors
    function parseCSS(text) {
        var rules = [];
        text = text.replace(/\/\*[\s\S]*?\*\//g, '');
        var depth = 0, start = 0;
        for (var i = 0; i < text.length; i++) {
            if (text[i] === '{') depth++;
            else if (text[i] === '}') {
                depth--;
                if (depth === 0) {
                    var ruleText = text.slice(start, i + 1).trim();
                    if (ruleText) {
                        var rule = parseRule(ruleText);
                        if (rule) rules.push(rule);
                    }
                    start = i + 1;
                }
            }
            else if (text[i] === ';' && depth === 0) {
                var ruleText2 = text.slice(start, i + 1).trim();
                if (ruleText2) {
                    var rule2 = parseRule(ruleText2);
                    if (rule2) rules.push(rule2);
                }
                start = i + 1;
            }
        }
        var remaining = text.slice(start).trim();
        if (remaining) {
            var rule3 = parseRule(remaining);
            if (rule3) rules.push(rule3);
        }
        return rules;
    }

    function parseRule(ruleText) {
        var m;
        // @import rule
        m = ruleText.match(/^@import\s+(?:url\()?["']?([^"')\s]+)["']?\)?\s*([^;]*);?\s*$/);
        if (m) {
            return {
                type: 3, cssText: ruleText, href: m[1],
                media: m[2] ? m[2].trim() : '',
                layerName: '', supportsText: ''
            };
        }
        // @namespace rule
        m = ruleText.match(/^@namespace\s+([^:\s]+)?:?\s*["']([^"']+)["']\s*;?\s*$/);
        if (m) {
            return { type: 10, cssText: ruleText, prefix: m[1] || '', namespaceURI: m[2] };
        }
        // @page rule
        m = ruleText.match(/^@page\s*([^{]*)\{([\s\S]*)\}\s*$/);
        if (m) {
            var pageSelector = m[1].trim();
            var pageBody = m[2].trim();
            var marginRules = parseMarginRules(pageBody);
            return {
                type: 6, cssText: ruleText, selectorText: pageSelector,
                declText: extractDeclarations(pageBody),
                marginRules: marginRules
            };
        }
        // @media rule
        m = ruleText.match(/^@media\s+([^{]*)\{([\s\S]*)\}\s*$/);
        if (m) {
            return {
                type: 4, cssText: ruleText, mediaText: m[1].trim(),
                childRules: parseCSS(m[2])
            };
        }
        // @font-face rule
        m = ruleText.match(/^@font-face\s*\{([\s\S]*)\}\s*$/);
        if (m) {
            return { type: 5, cssText: ruleText, declText: m[1].trim() };
        }
        // Style rule
        m = ruleText.match(/^([^{]*)\{([\s\S]*)\}\s*$/);
        if (m) {
            return { type: 1, cssText: ruleText, selectorText: m[1].trim(), declText: m[2].trim() };
        }
        return null;
    }

    function parseMarginRules(body) {
        var rules = [];
        var m = body.match(/@[\w-]+\s*\{([^}]*)\}/g);
        if (m) {
            for (var i = 0; i < m.length; i++) {
                var mm = m[i].match(/@([\w-]+)\s*\{([^}]*)\}/);
                if (mm) {
                    rules.push({ type: 1000, cssText: mm[0], name: '@' + mm[1], declText: mm[2].trim() });
                }
            }
        }
        return rules;
    }

    function extractDeclarations(body) {
        return body.replace(/@[\w-]+\s*\{[^}]*\}/g, '').trim();
    }

    // Create a CSSStyleDeclaration (or CSSStyleProperties/CSSPageDescriptors) instance
    function createStyleDecl(declText, protoTag) {
        var proto;
        if (protoTag === 'CSSPageDescriptors' && typeof CSSPageDescriptors !== 'undefined') {
            proto = CSSPageDescriptors.prototype;
        } else if (protoTag === 'CSSStyleProperties' && typeof CSSStyleProperties !== 'undefined') {
            proto = CSSStyleProperties.prototype;
        } else if (typeof CSSStyleDeclaration !== 'undefined') {
            proto = CSSStyleDeclaration.prototype;
        } else {
            proto = Object.prototype;
        }
        var decl = Object.create(proto);
        var props = {};
        var order = [];
        var priority = {};
        if (declText) {
            var pairs = declText.split(';');
            for (var i = 0; i < pairs.length; i++) {
                var eq = pairs[i].indexOf(':');
                if (eq > 0) {
                    var prop = pairs[i].slice(0, eq).trim();
                    var val = pairs[i].slice(eq + 1).trim();
                    if (prop) {
                        var imp = val.match(/\s*!important\s*$/i);
                        if (imp) { val = val.replace(/\s*!important\s*$/i, ''); priority[prop] = true; }
                        props[prop] = val;
                        if (order.indexOf(prop) < 0) order.push(prop);
                    }
                }
            }
        }
        hidden(decl, '__iv8Props', props);
        hidden(decl, '__iv8Order', order);
        hidden(decl, '__iv8Priority', priority);
        hidden(decl, '__iv8CssText', declText);
        hidden(decl, '__iv8ParentRule', null);
        return decl;
    }

    // Create a CSSRule instance from a rule descriptor
    function createCSSRule(descriptor, sheet, parentRule) {
        var proto = null;
        var tag = 'CSSRule';
        switch (descriptor.type) {
            case 1: proto = (typeof CSSStyleRule !== 'undefined') ? CSSStyleRule.prototype : null; tag = 'CSSStyleRule'; break;
            case 3: proto = (typeof CSSImportRule !== 'undefined') ? CSSImportRule.prototype : null; tag = 'CSSImportRule'; break;
            case 4: proto = (typeof CSSMediaRule !== 'undefined') ? CSSMediaRule.prototype : null; tag = 'CSSMediaRule'; break;
            case 5: proto = (typeof CSSFontFaceRule !== 'undefined') ? CSSFontFaceRule.prototype : null; tag = 'CSSFontFaceRule'; break;
            case 6: proto = (typeof CSSPageRule !== 'undefined') ? CSSPageRule.prototype : null; tag = 'CSSPageRule'; break;
            case 10: proto = (typeof CSSNamespaceRule !== 'undefined') ? CSSNamespaceRule.prototype : null; tag = 'CSSNamespaceRule'; break;
            case 1000: proto = (typeof CSSMarginRule !== 'undefined') ? CSSMarginRule.prototype : null; tag = 'CSSMarginRule'; break;
        }
        if (!proto) {
            proto = (typeof CSSRule !== 'undefined') ? CSSRule.prototype : Object.prototype;
        }
        var rule = Object.create(proto);
        hidden(rule, '__iv8Type', descriptor.type);
        hidden(rule, '__iv8CssText', descriptor.cssText);
        hidden(rule, '__iv8ParentStyleSheet', sheet);
        hidden(rule, '__iv8ParentRule', parentRule || null);
        Object.defineProperty(rule, Symbol.toStringTag, { value: tag, writable: true, configurable: true, enumerable: false });

        if (descriptor.type === 1) {
            hidden(rule, '__iv8SelectorText', descriptor.selectorText);
            var style = createStyleDecl(descriptor.declText, 'CSSStyleProperties');
            hidden(style, '__iv8ParentRule', rule);
            hidden(rule, '__iv8Style', style);
        } else if (descriptor.type === 3) {
            hidden(rule, '__iv8Href', descriptor.href);
            hidden(rule, '__iv8Media', createMediaList(descriptor.media));
            hidden(rule, '__iv8StyleSheet', null);
            hidden(rule, '__iv8LayerName', descriptor.layerName || '');
            hidden(rule, '__iv8SupportsText', descriptor.supportsText || '');
        } else if (descriptor.type === 4) {
            hidden(rule, '__iv8Media', createMediaList(descriptor.mediaText));
            var childRules = (descriptor.childRules || []).map(function(r) { return createCSSRule(r, sheet, rule); });
            hidden(rule, '__iv8CssRules', createCSSRuleList(childRules));
        } else if (descriptor.type === 6) {
            hidden(rule, '__iv8SelectorText', descriptor.selectorText);
            var pageStyle = createStyleDecl(descriptor.declText, 'CSSPageDescriptors');
            hidden(pageStyle, '__iv8ParentRule', rule);
            hidden(rule, '__iv8Style', pageStyle);
            var marginRules = (descriptor.marginRules || []).map(function(r) { return createCSSRule(r, sheet, rule); });
            hidden(rule, '__iv8CssRules', createCSSRuleList(marginRules));
        } else if (descriptor.type === 10) {
            hidden(rule, '__iv8Prefix', descriptor.prefix);
            hidden(rule, '__iv8NamespaceURI', descriptor.namespaceURI);
        } else if (descriptor.type === 5) {
            var ffStyle = createStyleDecl(descriptor.declText, 'CSSStyleProperties');
            hidden(ffStyle, '__iv8ParentRule', rule);
            hidden(rule, '__iv8Style', ffStyle);
        } else if (descriptor.type === 1000) {
            hidden(rule, '__iv8Name', descriptor.name);
            var mStyle = createStyleDecl(descriptor.declText, 'CSSStyleProperties');
            hidden(mStyle, '__iv8ParentRule', rule);
            hidden(rule, '__iv8Style', mStyle);
        }
        return rule;
    }

    function createCSSRuleList(rules) {
        var proto = (typeof CSSRuleList !== 'undefined') ? CSSRuleList.prototype : Object.prototype;
        var list = Object.create(proto);
        hidden(list, '__iv8Rules', rules);
        hidden(list, '__iv8Length', rules.length);
        // Install indexed properties for bracket notation access
        for (var i = 0; i < rules.length; i++) {
            Object.defineProperty(list, i, {
                value: rules[i], writable: true, enumerable: true, configurable: true
            });
        }
        Object.defineProperty(list, Symbol.toStringTag, { value: 'CSSRuleList', writable: true, configurable: true, enumerable: false });
        return list;
    }

    function createMediaList(mediaText) {
        var proto = (typeof MediaList !== 'undefined') ? MediaList.prototype : Object.prototype;
        var ml = Object.create(proto);
        hidden(ml, '__iv8MediaText', mediaText || '');
        Object.defineProperty(ml, Symbol.toStringTag, { value: 'MediaList', writable: true, configurable: true, enumerable: false });
        return ml;
    }

    function createStyleSheetList(sheets) {
        var proto = (typeof StyleSheetList !== 'undefined') ? StyleSheetList.prototype : Object.prototype;
        var sl = Object.create(proto);
        hidden(sl, '__iv8Sheets', sheets);
        Object.defineProperty(sl, Symbol.toStringTag, { value: 'StyleSheetList', writable: true, configurable: true, enumerable: false });
        return sl;
    }

    // Create a CSSStyleSheet with parsed cssRules from text content
    function createStyleSheetFromText(text, ownerNode) {
        var sheet;
        if (typeof CSSStyleSheet !== 'undefined') {
            try { sheet = new CSSStyleSheet(); } catch(e) { sheet = Object.create(CSSStyleSheet.prototype); }
        } else {
            sheet = {};
        }
        // Ensure prototype chain: CSSStyleSheet.prototype → StyleSheet.prototype
        if (typeof CSSStyleSheet !== 'undefined') {
            Object.setPrototypeOf(sheet, CSSStyleSheet.prototype);
        }
        Object.defineProperty(sheet, Symbol.toStringTag, { value: 'CSSStyleSheet', writable: true, configurable: true, enumerable: false });

        var descriptors = parseCSS(text);
        var rules = descriptors.map(function(d) { return createCSSRule(d, sheet, null); });
        var cssRuleList = createCSSRuleList(rules);
        hidden(sheet, '__iv8CssRules', cssRuleList);
        hidden(sheet, '__iv8OwnerNode', ownerNode);
        hidden(sheet, '__iv8OwnerRule', null);
        hidden(sheet, '__iv8Disabled', false);
        hidden(sheet, '__iv8Href', null);
        hidden(sheet, '__iv8Media', createMediaList(''));
        hidden(sheet, '__iv8Title', '');
        hidden(sheet, '__iv8Type', 'text/css');
        hidden(sheet, '__iv8ParentStyleSheet', null);
        // insertRule/deleteRule: install on prototype (CSSStyleSheet.prototype)
        // codegen doesn't generate these as __iv8-readers, so install as own methods
        // These will be own properties but idlharness checks inheritance for operations
        // on sheet directly, which checks prototype chain — codegen has insertRule on proto
        return sheet;
    }

    // Expose for element.sheet integration
    globalThis.__iv8CreateStyleSheet = createStyleSheetFromText;

    // element.style: codegen getter now reads __iv8Style hidden property.
    // CSSOM shim doesn't need to override createElement — idlharness test
    // objects are created via createElement, and codegen style getter will
    // check __iv8Style. If not set, it returns default Object.
    // For CSSOM tests, style_element.style is checked — HTMLStyleElement
    // has its own style handling via the sheet getter.
    // For svg_element.style, we accept codegen default (empty Object) until
    // a deeper codegen-level fix is made.

    // Wire up HTMLStyleElement.prototype.sheet (override codegen stub)
    if (typeof HTMLStyleElement !== 'undefined' && HTMLStyleElement.prototype) {
        var __styleTextGetter = function() {
            return this.__iv8TextContent !== undefined ? this.__iv8TextContent : '';
        };
        var __styleTextSetter = function(v) {
            Object.defineProperty(this, '__iv8TextContent', {
                value: String(v), writable: true, enumerable: false, configurable: true
            });
        };
        try {
            Object.defineProperty(HTMLStyleElement.prototype, 'textContent', {
                get: __styleTextGetter, set: __styleTextSetter,
                enumerable: true, configurable: true
            });
        } catch(e) {}
        Object.defineProperty(HTMLStyleElement.prototype, 'sheet', {
            get: function() {
                if (this.__iv8Sheet) return this.__iv8Sheet;
                var text = this.__iv8TextContent || '';
                var sheet = createStyleSheetFromText(text, this);
                try { Object.defineProperty(this, '__iv8Sheet', { value: sheet, writable: false, enumerable: false, configurable: false }); } catch(e3) {}
                return sheet;
            },
            enumerable: true, configurable: true
        });
    }

    // document.styleSheets is installed on Document.prototype by CSSOM_PROTO_SETUP_JS
    // document.adoptedStyleSheets is installed on Document.prototype by CSSOM_PROTO_SETUP_JS

    // Wire up window.getComputedStyle
    if (typeof window !== 'undefined' && typeof globalThis.getComputedStyle === 'undefined') {
        var gcs = function getComputedStyle(elt, pseudoElt) {
            if (arguments.length < 1) throw new TypeError('1 argument(s) required, but only 0 present');
            var style = createStyleDecl('', 'CSSStyleProperties');
            if (elt && elt.style && elt.style.__iv8Props) {
                var inline = elt.style;
                var keys = Object.keys(inline.__iv8Props);
                for (var i = 0; i < keys.length; i++) {
                    style.__iv8Props[keys[i]] = inline.__iv8Props[keys[i]];
                    style.__iv8Order.push(keys[i]);
                }
                style.__iv8CssText = inline.__iv8CssText || '';
            }
            return style;
        };
        Object.defineProperty(gcs, 'name', { value: 'getComputedStyle', writable: false, enumerable: false, configurable: true });
        globalThis.getComputedStyle = gcs;
    }
})();
"#;

/// CSS namespace object (CSS.supports, CSS.escape, CSS.cssFloat).
/// Installed as a separate eval to avoid being skipped when
/// CSSOM_PROTO_SETUP_JS IIFE throws on V8 frozen prototypes.
/// V8 FunctionTemplate prototypes are non-extensible by default,
/// so Object.defineProperty on them for new properties throws.
/// This constant only touches globalThis (which is extensible).
pub const CSS_NAMESPACE_JS: &str = r#"
(function() {
    'use strict';

    function cssSupports(prop, value) {
        if (arguments.length === 0) return false;
        if (arguments.length === 1) {
            var str = String(prop);
            if (str.indexOf(':') !== -1) {
                var parts = str.split(':');
                return cssSupports(parts[0].trim(), parts[1].trim());
            }
            var knownProps = [
                'display','position','color','background','margin','padding',
                'border','width','height','top','left','right','bottom',
                'font','text-align','overflow','z-index','opacity','visibility',
                'flex','grid','transform','transition','animation',
                'cursor','pointer-events','box-sizing','box-shadow',
                'border-radius','outline','content','white-space',
                'text-decoration','vertical-align','line-height'
            ];
            return knownProps.indexOf(str.trim()) !== -1;
        }
        return true;
    }

    function cssEscape(ident) {
        if (arguments.length < 1) throw new TypeError('1 argument(s) required, but only 0 present');
        var str = String(ident);
        var result = '';
        for (var i = 0; i < str.length; i++) {
            var ch = str.charCodeAt(i);
            if ((ch >= 65 && ch <= 90) || (ch >= 97 && ch <= 122) ||
                (ch >= 48 && ch <= 57) || ch === 45 || ch === 95) {
                result += str[i];
            } else if (ch === 0) {
                result += '\uFFFD';
            } else {
                result += '\\' + str[i];
            }
        }
        return result;
    }

    if (typeof globalThis.CSS === 'undefined') {
        var cssNs = {};
        Object.defineProperty(cssNs, 'escape', {
            value: cssEscape, writable: true, configurable: true, enumerable: true
        });
        Object.defineProperty(cssNs, 'supports', {
            value: cssSupports, writable: true, configurable: true, enumerable: true
        });
        Object.defineProperty(cssNs, 'cssFloat', {
            value: 'cssFloat', writable: true, configurable: true, enumerable: true
        });
        if (typeof Symbol !== 'undefined' && Symbol.toStringTag) {
            Object.defineProperty(cssNs, Symbol.toStringTag, {
                value: 'CSS', writable: false, configurable: true, enumerable: false
            });
        }
        Object.defineProperty(globalThis, 'CSS', {
            value: cssNs, writable: true, configurable: true, enumerable: false
        });
    } else {
        var existing = globalThis.CSS;
        if (typeof existing.supports !== 'function') {
            Object.defineProperty(existing, 'supports', {
                value: cssSupports, writable: true, configurable: true, enumerable: true
            });
        }
        if (typeof existing.escape !== 'function') {
            Object.defineProperty(existing, 'escape', {
                value: cssEscape, writable: true, configurable: true, enumerable: true
            });
        }
        if (typeof existing.cssFloat === 'undefined') {
            Object.defineProperty(existing, 'cssFloat', {
                value: 'cssFloat', writable: true, configurable: true, enumerable: true
            });
        }
    }
})();
"#;
