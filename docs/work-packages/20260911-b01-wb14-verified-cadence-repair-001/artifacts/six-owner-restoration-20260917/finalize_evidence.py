"""Reconcile this diagnostic's already executed terminal receipts; no test selection."""
from pathlib import Path
import datetime
import hashlib
import importlib.util
import json
import re

HERE = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location("recorded", HERE / "run_recorded.py")
recorded = importlib.util.module_from_spec(spec)
spec.loader.exec_module(recorded)


def read(name):
    return json.loads((HERE / name).read_text())


def tree(entries):
    return hashlib.sha256(json.dumps(entries, sort_keys=True).encode()).hexdigest()


def audit(label):
    receipt = read(label + ".json")
    assert receipt["exit_code"] == 0
    assert all(receipt[key] for key in (
        "source_unchanged", "binary_unchanged", "pinned_files_unchanged", "inputs_unchanged"
    ))
    output = (HERE / (label + ".stdout")).read_text()
    assert "test result: ok. 1 passed; 0 failed" in output
    counters = {}
    for prefix in ("probe-mechanism-audit:", "probe-direct-audit:",
                   "probe-gsi-receipt-advance-audit:", "probe-native-physics-audit:"):
        line = next(line for line in output.splitlines() if line.startswith(prefix))
        values = re.findall(r"([a-z0-9_]+): (\d+)", line)
        allowed = {"run_frame_constructions", "validator_advances"}
        assert all(int(value) == 0 for key, value in values if key not in allowed)
        counters[prefix] = line
    resources = (HERE / (label + ".resources")).read_text()
    return {
        "receipt": label + ".json", "source_sha256": receipt["source_tree_sha256"],
        "binary_sha256": receipt["binary_sha256"], "exit_code": receipt["exit_code"],
        "elapsed_seconds": receipt["elapsed_seconds"],
        "maximum_rss_KiB": int(re.search(r"Maximum resident set size \(kbytes\): (\d+)", resources)[1]),
        "counters": counters,
    }


terminal = read("terminal-source-reconstruction.json")
entries = recorded.RETAINED.snapshot(recorded.SOURCE)[0]
assert entries == terminal["entries"]
assert recorded.RETAINED.snapshot(Path(terminal["reconstructed_source"]))[0] == entries
preserved = {}
for directory, expected in (
    ("deferred-context-admission-20260916", "ab221bc5280eaee458be9a6066ba628efe292c0a17394ed1e754ff8f265d8eed"),
    ("native-context-restoration-reader-20260916", "7c04f52e6cae6327202591b37f0c4e7a5fff3ae34a103255a35c49a3dbdf37ab"),
    ("snowfree-recorder-candidate-20260915", "ecc48d5235d8d53af496856459e3eaa29cca2dda2511697350a67bac172f9592"),
):
    actual = tree(recorded.RETAINED.snapshot(recorded.SOURCE.parent / directory)[0])
    assert actual == expected
    preserved[directory] = actual
labels = ("terminal-v20-ordinary-refusal", "terminal-v20-late-controls", "terminal-v20-composed-parity")
runs = [audit(label) for label in labels]
assert all(run["source_sha256"] == tree(entries) for run in runs)
assert len({run["binary_sha256"] for run in runs}) == 1
assert recorded.sha(Path(read(labels[-1] + ".json")["binary"])) == runs[-1]["binary_sha256"]
owners = read("terminal-v20-owner-bytes/owners.json")
original = read("report-v16-owner-bytes/owners.json")
assert owners["all_equal"] and len(owners["owners"]) == 6
assert all(row["equal"] for row in owners["owners"])
assert {row["owner"]: row["expected_sha256"] for row in owners["owners"]} == {
    row["owner"]: row["expected_sha256"] for row in original["owners"]
}
late = (HERE / (labels[1] + ".stdout")).read_text()
cases = [line for line in late.splitlines() if line.startswith("late control PASS:")]
assert len(cases) == 6 and len(set(cases)) == 6
positive = (HERE / (labels[-1] + ".stdout")).read_text()
postcondition = next(line for line in positive.splitlines() if line.startswith("restored native:"))
result = {
    "evidence_class": "Ran: terminal receipt, exact owner bytes, source recovery and work-counter reconciliation; not independent review",
    "utc": datetime.datetime.now(datetime.timezone.utc).isoformat(),
    "source_tree_sha256": tree(entries), "binary_sha256": runs[-1]["binary_sha256"],
    "preserved_base_trees": preserved, "reconstructed_source_equal": True,
    "runs": runs, "six_owner_exact_equality": True, "original_expected_map_unchanged": True,
    "owners": owners["owners"], "subsequent_postconditions": postcondition,
    "late_cases": cases, "early_lifecycle_reuse": "terminal-evidence-reuse.json",
    "quality": "terminal-quality-disposition.json",
    "independent_correctness": "UNAVAILABLE / NOT MET",
    "independent_QA": "UNAVAILABLE / NOT MET",
    "disposition": "Private diagnostic byte parity and reachable controls PASS; overall acceptance INCOMPLETE / HOLD. Draft unpromoted; no production adoption or scientific/restart/conservation/cadence qualification.",
}
(HERE / "terminal-results.json").write_text(json.dumps(result, indent=2) + "\n")
print(json.dumps({key: result[key] for key in ("source_tree_sha256", "binary_sha256", "six_owner_exact_equality", "disposition")}))
