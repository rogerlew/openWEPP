#!/usr/bin/env python3
"""Nonphysical IPC checks for the separate-accuracy runner.

The test binary is generated in a temporary directory.  It only speaks the
runner's line protocol and returns synthetic JSON arrays; it imports no Rust
code and never opens the real runner state or binary.
"""
import argparse
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile


ROOT = Path(__file__).resolve().parent
RUNNER = ROOT / "separate-accuracy-runner.py"
INPUT = ROOT / "separate-accuracy-input-check.json"
MEASURE = ROOT / "separate-accuracy-measurement-draft.json"
TEST = "m1_coupled_tests::m1_separate_accuracy_diagnostic_ipc"
CONTROL_TEST = "m1_coupled_tests::m1_separate_accuracy_six_frozen_controls"
RECEIPT = ROOT / "separate-accuracy-runner-mock-check-receipt.json"


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


FAKE = r'''#!/usr/bin/env python3
import json, os, sys, time
TEST = "m1_coupled_tests::m1_separate_accuracy_diagnostic_ipc"
CONTROL_TEST = "m1_coupled_tests::m1_separate_accuracy_six_frozen_controls"
mode = os.environ.get("MOCK_MODE", "success")
if "--list" in sys.argv:
    print(TEST + ": test")
    print(CONTROL_TEST + ": test")
    raise SystemExit(0)
if CONTROL_TEST in sys.argv:
    print("test result: ok. 1 passed; 0 failed", flush=True)
    raise SystemExit(0)
line = sys.stdin.readline()
if mode == "noframes":
    raise SystemExit(0)
if not line.startswith("RUN_INIT "):
    raise SystemExit(11)
request = json.loads(line[len("RUN_INIT "):])
support = 86400 // request["cadence_seconds"]
if request["case"] == "C_freeze_then_melt_day" and request["cadence_seconds"] > 60:
    support += 1
def marker(value):
    print("M1_SEPARATE_ACCURACY " + json.dumps(value, sort_keys=True), flush=True)
if mode == "delay_ready":
    time.sleep(0.1)
marker({"event":"READY", "pid":os.getpid(), "base_sha256":"4b324b0a9c136f2203e8073ca260db5be9e7dd07722a37aec60f2cd528be2d54", "request":request, "support_count":support})
command = sys.stdin.readline().strip()
if mode == "rust_frame":
    # Model the old Rust endpoint: only a literal command is recognized.
    raise SystemExit(12 if command != "RUN_FRESH_DAYS" else 0)
if mode == "child_exit":
    raise SystemExit(13)
if not command.startswith("RUN_FRESH_DAYS "):
    raise SystemExit(14)
days = int(command.split()[1])
if mode == "delay_end":
    time.sleep(0.1)
marker({"event":"END", "day_count":days, "support_count":support})
if sys.stdin.readline().strip() != "DUMP":
    raise SystemExit(15)
if mode == "delay_dump":
    time.sleep(0.1)
value = 1 if mode == "different" else 0
payload = [{"records":[{"support_id": index, "synthetic":value} for index in range(support)], "completed_supports":support, "first_failure":None} for _ in range(days)]
marker({"event":"DUMP", "days":payload})
print("test result: ok. 1 passed; 0 failed", flush=True)
'''


def command(work, *args, mode="success"):
    env = os.environ.copy()
    env["MOCK_MODE"] = mode
    return subprocess.run(
        [sys.executable, str(work / "separate-accuracy-runner.py"), *args],
        cwd=work, text=True, capture_output=True, timeout=20, env=env,
    )


