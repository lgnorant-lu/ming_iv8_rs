#!/usr/bin/env node
"use strict";

/**
 * type-mapper.js — Map IDL types to IR types and compute coverage statistics.
 *
 * Covers:
 *   - Primitive types: boolean, long, unsigned long, short, unsigned short,
 *     byte, octet, float, double, unrestricted float, unrestricted double,
 *     DOMString, ByteString, USVString, undefined, any, object, symbol, bigint
 *   - BufferSource types: ArrayBuffer, ArrayBufferView, DataView,
 *     Int8Array, Uint8Array, Uint8ClampedArray, Int16Array, Uint16Array,
 *     Int32Array, Uint32Array, Float32Array, Float64Array, BigInt64Array, BigUint64Array
 *   - Generic types: Promise<T>, sequence<T>, FrozenArray<T>,
 *     ObservableArray<T>, record<K,V>
 *   - Union types: (A or B)
 *   - Nullable types: T?
 *
 * Unknown types are marked as "unknown_type" rather than dropped.
 */

const path = require("path");
const fs = require("fs");

// ── Known type mappings ─────────────────────────────────────────────────────

const PRIMITIVE_TYPES = new Set([
  "undefined", "any", "boolean", "byte", "octet",
  "short", "unsigned short", "long", "unsigned long",
  "long long", "unsigned long long",
  "float", "unrestricted float",
  "double", "unrestricted double",
  "DOMString", "ByteString", "USVString",
  "object", "symbol", "bigint", "void",
]);

const BUFFERSOURCE_TYPES = new Set([
  "ArrayBuffer", "ArrayBufferView", "DataView",
  "Int8Array", "Uint8Array", "Uint8ClampedArray",
  "Int16Array", "Uint16Array", "Int32Array", "Uint32Array",
  "Float32Array", "Float64Array",
  "BigInt64Array", "BigUint64Array",
]);

function isPrimitiveOrBuffer(name) {
  return PRIMITIVE_TYPES.has(name) || BUFFERSOURCE_TYPES.has(name);
}

// ── Type extraction ─────────────────────────────────────────────────────────

function extractTypeNames(type, knownDefs, unknownSet) {
  if (!type) return [];
  if (typeof type === "string") {
    if (!isPrimitiveOrBuffer(type) && !knownDefs.has(type)) {
      unknownSet.add(type);
    }
    return [type];
  }

  const names = [];

  if (type.kind === "name") {
    if (!isPrimitiveOrBuffer(type.name) && !knownDefs.has(type.name)) {
      unknownSet.add(type.name);
    }
    names.push(type.name);
  }

  if (type.kind === "union" && type.types) {
    for (const t of type.types) {
      names.push(...extractTypeNames(t, knownDefs, unknownSet));
    }
  }

  if (type.kind === "generic" && type.inner) {
    names.push(...extractTypeNames(type.inner, knownDefs, unknownSet));
  }

  return names;
}

// ── Coverage computation ────────────────────────────────────────────────────

function computeCoverage(definitions) {
  const knownDefs = new Set(definitions.map(d => d.name).filter(Boolean));
  const unknownSet = new Set();
  let totalRefs = 0;
  let coveredRefs = 0;

  for (const def of definitions) {
    // Check inheritance
    if (def.inheritance) {
      totalRefs++;
      if (isPrimitiveOrBuffer(def.inheritance) || knownDefs.has(def.inheritance)) {
        coveredRefs++;
      } else {
        unknownSet.add(def.inheritance);
      }
    }

    // Check member types
    for (const member of (def.members || [])) {
      if (member.type) {
        const names = extractTypeNames(member.type, knownDefs, unknownSet);
        totalRefs += names.length;
        for (const name of names) {
          if (isPrimitiveOrBuffer(name) || knownDefs.has(name)) {
            coveredRefs++;
          }
        }
      }
      if (member.return_type) {
        const names = extractTypeNames(member.return_type, knownDefs, unknownSet);
        totalRefs += names.length;
        for (const name of names) {
          if (isPrimitiveOrBuffer(name) || knownDefs.has(name)) {
            coveredRefs++;
          }
        }
      }
      for (const arg of (member.arguments || [])) {
        if (arg.type) {
          const names = extractTypeNames(arg.type, knownDefs, unknownSet);
          totalRefs += names.length;
          for (const name of names) {
            if (isPrimitiveOrBuffer(name) || knownDefs.has(name)) {
              coveredRefs++;
            }
          }
        }
      }
    }

    // Check typedef/callback types
    if (def.idl_type) {
      const names = extractTypeNames(def.idl_type, knownDefs, unknownSet);
      totalRefs += names.length;
      for (const name of names) {
        if (isPrimitiveOrBuffer(name) || knownDefs.has(name)) {
          coveredRefs++;
        }
      }
    }
  }

  const coverage = totalRefs > 0 ? ((coveredRefs / totalRefs) * 100) : 100;

  return {
    total_refs: totalRefs,
    covered_refs: coveredRefs,
    coverage: parseFloat(coverage.toFixed(1)),
    unknown_types: Array.from(unknownSet).sort(),
    unknown_count: unknownSet.size,
  };
}

// ── CLI ──────────────────────────────────────────────────────────────────────

function main() {
  const args = process.argv.slice(2);
  let inputFile = null;

  for (let i = 0; i < args.length; i++) {
    if (args[i] === "--input" && i + 1 < args.length) {
      inputFile = args[++i];
    }
  }

  if (!inputFile) {
    inputFile = path.join(__dirname, "tmp", "merged-w3c.json");
  }

  const data = JSON.parse(fs.readFileSync(inputFile, "utf8"));
  const definitions = data.definitions || data;
  const result = computeCoverage(Array.isArray(definitions) ? definitions : []);

  const outPath = path.join(__dirname, "tmp", "type-coverage.json");
  fs.writeFileSync(outPath, JSON.stringify(result, null, 2), "utf8");

  console.error(`[type-mapper] Coverage: ${result.coverage}% (${result.covered_refs}/${result.total_refs})`);
  console.error(`[type-mapper] Unknown types: ${result.unknown_count}`);
  if (result.unknown_types.length > 0) {
    console.error(`  First 20: ${result.unknown_types.slice(0, 20).join(", ")}`);
  }

  console.log(JSON.stringify(result));
}

main();
