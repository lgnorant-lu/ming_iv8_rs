"""H03 L2: IV8 surface sampler — collect all fingerprint values from IV8.

Usage: python scripts/sample_iv8_surface.py [--output actual.json]
"""

import argparse
import json
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent


def sample_iv8():
    """Run IV8 and collect all fingerprint surface values."""
    sys.path.insert(0, str(REPO_ROOT))
    from iv8_rs import JSContext

    ctx = JSContext()
    ctx.page_load("<!DOCTYPE html><html><body></body></html>", None)

    # WebGL1 + WebGL2 parameters
    webgl1_params = ctx.eval("""
        (function() {
            var canvas = document.createElement('canvas');
            var gl = canvas.getContext('webgl');
            if (!gl) return null;
            var result = {};
            var ext = gl.getExtension('WEBGL_debug_renderer_info');

            // All Khronos-defined pname values
            var pnames = {
                0x1F00: 'VENDOR', 0x1F01: 'RENDERER', 0x1F02: 'VERSION',
                0x8B8C: 'SHADING_LANGUAGE_VERSION',
                0x0D33: 'MAX_TEXTURE_SIZE', 0x84E8: 'MAX_RENDERBUFFER_SIZE',
                0x8869: 'MAX_VERTEX_ATTRIBS', 0x8DFB: 'MAX_VERTEX_UNIFORM_VECTORS',
                0x8B49: 'MAX_FRAGMENT_UNIFORM_VECTORS', 0x8B4A: 'MAX_VARYING_VECTORS',
                0x8872: 'MAX_TEXTURE_IMAGE_UNITS', 0x8DFC: 'MAX_COMBINED_TEXTURE_IMAGE_UNITS',
                0x8DFD: 'MAX_VERTEX_TEXTURE_IMAGE_UNITS', 0x8508: 'MAX_CUBE_MAP_TEXTURE_SIZE',
                0x0D31: 'MAX_VIEWPORT_DIMS', 0x846E: 'ALIASED_LINE_WIDTH_RANGE',
                0x846D: 'ALIASED_POINT_SIZE_RANGE',
                0x0D50: 'SUBPIXEL_BITS', 0x0D52: 'RED_BITS', 0x0D53: 'GREEN_BITS',
                0x0D54: 'BLUE_BITS', 0x0D55: 'ALPHA_BITS', 0x0D56: 'DEPTH_BITS',
                0x0D57: 'STENCIL_BITS', 0x80A8: 'SAMPLE_BUFFERS', 0x80A9: 'SAMPLES',
                0x86A2: 'NUM_COMPRESSED_TEXTURE_FORMATS', 0x86A3: 'COMPRESSED_TEXTURE_FORMATS',
                0x8192: 'GENERATE_MIPMAP_HINT', 0x0B21: 'LINE_WIDTH',
            };
            for (var hex in pnames) {
                var val = gl.getParameter(parseInt(hex));
                result[pnames[hex]] = (val && val.length !== undefined && typeof val !== 'string')
                    ? Array.from(val) : val;
            }
            if (ext) {
                result['UNMASKED_VENDOR_WEBGL'] = gl.getParameter(ext.UNMASKED_VENDOR_WEBGL);
                result['UNMASKED_RENDERER_WEBGL'] = gl.getParameter(ext.UNMASKED_RENDERER_WEBGL);
            }
            result['extensions'] = gl.getSupportedExtensions();

            // Shader precision (12 combinations)
            var sp = {};
            var types = [[35633, 'VERTEX'], [35632, 'FRAGMENT']];
            var precs = [[36336, 'LOW_FLOAT'], [36337, 'MEDIUM_FLOAT'], [36338, 'HIGH_FLOAT'],
                         [36339, 'LOW_INT'], [36340, 'MEDIUM_INT'], [36341, 'HIGH_INT']];
            for (var t of types) {
                for (var p of precs) {
                    var fmt = gl.getShaderPrecisionFormat(t[0], p[0]);
                    sp[t[1] + '_' + p[1]] = fmt ? {rangeMin: fmt.rangeMin, rangeMax: fmt.rangeMax, precision: fmt.precision} : null;
                }
            }
            result['shaderPrecision'] = sp;
            return result;
        })()
    """)

    webgl2_params = ctx.eval("""
        (function() {
            var canvas = document.createElement('canvas');
            var gl = canvas.getContext('webgl2');
            if (!gl) return null;
            var result = {};
            var pnames = {
                0x8073: 'MAX_3D_TEXTURE_SIZE', 0x88FF: 'MAX_ARRAY_TEXTURE_LAYERS',
                0x8CDF: 'MAX_COLOR_ATTACHMENTS', 0x8068: 'MAX_DRAW_BUFFERS',
                0x8D6B: 'MAX_ELEMENT_INDEX', 0x9125: 'MAX_FRAGMENT_INPUT_COMPONENTS',
                0x8B49: 'MAX_FRAGMENT_UNIFORM_COMPONENTS', 0x8A2D: 'MAX_FRAGMENT_UNIFORM_BLOCKS',
                0x8905: 'MAX_PROGRAM_TEXEL_OFFSET', 0x8D57: 'MAX_SAMPLES',
                0x9111: 'MAX_SERVER_WAIT_TIMEOUT', 0x84FD: 'MAX_TEXTURE_LOD_BIAS',
                0x8C8A: 'MAX_TRANSFORM_FEEDBACK_INTERLEAVED_COMPONENTS',
                0x8C8B: 'MAX_TRANSFORM_FEEDBACK_SEPARATE_ATTRIBS',
                0x8C80: 'MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS',
                0x8A30: 'MAX_UNIFORM_BLOCK_SIZE', 0x8A2F: 'MAX_UNIFORM_BUFFER_BINDINGS',
                0x8B4B: 'MAX_VARYING_COMPONENTS', 0x9122: 'MAX_VERTEX_OUTPUT_COMPONENTS',
                0x8A2C: 'MAX_VERTEX_UNIFORM_BLOCKS', 0x8B4A: 'MAX_VERTEX_UNIFORM_COMPONENTS',
                0x8A31: 'MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS',
                0x8A33: 'MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS',
                0x8A2E: 'MAX_COMBINED_UNIFORM_BLOCKS',
                0x8A34: 'UNIFORM_BUFFER_OFFSET_ALIGNMENT',
                0x8904: 'MIN_PROGRAM_TEXEL_OFFSET', 0x9247: 'MAX_CLIENT_WAIT_TIMEOUT_WEBGL',
            };
            for (var hex in pnames) {
                var val = gl.getParameter(parseInt(hex));
                result[pnames[hex]] = (val && val.length !== undefined && typeof val !== 'string')
                    ? Array.from(val) : val;
            }
            result['extensions'] = gl.getSupportedExtensions();
            return result;
        })()
    """)

    # Navigator properties
    navigator_props = ctx.eval("""
        (function() {
            var r = {};
            r.userAgent = navigator.userAgent;
            r.platform = navigator.platform;
            r.vendor = navigator.vendor;
            r.language = navigator.language;
            r.languages = navigator.languages;
            r.hardwareConcurrency = navigator.hardwareConcurrency;
            r.deviceMemory = navigator.deviceMemory;
            r.maxTouchPoints = navigator.maxTouchPoints;
            r.webdriver = navigator.webdriver;
            r.pdfViewerEnabled = navigator.pdfViewerEnabled;
            r.cookieEnabled = navigator.cookieEnabled;
            r.onLine = navigator.onLine;
            r.doNotTrack = navigator.doNotTrack;
            r.appName = navigator.appName;
            r.appCodeName = navigator.appCodeName;
            r.appVersion = navigator.appVersion;
            r.product = navigator.product;
            r.productSub = navigator.productSub;
            r.vendorSub = navigator.vendorSub;
            // Connection
            if (navigator.connection) {
                r.connection = {
                    effectiveType: navigator.connection.effectiveType,
                    downlink: navigator.connection.downlink,
                    rtt: navigator.connection.rtt,
                    saveData: navigator.connection.saveData,
                    type: navigator.connection.type
                };
            }
            // Battery
            // Permissions
            r.permissions = typeof navigator.permissions;
            // Plugins
            r.pluginsLength = navigator.plugins.length;
            r.mimeTypesLength = navigator.mimeTypes.length;
            return r;
        })()
    """)

    # Screen properties
    screen_props = ctx.eval("""
        (function() {
            return {
                width: screen.width, height: screen.height,
                availWidth: screen.availWidth, availHeight: screen.availHeight,
                colorDepth: screen.colorDepth, pixelDepth: screen.pixelDepth,
                availLeft: screen.availLeft, availTop: screen.availTop
            };
        })()
    """)

    # AudioContext
    audio_props = ctx.eval("""
        (function() {
            try {
                var ctx = new AudioContext();
                return {
                    sampleRate: ctx.sampleRate,
                    baseLatency: ctx.baseLatency,
                    outputLatency: ctx.outputLatency,
                    state: ctx.state
                };
            } catch(e) { return {error: e.message}; }
        })()
    """)

    # matchMedia
    media_props = ctx.eval("""
        (function() {
            var features = [
                'prefers-color-scheme:light', 'prefers-color-scheme:dark',
                'prefers-reduced-motion:no-preference', 'prefers-reduced-motion:reduce',
                'prefers-contrast:no-preference', 'prefers-contrast:more',
                'forced-colors:none', 'forced-colors:active',
                'color-gamut:srgb', 'color-gamut:p3',
                'scripting:enabled', 'scripting:none',
                'pointer:fine', 'pointer:coarse',
                'hover:hover', 'hover:none',
                'any-pointer:fine', 'any-pointer:coarse',
                'any-hover:hover', 'any-hover:none',
                'display-mode:browser', 'display-mode:fullscreen',
                'inverted-colors:none', 'inverted-colors:inverted',
                'update:fast', 'update:slow',
                'dynamic-range:standard', 'dynamic-range:high'
            ];
            var r = {};
            features.forEach(function(f) {
                r[f] = window.matchMedia('(' + f + ')').matches;
            });
            return r;
        })()
    """)

    return {
        "webgl1": webgl1_params,
        "webgl2": webgl2_params,
        "navigator": navigator_props,
        "screen": screen_props,
        "audio": audio_props,
        "media": media_props,
    }


def main():
    parser = argparse.ArgumentParser(description="H03 L2: IV8 surface sampler")
    parser.add_argument("--output", "-o", default="actual_surface.json", help="Output JSON file")
    args = parser.parse_args()

    print("Sampling IV8 fingerprint surface...")
    data = sample_iv8()

    output_path = Path(args.output)
    output_path.write_text(json.dumps(data, indent=2, default=str), encoding="utf-8")
    print(f"Written to {output_path}")
    print(f"Sections: {list(data.keys())}")


if __name__ == "__main__":
    main()
