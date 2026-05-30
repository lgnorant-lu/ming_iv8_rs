import pathlib, sys

try:
    import yaml
except ImportError:
    print("pyyaml not installed, run: uv run --with pyyaml python scripts/check_yaml.py")
    sys.exit(1)

for f in pathlib.Path(r'd:\dogepy\Tools\IV8\.github\workflows').glob('*.yml'):
    try:
        with open(f, encoding='utf-8') as fh:
            yaml.safe_load(fh)
        print(f'OK: {f.name}')
    except Exception as e:
        print(f'ERROR {f.name}: {e}')
