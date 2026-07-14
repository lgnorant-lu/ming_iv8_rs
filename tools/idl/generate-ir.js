#!/usr/bin/env node
"use strict";

/**
 * generate-ir.js — Main pipeline orchestrator for v0.8.18 IDL toolchain.
 *
 * Runs: fetch → normalize → merge → type-map → validate → output
 *
 * Usage: node generate-ir.js [--source w3c|chrome]
 * Output: output/unified_ir.json
 */

const { execSync } = require("child_process");
const path = require("path");
const fs = require("fs");

function run(cmd, label) {
  console.error(`\n[generate-ir] ${label}...`);
  console.error(`  $ ${cmd}`);
  try {
    const output = execSync(cmd, {
      cwd: __dirname,
      encoding: "utf8",
      stdio: ["pipe", "pipe", "pipe"],
    });
    if (output.stdout) console.error(output.stdout);
    // Parse JSON result from stdout
    try {
      return JSON.parse(output.trim().split("\n").pop());
    } catch {
      return output.trim();
    }
  } catch (err) {
    console.error(`  ERROR: ${err.message}`);
    if (err.stdout) console.error(err.stdout.toString());
    if (err.stderr) console.error(err.stderr.toString());
    process.exit(1);
  }
}

function main() {
  const args = process.argv.slice(2);
  const source = args.includes("--source") ? args[args.indexOf("--source") + 1] : "all";

  console.error("=".repeat(60));
  console.error("IV8 IDL Toolchain — generate-ir.js");
  console.error("=".repeat(60));

  // Step 1: Fetch
  const fetchResult = run("node fetch-webref.js", "Step 1: Fetch W3C Webref IDL");
  console.error(`  => ${fetchResult.specs} specs, ${fetchResult.definitions} definitions`);

  // Step 2: Normalize W3C
  const normResult = run("node normalize-ast.js", "Step 2: Normalize W3C AST");
  console.error(`  => ${normResult.interface} interfaces, ${normResult.total_members} members`);

  // Step 3: Merge W3C + Chrome manual
  const mergeFiles = [
    path.join(__dirname, "tmp", "normalized-w3c.json"),
  ];
  // Include chrome-manual IR if it exists
  const chromeManualDir = path.join(__dirname, "chrome-manual");
  if (fs.existsSync(chromeManualDir)) {
    const irFiles = fs.readdirSync(chromeManualDir).filter(f => f.endsWith(".ir.json"));
    for (const f of irFiles) {
      mergeFiles.push(path.join(chromeManualDir, f));
    }
    console.error(`  Found ${irFiles.length} chrome-manual IR files`);
  }
  const mergeResult = run(
    `node merge-tool.js --input ${mergeFiles.map(f => `"${f}"`).join(" ")}`,
    "Step 3: Merge W3C + Chrome manual"
  );
  console.error(`  => ${mergeResult.definitions} merged definitions, ${mergeResult.cycles} cycles`);

  // Step 4: Type map
  const typeResult = run("node type-mapper.js", "Step 4: Type mapping");
  console.error(`  => ${typeResult.coverage}% coverage, ${typeResult.unknown_count} unknown types`);

  // Step 5: Build unified_ir.json
  console.error("\n[generate-ir] Step 5: Building unified_ir.json...");

  const mergedData = JSON.parse(
    fs.readFileSync(path.join(__dirname, "tmp", "merged-w3c.json"), "utf8")
  );

  const unified = {
    schema_version: "unified-ir.v0.1",
    metadata: {
      generated_at: new Date().toISOString(),
      webref_version: require("@webref/idl/package.json").version,
      webidl2_version: require("webidl2/package.json").version,
      total_interfaces: mergedData.definitions.filter(d => d.kind === "interface").length,
      total_dictionaries: mergedData.definitions.filter(d => d.kind === "dictionary").length,
      total_enums: mergedData.definitions.filter(d => d.kind === "enum").length,
      total_typedefs: mergedData.definitions.filter(d => d.kind === "typedef").length,
      total_callbacks: mergedData.definitions.filter(d => d.kind === "callback" || d.kind === "callback_interface").length,
      total_namespaces: mergedData.definitions.filter(d => d.kind === "namespace").length,
      type_coverage: {
        covered: typeResult.coverage,
        unknown_types: typeResult.unknown_types || [],
      },
    },
    definitions: mergedData.definitions,
  };

  const outDir = path.join(__dirname, "output");
  fs.mkdirSync(outDir, { recursive: true });
  const outPath = path.join(outDir, "unified_ir.json");
  fs.writeFileSync(outPath, JSON.stringify(unified, null, 2), "utf8");

  const sizeKB = (fs.statSync(outPath).size / 1024).toFixed(1);
  console.error(`  => ${outPath} (${sizeKB} KB)`);

  // Step 6: Validate
  console.error("\n[generate-ir] Step 6: Running Go Gate validation...");
  try {
    execSync("node validate.js --go-gate", {
      cwd: __dirname,
      stdio: "inherit",
    });
  } catch {
    console.error("\n[generate-ir] Go Gate FAILED. See above for details.");
    process.exit(1);
  }

  console.error("\n[generate-ir] Done.");
}

main();
