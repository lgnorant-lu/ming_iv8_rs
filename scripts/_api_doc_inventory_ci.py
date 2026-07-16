import re
import pathlib

root = pathlib.Path(__file__).resolve().parents[1]
init = (root / "python" / "iv8_rs" / "__init__.py").read_text(encoding="utf-8")
names = re.findall(r'"([^"]+)"', re.search(r'__all__\s*=\s*\[(.*?)\]', init, re.S).group(1))

api_text = "\n".join(p.read_text(encoding="utf-8") for p in (root / "docs/api").rglob("*.md"))
missing = sorted(n for n in names if n not in api_text)
if missing:
    print(f"MISSING from docs/api: {missing}")
    raise SystemExit(1)
print(f"D1a OK: {len(names)} exports, {len(names) - len(missing)} found")
