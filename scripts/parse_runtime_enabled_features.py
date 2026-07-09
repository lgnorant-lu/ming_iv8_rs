import json
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent
IR_PATH = REPO_ROOT / "data" / "chromium-148.ir.json"
OUTPUT_PATH = REPO_ROOT / "data" / "runtime_enabled_features.json"

data = json.loads(IR_PATH.read_text(encoding="utf-8"))
definitions = data["definitions"]

runtime_enabled_interfaces = []
runtime_flagged_interfaces = []
context_enabled_interfaces = []

for iface in definitions:
    ext_attrs = iface.get("ext_attrs") or []
    name = iface.get("name", "")
    if "RuntimeEnabled" in ext_attrs:
        runtime_enabled_interfaces.append(name)
    if "RuntimeFlag" in ext_attrs:
        runtime_flagged_interfaces.append(name)
    if "ContextEnabled" in ext_attrs:
        context_enabled_interfaces.append(name)

result = {
    "source": "chromium-148.ir.json",
    "schema_version": data.get("schema_version", "unknown"),
    "note": "IR format uses boolean RuntimeEnabled/RuntimeFlag ext_attrs, not named features",
    "runtime_enabled_count": len(runtime_enabled_interfaces),
    "runtime_flagged_count": len(runtime_flagged_interfaces),
    "context_enabled_count": len(context_enabled_interfaces),
    "runtime_enabled_interfaces": sorted(runtime_enabled_interfaces),
    "runtime_flagged_interfaces": sorted(runtime_flagged_interfaces),
    "context_enabled_interfaces": sorted(context_enabled_interfaces),
}

OUTPUT_PATH.write_text(json.dumps(result, indent=2), encoding="utf-8")
print(f"RuntimeEnabled interfaces: {len(runtime_enabled_interfaces)}")
print(f"RuntimeFlag interfaces: {len(runtime_flagged_interfaces)}")
print(f"ContextEnabled interfaces: {len(context_enabled_interfaces)}")
print(f"Output: {OUTPUT_PATH}")
