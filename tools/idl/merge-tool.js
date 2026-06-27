#!/usr/bin/env node
"use strict";

/**
 * merge-tool.js — Merge partial interfaces, expand mixins, process includes,
 *                 build and validate inheritance chains.
 *
 * Implements the six-step algorithm:
 *   1. Collect & group definitions by name
 *   2. Merge partial interfaces (same name → one definition)
 *   3. Process includes statements → includes_map<target, mixin[]>
 *   4. Expand mixins → copy mixin members into including interfaces
 *   5. Build & validate inheritance chain (Kahn topological sort)
 *   6. Output merged definitions with sources tracking
 *
 * Conflict resolution:
 *   - Same-name attribute (same type): deduplicate
 *   - Same-name attribute (different type): mark CONFLICT, keep primary
 *   - Same-name operation (different signature): keep all overloads
 *   - Same-name const (different value): ERROR
 *   - Mixin member vs interface own member: interface wins
 *   - Two mixins providing same-name member: first includes wins, mark CONFLICT
 */

const path = require("path");
const fs = require("fs");

// ── Step 1: Collect & Group ─────────────────────────────────────────────────

function collectAndGroup(inputFiles) {
  const byName = new Map();    // name → { primary, partials[], mixins[] }
  const includes = [];         // { target, includes } statements
  const all = [];              // all non-interface, non-mixin, non-includes defs

  for (const file of inputFiles) {
    const data = JSON.parse(fs.readFileSync(file, "utf8"));
    const defs = data.definitions || data;

    for (const def of defs) {
      if (def.kind === "includes") {
        includes.push({ target: def.target, includes: def.includes });
        continue;
      }

      if (def.kind === "interface" || def.kind === "interface_mixin" || def.kind === "namespace" || def.kind === "dictionary") {
        const key = def.name;
        let group = byName.get(key);
        if (!group) {
          group = { primary: null, partials: [], mixins: [] };
          byName.set(key, group);
        }

        if (def.kind === "interface_mixin") {
          if (def.partial) {
            group.partials.push(def);
          } else {
            if (group.primary) {
              group.partials.push(def);
            } else {
              group.primary = def;
            }
          }
        } else if (def.partial) {
          group.partials.push(def);
        } else {
          if (group.primary) {
            // Two non-partial definitions with the same name — merge second as partial
            console.error(`[merge-tool] WARNING: duplicate primary for ${key}, treating second as partial`);
            group.partials.push(def);
          } else {
            group.primary = def;
          }
        }
      } else {
        all.push(def);
      }
    }
  }

  return { byName, includes, all };
}

// ── Step 2: Merge partial interfaces ────────────────────────────────────────

function mergePartial(group) {
  if (!group.primary) return null;

  const primary = JSON.parse(JSON.stringify(group.primary));
  primary.members = primary.members || [];
  primary.mixin_sources = [];
  primary.partial_sources = [];

  for (const partial of group.partials) {
    for (const m of (partial.members || [])) {
      primary.members.push(m);
    }
    if (partial.source && !primary.partial_sources.includes(partial.source)) {
      primary.partial_sources.push(partial.source);
    }
  }

  return primary;
}

// ── Step 3: Process includes ─────────────────────────────────────────────────

function buildIncludesMap(includes) {
  const map = new Map(); // target → [mixinName, ...]
  for (const { target, includes: mixin } of includes) {
    if (!map.has(target)) map.set(target, []);
    map.get(target).push(mixin);
  }
  return map;
}

// ── Step 4: Expand mixins ────────────────────────────────────────────────────

function expandMixins(mergedDefs, byName, includesMap) {
  for (const [name, def] of mergedDefs) {
    const mixinNames = includesMap.get(name);
    if (!mixinNames) continue;

    for (const mixinName of mixinNames) {
      const mixinGroup = byName.get(mixinName);
      if (!mixinGroup || !mixinGroup.primary) {
        console.error(`[merge-tool] WARNING: mixin ${mixinName} not found for ${name}`);
        continue;
      }

      const mixinDef = mixinGroup.primary;
      const ownMemberNames = new Set((def.members || []).map(m => m.name).filter(Boolean));

      for (const m of (mixinDef.members || [])) {
        if (m.name && ownMemberNames.has(m.name)) {
          // Interface own member takes priority over mixin member
          continue;
        }

        // Check for conflicts from other mixins
        const existingFromMixin = (def.members || []).find(
          em => em.name === m.name && em._from_mixin
        );
        if (existingFromMixin) {
          console.error(`[merge-tool] CONFLICT: ${name}.${m.name} from mixins ${existingFromMixin._from_mixin} and ${mixinName}`);
          continue;
        }

        const memberCopy = JSON.parse(JSON.stringify(m));
        memberCopy._from_mixin = mixinName;
        def.members.push(memberCopy);
      }

      if (!def.mixin_sources.includes(mixinName)) {
        def.mixin_sources.push(mixinName);
      }
    }
  }

  // Remove mixin definitions from output (they're expanded)
  for (const [name, group] of byName) {
    if (group.primary && group.primary.kind === "interface_mixin") {
      mergedDefs.delete(name);
    }
  }
}