def prepare(measure_mutator=None):
    tmp = tempfile.TemporaryDirectory(prefix="separate-accuracy-runner-mock-")
    work = Path(tmp.name)
    for item in (RUNNER, INPUT, MEASURE, ROOT / 'separate-accuracy-source-custody.py', ROOT / 'separate-accuracy-build-custody.py'):
        shutil.copy2(item, work / item.name)
    if measure_mutator:
        measure = json.loads((work / MEASURE.name).read_text())
        measure_mutator(measure)
        (work / MEASURE.name).write_text(json.dumps(measure, indent=2) + "\n")
    binary = work / "fake-child.py"
    binary.write_text(FAKE)
    binary.chmod(0o755)
    source = work / 'fake-source'
    source.mkdir()
    (source / 'source.txt').write_text('synthetic source\n')
    archive, source_receipt = work / 'source.tar.gz', work / 'source-receipt.json'
    subprocess.run([sys.executable, str(work / 'separate-accuracy-source-custody.py'), 'pack', str(source), str(archive), str(source_receipt)], check=True, capture_output=True, text=True)
    deps = work / 'deps'; deps.mkdir()
    (deps / 'fake.d').write_text('fake: source.txt\n')
    build_receipt = work / 'build-receipt.json'
    subprocess.run([sys.executable, str(work / 'separate-accuracy-build-custody.py'), 'freeze', str(build_receipt), '--source', str(source), '--deps', str(deps), '--binary', str(binary)], check=True, capture_output=True, text=True)
    pin_sources = (RUNNER, INPUT, MEASURE, ROOT / 'separate-accuracy-protocol-draft.json', ROOT / 'separate-accuracy-control-fixtures.json', ROOT / 'separate-accuracy-independent-reconstruction.py', ROOT / 'separate-accuracy-source-custody.py', ROOT / 'separate-accuracy-build-custody.py', ROOT / 'architecture-decision-successor-cases.json', ROOT / 'original-input-reference.json', ROOT / 'continuous-fixtures-draft01.json')
    for item in pin_sources:
        name = item.name
        if not (work / name).exists():
            shutil.copy2(item, work / name)
    pins = {item.name: sha(work / item.name) for item in pin_sources}
    receipt = {
        "input_check_sha256": sha(work / INPUT.name),
        "measurement_sha256": sha(work / MEASURE.name),
        "binary_sha256": sha(binary),
        'artifact_pins': pins, 'source_root': str(source),
        'source_archive': {'path': str(archive), 'sha256': sha(archive)},
        'source_receipt': {'path': str(source_receipt), 'sha256': sha(source_receipt)},
        'build_receipt': {'path': str(build_receipt), 'sha256': sha(build_receipt)},
    }
    (work / "separate-accuracy-source-review-approved.json").write_text(json.dumps(receipt))
    return tmp, work, binary


def cell(index=0):
    item = json.loads(MEASURE.read_text())["cell_sequence"][index]
    return json.dumps(item, sort_keys=True, separators=(",", ":"))


def require(condition, name, detail):
    if not condition:
        raise AssertionError(f"{name}: {detail}")


def ready(work):
    result = command(work, "READY")
    require(result.returncode == 0, "READY", result.stderr)


def prerequisites(work, binary):
    controls = json.loads((work / 'separate-accuracy-control-fixtures.json').read_text())['controls']
    for control in controls:
        result = command(work, 'RUN_FRESH_DAYS', '--kind', 'control', '--control-id', control['id'], '--test-binary', str(binary))
        require(result.returncode == 0, 'control ' + control['id'], result.stderr)
    measure = json.loads((work / MEASURE.name).read_text())
    cases = list(dict.fromkeys(value['case'] for value in measure['cell_sequence']))
    for case in cases:
        reference = next(value for value in measure['cell_sequence'] if value['case'] == case and value['investigation'] == 'A' and value['formulation'] == 'FULL' and value['profile'] == 'P0')
        raw = json.dumps(reference, sort_keys=True, separators=(',', ':'))
        for kind, cell_value in (('reference', raw), ('stability', json.dumps(dict(reference, stability_variant='strict_reference'), sort_keys=True, separators=(',', ':')))):
            result = command(work, 'RUN_FRESH_DAYS', '--kind', kind, '--cell', cell_value, '--test-binary', str(binary))
            require(result.returncode == 0, kind + ' ' + case, result.stderr)


def success_schedule():
    tmp, work, binary = prepare()
    try:
        ready(work)
        prerequisites(work, binary)
        first = cell()
        diag = command(work, "RUN_FRESH_DAYS", "--cell", first, "--test-binary", str(binary))
        require(diag.returncode == 0, "diagnostic", diag.stderr)
        for batch in range(6):
            timed = command(work, "RUN_FRESH_DAYS", "--cell", first, "--test-binary", str(binary), "--kind", "timing", "--batch", str(batch))
            require(timed.returncode == 0, f"timing batch {batch}", timed.stderr)
        state = json.loads((work / "separate-accuracy-runner-state.json").read_text())
        results = state["results"]
        require(len(results) == 19, "success result count", len(results))
        require(results[12]["fresh_complete_days"] == 1, "diagnostic day count", results[12])
        require(all(row["fresh_complete_days"] == 4 for row in results[13:]), "timing day count", results[13:])
        require(results[12]['reused_reference'] == results[6]['key'], 'A reference reuse', results[12])
        return {"result_count": len(results), "physical_seconds": state["physical_seconds"], "controls_and_reference_operations": 12}
    finally:
        tmp.cleanup()


