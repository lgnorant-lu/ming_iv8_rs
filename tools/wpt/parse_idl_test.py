"""Parse idl_test() call parameters from WPT test files.

Extracts srcs, deps, and add_objects calls from WPT official test
files (idlharness.https.html, idlharness.window.js) without executing
the full script (which may contain DOM-dependent setup code that
crashes in IV8).
"""
from __future__ import annotations

import re
from pathlib import Path


def parse_idl_test_params(test_file: Path) -> dict:
    """Parse idl_test() call from a WPT test file.

    Returns dict with:
        srcs: list of spec names
        deps: list of dependency spec names
        objects: dict of {interface_name: [eval_string, ...]}
        prevent_multiple: list of interface names
        untested_idls: list of IDL strings
    """
    if test_file.suffix == ".html":
        html = test_file.read_text(encoding="utf-8")
        scripts = re.findall(
            r'<script(?![^>]*\bsrc=)[^>]*>(.*?)</script>',
            html, re.DOTALL,
        )
        code = "\n".join(scripts)
    else:
        code = test_file.read_text(encoding="utf-8")

    result = {
        "srcs": [],
        "deps": [],
        "objects": {},
        "prevent_multiple": [],
        "untested_idls": [],
    }

    # Extract srcs (first array argument)
    # Pattern: idl_test(\n  ['html'],\n  ['wai-aria', 'SVG', ...],
    src_match = re.search(
        r"idl_test\s*\(\s*\[([^\]]+)\]",
        code,
    )
    if src_match:
        srcs_str = src_match.group(1)
        result["srcs"] = re.findall(r"'([^']+)'", srcs_str)

    # Extract deps (second array argument)
    # Pattern: ,\n  ['wai-aria', 'SVG', ...],
    deps_match = re.search(
        r"idl_test\s*\(\s*\[[^\]]+\]\s*,\s*\[([^\]]+)\]",
        code,
    )
    if deps_match:
        deps_str = deps_match.group(1)
        result["deps"] = re.findall(r"'([^']+)'", deps_str)

    # Extract add_objects calls — match across multiple lines
    # The object literal can span many lines with nested brackets
    objects_match = re.search(
        r"add_objects\s*\(\s*\{(.+?)\}\s*\)",
        code,
        re.DOTALL,
    )
    if objects_match:
        objects_str = objects_match.group(1)
        # Match: InterfaceName: ['expr', ...] or InterfaceName: []
        # Need to handle multi-line and nested quotes
        for m in re.finditer(
            r"(\w+)\s*:\s*\[([^\]]*)\]",
            objects_str,
            re.DOTALL,
        ):
            iface = m.group(1)
            exprs_str = m.group(2)
            exprs = re.findall(r"'([^']*)'", exprs_str)
            result["objects"][iface] = exprs

    # Extract prevent_multiple_testing calls
    for m in re.finditer(
        r"prevent_multiple_testing\s*\(\s*'([^']+)'\s*\)",
        code,
    ):
        result["prevent_multiple"].append(m.group(1))

    # Extract add_untested_idls calls
    for m in re.finditer(
        r"add_untested_idls\s*\(\s*'([^']+)'\s*\)",
        code,
    ):
        result["untested_idls"].append(m.group(1))

    return result


def build_idl_test_code(params: dict) -> str:
    """Build idl_test() JS code from parsed parameters.

    This replaces the original test file's script, avoiding DOM-dependent
    setup code that crashes in IV8.
    """
    srcs = ", ".join(f"'{s}'" for s in params["srcs"])
    deps = ", ".join(f"'{d}'" for d in params["deps"])

    # Build add_objects
    obj_lines = []
    for iface, exprs in params["objects"].items():
        if exprs:
            expr_str = ", ".join(f"'{e}'" for e in exprs)
            obj_lines.append(f"      {iface}: [{expr_str}],")
        else:
            obj_lines.append(f"      {iface}: [],")
    objects_code = "\n".join(obj_lines)

    # Build untested_idls
    untested_lines = []
    for idl in params["untested_idls"]:
        escaped = idl.replace("\\", "\\\\").replace("'", "\\'")
        untested_lines.append(f"  idlArray.add_untested_idls('{escaped}');")

    # Build prevent_multiple
    prevent_lines = []
    for iface in params["prevent_multiple"]:
        prevent_lines.append(f"  idlArray.prevent_multiple_testing('{iface}');")

    code = f"""
idl_test(
  [{srcs}],
  [{deps}],
  async function(idlArray) {{
{chr(10).join(untested_lines)}
    idlArray.add_objects({{
{objects_code}
    }});
{chr(10).join(prevent_lines)}
  }}
);
"""
    return code


def main():
    """Test parser on all WPT test files."""
    import json

    fixtures = Path("tools/wpt/fixtures")
    test_files = [
        fixtures / "html" / "dom" / "idlharness.https.html",
        fixtures / "dom" / "idlharness.window.js",
        fixtures / "css" / "cssom-view" / "idlharness.html",
    ]

    for tf in test_files:
        if not tf.exists():
            continue
        params = parse_idl_test_params(tf)
        print(f"=== {tf.name} ===")
        print(f"  srcs: {params['srcs']}")
        print(f"  deps: {params['deps']}")
        print(f"  objects: {len(params['objects'])} interfaces")
        for iface, exprs in list(params["objects"].items())[:5]:
            print(f"    {iface}: {exprs}")
        if len(params["objects"]) > 5:
            print(f"    ... ({len(params['objects'])} total)")
        print(f"  prevent_multiple: {params['prevent_multiple']}")
        print(f"  untested_idls: {len(params['untested_idls'])}")
        print()


if __name__ == "__main__":
    main()