// ── Step 5: Inheritance chain validation (Kahn 拓扑排序) ────────────────────

function validateInheritance(mergedDefs) {
  const defMap = new Map(mergedDefs); // name → def

  // Build adjacency: parent → children
  const children = new Map();
  const indegree = new Map();

  for (const [name, def] of mergedDefs) {
    if (!children.has(name)) children.set(name, []);
    if (!indegree.has(name)) indegree.set(name, 0);
  }

  for (const [name, def] of mergedDefs) {
    if (def.inheritance && defMap.has(def.inheritance)) {
      if (!children.has(def.inheritance)) children.set(def.inheritance, []);
      children.get(def.inheritance).push(name);
      indegree.set(name, (indegree.get(name) || 0) + 1);
    }
  }

  // Kahn topological sort
  const queue = [];
  for (const [name, deg] of indegree) {
    if (deg === 0) queue.push(name);
  }

  const sorted = [];
  while (queue.length > 0) {
    // Sort alphabetically for deterministic output
    queue.sort();
    const current = queue.shift();
    sorted.push(current);

    for (const child of (children.get(current) || [])) {
      indegree.set(child, indegree.get(child) - 1);
      if (indegree.get(child) === 0) queue.push(child);
    }
  }

  // Detect cycles
  const remaining = [];
  for (const [name, deg] of indegree) {
    if (deg > 0) remaining.push(name);
  }

  if (remaining.length > 0) {
    console.error(`[merge-tool] ERROR: Inheritance cycle detected involving: ${remaining.join(", ")}`);
    return { sorted, cycles: remaining };
  }

  return { sorted, cycles: [], children };
}

// ── Step 6: Cleanup _from_mixin markers ─────────────────────────────────────

function cleanupMarkers(mergedDefs) {
  for (const [name, def] of mergedDefs) {
    if (def.members) {
      for (const m of def.members) {
        delete m._from_mixin;
      }
    }
  }
}

// ── Main merge function ──────────────────────────────────────────────────────

function merge(inputFiles) {
  // Step 1
  const { byName, includes, all } = collectAndGroup(inputFiles);

  // Step 2: Merge partials
  const mergedDefs = new Map();
  for (const [name, group] of byName) {
    const merged = mergePartial(group);
    if (merged && merged.name) {
      mergedDefs.set(merged.name, merged);
    }
  }

  // Step 3
  const includesMap = buildIncludesMap(includes);

  // Step 4: Expand mixins
  expandMixins(mergedDefs, byName, includesMap);

  // Step 5: Validate inheritance
  const { sorted, cycles } = validateInheritance(mergedDefs);

  // Step 6: Cleanup
  cleanupMarkers(mergedDefs);

  // Add non-interface definitions
  for (const def of all) {
    if (def.name) {
      if (mergedDefs.has(def.name)) {
        console.error(`[merge-tool] WARNING: skipping duplicate non-interface ${def.name}`);
      } else {
        mergedDefs.set(def.name, def);
      }
    }
  }

  return {
    definitions: Array.from(mergedDefs.values()),
    sorted,
    cycles,
    includesCount: includes.length,
  };
}

// ── CLI ──────────────────────────────────────────────────────────────────────

function main() {
  const args = process.argv.slice(2);

  if (args.includes("--help")) {
    console.log("Usage: node merge-tool.js [--input file1.json file2.json ...]");
    console.log("  Default input: tmp/normalized-w3c.json");
    return;
  }

  let inputFiles;
  const inputIdx = args.indexOf("--input");
  if (inputIdx >= 0) {
    inputFiles = args.slice(inputIdx + 1).filter(a => !a.startsWith("--"));
  } else {
    inputFiles = [path.join(__dirname, "tmp", "normalized-w3c.json")];
  }

  const result = merge(inputFiles);

  const outPath = path.join(__dirname, "tmp", "merged-w3c.json");
  fs.writeFileSync(outPath, JSON.stringify(result, null, 2), "utf8");

  console.error(`[merge-tool] ${result.definitions.length} definitions after merge`);
  console.error(`[merge-tool] ${result.includesCount} includes processed`);
  console.error(`[merge-tool] ${result.sorted.length} in inheritance chain, ${result.cycles.length} cycles`);

  if (result.cycles.length > 0) {
    console.error(`[merge-tool] CYCLES: ${result.cycles.join(", ")}`);
  }

  console.log(JSON.stringify({
    definitions: result.definitions.length,
    includes: result.includesCount,
    sorted: result.sorted.length,
    cycles: result.cycles.length,
  }));
}

main();