def expected_failure(name, mode, expected):
    tmp, work, binary = prepare()
    try:
        ready(work)
        result = command(work, "RUN_FRESH_DAYS", "--cell", cell(), "--test-binary", str(binary), mode=mode)
        require(result.returncode != 0 and expected in result.stderr, name, result.stderr)
        state = json.loads((work / "separate-accuracy-runner-state.json").read_text())
        charge = state["failed"][0]["charged_seconds"]
        return {"stderr": result.stderr.strip(), "charged_seconds": charge}
    finally:
        tmp.cleanup()


def digest_mismatch():
    tmp, work, binary = prepare()
    try:
        ready(work)
        diagnostic = command(work, "RUN_FRESH_DAYS", "--cell", cell(), "--test-binary", str(binary))
        require(diagnostic.returncode == 0, "digest diagnostic", diagnostic.stderr)
        timing = command(work, "RUN_FRESH_DAYS", "--cell", cell(), "--test-binary", str(binary), "--kind", "timing", "--batch", "0", mode="different")
        require(timing.returncode != 0 and "timed day digest differs" in timing.stderr, "digest mismatch", timing.stderr)
        return {"stderr": timing.stderr.strip()}
    finally:
        tmp.cleanup()


def order_and_b_eligibility():
    tmp, work, binary = prepare()
    try:
        ready(work)
        out_of_order = command(work, "RUN_FRESH_DAYS", "--cell", cell(1), "--test-binary", str(binary))
        require(out_of_order.returncode != 0 and "next frozen cell_sequence" in out_of_order.stderr, "order", out_of_order.stderr)
        # Simulate completed A diagnostics so B is the next scheduled diagnostic.
        state_path = work / "separate-accuracy-runner-state.json"
        state = json.loads(state_path.read_text())
        for i in range(24):
            item = json.loads(cell(i))
            state.setdefault("results", []).append({"kind":"diagnostic", "cell":item, "key":json.dumps({"cell":item,"kind":"diagnostic","batch":0}, sort_keys=True, separators=(",",":"))})
        state_path.write_text(json.dumps(state))
        b_cell = json.loads(cell(24))
        b_key = json.dumps({"cell":b_cell,"kind":"diagnostic","batch":0}, sort_keys=True, separators=(",",":"))
        state = json.loads(state_path.read_text())
        state["completed"].append(b_key)
        state["results"].append({"kind":"diagnostic", "cell":b_cell, "key":b_key, "day_digests":["synthetic"]})
        state_path.write_text(json.dumps(state))
        b = command(work, "RUN_FRESH_DAYS", "--cell", cell(24), "--test-binary", str(binary), "--kind", "timing", "--batch", "0")
        return {
            "order": out_of_order.stderr.strip(),
            "b_eligibility_rejected": b.returncode != 0 and "B eligibility" in b.stderr,
            "b_returncode": b.returncode,
            "b_stderr": b.stderr.strip(),
        }
    finally:
        tmp.cleanup()


def global_limit():
    tmp, work, binary = prepare(lambda value: value["limits"].__setitem__("all_physical_elapsed_seconds", 0))
    try:
        ready(work)
        result = command(work, "RUN_FRESH_DAYS", "--cell", cell(), "--test-binary", str(binary))
        require(result.returncode != 0 and "physical cap exhausted" in result.stderr, "global cap", result.stderr)
        return {"stderr": result.stderr.strip()}
    finally:
        tmp.cleanup()


def short_cap_timeout(name, mode, expected):
    tmp, work, binary = prepare(lambda value: value["limits"].update({"candidate_cell_elapsed_seconds": 0.03, "all_physical_elapsed_seconds": 0.03, "command_seconds": 1}))
    try:
        ready(work)
        result = command(work, "RUN_FRESH_DAYS", "--cell", cell(), "--test-binary", str(binary), mode=mode)
        require(result.returncode != 0 and expected in result.stderr, name, result.stderr)
        state = json.loads((work / "separate-accuracy-runner-state.json").read_text())
        charge = state["failed"][0]["charged_seconds"]
        require(charge >= 0.03, name + " charge", charge)
        return {"stderr": result.stderr.strip(), "charged_seconds": charge}
    finally:
        tmp.cleanup()


