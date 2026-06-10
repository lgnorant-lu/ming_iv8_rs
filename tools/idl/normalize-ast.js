#!/usr/bin/env node
"use strict";

/**
 * normalize-ast.js — Normalize webidl2 AST nodes to IV8 internal JSON IR format.
 *
 * Input:  webidl2-parsed AST nodes (from fetch-webref.js or parse-chromium-idl.js)
 * Output: Normalized {}definitions{} keyed by (source, name), with:
 *   kind, name, source, inheritance, ext_attrs, members, mixin_sources, partial_sources
 *
 * WebIDL type references are preserved as-is (type mapping happens in type-mapper.js).
 */

const path = require("path");
const fs = require("fs");

// ── Helpers ──────────────────────────────────────────────────────────────────

function extractExtAttrs(node) {
  if (!node.extAttrs || node.extAttrs.length === 0) return [];
  return node.extAttrs.map(ea => {
    if (typeof ea === "object" && ea.name !== undefined) {
      const r = { name: ea.name };
      if (ea.rhs) r.value = ea.rhs.value ?? ea.rhs.type ?? true;
      else r.value = true;
      return r;
    }
    return { raw: JSON.stringify(ea) };
  });
}

function normalizeIdlType(idlType) {
  if (!idlType) return null;
  if (typeof idlType === "string") return { kind: "primitive", name: idlType };

  // Union type
  if (idlType.union && idlType.idlType) {
    const types = idlType.idlType.map(t => normalizeIdlType(t));
    return { kind: "union", types, nullable: idlType.nullable || false };
  }

  // Generic: sequence<T>, Promise<T>, FrozenArray<T>, record<K,V>, ObservableArray<T>
  if (idlType.generic) {
    const inner = normalizeIdlType(idlType.idlType);
    return {
      kind: "generic",
      generic: idlType.generic,
      inner,
      nullable: idlType.nullable || false,
    };
  }

  // Simple named type
  const name = idlType.idlType || idlType;
  return {
    kind: "name",
    name,
    nullable: idlType.nullable || false,
  };
}

function normalizeArgs(args) {
  if (!args) return [];
  return args.map(a => ({
    name: a.name,
    type: normalizeIdlType(a.idlType),
    optional: a.optional || false,
    variadic: a.variadic || false,
    default: a.default ?? null,
  }));
}

function normalizeMember(m) {
  const base = {
    kind: m.type, // attribute | operation | const | constructor | iterable | stringifier | maplike | setlike
    name: m.name || null,
    ext_attrs: extractExtAttrs(m),
  };

  if (m.type === "attribute") {
    base.type = normalizeIdlType(m.idlType);
    base.readonly = m.readonly || false;
  }

  if (m.type === "operation" || m.type === "constructor") {
    base.return_type = normalizeIdlType(m.idlType);
    base.arguments = normalizeArgs(m.arguments);
    base.special = m.special || "";
  }

  if (m.type === "const") {
    base.type = normalizeIdlType(m.idlType);
    base.value = m.value ?? null;
  }

  if (m.type === "iterable" || m.type === "maplike" || m.type === "setlike") {
    if (m.idlType) base.type = normalizeIdlType(m.idlType);
    if (m.readonly) base.readonly = true;
  }

  return base;
}

// ── Normalize a single top-level AST node ────────────────────────────────────

function normalizeNode(node, source) {
  if (!node) return null;

  const kindMap = {
    "interface": "interface",
    "dictionary": "dictionary",
    "enum": "enum",
    "typedef": "typedef",
    "callback": "callback",
    "callback interface": "callback_interface",
    "namespace": "namespace",
    "interface mixin": "interface_mixin",
    "includes": "includes",
  };

  const kind = kindMap[node.type];
  if (!kind) return null;

  const def = {
    kind,
    name: node.name || null,
    source,
    inheritance: node.inheritance || null,
    ext_attrs: extractExtAttrs(node),
    members: (node.members || []).map(normalizeMember),
    partial: node.partial || false,
  };

  if (kind === "enum") {
    def.values = (node.values || []).map(v => v.value);
    delete def.inheritance;
    delete def.members;
  }

  if (kind === "typedef" || kind === "callback" || kind === "callback_interface") {
    def.idl_type = normalizeIdlType(node.idlType);
    if (node.arguments) def.arguments = normalizeArgs(node.arguments);
    if (def.members) delete def.members;
    if (def.inheritance === null) delete def.inheritance;
  }

  if (kind === "includes") {
    def.target = node.target || null;
    def.includes = node.includes || null;
    delete def.members;
    delete def.inheritance;
  }

  return def;
}

// ── Main ─────────────────────────────────────────────────────────────────────

function main() {
  const args = process.argv.slice(2);
  const source = args.includes("--source") ? args[args.indexOf("--source") + 1] : "w3c";
  const inputFile = args.includes("--input") ? args[args.indexOf("--input") + 1] : null;

  let input;
  if (inputFile) {
    input = JSON.parse(fs.readFileSync(inputFile, "utf8"));
  } else {
    // Read from tmp/webref-ast.json by default
    const defaultPath = path.join(__dirname, "tmp", "webref-ast.json");
    input = JSON.parse(fs.readFileSync(defaultPath, "utf8"));
  }

  const definitions = [];
  const stats = {
    interface: 0, dictionary: 0, enum: 0, typedef: 0,
    callback: 0, callback_interface: 0, namespace: 0,
    interface_mixin: 0, includes: 0, total_members: 0,
  };

  // Input is { shortname: [nodes] }
  for (const [shortname, nodes] of Object.entries(input)) {
    for (const node of nodes) {
      const def = normalizeNode(node, source);
      if (!def) continue;
      definitions.push(def);
      if (stats[def.kind] !== undefined) stats[def.kind]++;
      if (def.members) stats.total_members += def.members.length;
    }
  }

  const result = { source, definitions, stats };
  const outPath = path.join(__dirname, "tmp", `normalized-${source}.json`);
  fs.writeFileSync(outPath, JSON.stringify(result, null, 2), "utf8");

  console.error(`[normalize-ast] source=${source}: ${definitions.length} definitions`);
  for (const [k, v] of Object.entries(stats)) {
    if (k === "total_members") console.error(`  ${k}: ${v}`);
    else if (v > 0) console.error(`  ${k}: ${v}`);
  }

  // Stats to stdout
  console.log(JSON.stringify(stats));
}

main();
