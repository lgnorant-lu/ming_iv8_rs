//! AMD define/require subset for pre-bundled modules (S7 Branch A).
//!
//! This is **not** full RequireJS (no async script fetch, no path config,
//! no loader plugins like text!). It implements almond-style **already-defined
//! module registry** so bundled AMD that only needs `define`/`require` sync
//! resolution can run. See: requirejs/almond (minimal AMD for built bundles).

/// Detect AMD define() markers in source.
pub fn detect_amd_markers(source: &str) -> bool {
    source.contains("define(")
        && (source.contains("require")
            || source.contains("exports")
            || source.contains("module")
            || source.contains("define.amd"))
}

/// JS prelude: sync AMD registry (define + require) for preloaded modules.
///
/// Supports:
/// - `define(id, deps, factory)`
/// - `define(deps, factory)` (anonymous → `__anon_N`)
/// - `define(factory)`
/// - `require(deps, callback)` sync when all deps already defined
/// - `require(id)` returns exports
///
/// Does **not** support: path maps, shim config, async script load, plugins.
pub fn amd_prelude() -> &'static str {
    r#"
(function(){
  if (globalThis.__iv8_amd && globalThis.__iv8_amd.ready) return;
  var modules = {};
  var defined = {};
  var anon = 0;
  function normalize(id){ return String(id); }
  function getExports(id){
    id = normalize(id);
    if (defined[id]) return defined[id].exports;
    var def = modules[id];
    if (!def) throw new Error('AMD module not found: ' + id);
    if (def.loading) return def.exports;
    def.loading = true;
    var exports = {};
    var module = { exports: exports };
    def.exports = exports;
    var deps = def.deps || [];
    var args = [];
    for (var i = 0; i < deps.length; i++) {
      var d = deps[i];
      if (d === 'exports') args.push(exports);
      else if (d === 'module') args.push(module);
      else if (d === 'require') args.push(req);
      else args.push(getExports(d));
    }
    var ret = typeof def.factory === 'function'
      ? def.factory.apply(null, args)
      : def.factory;
    if (ret !== undefined) module.exports = ret;
    defined[id] = module;
    def.loading = false;
    return module.exports;
  }
  function define(a, b, c){
    var id, deps, factory;
    if (typeof a === 'string') {
      id = a; deps = b; factory = c;
      if (typeof deps === 'function') { factory = deps; deps = []; }
    } else if (Array.isArray(a)) {
      id = '__anon_' + (anon++); deps = a; factory = b;
    } else {
      id = '__anon_' + (anon++); deps = []; factory = a;
    }
    modules[normalize(id)] = { deps: deps || [], factory: factory, exports: {} };
  }
  define.amd = { jQuery: true };
  function req(deps, cb){
    if (typeof deps === 'string') return getExports(deps);
    var args = [];
    for (var i = 0; i < deps.length; i++) args.push(getExports(deps[i]));
    if (typeof cb === 'function') return cb.apply(null, args);
    return args;
  }
  globalThis.define = define;
  globalThis.require = req;
  globalThis.__iv8_amd = { ready: true, modules: modules, defined: defined, require: req, define: define };
})();
"#
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kernel::embedded_v8::EmbeddedV8Kernel;
    use crate::kernel::{EvalOpts, KernelConfig};
    use crate::convert::RustValue;

    #[test]
    fn test_amd_detect() {
        assert!(detect_amd_markers(
            "define(['exports'], function(exports){ exports.x=1; });"
        ));
    }

    #[test]
    fn test_amd_prelude_define_require() {
        let mut k = EmbeddedV8Kernel::new(KernelConfig::default()).unwrap();
        k.eval(amd_prelude(), EvalOpts::default()).unwrap();
        k.eval(
            r#"
define('math', ['exports'], function(exports){ exports.add = function(a,b){ return a+b; }; });
define('app', ['math', 'exports'], function(math, exports){ exports.v = math.add(2,3); });
globalThis.__amd_v = require('app').v;
"#,
            EvalOpts::default(),
        )
        .unwrap();
        assert_eq!(
            k.eval_to_rust_value("globalThis.__amd_v"),
            RustValue::Int(5)
        );
    }
}
