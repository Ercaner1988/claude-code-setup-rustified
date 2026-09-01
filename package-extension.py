#!/usr/bin/env python3
"""Claude Desktop eklenti paketi (.mcpb) uretir.

Kullanim: python package-extension.py <binary-yolu> [cikti.mcpb]
.mcpb = manifest.json'u kokunde tutan bir ZIP arsivi.
"""
import json
import pathlib
import sys
import zipfile

ROOT = pathlib.Path(__file__).parent
EXTRAS = ["manifest.json", "icon.png", "README.md", "LICENSE"]


def main() -> int:
    if len(sys.argv) < 2:
        print(__doc__)
        return 2

    binary = pathlib.Path(sys.argv[1])
    if not binary.is_file():
        print(f"HATA: binary bulunamadi: {binary}")
        return 1

    manifest = json.loads((ROOT / "manifest.json").read_text(encoding="utf-8"))
    entry = manifest["server"]["entry_point"]
    out = pathlib.Path(sys.argv[2]) if len(sys.argv) > 2 else ROOT / f"{manifest['name']}-{manifest['version']}.mcpb"

    with zipfile.ZipFile(out, "w", zipfile.ZIP_DEFLATED) as z:
        for name in EXTRAS:
            path = ROOT / name
            if path.is_file():
                z.write(path, name)
            else:
                print(f"UYARI: atlandi (yok): {name}")
        z.write(binary, entry)

    print(f"OK: {out}  ({out.stat().st_size / 1_048_576:.1f} MB, entry_point={entry})")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