def cumulative_cell_cap():
    tmp, work, binary = prepare(lambda value: value["limits"].update({"candidate_cell_elapsed_seconds": 0.04, "all_physical_elapsed_seconds": 1, "command_seconds": 1}))
    try:
        ready(work)
        first = cell()
        diagnostic = command(work, "RUN_FRESH_DAYS", "--cell", first, "--test-binary", str(binary))
        require(diagnostic.returncode == 0, "short-cap diagnostic", diagnostic.stderr)
        first_timing = command(work, "RUN_FRESH_DAYS", "--cell", first, "--test-binary", str(binary), "--kind", "timing", "--batch", "0", mode="delay_end")
        require(first_timing.returncode != 0, "short-cap timing", first_timing.stderr)
        state = json.loads((work / "separate-accuracy-runner-state.json").read_text())
        require(state["physical_seconds"] >= 0.04, "short-cap aggregate charge", state)
        return {"stderr": first_timing.stderr.strip(), "physical_seconds": state["physical_seconds"]}
    finally:
        tmp.cleanup()


def cell_failure_is_terminal():
    tmp, work, binary = prepare()
    try:
        ready(work)
        first = cell()
        diagnostic = command(work, "RUN_FRESH_DAYS", "--cell", first, "--test-binary", str(binary))
        require(diagnostic.returncode == 0, "terminal diagnostic", diagnostic.stderr)
        failed = command(work, "RUN_FRESH_DAYS", "--cell", first, "--test-binary", str(binary), "--kind", "timing", "--batch", "0", mode="child_exit")
        require(failed.returncode != 0, "terminal failed batch", failed.stderr)
        retry = command(work, "RUN_FRESH_DAYS", "--cell", first, "--test-binary", str(binary), "--kind", "timing", "--batch", "0")
        later = command(work, "RUN_FRESH_DAYS", "--cell", first, "--test-binary", str(binary), "--kind", "timing", "--batch", "1")
        require(retry.returncode != 0 and "no retry" in retry.stderr, "terminal retry", retry.stderr)
        require(later.returncode != 0 and "prior consecutive batch" in later.stderr, "terminal later batch", later.stderr)
        return {"retry": retry.stderr.strip(), "later_batch": later.stderr.strip()}
    finally:
        tmp.cleanup()


def ready_reset_preserves_state():
    tmp, work, binary = prepare()
    try:
        ready(work)
        first = cell()
        diagnostic = command(work, "RUN_FRESH_DAYS", "--cell", first, "--test-binary", str(binary))
        require(diagnostic.returncode == 0, "reset diagnostic", diagnostic.stderr)
        reset = command(work, "READY")
        require(reset.returncode == 0, "second READY", reset.stderr)
        retry = command(work, "RUN_FRESH_DAYS", "--cell", first, "--test-binary", str(binary))
        state = json.loads((work / "separate-accuracy-runner-state.json").read_text())
        require(retry.returncode != 0, "reset retry", retry.stderr)
        require(len(state["results"]) == 1, "reset result preservation", state)
        return {"retry": retry.stderr.strip(), "result_count": len(state["results"])}
    finally:
        tmp.cleanup()


def receipt_pin_checks():
    results = {}
    for label, filename in (("input", INPUT.name), ("measurement", MEASURE.name)):
        tmp, work, binary = prepare()
        try:
            ready(work)
            path = work / filename
            path.write_text(path.read_text() + "\n")
            result = command(work, "RUN_FRESH_DAYS", "--cell", cell(), "--test-binary", str(binary))
            require(result.returncode != 0 and "receipt" in result.stderr, label + " receipt pin", result.stderr)
            results[label] = result.stderr.strip()
        finally:
            tmp.cleanup()
    return results


def interrupted_state_refusal():
    tmp, work, _binary = prepare()
    try:
        state = {'phase':'READY', 'physical_seconds':0.0, 'reference_seconds':0.0,
                 'operations':{}, 'completed':[], 'failed':[], 'results':[],
                 'in_flight':{'key':'synthetic-interrupted', 'maximum_charge_seconds':1.0, 'launch_utc':0.0}}
        (work / 'separate-accuracy-runner-state.json').write_text(json.dumps(state))
        result = command(work, 'READY')
        require(result.returncode != 0 and 'Interrupted physical operation' in result.stderr, 'interrupted-state refusal', result.stderr)
        return {'stderr':result.stderr.strip()}
    finally:
        tmp.cleanup()


