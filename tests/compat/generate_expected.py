"""Generate expected outputs by running fixtures against iv8 0.1.2.

Usage:
    .venv-probe/Scripts/python.exe tests/compat/generate_expected.py

Reads all *.js files in tests/compat/fixtures/**/ and runs them through iv8,
saving results to *.expected.json alongside each .js file.
"""
import json
import pathlib
import sys

FIXTURES_DIR = pathlib.Path(__file__).parent / "fixtures"


def normalize_value(val):
    """Convert iv8 result to JSON-serializable form matching iv8-rs output."""
    if val is None:
        return None
    if isinstance(val, (bool, int, float, str)):
        return val
    if isinstance(val, bytes):
        return list(val)  # bytes → list of ints for JSON
    if isinstance(val, list):
        return [normalize_value(v) for v in val]
    if isinstance(val, dict):
        return {k: normalize_value(v) for k, v in val.items()}
    # JSObject wrappers — convert to string representation
    return str(val)


def main():
    try:
        import iv8
    except ImportError:
        print("ERROR: iv8 not installed. Run from .venv-probe with iv8 0.1.2 installed.")
        sys.exit(1)

    js_files = sorted(FIXTURES_DIR.rglob("*.js"))
    print(f"Found {len(js_files)} fixture(s)")

    for js_file in js_files:
        expected_file = js_file.with_suffix(".expected.json")
        src = js_file.read_text(encoding="utf-8")

        ctx = iv8.JSContext()
        try:
            result = ctx.eval(src, to_py=False)
            # Normalize: iv8 to_py=False returns Python native for scalars,
            # bytes for TypedArray, and JSObject wrappers for complex types.
            # We need to serialize the result to JSON-compatible form.
            output = {
                "ok": True,
                "value": normalize_value(result),
                "type": type(result).__name__,
            }
        except Exception as e:
            output = {
                "ok": False,
                "error": str(e),
                "error_type": type(e).__name__,
            }

        expected_file.write_text(
            json.dumps(output, ensure_ascii=False, default=str),
            encoding="utf-8",
        )
        status = "OK" if output["ok"] else f"ERR({output['error_type']})"
        print(f"  {js_file.relative_to(FIXTURES_DIR)} -> {status}")

    print(f"\nGenerated {len(js_files)} expected files.")


if __name__ == "__main__":
    main()
