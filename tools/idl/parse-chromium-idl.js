#!/usr/bin/env node
"use strict";

/**
 * parse-chromium-idl.js — Parse Chromium-style .idl files using webidl2.
 *
 * Reads .idl files from a given directory, parses each with webidl2,
 * normalizes to internal IR format, and writes the result.
 *
 * Usage: node parse-chromium-idl.js --dir <path> [--stats]
 *
 * Chromium's IDL dialect differences from W3C Web IDL:
 *   - Uses [RuntimeEnabled], [SecureContext], [CrossOrigin] extended attributes
 *   - May have callback-style interfaces not in standard format
 *   - May have partial interface definitions in separate files
 *
 * Unparseable files are reported but do not abort the pipeline.
 */

const path = require("path");
const fs = require("fs");
const webidl2 = require("webidl2");

function parseDirectory(dir) {
  if (!fs.existsSync(dir)) {
    console.error(`[parse-chromium-idl] Directory not found: ${dir}`);
    return [];
  }

  const files = fs.readdirSync(dir).filter(f => f.endsWith(".idl"));
  if (files.length === 0) {
    console.error(`[parse-chromium-idl] No .idl files found in ${dir}`);
    return [];
  }

  console.error(`[parse-chromium-idl] Found ${files.length} .idl files in ${dir}`);

  const results = {};
  let parseErrors = 0;
  let totalDefs = 0;

  for (const file of files) {
    const filePath = path.join(dir, file);
    const src = fs.readFileSync(filePath, "utf8");
    try {
      const ast = webidl2.parse(src);
      results[file.replace(".idl", "")] = ast;
      totalDefs += ast.length;
    } catch (err) {
      parseErrors++;
      console.error(`[parse-chromium-idl] Parse error in ${file}: ${err.message}`);
    }
  }

  console.error(`[parse-chromium-idl] Parsed ${files.length - parseErrors}/${files.length} files, ${totalDefs} definitions, ${parseErrors} errors`);
  return results;
}

// ── CLI ──────────────────────────────────────────────────────────────────────

function main() {
  const args = process.argv.slice(2);
  const dirIdx = args.indexOf("--dir");
  const dir = dirIdx >= 0 ? args[dirIdx + 1] : null;

  if (!dir) {
    console.error("Usage: node parse-chromium-idl.js --dir <path> [--stats]");
    console.error("  Example: node parse-chromium-idl.js --dir /path/to/chromium/idl");
    process.exit(1);
  }

  const results = parseDirectory(dir);

  if (args.includes("--stats")) {
    const counts = {};
    for (const [name, ast] of Object.entries(results)) {
      for (const node of ast) {
        counts[node.type] = (counts[node.type] || 0) + 1;
      }
    }
    console.log(JSON.stringify({ files: Object.keys(results).length, counts }));
  }

  // Write result
  const outPath = path.join(__dirname, "tmp", "chromium-ast.json");
  fs.writeFileSync(outPath, JSON.stringify(results, null, 2), "utf8");
  console.error(`[parse-chromium-idl] Wrote ${outPath}`);
}

main();