def c1_oracle_replay_namespace():
    tmp, work, binary = prepare()
    try:
        original = {'kind':'control', 'cell':{'control_id':'C1_empty_admissible'},
                    'batch':0, 'charged_seconds':0.002019944, 'status':'integrity_failure',
                    'key':json.dumps({'kind':'control','cell':{'control_id':'C1_empty_admissible'},'batch':0}, sort_keys=True, separators=(',',':'))}
        state = {'phase':'READY', 'physical_seconds':original['charged_seconds'], 'reference_seconds':0.0,
                 'operations':{}, 'completed':[], 'failed':[original['key']], 'results':[dict(original)],
                 'preserved_defect_operations':[dict(original)], 'verified_defect_namespace':'c1-oracle-replay01',
                 'defect_replay_consumed':1}
        (work / 'separate-accuracy-runner-state.json').write_text(json.dumps(state))
        result = command(work, 'RUN_FRESH_DAYS', '--kind', 'control', '--control-id', 'C1_empty_admissible', '--test-binary', str(binary))
        require(result.returncode == 0, 'C1 replay', result.stderr)
        state = json.loads((work / 'separate-accuracy-runner-state.json').read_text())
        replay = next(value for value in state['operations'].values() if value['kind'] == 'control')
        require(len(state['preserved_defect_operations']) == 1 and len(state['results']) == 2, 'preserved C1 evidence', state)
        require(replay['output'].endswith('-c1-oracle-replay01.json'), 'C1 replay output namespace', replay)
        require(abs(replay['cap_seconds'] - (30 - original['charged_seconds'])) < 1e-12, 'C1 charge retention cap', replay)
        require(state['physical_seconds'] > original['charged_seconds'], 'C1 aggregate charge retention', state)
        return {'cap_seconds':replay['cap_seconds'], 'output':replay['output'], 'physical_seconds':state['physical_seconds']}
    finally:
        tmp.cleanup()


def c1_oracle_replay_marker_refusals():
    outcomes = {}
    for label, marker in (('missing', None), ('zero', 0), ('two', 2), ('bool', True)):
        tmp, work, binary = prepare()
        try:
            original = {'kind':'control', 'cell':{'control_id':'C1_empty_admissible'}, 'batch':0,
                        'charged_seconds':0.002019944, 'status':'integrity_failure',
                        'key':json.dumps({'kind':'control','cell':{'control_id':'C1_empty_admissible'},'batch':0}, sort_keys=True, separators=(',',':'))}
            state = {'phase':'READY','physical_seconds':original['charged_seconds'],'reference_seconds':0.0,
                     'operations':{},'completed':[],'failed':[original['key']],'results':[dict(original)],
                     'preserved_defect_operations':[dict(original)],'verified_defect_namespace':'c1-oracle-replay01'}
            if marker is not None:
                state['defect_replay_consumed'] = marker
            (work / 'separate-accuracy-runner-state.json').write_text(json.dumps(state))
            result = command(work, 'RUN_FRESH_DAYS', '--kind', 'control', '--control-id', 'C1_empty_admissible', '--test-binary', str(binary))
            require(result.returncode != 0 and 'Invalid single-control defect recovery evidence' in result.stderr, 'replay marker '+label, result.stderr)
            outcomes[label] = result.stderr.strip().splitlines()[-1]
        finally:
            tmp.cleanup()
    return outcomes


def run():
    receipt = {"schema": 1, "evidence_class": "Ran: synthetic fake child only; no Rust/binary/physical execution.", "runner_sha256": sha(RUNNER), "measurement_sha256": sha(MEASURE), "checks": {}}
    try:
        receipt["checks"]["success_1_diagnostic_plus_6x4"] = {"status":"PASS", "detail":success_schedule()}
        receipt["checks"]["custody_and_pins_prepost"] = {"status":"PASS", "detail":"Synthetic source archive, source receipt, depfile build receipt, binary and all runner artifact pins were accepted before and after each mock operation."}
        receipt["checks"]["interrupted_state_refuses_launch"] = {"status":"PASS", "detail":interrupted_state_refusal()}
        receipt["checks"]["c1_oracle_replay_namespace_preserves_charge"] = {"status":"PASS", "detail":c1_oracle_replay_namespace()}
        receipt["checks"]["c1_oracle_replay_marker_refusals"] = {"status":"PASS", "detail":c1_oracle_replay_marker_refusals()}
        receipt["unimplemented_cli_coverage"] = []
        receipt["status"] = "PASS" if all(value["status"] == "PASS" for value in receipt["checks"].values()) else "FAIL"
    except Exception as error:
        receipt["status"] = "FAIL"
        receipt["error"] = str(error)
    return receipt


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--write-receipt", action="store_true")
    args = parser.parse_args()
    output = run()
    if args.write_receipt:
        RECEIPT.write_text(json.dumps(output, indent=2, sort_keys=True) + "\n")
    print(json.dumps(output, indent=2, sort_keys=True))
    raise SystemExit(0 if output["status"] == "PASS" else 1)
