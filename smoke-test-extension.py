#!/usr/bin/env python3
"""Paketlenmis .mcpb eklentisini gercekten calistirip dogrular.

Kullanim: python smoke-test-extension.py <paket.mcpb>

Claude Desktop'in bir eklentiyi kurarken yaptigini taklit eder:

1. Paketi isletim sisteminin KENDI acicisiyla acar. macOS/Linux'ta bu
   `unzip`tir; ikilinin calistirma izni ancak boyle sinanir (Python'un
   zipfile modulu Unix izinlerini uygulamaz, sinanmasi gereken seyi atlar).
2. manifest.json'daki komutu ${__dirname} cozerek calistirir.
3. MCP el sikismasini yapar, araclari listeler, salt-okunur bir araci
   gercekten cagirir.

Sinirlari: Claude Desktop'in kendi aciciyi degil isletim sisteminin
`unzip`ini kullanir; macOS Gatekeeper davranisini sinamaz (CI koşucusunda
indirilen dosyaya karantina isareti konmaz).
"""
import json
import os
import pathlib
import shutil
import subprocess
import sys
import tempfile
import zipfile

READ_ONLY_TOOL = "status"


def fail(message: str) -> None:
    print(f"BASARISIZ: {message}")
    raise SystemExit(1)


def extract(package: pathlib.Path, dest: pathlib.Path) -> None:
    if os.name == "nt":
        zipfile.ZipFile(package).extractall(dest)
        return
    if shutil.which("unzip") is None:
        fail("`unzip` bulunamadi; calistirma izni sinanamaz, zipfile'a dusmuyoruz")
    subprocess.run(["unzip", "-q", str(package), "-d", str(dest)], check=True)


def rpc(request_id: int, method: str, params: dict | None = None) -> str:
    message = {"jsonrpc": "2.0", "id": request_id, "method": method}
    if params is not None:
        message["params"] = params
    return json.dumps(message)


def main() -> int:
    if len(sys.argv) != 2:
        print(__doc__)
        return 2

    package = pathlib.Path(sys.argv[1]).resolve()
    if not package.is_file():
        fail(f"paket bulunamadi: {package}")

    with tempfile.TemporaryDirectory() as tmp:
        root = pathlib.Path(tmp)
        extract(package, root)

        manifest = json.loads((root / "manifest.json").read_text(encoding="utf-8"))
        config = manifest["server"]["mcp_config"]
        command = pathlib.Path(config["command"].replace("${__dirname}", str(root)))
        args = config.get("args", [])

        if not command.is_file():
            fail(f"manifest'in gosterdigi ikili pakette yok: {command.name}")
        if os.name != "nt" and not os.access(command, os.X_OK):
            fail(f"{command.name} calistirilabilir degil (mod {oct(command.stat().st_mode & 0o777)})")

        requests = "\n".join([
            rpc(1, "initialize", {
                "protocolVersion": "2024-11-05",
                "capabilities": {},
                "clientInfo": {"name": "smoke-test", "version": "1"},
            }),
            rpc(2, "tools/list"),
            rpc(3, "tools/call", {"name": READ_ONLY_TOOL, "arguments": {}}),
        ]) + "\n"

        try:
            proc = subprocess.run(
                [str(command), *args], input=requests, capture_output=True,
                text=True, encoding="utf-8", errors="replace", timeout=120,
            )
        except OSError as err:
            # Burasi, ikili bu mimaride hic calistirilamiyorsa dusulen yer
            # (ornegin Rosetta'siz bir ARM Mac'te x86_64 ikili).
            fail(f"ikili baslatilamadi: {err}")

        responses = {}
        for line in proc.stdout.splitlines():
            line = line.strip()
            if line.startswith("{"):
                message = json.loads(line)
                responses[message.get("id")] = message

        for request_id in (1, 2, 3):
            if request_id not in responses:
                fail(f"istek {request_id} yanitsiz kaldi. stderr:\n{proc.stderr[-2000:]}")
            if "error" in responses[request_id]:
                fail(f"istek {request_id} hata dondu: {responses[request_id]['error']}")

        server = responses[1]["result"]["serverInfo"]
        if server["name"] != manifest["name"]:
            fail(f"sunucu adi {server['name']!r}, manifest {manifest['name']!r}")
        if server["version"] != manifest["version"]:
            fail(f"sunucu surumu {server['version']!r}, manifest {manifest['version']!r} "
                 "(Cargo.toml ile manifest.json kaymis)")

        served = sorted(tool["name"] for tool in responses[2]["result"]["tools"])
        declared = sorted(tool["name"] for tool in manifest.get("tools", []))
        if served != declared:
            fail(f"manifest'in ilan ettigi araclar sunulanlarla ayni degil\n"
                 f"  ilan edilen: {declared}\n  sunulan:     {served}")

        call = responses[3]["result"]
        text = " ".join(part.get("text", "") for part in call.get("content", []))
        if call.get("isError") or not text.strip():
            fail(f"{READ_ONLY_TOOL} araci basarisiz: {text[:500]}")

    print(f"GECTI: {package.name}")
    print(f"  platform : {manifest['compatibility']['platforms']}")
    print(f"  sunucu   : {server['name']} {server['version']}")
    print(f"  araclar  : {len(served)} -> {', '.join(served)}")
    print(f"  {READ_ONLY_TOOL:9}: {text.strip().splitlines()[0]}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
