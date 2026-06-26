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

function collectIdlFiles(dir, recursive) {
  if (recursive) {
    const out = [];
    function walk(d) {
      for (const entry of fs.readdirSync(d, { withFileTypes: true })) {
        const full = path.join(d, entry.name);
        if (entry.isDirectory()) walk(full);
        else if (entry.name.endsWith(".idl")) out.push(full);
      }
    }
    walk(dir);
    return out;
  }
  return fs.readdirSync(dir)
    .filter(f => f.endsWith(".idl"))
    .map(f => path.join(dir, f));
}

function parseDirectory(dir, recursive) {
  if (!fs.existsSync(dir)) {
    console.error(`[parse-chromium-idl] Directory not found: ${dir}`);
    return [];
  }

  const files = collectIdlFiles(dir, !!recursive);
  if (files.length === 0) {
    console.error(`[parse-chromium-idl] No .idl files found in ${dir}`);
    return [];
  }

  console.error(`[parse-chromium-idl] Found ${files.length} .idl files in ${dir} (recursive=${!!recursive})`);

  const results = {};
  let parseErrors = 0;
  let totalDefs = 0;

  for (const filePath of files) {
    const baseName = path.basename(filePath, ".idl");
    const relName = recursive ? path.relative(dir, filePath).replace(/\\/g, "/").replace(/\.idl$/, "") : baseName;
    const src = fs.readFileSync(filePath, "utf8");
    try {
      const ast = webidl2.parse(src);
      // Multiple files may define the same base name in recursive mode;
      // key by relative path to avoid collisions.
      results[relName] = ast;
      totalDefs += ast.length;
    } catch (err) {
      parseErrors++;
      console.error(`[parse-chromium-idl] Parse error in ${relName}: ${err.message}`);
    }
  }

  console.error(`[parse-chromium-idl] Parsed ${files.length - parseErrors}/${files.length} files, ${totalDefs} definitions, ${parseErrors} errors`);
  return results;
}

// ── IR normalization (interface extraction) ─────────────────────────────────
function extractInterfaces(results) {
  const interfaces = {};
  for (const [fileKey, ast] of Object.entries(results)) {
    for (const node of ast) {
      if (node.type === "interface" || node.type === "interface mixin" ||
          node.type === "callback interface" || node.type === "namespace") {
        const name = node.name;
        if (!interfaces[name]) {
          interfaces[name] = {
            name,
            kind: node.type,
            source: "chromium",
            source_file: fileKey,
            inheritance: node.inheritance ? node.inheritance.id : null,
            ext_attrs: (node.extAttrs || []).map(a => a.name),
            members: [],
          };
        }
        for (const m of node.members || []) {
          interfaces[name].members.push({
            kind: m.type,
            name: m.name || null,
            type: m.idlType ? idlTypeToString(m.idlType) : null,
            static: !!m.static,
            readonly: !!m.readonly,
          });
        }
      } else if (node.type === "includes") {
        // includes statement: Interface includes Mixin — recorded separately
        const target = node.target;
        const includes = node.includes;
        if (!interfaces[target]) {
          interfaces[target] = { name: target, kind: "interface", source: "chromium", source_file: fileKey, inheritance: null, ext_attrs: [], members: [], includes: [] };
        }
        if (!interfaces[target].includes) interfaces[target].includes = [];
        interfaces[target].includes.push(includes);
      }
    }
  }
  return interfaces;
}

function idlTypeToString(idlType) {
  if (!idlType) return null;
  if (typeof idlType === "string") return idlType;
  if (idlType.union) {
    return "(" + (idlType.idlType || []).map(idlTypeToString).join(" or ") + ")";
  }
  if (idlType.generic) {
    return idlType.generic + "<" + idlTypeToString(idlType.idlType) + ">";
  }
  if (idlType.prefix) {
    return idlType.prefix + " " + idlTypeToString(idlType.idlType);
  }
  return idlType.idlType || idlType.value || null;
}

// ── CLI ──────────────────────────────────────────────────────────────────────

function main() {
  const args = process.argv.slice(2);
  const dirIdx = args.indexOf("--dir");
  const dir = dirIdx >= 0 ? args[dirIdx + 1] : null;
  const recursive = args.includes("--recursive") || args.includes("-r");
  const outIdx = args.indexOf("--output");
  const outPath = outIdx >= 0 ? args[outIdx + 1] : null;
  const irMode = args.includes("--ir");

  if (!dir) {
    console.error("Usage: node parse-chromium-idl.js --dir <path> [--recursive] [--output <file>] [--ir] [--stats]");
    console.error("  Example: node parse-chromium-idl.js --dir /path/to/blink --recursive --ir --output chromium-148.ir.json");
    process.exit(1);
  }

  const results = parseDirectory(dir, recursive);

  if (args.includes("--stats")) {
    const counts = {};
    for (const [name, ast] of Object.entries(results)) {
      for (const node of ast) {
        counts[node.type] = (counts[node.type] || 0) + 1;
      }
    }
    console.log(JSON.stringify({ files: Object.keys(results).length, counts }));
  }

  if (irMode) {
    // Produce normalized IR output
    const interfaces = extractInterfaces(results);
    const ir = {
      schema_version: "chromium-idl-ir.v0.1",
      metadata: {
        generated_at: new Date().toISOString(),
        source: "chromium-blink",
        source_dir: dir,
        recursive: recursive,
        total_files: Object.keys(results).length,
        total_interfaces: Object.keys(interfaces).length,
      },
      definitions: Object.values(interfaces),
    };
    const dest = outPath || path.join(__dirname, "output", "chromium-148.ir.json");
    fs.mkdirSync(path.dirname(dest), { recursive: true });
    fs.writeFileSync(dest, JSON.stringify(ir, null, 2), "utf8");
    console.error(`[parse-chromium-idl] IR written: ${dest} (${ir.metadata.total_interfaces} interfaces)`);
  } else {
    // Default: write raw AST
    const dest = outPath || path.join(__dirname, "tmp", "chromium-ast.json");
    fs.mkdirSync(path.dirname(dest), { recursive: true });
    fs.writeFileSync(dest, JSON.stringify(results, null, 2), "utf8");
    console.error(`[parse-chromium-idl] Wrote ${dest}`);
  }
}

main();
