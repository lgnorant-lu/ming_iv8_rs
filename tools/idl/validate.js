#!/usr/bin/env node
"use strict";

/**
 * validate.js — Go Gate validator for unified_ir.json.
 *
 * Checks:
 *   --count <category>   Count definitions by kind/source
 *   --topo-check          Verify inheritance chain has no cycles
 *   --type-coverage       Show type mapping coverage
 *   --schema-check        Verify each definition has required fields
 *   --stats               Print full statistics
 *   --go-gate             Run ALL validation gates (exit 0 on PASS)
 */

const path = require("path");
const fs = require("fs");

function loadDefinitions(inputFile) {
  const data = JSON.parse(fs.readFileSync(inputFile, "utf8"));
  return data.definitions || data;
}

function countByKind(defs) {
  const counts = {};
  for (const d of defs) {
    counts[d.kind] = (counts[d.kind] || 0) + 1;
  }
  return counts;
}

function countBySource(defs) {
  const counts = {};
  for (const d of defs) {
    counts[d.source] = (counts[d.source] || 0) + 1;
  }
  return counts;
}

function checkTopology(defs) {
  const defMap = new Map(defs.filter(d => d.name).map(d => [d.name, d]));
  const indegree = new Map();
  const children = new Map();

  for (const [name] of defMap) {
    indegree.set(name, 0);
    children.set(name, []);
  }

  for (const [name, def] of defMap) {
    if (def.inheritance && defMap.has(def.inheritance)) {
      children.get(def.inheritance).push(name);
      indegree.set(name, (indegree.get(name) || 0) + 1);
    }
  }

  const queue = [];
  for (const [name, deg] of indegree) {
    if (deg === 0) queue.push(name);
  }

  const sorted = [];
  while (queue.length > 0) {
    queue.sort();
    const current = queue.shift();
    sorted.push(current);
    for (const child of children.get(current)) {
      indegree.set(child, indegree.get(child) - 1);
      if (indegree.get(child) === 0) queue.push(child);
    }
  }

  const cycles = [];
  for (const [name, deg] of indegree) {
    if (deg > 0) cycles.push(name);
  }

  return { sorted: sorted.length, cycles, pass: cycles.length === 0 };
}

function checkSchema(defs) {
  const errors = [];
  for (const d of defs) {
    if (!d.kind) errors.push(`${d.name || "(no name)"}: missing kind`);
    if (d.kind === "interface" || d.kind === "namespace" || d.kind === "dictionary") {
      if (!d.name) errors.push("(no name): interface without name");
      if (!d.members && d.kind !== "dictionary") errors.push(`${d.name}: interface missing members`);
    }
    if (d.kind === "enum") {
      if (!d.values) errors.push(`${d.name}: enum missing values`);
    }
  }
  return { errors, pass: errors.length === 0 };
}

function checkChromeRuntime(defs) {
  const required = ["onMessage", "sendMessage", "getManifest", "getURL", "connect"];
  const chromeRuntime = defs.find(d => d.name === "ChromeRuntime" || d.name === "chrome.runtime");
  if (!chromeRuntime) return { pass: false, reason: "chrome.runtime not found" };

  const memberNames = new Set((chromeRuntime.members || []).map(m => m.name));
  const missing = required.filter(r => !memberNames.has(r));
  return { pass: missing.length === 0, missing };
}

function checkInheritance(defs) {
  const defNames = new Set(defs.map(d => d.name).filter(Boolean));
  const dangling = [];
  for (const d of defs) {
    if (d.inheritance && d.inheritance !== "null" && !defNames.has(d.inheritance)) {
      dangling.push(`${d.name} inherits ${d.inheritance} (not found)`);
    }
  }
  return { pass: dangling.length === 0, dangling };
}

// ── Main ─────────────────────────────────────────────────────────────────────

