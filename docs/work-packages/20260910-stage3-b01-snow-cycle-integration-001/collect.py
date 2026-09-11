#!/usr/bin/env python3
"""Cross-policy extension of the retained fresh-process collector.

Keeps the historical strict identity collector unchanged. This mode admits
declared physical/work differences, retaining every failure and raw stream.
Scientific bands and independent balances are analyzed separately.
"""
import argparse
import importlib.util
import json
import os
from pathlib import Path
import signal
import subprocess
import time
import ijson  # pinned local dependency: ijson==3.4.0

PACKAGE = Path(__file__).resolve().parent
PREDECESSOR = PACKAGE.parent / "20260906-stage3-prospective-mechanism-experiments-001/artifacts/reproduction/run_series.py"
SPEC = importlib.util.spec_from_file_location("strict_collector", PREDECESSOR)
STRICT = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(STRICT)
digest = STRICT.digest
TEST = "hillslope::tests::stage3_snow_accuracy_case"


def collect(binary, output, policy, case, ofes=1, physical=True, timeout=180,
            test=TEST, cases_path=None, checkpoint_day=None, checkpoint=None, resume=None, reuse_run_dir=None, parent_poison_check=False, cpu=0, archive_ack_poison_check=False):
    if cpu not in os.sched_getaffinity(0):
        raise ValueError("requested CPU is not available")
    if ofes == 2 and case != "mixed_resolved_trace":
        raise ValueError("two-OFE posture requires the named mixed fixture")
    binary, output = Path(binary).resolve(), Path(output).resolve()
    output.mkdir(parents=True, exist_ok=False)
    cases = Path(cases_path).resolve() if cases_path else PACKAGE.parent / "20260910-stage3-snow-accuracy-runtime-001/artifacts/cases.json"
    if case not in json.loads(cases.read_text())["cases"]:
        raise ValueError("undeclared case")
    env = {k: v for k, v in os.environ.items() if not k.startswith("OPENWEPP_")}
    treatment = dict(RUST_MIN_STACK="67108864", OPENWEPP_EXPERIMENT_OFES=str(ofes),
                     OPENWEPP_EXPERIMENT_REPEATS="1", OPENWEPP_COMPACT_COST="off",
                     OPENWEPP_ACCURACY_POLICY=policy, OPENWEPP_ACCURACY_CASE=case,
                     OPENWEPP_ACCURACY_CASES=str(cases),
                     OPENWEPP_ACCURACY_OUTPUT=str(output),
                     OPENWEPP_ACCURACY_PHYSICAL=str(int(physical)))
    if parent_poison_check:
        if case != "mixed_resolved_trace" or ofes != 2 or not physical:
            raise ValueError("parent poison checks require physical named mixed case")
        treatment["OPENWEPP_B01_PARENT_POISON_CHECK"] = "1"
    if archive_ack_poison_check:
        if policy != "B01" or not physical:
            raise ValueError("archive acknowledgement checks require physical B01 execution")
        treatment["OPENWEPP_B01_ARCHIVE_ACK_POISON_CHECK"] = "1"
    if checkpoint_day is not None:
        if checkpoint is None or resume is not None: raise ValueError("checkpoint write arguments")
        checkpoint = Path(checkpoint).resolve()
        if checkpoint.exists(): raise ValueError("checkpoint already exists")
        treatment.update(OPENWEPP_B01_CHECKPOINT_AFTER_DAY=str(checkpoint_day), OPENWEPP_B01_CHECKPOINT_PATH=str(checkpoint))
    if resume is not None:
        if reuse_run_dir is None: raise ValueError("resume requires original fixture")
        resume = Path(resume).resolve()
        treatment["OPENWEPP_B01_RESUME_PATH"] = str(resume)
    if reuse_run_dir is not None: treatment["OPENWEPP_B01_REUSE_RUN_DIR"] = str(Path(reuse_run_dir).resolve())
    env.update(treatment)
    argv = ["taskset", "-c", str(cpu), str(binary), test,
            "--ignored", "--exact", "--nocapture", "--test-threads=1"]
    receipt = dict(schema="snow_accuracy_process_v1", argv=argv, cwd=str(output),
                   binary_sha256=digest(binary), cases_sha256=digest(cases),
                   collector_sha256=digest(Path(__file__)),
                   predecessor_collector_sha256=digest(PREDECESSOR),
                   policy=policy, case=case, ofes=ofes, physical=physical,
                   environment=treatment, timeout_s=timeout, timeout=False)
    if resume is not None: receipt["external_checkpoint_sha256"] = digest(resume)
    (output / "started.json").write_text(json.dumps(receipt, indent=2) + "\n")
    started = time.monotonic()
    process = None
    receipt["execution_valid"] = False
    receipt["scientific_qualification"] = "NOT_ASSESSED"
    try:
        with (output / "stdout.log").open("xb") as stdout, (output / "stderr.log").open("xb") as stderr:
            process = subprocess.Popen(argv, cwd=output, env=env, stdout=stdout,
                                       stderr=stderr, start_new_session=True)
            while True:
                pid, status, usage = os.wait4(process.pid, os.WNOHANG)
                if pid:
                    break
                if time.monotonic() - started >= timeout:
                    receipt["timeout"] = True
                    os.killpg(process.pid, signal.SIGKILL)
                    pid, status, usage = os.wait4(process.pid, 0)
                    break
                time.sleep(0.05)
            process.returncode = os.waitstatus_to_exitcode(status)
        receipt.update(exit_code=process.returncode,
                       process_user_s=usage.ru_utime, process_system_s=usage.ru_stime,
                       lifetime_peak_rss_kib=usage.ru_maxrss,
                       binary_unchanged=digest(binary) == receipt["binary_sha256"])
        result_path = output / "run.json"
        receipt["runner_receipt_present"] = result_path.exists()
        result = json.loads(result_path.read_text()) if result_path.exists() else {}
        if result:
            receipt.update(runner_execution=result["execution"], runner_wall_s=result["runner_wall_s"],
                           runner_receipt_sha256=digest(result_path),
                           policy_identity_valid=result["policy"] == policy and result["case"] == case and result["ofes"] == ofes,
                           completed_days=(result.get("snapshot") or {}).get("committed_day_count"))
        observation_path = output / "observations.json"
        observation, physical_rows = {}, 0
        if observation_path.exists():
            with observation_path.open("rb") as source:
                for prefix, event, value in ijson.parse(source):
                    if prefix == "physical.item" and event == "start_map": physical_rows += 1
                    if prefix in ("schema", "physical_enabled", "overflow", "counts_poisoned") and event in ("string", "boolean"):
                        observation[prefix] = value
                    if prefix == "counts" and event == "start_map": observation["counts_present"] = True
        receipt["physical_rows"] = physical_rows
        receipt["observation_complete"] = (
            observation.get("schema") == "snow_accuracy_observation_v2"
            and observation.get("physical_enabled") == physical
            and not observation.get("overflow", True)
            and not observation.get("counts_poisoned", True)
            and observation.get("counts_present", False)
            and (not physical or physical_rows > 0))
        requested_days = len(json.loads(cases.read_text())["cases"][case]["forcing"])
        receipt["days_complete"] = result.get("days_requested") == requested_days == receipt.get("completed_days")
        carrier, lse = result.get("carrier", {}), result.get("lse", {})
        receipt["counter_errors"] = {"carrier": [c.get("errors") for c in carrier.get("counts", [])],
                                     "lse": {k:v["errors"] for k,v in lse.items() if isinstance(v, dict) and "errors" in v}}
        receipt["counters_complete"] = (carrier.get("dropped_records") == 0 and lse.get("dropped_events") == 0
            and lse.get("overflow") is False and bool(carrier.get("counts"))
            and all(c["started"] == c["completed"] + c["errors"] for c in carrier["counts"])
            and all(v["starts"] == v["completions"] + v["errors"] for v in lse.values() if isinstance(v, dict) and "starts" in v))
        inputs_path, outputs_path = output / "inputs.json", output / "outputs.json"
        receipt["files_complete"] = False
        if inputs_path.exists() and outputs_path.exists():
            inputs, outputs = json.loads(inputs_path.read_text()), json.loads(outputs_path.read_text())
            root = Path((output / "fixture-path.txt").read_text().strip())
            expected_output = output / "runtime-output" if reuse_run_dir is not None else root / "output"
            # New invocation-isolated protocol declares its actual publication root.
            declared_output = Path(outputs["output_dir"]) if "output_dir" in outputs else root / "output"
            receipt["output_directory_valid"] = declared_output.resolve() == expected_output.resolve()
            receipt["files_complete"] = receipt["output_directory_valid"] and bool(inputs["files"]) and bool(outputs["files"]) and all(
                (base / name).is_file() and digest(base / name) == expected
                for base, files in ((root, inputs["files"]), (declared_output, outputs["files"]))
                for name, expected in files.items())
        receipt["execution_valid"] = (receipt.get("exit_code") == 0 and not receipt["timeout"]
            and receipt.get("binary_unchanged", False) and receipt.get("policy_identity_valid", False)
            and receipt["observation_complete"] and receipt["days_complete"] and receipt["counters_complete"]
            and receipt["files_complete"] and receipt.get("runner_execution") == "PASS")
        if checkpoint_day is not None:
            receipt["checkpoint_written"] = (receipt.get("exit_code") == 86 and not receipt["timeout"]
                and checkpoint.is_file() and receipt.get("binary_unchanged", False)
                and receipt["observation_complete"])
            if checkpoint.is_file(): receipt["checkpoint_sha256"] = digest(checkpoint)
            receipt["execution_valid"] = False  # deliberate process exit is not a completed cycle
        if resume is not None:
            receipt["external_checkpoint_unchanged"] = digest(resume) == receipt["external_checkpoint_sha256"]
            receipt["execution_valid"] = receipt["execution_valid"] and receipt["external_checkpoint_unchanged"]
    except Exception as error:
        receipt["infrastructure_error"] = f"{type(error).__name__}: {error}"
        if process is not None and process.returncode is None:
            try:
                os.killpg(process.pid, signal.SIGKILL)
                process.wait()
            except ProcessLookupError:
                pass
    finally:
        receipt["wall_s"] = time.monotonic() - started
        receipt["artifact_sha256"] = {name: digest(output / name) for name in
            ("stdout.log", "stderr.log", "run.json", "observations.json", "inputs.json", "outputs.json")
            if (output / name).exists()}
        (output / "receipt.json").write_text(json.dumps(receipt, indent=2) + "\n")
    return receipt


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("binary", type=Path)
    parser.add_argument("output", type=Path)
    parser.add_argument("--policy", required=True)
    parser.add_argument("--case", required=True)
    parser.add_argument("--ofes", type=int, choices=(1, 2, 10, 19), default=1)
    parser.add_argument("--timeout", type=float, default=180)
    parser.add_argument("--cpu", type=int, default=0, help="explicit recorded CPU affinity; default preserves historical CPU 0")
    parser.add_argument("--minimal", action="store_true")
    parser.add_argument("--test", default=TEST)
    parser.add_argument("--cases", type=Path)
    parser.add_argument("--checkpoint-after-day", type=int)
    parser.add_argument("--checkpoint", type=Path)
    parser.add_argument("--resume", type=Path)
    parser.add_argument("--reuse-run-dir", type=Path)
    parser.add_argument("--parent-poison-check", action="store_true")
    parser.add_argument("--archive-ack-poison-check", action="store_true")
    args = parser.parse_args()
    result = collect(args.binary, args.output, args.policy, args.case, args.ofes,
                     not args.minimal, args.timeout, args.test, args.cases, args.checkpoint_after_day, args.checkpoint, args.resume, args.reuse_run_dir, args.parent_poison_check, args.cpu, args.archive_ack_poison_check)
    print(json.dumps({k: result.get(k) for k in ("case", "policy", "ofes", "exit_code", "timeout", "runner_wall_s", "execution_valid", "completed_days", "checkpoint_written")}))


if __name__ == "__main__":
    main()
