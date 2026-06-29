/* IV8 testharnessreport.js — vendor integration for IV8 V8 isolate.
 *
 * This file replaces the default WPT testharnessreport.js to configure
 * testharness.js for non-browser execution:
 * - output: false — disable HTML result rendering (avoids make_dom crashes)
 * - explicit_timeout: true — IV8 controls test completion via eval_promise
 *
 * Per WPT testharness.js API documentation:
 * "output_document — The document to which results should be logged.
 *  By default this is the current document."
 * We set output: false to skip rendering entirely.
 *
 * Per Chromium's testharnessreport.js pattern:
 * "The default output formats test results into an HTML table, but for
 *  the Blink layout test runner, we dump the results as text in the
 *  completion callback, so we disable the default output."
 */

setup({
    output: false,
});