function main() {
  const args = process.argv.slice(2);

  let inputFile = null;
  for (let i = 0; i < args.length; i++) {
    if (args[i] === "--input" && i + 1 < args.length) {
      inputFile = args[++i];
    }
  }
  if (!inputFile) inputFile = path.join(__dirname, "output", "unified_ir.json");

  if (!fs.existsSync(inputFile)) {
    // Try merged file
    inputFile = path.join(__dirname, "tmp", "merged-w3c.json");
    if (!fs.existsSync(inputFile)) {
      console.error("[validate] No input file found.");
      process.exit(1);
    }
  }

  const defs = loadDefinitions(inputFile);

  if (args.includes("--count")) {
    const kind = args[args.indexOf("--count") + 1];
    if (kind === "interfaces") {
      const ifaces = defs.filter(d => d.kind === "interface").length;
      console.log(ifaces);
    } else if (kind === "w3c") {
      const w3c = defs.filter(d => d.kind === "interface" && d.source === "w3c").length;
      console.log(w3c);
    } else if (kind === "chrome") {
      const chrome = defs.filter(d => d.kind === "interface" && (d.source === "chromium" || d.source === "manual")).length;
      console.log(chrome);
    } else {
      const counts = countByKind(defs);
      console.log(counts[kind] || 0);
    }
    return;
  }

  if (args.includes("--topo-check")) {
    const result = checkTopology(defs);
    console.log(result.pass ? "PASS: no cycles" : `FAIL: cycles in ${result.cycles.join(", ")}`);
    if (!result.pass) process.exit(1);
    return;
  }

  if (args.includes("--type-coverage")) {
    const covFile = path.join(__dirname, "tmp", "type-coverage.json");
    if (fs.existsSync(covFile)) {
      const cov = JSON.parse(fs.readFileSync(covFile, "utf8"));
      console.log(`${cov.coverage}%`);
    } else {
      console.error("Run type-mapper.js first");
      process.exit(1);
    }
    return;
  }

  if (args.includes("--schema-check")) {
    const result = checkSchema(defs);
    console.log(result.pass ? "PASS" : `FAIL: ${result.errors.length} errors`);
    if (!result.pass) process.exit(1);
    return;
  }

  if (args.includes("--chrome-runtime-check")) {
    const result = checkChromeRuntime(defs);
    console.log(result.pass ? "PASS" : `FAIL: missing ${result.missing.join(", ")}`);
    if (!result.pass) process.exit(1);
    return;
  }

  if (args.includes("--inheritance-check")) {
    const result = checkInheritance(defs);
    console.log(result.pass ? "PASS" : `FAIL: ${result.dangling.length} dangling`);
    if (!result.pass) process.exit(1);
    return;
  }

  if (args.includes("--stats")) {
    const counts = countByKind(defs);
    const sources = countBySource(defs);
    console.log("=== Definition Counts ===");
    for (const [k, v] of Object.entries(counts).sort()) {
      console.log(`  ${k}: ${v}`);
    }
    console.log("=== Source Distribution ===");
    for (const [k, v] of Object.entries(sources).sort()) {
      console.log(`  ${k}: ${v}`);
    }
    console.log(`  Total: ${defs.length}`);
    return;
  }

  // ── go-gate: all checks ───────────────────────────────────────────────────
  if (args.includes("--go-gate")) {
    const gates = [];
    let allPass = true;

    // Count gates
    const counts = countByKind(defs);
    const sources = countBySource(defs);
    const ifaceCount = counts.interface || 0;
    const w3cCount = defs.filter(d => d.kind === "interface" && (d.source === "w3c" || !d.source)).length;
    const chromeCount = defs.filter(d => d.kind === "interface" && (d.source === "chromium" || d.source === "manual")).length;

    gates.push({ name: "interfaces >= 700", pass: ifaceCount >= 700, value: ifaceCount });
    gates.push({ name: "W3C interfaces >= 550", pass: w3cCount >= 550, value: w3cCount });
    gates.push({ name: "Chrome interfaces >= 150", pass: chromeCount >= 150, value: chromeCount });
    gates.push({ name: "dictionaries >= 100", pass: (counts.dictionary || 0) >= 100, value: counts.dictionary || 0 });
    gates.push({ name: "enums >= 50", pass: (counts.enum || 0) >= 50, value: counts.enum || 0 });
    gates.push({ name: "typedefs >= 30", pass: (counts.typedef || 0) >= 30, value: counts.typedef || 0 });
    gates.push({ name: "callbacks >= 20", pass: (counts.callback || 0) >= 20, value: counts.callback || 0 });
    gates.push({ name: "namespaces >= 3", pass: (counts.namespace || 0) >= 3, value: counts.namespace || 0 });

    // Topology
    const topo = checkTopology(defs);
    gates.push({ name: "topology no cycles", pass: topo.pass, value: topo.sorted });

    // Schema
    const schema = checkSchema(defs);
    gates.push({ name: "schema valid", pass: schema.pass, value: schema.errors.length });

    // Type coverage
    const covFile = path.join(__dirname, "tmp", "type-coverage.json");
    let coverage = 0;
    if (fs.existsSync(covFile)) {
      coverage = JSON.parse(fs.readFileSync(covFile, "utf8")).coverage;
    }
    gates.push({ name: "type coverage >= 90%", pass: coverage >= 90, value: coverage });

    // Inheritance
    const inheritance = checkInheritance(defs);
    gates.push({ name: "inheritance targets exist", pass: inheritance.pass, value: inheritance.dangling.length });

    // Print results
    console.log("=== Go Gate Results ===");
    for (const g of gates) {
      const mark = g.pass ? "PASS" : "FAIL";
      console.log(`  ${mark} ${g.name}: ${g.value}`);
      if (!g.pass) allPass = false;
    }

    if (allPass) {
      console.log("\n  OVERALL: PASS");
      process.exit(0);
    } else {
      console.log("\n  OVERALL: FAIL");
      process.exit(1);
    }
  }
}

main();
