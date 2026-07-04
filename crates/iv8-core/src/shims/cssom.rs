//! CSSOM shim: CSSStyleSheet/CSSRule/CSSStyleDeclaration runtime
//!
//! Implements a CSS parser + CSSOM object model that creates properly typed
//! CSSRule instances (CSSImportRule, CSSStyleRule, CSSMediaRule, etc.) with
//! correct prototype chains using codegen constructors.
//!
//! Reference: https://drafts.csswg.org/cssom/

/// JS shim for CSSOM — CSS parser + CSSStyleSheet/CSSRule population.
pub const CSSOM_SHIM_JS: &str = r#"
(function() {
    // CSSRule type constants (per CSSOM spec §6.4)
    if (typeof CSSRule !== 'undefined') {
        CSSRule.STYLE_RULE = 1;
        CSSRule.IMPORT_RULE = 3;
        CSSRule.MEDIA_RULE = 4;
        CSSRule.FONT_FACE_RULE = 5;
        CSSRule.PAGE_RULE = 6;
        CSSRule.NAMESPACE_RULE = 10;
        CSSRule.MARGIN_RULE = 1000;
        CSSRule.CONDITIONAL_RULE = 0;
        CSSRule.GROUPING_RULE = 0;
        CSSRule.NESTING_RULE = 0;
        CSSRule.STYLE_DECLARATION = 0;
    }

    // CSS parser: parse CSS text into rule descriptors
    function parseCSS(text) {
        var rules = [];
        // Remove comments
        text = text.replace(/\/\*[\s\S]*?\*\//g, '');
        // Split into rules by tracking brace depth and semicolons
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
                // At-rules ending with semicolon (e.g., @import, @namespace)
                var ruleText = text.slice(start, i + 1).trim();
                if (ruleText) {
                    var rule = parseRule(ruleText);
                    if (rule) rules.push(rule);
                }
                start = i + 1;
            }
        }
        // Handle any remaining text (rule without trailing semicolon/brace)
        var remaining = text.slice(start).trim();
        if (remaining) {
            var rule = parseRule(remaining);
            if (rule) rules.push(rule);
        }
        return rules;
    }

    function parseRule(ruleText) {
        // @import rule
        var m = ruleText.match(/^@import\s+(?:url\()?["']?([^"')\s]+)["']?\)?\s*([^;]*);?\s*$/);
        if (m) {
            return {
                type: 3, // CSSRule.IMPORT_RULE
                cssText: ruleText,
                href: m[1],
                media: m[2] ? m[2].trim() : '',
                style: createStyleDecl('')
            };
        }
        // @namespace rule
        m = ruleText.match(/^@namespace\s+([^:\s]+)?:?\s*["']([^"']+)["']\s*;?\s*$/);
        if (m) {
            return {
                type: 10, // CSSRule.NAMESPACE_RULE
                cssText: ruleText,
                prefix: m[1] || '',
                namespaceURI: m[2]
            };
        }
        // @page rule
        m = ruleText.match(/^@page\s*([^{]*)\{([\s\S]*)\}\s*$/);
        if (m) {
            var pageSelector = m[1].trim();
            var pageBody = m[2].trim();
            var marginRules = parseMarginRules(pageBody);
            return {
                type: 6, // CSSRule.PAGE_RULE
                cssText: ruleText,
                selectorText: pageSelector,
                style: createStyleDecl(extractDeclarations(pageBody)),
                cssRules: marginRules
            };
        }
        // @media rule
        m = ruleText.match(/^@media\s+([^{]*)\{([\s\S]*)\}\s*$/);
        if (m) {
            var mediaText = m[1].trim();
            var mediaRules = parseCSS(m[2]);
            return {
                type: 4, // CSSRule.MEDIA_RULE
                cssText: ruleText,
                media: { mediaText: mediaText, length: mediaText ? 1 : 0, item: function(i) { return mediaText; }, toString: function() { return mediaText; } },
                cssRules: mediaRules
            };
        }
        // @font-face rule
        m = ruleText.match(/^@font-face\s*\{([\s\S]*)\}\s*$/);
        if (m) {
            return {
                type: 5, // CSSRule.FONT_FACE_RULE
                cssText: ruleText,
                style: createStyleDecl(m[1].trim())
            };
        }
        // Style rule (selector { declarations })
        m = ruleText.match(/^([^{]*)\{([\s\S]*)\}\s*$/);
        if (m) {
            var selector = m[1].trim();
            var declText = m[2].trim();
            return {
                type: 1, // CSSRule.STYLE_RULE
                cssText: ruleText,
                selectorText: selector,
                style: createStyleDecl(declText)
            };
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
                    rules.push({
                        type: 1000, // CSSRule.MARGIN_RULE
                        cssText: mm[0],
                        name: '@' + mm[1],
                        style: createStyleDecl(mm[2].trim())
                    });
                }
            }
        }
        return rules;
    }

    function extractDeclarations(body) {
        // Remove @margin { } blocks, keep only declarations
        return body.replace(/@[\w-]+\s*\{[^}]*\}/g, '').trim();
    }

    function createStyleDecl(declText) {
        var props = {};
        var order = [];
        if (declText) {
            var pairs = declText.split(';');
            for (var i = 0; i < pairs.length; i++) {
                var eq = pairs[i].indexOf(':');
                if (eq > 0) {
                    var prop = pairs[i].slice(0, eq).trim();
                    var val = pairs[i].slice(eq + 1).trim();
                    if (prop) {
                        props[prop] = val;
                        order.push(prop);
                    }
                }
            }
        }
        props.length = order.length;
        props.cssText = declText;
        props.getPropertyValue = function(p) { return props[p] || ''; };
        props.setProperty = function(p, v) { props[p] = v; order.push(p); props.length = order.length; };
        props.item = function(i) { return order[i] || ''; };
        return props;
    }

    // Create a CSSRule instance from a rule descriptor
    function createCSSRule(descriptor, sheet) {
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
        // Set common properties
        Object.defineProperty(rule, 'type', { value: descriptor.type, writable: true, enumerable: true, configurable: true });
        Object.defineProperty(rule, 'cssText', { value: descriptor.cssText, writable: true, enumerable: true, configurable: true });
        Object.defineProperty(rule, 'parentStyleSheet', { value: sheet, writable: true, enumerable: true, configurable: true });
        Object.defineProperty(rule, 'parentRule', { value: null, writable: true, enumerable: true, configurable: true });
        Object.defineProperty(rule, Symbol.toStringTag, { value: tag, writable: true, configurable: true, enumerable: false });
        // Set type-specific properties
        if (descriptor.type === 1) { // CSSStyleRule
            Object.defineProperty(rule, 'selectorText', { value: descriptor.selectorText, writable: true, enumerable: true, configurable: true });
            Object.defineProperty(rule, 'style', { value: descriptor.style, writable: true, enumerable: true, configurable: true });
        } else if (descriptor.type === 3) { // CSSImportRule
            Object.defineProperty(rule, 'href', { value: descriptor.href, writable: true, enumerable: true, configurable: true });
            Object.defineProperty(rule, 'media', { value: descriptor.media, writable: true, enumerable: true, configurable: true });
            Object.defineProperty(rule, 'styleSheet', { value: null, writable: true, enumerable: true, configurable: true });
        } else if (descriptor.type === 4) { // CSSMediaRule
            Object.defineProperty(rule, 'media', { value: descriptor.media, writable: true, enumerable: true, configurable: true });
            var childRules = (descriptor.cssRules || []).map(function(r) { return createCSSRule(r, sheet); });
            Object.defineProperty(rule, 'cssRules', { value: childRules, writable: true, enumerable: true, configurable: true });
        } else if (descriptor.type === 6) { // CSSPageRule
            Object.defineProperty(rule, 'selectorText', { value: descriptor.selectorText, writable: true, enumerable: true, configurable: true });
            Object.defineProperty(rule, 'style', { value: descriptor.style, writable: true, enumerable: true, configurable: true });
            var marginRules = (descriptor.cssRules || []).map(function(r) { return createCSSRule(r, sheet); });
            Object.defineProperty(rule, 'cssRules', { value: marginRules, writable: true, enumerable: true, configurable: true });
        } else if (descriptor.type === 10) { // CSSNamespaceRule
            Object.defineProperty(rule, 'prefix', { value: descriptor.prefix, writable: true, enumerable: true, configurable: true });
            Object.defineProperty(rule, 'namespaceURI', { value: descriptor.namespaceURI, writable: true, enumerable: true, configurable: true });
        } else if (descriptor.type === 5) { // CSSFontFaceRule
            Object.defineProperty(rule, 'style', { value: descriptor.style, writable: true, enumerable: true, configurable: true });
        } else if (descriptor.type === 1000) { // CSSMarginRule
            Object.defineProperty(rule, 'name', { value: descriptor.name, writable: true, enumerable: true, configurable: true });
            Object.defineProperty(rule, 'style', { value: descriptor.style, writable: true, enumerable: true, configurable: true });
        }
        return rule;
    }

    // Create a CSSStyleSheet with parsed cssRules from text content
    function createStyleSheetFromText(text, ownerNode) {
        var sheet = null;
        if (typeof CSSStyleSheet !== 'undefined') {
            try { sheet = new CSSStyleSheet(); } catch(e) { sheet = {}; }
        } else {
            sheet = {};
        }
        // Set prototype chain
        if (typeof CSSStyleSheet !== 'undefined' && typeof StyleSheet !== 'undefined') {
            Object.setPrototypeOf(sheet, CSSStyleSheet.prototype);
        }
        Object.defineProperty(sheet, Symbol.toStringTag, { value: 'CSSStyleSheet', writable: true, configurable: true, enumerable: false });
        // Parse CSS text
        var descriptors = parseCSS(text);
        var rules = descriptors.map(function(d) { return createCSSRule(d, sheet); });
        // Create CSSRuleList-like array
        Object.defineProperty(sheet, 'cssRules', { value: rules, writable: true, enumerable: true, configurable: true });
        Object.defineProperty(sheet, 'ownerNode', { value: ownerNode, writable: true, enumerable: true, configurable: true });
        Object.defineProperty(sheet, 'ownerRule', { value: null, writable: true, enumerable: true, configurable: true });
        Object.defineProperty(sheet, 'disabled', { value: false, writable: true, enumerable: true, configurable: true });
        Object.defineProperty(sheet, 'href', { value: null, writable: true, enumerable: true, configurable: true });
        Object.defineProperty(sheet, 'media', { value: { mediaText: '', length: 0, item: function() { return null; }, toString: function() { return ''; } }, writable: true, enumerable: true, configurable: true });
        Object.defineProperty(sheet, 'title', { value: '', writable: true, enumerable: true, configurable: true });
        Object.defineProperty(sheet, 'type', { value: 'text/css', writable: true, enumerable: true, configurable: true });
        // insertRule implementation
        sheet.insertRule = function(ruleText, index) {
            var desc = parseRule(ruleText);
            if (!desc) throw new TypeError('SyntaxError');
            var newRule = createCSSRule(desc, sheet);
            if (index === undefined) index = rules.length;
            rules.splice(index, 0, newRule);
            return index;
        };
        sheet.deleteRule = function(index) {
            rules.splice(index, 1);
        };
        return sheet;
    }

    // Expose for element.sheet integration
    globalThis.__iv8CreateStyleSheet = createStyleSheetFromText;

    // Wire up HTMLStyleElement.prototype.sheet (override codegen stub)
    if (typeof HTMLStyleElement !== 'undefined' && HTMLStyleElement.prototype) {
        Object.defineProperty(HTMLStyleElement.prototype, 'sheet', {
            get: function() {
                if (this.__iv8Sheet) return this.__iv8Sheet;
                var text = '';
                try { text = this.textContent || ''; } catch(e) {
                    try { text = this.innerHTML || ''; } catch(e2) {}
                }
                var sheet = createStyleSheetFromText(text, this);
                try { Object.defineProperty(this, '__iv8Sheet', { value: sheet, writable: false, enumerable: false, configurable: false }); } catch(e3) {}
                return sheet;
            },
            enumerable: true, configurable: true
        });
    }

    // Wire up document.styleSheets
    if (typeof document !== 'undefined') {
        var _styleSheets = [];
        Object.defineProperty(document, 'styleSheets', {
            get: function() {
                _styleSheets.length = 0;
                var styles = document.querySelectorAll ? document.querySelectorAll('style, link[rel="stylesheet"]') : [];
                for (var i = 0; i < styles.length; i++) {
                    var el = styles[i];
                    if (el.tagName === 'STYLE' && el.sheet) {
                        _styleSheets.push(el.sheet);
                    }
                }
                // Make it look like StyleSheetList
                _styleSheets.item = function(i) { return this[i] || null; };
                return _styleSheets;
            },
            enumerable: true, configurable: true
        });
    }

    // Wire up window.getComputedStyle
    if (typeof window !== 'undefined' && typeof globalThis.getComputedStyle === 'undefined') {
        globalThis.getComputedStyle = function(elt, pseudoElt) {
            var style = createStyleDecl('');
            Object.defineProperty(style, Symbol.toStringTag, { value: 'CSSStyleDeclaration', configurable: true });
            // Return element's inline style if available
            if (elt && elt.style) {
                var inline = elt.style;
                var keys = Object.keys(inline);
                for (var i = 0; i < keys.length; i++) {
                    if (keys[i] !== 'length' && keys[i] !== 'cssText') {
                        style[keys[i]] = inline[keys[i]];
                    }
                }
            }
            return style;
        };
    }
})();
"#;
