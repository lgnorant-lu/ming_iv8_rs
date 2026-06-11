//! DOM prototype chain: EventTarget → Node → Element → HTMLElement → specific classes.
//!
//! Installed via JS shim. Creates constructor functions with proper prototype chain
//! so that `instanceof` checks work correctly.

pub const DOM_PROTOTYPES_JS: &str = r#"
(function() {
    // ─── Base classes ───────────────────────────────────────────────────────
    function EventTarget() {}
    function Node() {}
    Node.prototype = Object.create(EventTarget.prototype);
    Node.prototype.constructor = Node;
    Node.prototype.ELEMENT_NODE = 1;
    Node.prototype.TEXT_NODE = 3;
    Node.prototype.COMMENT_NODE = 8;
    Node.prototype.DOCUMENT_NODE = 9;

    function Element() {}
    Element.prototype = Object.create(Node.prototype);
    Element.prototype.constructor = Element;

    function HTMLElement() {}
    HTMLElement.prototype = Object.create(Element.prototype);
    HTMLElement.prototype.constructor = HTMLElement;

    // ─── Specific element classes ───────────────────────────────────────────
    var elementClasses = [
        'HTMLDivElement', 'HTMLSpanElement', 'HTMLParagraphElement',
        'HTMLAnchorElement', 'HTMLImageElement', 'HTMLInputElement',
        'HTMLButtonElement', 'HTMLFormElement', 'HTMLTableElement',
        'HTMLScriptElement', 'HTMLStyleElement', 'HTMLLinkElement',
        'HTMLMetaElement', 'HTMLHeadElement', 'HTMLBodyElement',
        'HTMLHtmlElement', 'HTMLUListElement', 'HTMLOListElement',
        'HTMLLIElement', 'HTMLCanvasElement', 'HTMLVideoElement',
        'HTMLAudioElement', 'HTMLIFrameElement', 'HTMLTextAreaElement',
        'HTMLSelectElement', 'HTMLOptionElement', 'HTMLLabelElement',
        'HTMLHeaderElement', 'HTMLFooterElement', 'HTMLNavElement',
        'HTMLMainElement', 'HTMLSectionElement', 'HTMLArticleElement',
        'HTMLUnknownElement',
    ];

    var tagToClass = {
        'div': 'HTMLDivElement', 'span': 'HTMLSpanElement', 'p': 'HTMLParagraphElement',
        'a': 'HTMLAnchorElement', 'img': 'HTMLImageElement', 'input': 'HTMLInputElement',
        'button': 'HTMLButtonElement', 'form': 'HTMLFormElement', 'table': 'HTMLTableElement',
        'script': 'HTMLScriptElement', 'style': 'HTMLStyleElement', 'link': 'HTMLLinkElement',
        'meta': 'HTMLMetaElement', 'head': 'HTMLHeadElement', 'body': 'HTMLBodyElement',
        'html': 'HTMLHtmlElement', 'ul': 'HTMLUListElement', 'ol': 'HTMLOListElement',
        'li': 'HTMLLIElement', 'canvas': 'HTMLCanvasElement', 'video': 'HTMLVideoElement',
        'audio': 'HTMLAudioElement', 'iframe': 'HTMLIFrameElement', 'textarea': 'HTMLTextAreaElement',
        'select': 'HTMLSelectElement', 'option': 'HTMLOptionElement', 'label': 'HTMLLabelElement',
        'header': 'HTMLHeaderElement', 'footer': 'HTMLFooterElement', 'nav': 'HTMLNavElement',
        'main': 'HTMLMainElement', 'section': 'HTMLSectionElement', 'article': 'HTMLArticleElement',
    };

    // Create all element class constructors
    var classMap = {};
    elementClasses.forEach(function(name) {
        var Ctor = function() {};
        Ctor.prototype = Object.create(HTMLElement.prototype);
        Ctor.prototype.constructor = Ctor;
        Object.defineProperty(Ctor, 'name', {value: name});
        classMap[name] = Ctor;
        globalThis[name] = Ctor;
    });

    // Export base classes
    globalThis.EventTarget = EventTarget;
    globalThis.Node = Node;
    globalThis.Element = Element;
    globalThis.HTMLElement = HTMLElement;

    // Store for use by __addPrototype__
    globalThis.__domClassMap__ = classMap;
    globalThis.__domTagToClass__ = tagToClass;
    globalThis.__domHTMLElement__ = HTMLElement;
    globalThis.__domElement__ = Element;
    globalThis.__domNode__ = Node;

    // Function to set prototype on a node object based on its tagName
    globalThis.__setNodePrototype__ = function(node) {
        if (!node || typeof node !== 'object') return node;
        if (node.__protoSet__) return node;

        var tagName = node.tagName;
        if (tagName) {
            var tag = tagName.toLowerCase();
            var className = tagToClass[tag] || 'HTMLUnknownElement';
            var Ctor = classMap[className] || classMap['HTMLUnknownElement'];
            if (Ctor) {
                Object.setPrototypeOf(node, Ctor.prototype);
            }
        } else if (node.nodeType === 9) {
            // Document node
            Object.setPrototypeOf(node, Node.prototype);
        } else if (node.nodeType === 3 || node.nodeType === 8) {
            // Text/Comment node
            Object.setPrototypeOf(node, Node.prototype);
        }
        Object.defineProperty(node, '__protoSet__', {value: true, enumerable: false});
        return node;
    };
})();
"#;
