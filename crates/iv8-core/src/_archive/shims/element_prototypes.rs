//! Element-specific prototype methods.
//!
//! Fills HTMLCanvasElement.prototype.getContext, HTMLInputElement.prototype.focus, etc.
//! Must run AFTER dom_prototypes.rs (which creates the class constructors).

pub const ELEMENT_PROTOTYPES_JS: &str = r#"
(function() {
    var classMap = globalThis.__domClassMap__;
    if (!classMap) return;

    // ─── HTMLCanvasElement ───────────────────────────────────────────────
    var CanvasProto = classMap['HTMLCanvasElement'] ? classMap['HTMLCanvasElement'].prototype : null;
    if (CanvasProto) {
        CanvasProto.width = 300;
        CanvasProto.height = 150;

        CanvasProto.getContext = function(type, attrs) {
            if (type === '2d') {
                return createCanvas2DContext(this);
            }
            if (type === 'webgl' || type === 'experimental-webgl') {
                return globalThis.__webglContext__ || null;
            }
            if (type === 'webgl2' || type === 'experimental-webgl2') {
                return globalThis.__webglContext__ || null;
            }
            return null;
        };

        CanvasProto.toDataURL = function(type, quality) {
            // Return a minimal valid 1x1 transparent PNG
            return 'data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNk+M9QDwADhgGAWjR9awAAAABJRU5ErkJggg==';
        };

        CanvasProto.toBlob = function(callback, type, quality) {
            if (callback) setTimeout(function() { callback(null); }, 0);
        };

        CanvasProto.captureStream = function() { return null; };
    }

    // Canvas2D context factory
    function createCanvas2DContext(canvas) {
        return {
            canvas: canvas,
            fillStyle: '#000000',
            strokeStyle: '#000000',
            lineWidth: 1,
            font: '10px sans-serif',
            textAlign: 'start',
            textBaseline: 'alphabetic',
            globalAlpha: 1,
            globalCompositeOperation: 'source-over',
            shadowBlur: 0,
            shadowColor: 'rgba(0, 0, 0, 0)',
            shadowOffsetX: 0,
            shadowOffsetY: 0,
            lineCap: 'butt',
            lineJoin: 'miter',
            miterLimit: 10,
            imageSmoothingEnabled: true,

            // Drawing methods (no-ops for fingerprint purposes)
            fillRect: function() {},
            strokeRect: function() {},
            clearRect: function() {},
            fillText: function() {},
            strokeText: function() {},
            measureText: function(text) {
                // Font-aware width estimation
                var font = this.font || '10px sans-serif';
                var sizeMatch = font.match(/(\d+(?:\.\d+)?)(px|pt|em)/);
                var fontSize = sizeMatch ? parseFloat(sizeMatch[1]) : 10;
                // Different fonts have different character widths
                var isMonospace = /monospace|courier|mono/i.test(font);
                var isSerif = /serif/i.test(font) && !/sans-serif/i.test(font);
                var charWidth = isMonospace ? fontSize * 0.6 :
                                isSerif ? fontSize * 0.55 :
                                fontSize * 0.5;
                var width = (text || '').length * charWidth;
                return {
                    width: width,
                    actualBoundingBoxAscent: fontSize * 0.8,
                    actualBoundingBoxDescent: fontSize * 0.2,
                    actualBoundingBoxLeft: 0,
                    actualBoundingBoxRight: width,
                    fontBoundingBoxAscent: fontSize,
                    fontBoundingBoxDescent: fontSize * 0.25,
                };
            },
            beginPath: function() {},
            closePath: function() {},
            moveTo: function() {},
            lineTo: function() {},
            arc: function() {},
            arcTo: function() {},
            bezierCurveTo: function() {},
            quadraticCurveTo: function() {},
            rect: function() {},
            fill: function() {},
            stroke: function() {},
            clip: function() {},
            save: function() {},
            restore: function() {},
            scale: function() {},
            rotate: function() {},
            translate: function() {},
            transform: function() {},
            setTransform: function() {},
            resetTransform: function() {},
            createLinearGradient: function() { return {addColorStop: function(){}}; },
            createRadialGradient: function() { return {addColorStop: function(){}}; },
            createPattern: function() { return null; },
            drawImage: function() {},
            createImageData: function(w, h) { return {width: w, height: h, data: new Uint8ClampedArray(w*h*4)}; },
            getImageData: function(x, y, w, h) { return {width: w, height: h, data: new Uint8ClampedArray(w*h*4)}; },
            putImageData: function() {},
            getLineDash: function() { return []; },
            setLineDash: function() {},
            isPointInPath: function() { return false; },
            isPointInStroke: function() { return false; },
        };
    }

    // ─── HTMLInputElement ────────────────────────────────────────────────
    var InputProto = classMap['HTMLInputElement'] ? classMap['HTMLInputElement'].prototype : null;
    if (InputProto) {
        InputProto.value = '';
        InputProto.type = 'text';
        InputProto.checked = false;
        InputProto.disabled = false;
        InputProto.focus = function() {};
        InputProto.blur = function() {};
        InputProto.select = function() {};
        InputProto.click = function() {};
        InputProto.setSelectionRange = function() {};
    }

    // ─── HTMLFormElement ─────────────────────────────────────────────────
    var FormProto = classMap['HTMLFormElement'] ? classMap['HTMLFormElement'].prototype : null;
    if (FormProto) {
        FormProto.submit = function() {};
        FormProto.reset = function() {};
        FormProto.checkValidity = function() { return true; };
        FormProto.reportValidity = function() { return true; };
    }

    // ─── HTMLVideoElement / HTMLAudioElement ─────────────────────────────
    var VideoProto = classMap['HTMLVideoElement'] ? classMap['HTMLVideoElement'].prototype : null;
    if (VideoProto) {
        VideoProto.play = function() { return Promise.resolve(); };
        VideoProto.pause = function() {};
        VideoProto.load = function() {};
        VideoProto.canPlayType = function() { return ''; };
        VideoProto.currentTime = 0;
        VideoProto.duration = 0;
        VideoProto.paused = true;
        VideoProto.ended = false;
        VideoProto.muted = false;
        VideoProto.volume = 1;
    }
    var AudioProto = classMap['HTMLAudioElement'] ? classMap['HTMLAudioElement'].prototype : null;
    if (AudioProto) {
        AudioProto.play = function() { return Promise.resolve(); };
        AudioProto.pause = function() {};
        AudioProto.load = function() {};
        AudioProto.canPlayType = function() { return ''; };
        AudioProto.currentTime = 0;
        AudioProto.duration = 0;
        AudioProto.paused = true;
    }

    // ─── HTMLIFrameElement ───────────────────────────────────────────────
    var IFrameProto = classMap['HTMLIFrameElement'] ? classMap['HTMLIFrameElement'].prototype : null;
    if (IFrameProto) {
        IFrameProto.contentWindow = null;
        IFrameProto.contentDocument = null;
        IFrameProto.src = '';
    }

    // ─── document.createEvent (legacy) ──────────────────────────────────
    if (typeof document !== 'undefined') {
        document.createEvent = function(type) {
            var e = new Event('');
            e.initEvent = function(type, bubbles, cancelable) {
                e.type = type;
                e.bubbles = !!bubbles;
                e.cancelable = !!cancelable;
            };
            return e;
        };
    }
})();
"#;
