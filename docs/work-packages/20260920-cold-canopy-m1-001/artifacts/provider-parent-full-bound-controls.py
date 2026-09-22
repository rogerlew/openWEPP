"""Non-launching controls for the existing Critical full-validation exception."""
import importlib.util
import json
import hashlib
import os
import sys
import tempfile
from pathlib import Path
from unittest.mock import patch

path = Path(__file__).with_name("run_recorded.py")
spec = importlib.util.spec_from_file_location("recorder", path)
recorder = importlib.util.module_from_spec(spec)
spec.loader.exec_module(recorder)
full = [
    "nix", "develop", "/workdir/openWEPP", "--command", "env",
    "CARGO_TARGET_DIR=/tmp/openwepp-cold-canopy-m1-target", "CARGO_BUILD_JOBS=2",
    "cargo", "nextest", "run", "--workspace", "--profile", "full", "--no-fail-fast",
]
cases = [
    ("ordinary physical at cap", ["physical-diagnostic"], 180, True, False, 180, True),
    ("ordinary physical over cap", ["physical-diagnostic"], 181, True, False, 900, False),
    ("unflagged full cannot exceed cap", full, 900, True, False, 900, False),
    ("exact required full exception", full, 900, True, True, 900, True),
    ("full reserve cannot fit", full, 900, True, True, 899.99, False),
    ("full exception wrong bound", full, 901, True, True, 1000, False),
    ("full exception shorter bound", full, 180, True, True, 1000, False),
    ("full cannot be called nonphysical", full, 900, False, True, 1000, False),
    ("unflagged full still cannot be nonphysical", full, 900, False, False, 1000, False),
    ("flag cannot admit another physical command", ["physical-diagnostic"], 900, True, True, 1000, False),
    ("extra selector changes required coverage", full + ["-E", "test(m1_parent_)"], 900, True, True, 1000, False),
    ("altered environment is not allowlisted", ["CARGO_BUILD_JOBS=8" if x == "CARGO_BUILD_JOBS=2" else x for x in full], 900, True, True, 1000, False),
    ("ordinary nonphysical existing bound", ["nonphysical-lint"], 900, False, False, 900, True),
    ("ordinary bound still protects reserve", ["nonphysical-lint"], 900, False, False, 899.99, False),
]
results = []
for name, argv, timeout, physical, exception, available, expected in cases:
    try:
        recorder.validate_declared_bound(argv, timeout, physical, exception, available)
        admitted = True
    except SystemExit:
        admitted = False
    assert admitted == expected, name
    results.append({"case": name, "admitted": admitted, "expected": expected})

# Traverse actual main preflight, all custody checks and both reserve guards.
# Intercept Popen before it can launch anything; temporary receipt files are
# never presented as an executed full-workspace result.
class LaunchIntercepted(Exception):
    pass


guard_calls = []
real_guard = recorder.validate_declared_bound


def observed_guard(*args):
    guard_calls.append(args)
    return real_guard(*args)


def intercept_launch(*args, **kwargs):
    raise LaunchIntercepted


with tempfile.TemporaryDirectory(prefix="m1-full-bound-control-") as directory:
    temporary = Path(directory)
    manifest = json.loads(path.with_name("provider-parent-body-build-support-04.json").read_text())
    entries = recorder.SNAPSHOT.snapshot(recorder.SOURCE)[0]
    manifest["source_tree_sha256"] = hashlib.sha256(json.dumps(entries, sort_keys=True).encode()).hexdigest()
    manifest["material_environment"] = {name: os.environ.get(name) for name in recorder.MATERIAL_ENVIRONMENT}
    for item in manifest["files"]:
        item["sha256"] = recorder.sha(item["path"])
    manifest.update(intended_argv=full, declared_timeout_seconds=900,
                    physical=True, required_full_validation=True)
    support = temporary / "support.json"
    support.write_text(json.dumps(manifest))
    argv = [str(path), "--source-root", str(recorder.SOURCE), "--support-record", str(support),
            "--physical", "--required-full-validation", "--timeout", "900", "control", "--", *full]
    intercepted = False
    with patch.object(sys, "argv", argv), patch.object(recorder, "LOGS", temporary), \
            patch.object(recorder, "validate_declared_bound", observed_guard), \
            patch.object(recorder.subprocess, "Popen", intercept_launch):
        try:
            recorder.main()
        except LaunchIntercepted:
            intercepted = True
    assert intercepted and len(guard_calls) == 2, "both real pre-launch guards must admit the exact exception"
    results.append({"case": "actual main reaches both guards and intercepted launch", "admitted": True,
                    "expected": True, "guard_calls": len(guard_calls), "processes_launched": 0})
print(json.dumps({"evidence": "Ran: boundary validation only; no command or physical process launched", "passed": len(results), "cases": results}, indent=2))
