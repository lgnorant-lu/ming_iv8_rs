/**
 * iv8-rs Browser Fingerprint Export Script
 *
 * Run in Chrome DevTools console (F12 -> Console).
 * Exports a complete browser fingerprint as JSON, compatible with
 * iv8_rs.load_profile() / JSContext(profile="path.json").
 *
 * Usage:
 *   1. Open Chrome DevTools (F12)
 *   2. Paste this entire script into Console
 *   3. Press Enter
 *   4. JSON is copied to clipboard + printed to console
 *   5. Save as .json file, use with iv8_rs.JSContext(profile="file.json")
 */
(async function() {
    "use strict";
    var env = {};

    // --- Navigator ---
    env["navigator.userAgent"] = navigator.userAgent;
    env["navigator.appVersion"] = navigator.appVersion;
    env["navigator.platform"] = navigator.platform;
    env["navigator.vendor"] = navigator.vendor || "";
    env["navigator.vendorSub"] = navigator.vendorSub || "";
    env["navigator.product"] = navigator.product || "Gecko";
    env["navigator.productSub"] = navigator.productSub || "";
    env["navigator.language"] = navigator.language;
    env["navigator.languages"] = JSON.stringify(navigator.languages || []);
    env["navigator.hardwareConcurrency"] = navigator.hardwareConcurrency || 4;
    env["navigator.deviceMemory"] = navigator.deviceMemory || 8;
    env["navigator.maxTouchPoints"] = navigator.maxTouchPoints || 0;
    env["navigator.cookieEnabled"] = navigator.cookieEnabled;
    env["navigator.onLine"] = navigator.onLine;
    env["navigator.doNotTrack"] = navigator.doNotTrack;
    env["navigator.webdriver"] = navigator.webdriver;
    env["navigator.appName"] = navigator.appName || "Netscape";
    env["navigator.appCodeName"] = navigator.appCodeName || "Mozilla";
    env["navigator.pdfViewerEnabled"] = navigator.pdfViewerEnabled || false;

    // --- Navigator UserAgentData ---
    if (navigator.userAgentData) {
        env["navigator.userAgentData.brands"] = JSON.stringify(navigator.userAgentData.brands);
        env["navigator.userAgentData.mobile"] = navigator.userAgentData.mobile;
        env["navigator.userAgentData.platform"] = navigator.userAgentData.platform;
        try {
            var hev = await navigator.userAgentData.getHighEntropyValues([
                "architecture", "bitness", "fullVersionList",
                "model", "platformVersion", "wow64"
            ]);
            env["navigator.userAgentData.architecture"] = hev.architecture || "";
            env["navigator.userAgentData.bitness"] = hev.bitness || "";
            env["navigator.userAgentData.fullVersionList"] = JSON.stringify(hev.fullVersionList || []);
            env["navigator.userAgentData.model"] = hev.model || "";
            env["navigator.userAgentData.platformVersion"] = hev.platformVersion || "";
            env["navigator.userAgentData.wow64"] = hev.wow64 || false;
        } catch(e) {
            console.warn("[iv8-rs export] getHighEntropyValues failed:", e);
        }
    }

    // --- Screen ---
    env["screen.width"] = screen.width;
    env["screen.height"] = screen.height;
    env["screen.availWidth"] = screen.availWidth;
    env["screen.availHeight"] = screen.availHeight;
    env["screen.colorDepth"] = screen.colorDepth;
    env["screen.pixelDepth"] = screen.pixelDepth;
    env["screen.orientation.angle"] = (screen.orientation && screen.orientation.angle) || 0;
    env["screen.orientation.type"] = (screen.orientation && screen.orientation.type) || "landscape-primary";
    env["screen.isExtended"] = screen.isExtended || false;

    // --- Window ---
    env["window.innerWidth"] = window.innerWidth;
    env["window.innerHeight"] = window.innerHeight;
    env["window.outerWidth"] = window.outerWidth;
    env["window.outerHeight"] = window.outerHeight;
    env["window.devicePixelRatio"] = window.devicePixelRatio;
    env["window.screenX"] = window.screenX || 0;
    env["window.screenY"] = window.screenY || 0;

    // --- Document ---
    env["document.characterSet"] = document.characterSet || "UTF-8";
    env["document.compatMode"] = document.compatMode || "CSS1Compat";

    // --- Location ---
    env["location.href"] = location.href;
    env["location.origin"] = location.origin;
    env["location.protocol"] = location.protocol;
    env["location.host"] = location.host;
    env["location.hostname"] = location.hostname;
    env["location.port"] = location.port;
    env["location.pathname"] = location.pathname;
    env["location.search"] = location.search;
    env["location.hash"] = location.hash;

    // --- Canvas Fingerprint ---
    try {
        var c = document.createElement("canvas");
        c.width = 300; c.height = 150;
        var ctx2d = c.getContext("2d");
        // Standard fingerprint pattern (used by most fingerprinting libraries)
        ctx2d.textBaseline = "top";
        ctx2d.font = "14px 'Arial'";
        ctx2d.fillStyle = "#f60";
        ctx2d.fillRect(125, 1, 62, 20);
        ctx2d.fillStyle = "#069";
        ctx2d.fillText("ClientJS,org <canvas", 2, 15);
        ctx2d.fillStyle = "rgba(102, 204, 0, 0.7)";
        ctx2d.fillText("ClientJS,org <canvas", 4, 17);
        env["canvas.fingerprint.toDataURL.png"] = c.toDataURL();
    } catch(e) {
        console.warn("[iv8-rs export] Canvas fingerprint failed:", e);
    }

    // --- WebGL ---
    try {
        var glCanvas = document.createElement("canvas");
        var gl = glCanvas.getContext("webgl") || glCanvas.getContext("experimental-webgl");
        if (gl) {
            var dbgInfo = gl.getExtension("WEBGL_debug_renderer_info");
            if (dbgInfo) {
                env["webgl.UNMASKED_VENDOR_WEBGL"] = gl.getParameter(dbgInfo.UNMASKED_VENDOR_WEBGL);
                env["webgl.UNMASKED_RENDERER_WEBGL"] = gl.getParameter(dbgInfo.UNMASKED_RENDERER_WEBGL);
            }
            env["webgl.VENDOR"] = gl.getParameter(gl.VENDOR);
            env["webgl.RENDERER"] = gl.getParameter(gl.RENDERER);
            env["webgl.VERSION"] = gl.getParameter(gl.VERSION);
            env["webgl.SHADING_LANGUAGE_VERSION"] = gl.getParameter(gl.SHADING_LANGUAGE_VERSION);
            env["webgl.MAX_TEXTURE_SIZE"] = gl.getParameter(gl.MAX_TEXTURE_SIZE);
            env["webgl.MAX_RENDERBUFFER_SIZE"] = gl.getParameter(gl.MAX_RENDERBUFFER_SIZE);
            env["webgl.MAX_VIEWPORT_DIMS"] = JSON.stringify(Array.from(gl.getParameter(gl.MAX_VIEWPORT_DIMS)));
            env["webgl.MAX_VERTEX_ATTRIBS"] = gl.getParameter(gl.MAX_VERTEX_ATTRIBS);
            env["webgl.MAX_VERTEX_UNIFORM_VECTORS"] = gl.getParameter(gl.MAX_VERTEX_UNIFORM_VECTORS);
            env["webgl.MAX_FRAGMENT_UNIFORM_VECTORS"] = gl.getParameter(gl.MAX_FRAGMENT_UNIFORM_VECTORS);
            env["webgl.MAX_VARYING_VECTORS"] = gl.getParameter(gl.MAX_VARYING_VECTORS);
            env["webgl.MAX_TEXTURE_IMAGE_UNITS"] = gl.getParameter(gl.MAX_TEXTURE_IMAGE_UNITS);
            env["webgl.MAX_COMBINED_TEXTURE_IMAGE_UNITS"] = gl.getParameter(gl.MAX_COMBINED_TEXTURE_IMAGE_UNITS);
            env["webgl.MAX_CUBE_MAP_TEXTURE_SIZE"] = gl.getParameter(gl.MAX_CUBE_MAP_TEXTURE_SIZE);
            env["webgl.extensions"] = JSON.stringify(gl.getSupportedExtensions());
        }
    } catch(e) {
        console.warn("[iv8-rs export] WebGL export failed:", e);
    }

    // --- Performance ---
    env["performance.memory.jsHeapSizeLimit"] = (performance.memory && performance.memory.jsHeapSizeLimit) || 0;
    env["performance.memory.totalJSHeapSize"] = (performance.memory && performance.memory.totalJSHeapSize) || 0;
    env["performance.memory.usedJSHeapSize"] = (performance.memory && performance.memory.usedJSHeapSize) || 0;

    // --- Connection ---
    if (navigator.connection) {
        env["navigator.connection.downlink"] = navigator.connection.downlink || 10;
        env["navigator.connection.effectiveType"] = navigator.connection.effectiveType || "4g";
        env["navigator.connection.rtt"] = navigator.connection.rtt || 50;
        env["navigator.connection.saveData"] = navigator.connection.saveData || false;
    }

    // --- Timezone ---
    try {
        env["timezone"] = Intl.DateTimeFormat().resolvedOptions().timeZone;
    } catch(e) {}

    // --- Meta ---
    env["_meta.exported_from"] = navigator.userAgent;
    env["_meta.exported_at"] = new Date().toISOString();
    env["_meta.url"] = location.href;
    env["_meta.screen"] = screen.width + "x" + screen.height;

    // --- Output ---
    var json = JSON.stringify(env, null, 2);
    console.log("[iv8-rs] Fingerprint exported (" + Object.keys(env).length + " fields):");
    console.log(json);

    // Copy to clipboard
    try {
        await navigator.clipboard.writeText(json);
        console.log("[iv8-rs] Copied to clipboard! Save as .json file.");
    } catch(e) {
        console.log("[iv8-rs] Clipboard copy failed. Copy the JSON above manually.");
    }

    return env;
})();
