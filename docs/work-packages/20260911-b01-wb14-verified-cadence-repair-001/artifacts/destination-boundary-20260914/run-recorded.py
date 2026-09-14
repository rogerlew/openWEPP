"""Session-local command evidence capture; chooses no checks or acceptance."""
import datetime
import difflib
import hashlib
import json
from pathlib import Path
import subprocess
import sys

HERE = Path(__file__).resolve().parent
SOURCE = Path("/workdir/openwepp-experiments/b01-wb14-cadence/current-context-capture-20260913")
BASE = SOURCE.parent / "baseline-red"

def snapshot():
    paths = sorted({str(p.relative_to(root)) for root in (SOURCE, BASE) for p in (root / "crates").rglob("*") if p.is_file()} | {"Cargo.toml", "Cargo.lock", "flake.nix", "flake.lock", "rust-toolchain.toml"})
    identities = {}
    changes = []
    patch = []
    for rel in paths:
        path = SOURCE / rel
        if not path.exists() and not (BASE / rel).exists():
            continue
        data = path.read_bytes() if path.exists() else b""
        digest = hashlib.sha256(data).hexdigest() if path.exists() else None
        identities[rel] = digest
        before = (BASE / rel).read_bytes() if (BASE / rel).exists() else b""
        if data != before:
            changes.append({"path": rel, "baseline_sha256": hashlib.sha256(before).hexdigest() if (BASE / rel).exists() else None, "source_sha256": digest})
            patch.extend(difflib.unified_diff(before.decode().splitlines(True), data.decode().splitlines(True), fromfile="a/" + rel, tofile="b/" + rel))
    return identities, changes, "".join(patch)

name, *argv = sys.argv[1:]
if not argv:
    raise SystemExit("missing explicit command")
identities, changed, patch = snapshot()
(HERE / (name + ".source.patch")).write_text(patch)
record = {"argv": argv, "cwd": str(SOURCE), "source": str(SOURCE), "base": str(BASE), "source_tree_sha256": hashlib.sha256(json.dumps(identities, sort_keys=True).encode()).hexdigest(), "changed_files": changed, "input_identity": "input-continuity.json", "patch_sha256": hashlib.sha256(patch.encode()).hexdigest(), "start_utc": datetime.datetime.now(datetime.timezone.utc).isoformat()}
(HERE / (name + ".json")).write_text(json.dumps(record, indent=2) + "\n")
with (HERE / (name + ".stdout")).open("wb") as stdout, (HERE / (name + ".stderr")).open("wb") as stderr:
    result = subprocess.run(argv, cwd=SOURCE, stdout=stdout, stderr=stderr)
record["end_utc"] = datetime.datetime.now(datetime.timezone.utc).isoformat()
record["exit"] = result.returncode
record["source_unchanged_during_command"] = snapshot()[0] == identities
(HERE / (name + ".json")).write_text(json.dumps(record, indent=2) + "\n")
print(json.dumps({key: record[key] for key in ("argv", "start_utc", "end_utc", "exit", "source_unchanged_during_command")}))
