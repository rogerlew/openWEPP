#!/usr/bin/env python3
"""Freeze an isolated correction against the authenticated build016 sources."""
import argparse
import difflib
import hashlib
import json
from pathlib import Path
import shutil
import subprocess

PACKAGE = Path(__file__).resolve().parent
PREVIOUS = PACKAGE.parent / "20260910-stage3-snow-accuracy-runtime-001/artifacts"


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def freeze(args):
    expected = json.loads((PREVIOUS / "R0-source.json").read_text())["files"]
    expected.update({r["path"]: r["sha256"] for r in json.loads(
        (PREVIOUS / "PBC-final-build016-source.json").read_text())["changed_files"]})
    for name, sha in expected.items():
        assert digest(args.original / name) == sha, name
    files = sorted(set(expected) | {
        str(p.relative_to(args.working)) for p in args.working.glob("crates/**/*.rs")})
    changes, patch, identities = [], [], {}
    for name in files:
        before, after = args.original / name, args.working / name
        if not after.exists():
            raise ValueError(f"missing source: {name}")
        identities[name] = digest(after)
        if before.exists() and before.read_bytes() == after.read_bytes():
            continue
        before_bytes = before.read_bytes() if before.exists() else b""
        fromfile = f"a/{name}"
        # The retained composition is deliberately partial. Compare other Rust
        # files against the pinned base, and retain genuinely new files too.
        if name not in expected and not before.exists():
            base = subprocess.run(
                ["git", "show", f"e89befa4678eadec039b3e7f7fe0a176af8e9dc5:{name}"],
                cwd=PACKAGE.parents[2], capture_output=True, check=False)
            if base.returncode == 0:
                before_bytes = base.stdout
                if before_bytes == after.read_bytes():
                    continue
            else:
                fromfile = "/dev/null"
        changes.append({"path": name, "sha256": identities[name]})
        patch.extend(difflib.unified_diff(
            before_bytes.decode().splitlines(True),
            after.read_text().splitlines(True), fromfile=fromfile, tofile=f"b/{name}"))
    prefix = PACKAGE / "artifacts" / args.cut
    patch_path = prefix.with_suffix(".patch")
    assert not patch_path.exists(), "cuts are immutable"
    patch_path.write_text("".join(patch))
    artifacts = []
    for line in args.build_log.read_text().splitlines():
        if line.startswith("{"):
            row = json.loads(line)
            if row.get("reason") == "compiler-artifact" and row.get("executable") and row["profile"]["test"]:
                artifacts.append(row)
    binaries = []
    for name in (("openwepp_runner",) if args.runner_only else ("openwepp_runner", "openwepp_hillslope_orchestrator")):
        row = next(r for r in artifacts if r["target"]["name"] == name)
        destination = args.working.parent / f"{args.cut}-{name.removeprefix('openwepp_')}"
        assert not destination.exists()
        shutil.copy2(row["executable"], destination)
        binaries.append({"path": str(destination), "sha256": digest(destination), "cargo_artifact": row})
    record = dict(base="B01-original final016 retained composition", changes=changes,
                  files=identities, policy_id=args.policy_id, patch_sha256=digest(patch_path),
                  build_log=str(args.build_log), build_log_sha256=digest(args.build_log),
                  build_note="release LTO=false; orchestrator/runner opt-level1; fixture/restart features; correctness edit loop", binaries=binaries)
    prefix.with_name(prefix.name + "-source.json").write_text(json.dumps(record, indent=2) + "\n")
    print(json.dumps({"cut": args.cut, "changes": len(changes), "binaries": [{"path": r["path"], "sha256": r["sha256"]} for r in binaries]}))


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("cut")
    parser.add_argument("--original", type=Path, required=True)
    parser.add_argument("--working", type=Path, required=True)
    parser.add_argument("--build-log", type=Path, required=True)
    parser.add_argument("--policy-id", type=int, required=True)
    parser.add_argument("--runner-only", action="store_true", help="freeze only the explicitly built runner test executable")
    freeze(parser.parse_args())
