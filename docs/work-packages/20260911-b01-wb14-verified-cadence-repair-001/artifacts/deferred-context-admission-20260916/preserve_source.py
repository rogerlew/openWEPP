"""Preserve the named terminal S cut and verify reconstruction from retained R.

This is source custody, not validation selection or an acceptance verdict.
Ancillary files remain available through the retained R/Q/base custody recipe.
"""
import difflib
import hashlib
import importlib.util
import json
from pathlib import Path
import shutil
import subprocess

HERE = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location("recorded", HERE / "run_recorded.py")
recorded = importlib.util.module_from_spec(spec)
spec.loader.exec_module(recorded)
S = recorded.SOURCE
R = S.parent / "native-context-restoration-reader-20260916"
Q = S.parent / "snowfree-recorder-candidate-20260915"
DEST = Path("/home/roger/openwepp-experiments/b01-wb14-deferred-context-admission-20260916/terminal-reconstruction")


def tree(entries):
    return hashlib.sha256(json.dumps(entries, sort_keys=True).encode()).hexdigest()


def main():
    r, _, _ = recorded.RETAINED.snapshot(R)
    q, _, _ = recorded.RETAINED.snapshot(Q)
    assert tree(r) == "7c04f52e6cae6327202591b37f0c4e7a5fff3ae34a103255a35c49a3dbdf37ab"
    assert tree(q) == "ecc48d5235d8d53af496856459e3eaa29cca2dda2511697350a67bac172f9592"
    entries, changes, cumulative = recorded.RETAINED.snapshot(S)
    delta = []
    for name in sorted(set(r) | set(entries)):
        old = (R / name).read_text() if r.get(name) is not None else ""
        new = (S / name).read_text() if entries.get(name) is not None else ""
        if old != new:
            delta.extend(difflib.unified_diff(old.splitlines(True), new.splitlines(True),
                                            fromfile="a/" + name, tofile="b/" + name))
    incremental = HERE / "terminal-incremental-from-R.patch"
    cumulative_path = HERE / "terminal-cumulative.patch"
    incremental.write_text("".join(delta))
    cumulative_path.write_text(cumulative)
    DEST.mkdir(exist_ok=False)
    for name, digest in r.items():
        if digest is not None:
            target = DEST / name
            target.parent.mkdir(parents=True, exist_ok=True)
            shutil.copy2(R / name, target)
    result = subprocess.run(["patch", "--batch", "--forward", "-p1", "-i", str(incremental)],
                            cwd=DEST, capture_output=True, text=True)
    (HERE / "terminal-reconstruction.stdout").write_text(result.stdout)
    (HERE / "terminal-reconstruction.stderr").write_text(result.stderr)
    result.check_returncode()
    reconstructed, _, _ = recorded.RETAINED.snapshot(DEST)
    assert reconstructed == entries
    assert recorded.RETAINED.snapshot(S)[0] == entries
    assert recorded.RETAINED.snapshot(R)[0] == r
    for name in ("docs", "tools", "tests"):
        (DEST / name).symlink_to(R / name, target_is_directory=True)
    identity = {
        "evidence_class": "Ran: exact scoped source reconstruction; no execution qualification",
        "source": str(S), "base": str(recorded.RETAINED.BASE),
        "source_tree_sha256": tree(entries), "entries": entries, "changes": changes,
        "cumulative_patch": cumulative_path.name, "cumulative_patch_sha256": recorded.sha(cumulative_path),
        "incremental_patch": incremental.name, "incremental_patch_sha256": recorded.sha(incremental),
        "incremental_base": str(R), "R_tree_sha256": tree(r), "Q_tree_sha256": tree(q),
        "reconstructed_source": str(DEST), "reconstructed_tree_sha256": tree(reconstructed),
        "reconstruction_equal": True, "source_and_R_unchanged": True,
        "recipe": "Retained R plus terminal-incremental-from-R.patch; R itself is Q plus the historical reader-incremental.patch. Do not apply cumulative patches atop Q/R.",
        "availability": "Full source bases and original corpus remain local. Publishing scoped patches does not publish the complete bases, binaries, or corpus.",
    }
    (HERE / "terminal-source-reconstruction.json").write_text(json.dumps(identity, indent=2) + "\n")
    print(json.dumps({k: identity[k] for k in ("source_tree_sha256", "incremental_patch_sha256", "reconstruction_equal")}))


if __name__ == "__main__":
    main()
