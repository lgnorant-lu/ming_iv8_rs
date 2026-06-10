#!/usr/bin/env node
"use strict";

/**
 * fetch-webref.js — Load and parse all W3C Web IDL specs from @webref/idl.
 *
 * Uses @webref/idl's parseAll() to get webidl2-parsed AST for every spec.
 * Writes the result to tmp/webref-ast.json for downstream normalization.
 */

const path = require("path");
const fs = require("fs");

async function main() {
  const { parseAll } = require("@webref/idl");

  console.error("[fetch-webref] Parsing all @webref/idl specs...");
  const parsed = await parseAll();

  const entries = Object.entries(parsed);
  let totalDefs = 0;
  const stats = { interface: 0, dictionary: 0, enum: 0, typedef: 0, callback: 0, "callback interface": 0, namespace: 0, "interface mixin": 0, includes: 0 };

  const output = {};
  for (const [shortname, ast] of entries) {
    output[shortname] = ast;
    for (const node of ast) {
      totalDefs++;
      if (stats[node.type] !== undefined) stats[node.type]++;
    }
  }

  const outPath = path.join(__dirname, "tmp", "webref-ast.json");
  fs.mkdirSync(path.dirname(outPath), { recursive: true });
  fs.writeFileSync(outPath, JSON.stringify(output, null, 2), "utf8");

  console.error(`[fetch-webref] Done. ${entries.length} specs, ${totalDefs} definitions.`);
  for (const [k, v] of Object.entries(stats)) {
    if (v > 0) console.error(`  ${k}: ${v}`);
  }

  // Print stats as JSON to stdout for piping
  console.log(JSON.stringify({ specs: entries.length, definitions: totalDefs, stats }));
}

main().catch(err => {
  console.error("[fetch-webref] Error:", err.message);
  process.exit(1);
});
