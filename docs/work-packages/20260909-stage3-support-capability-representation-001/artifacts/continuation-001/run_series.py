#!/usr/bin/env python3
"""Frozen one-OFE fresh-process off/on series; no builds or retries."""
import argparse
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import resource
import subprocess
import time
from datetime import datetime, timezone

COMMON_PATH = (Path(__file__).resolve().parents[3] /
               "20260906-stage3-prospective-mechanism-experiments-001/artifacts/reproduction/run_series.py")
SPEC = importlib.util.spec_from_file_location("prior_cost_collector", COMMON_PATH)
if SPEC is None or SPEC.loader is None:
    raise RuntimeError("required exact identity collector unavailable")
COMMON = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(COMMON)
ORDER = ("off", "on", "on", "off")


def digest(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def write(path, value):
    with Path(path).open("x") as stream:
        json.dump(value, stream, indent=2, sort_keys=True, allow_nan=False)
        stream.write("\n")


def accounting(record, posture):
    if record["compact_cost_posture"] != posture:
        raise ValueError("wrong observer posture")
    profile = record["compact_cost"]
    if posture == "off":
        if profile is not None:
            raise ValueError("off arm retained observation")
        return
    names, times = profile["bucket_order"], profile["exclusive_ns"]
    if (len(names) != len(times) or len(set(names)) != len(names)
            or any(type(x) is not int or x < 0 for x in times)
            or type(profile["runner_ns"]) is not int or profile["runner_ns"] <= 0
            or profile["accounting_difference_ns"] != profile["runner_ns"] - sum(times)
            or profile["accounting_difference_ns"] != 0):
        raise ValueError("invalid additive accounting")
    if profile["runner_ns"] // 1000 != record["run_wall_us"]:
        raise ValueError("attribution denominator differs from runner wall")
    families = profile["family_exclusive_ns"]
    if (len(families) != 4 or any(len(row) != len(times) for row in families)
            or any(type(x) is not int or x < 0 for row in families for x in row)
            or any(sum(row[i] for row in families) > times[i] for i in range(len(times)))):
        raise ValueError("invalid family drilldown")
    populations = [p for p in profile["populations"] if p["family"] is not None]
    if not populations or len({(p["family"], p["dimension"], p["inactive_input_leaf_coordinates"])
                               for p in populations}) != len(populations):
        raise ValueError("missing or duplicated solver population")
    for p in populations:
        if (any(type(v) is not int or v < 0 for k, v in p.items() if k != "family")
                or p["dimension"] == 0 or p["solves"] == 0
                or p["inactive_input_leaf_coordinates"] > p["dimension"]
                or p["inactive_input_leaf_fd_columns"] > p["fd_columns"]):
            raise ValueError("invalid solver population")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ("binary", "environment", "identity", "out", "cwd", "source-patch"):
        parser.add_argument("--" + name, type=Path, required=True)
    parser.add_argument("--timeout", type=float, default=180)
    parser.add_argument("--development-only", action="store_true")
    args = parser.parse_args()
    binary = args.binary.resolve(strict=True)
    admitted = json.loads(args.identity.read_text())
    environment = json.loads(args.environment.read_text())
    environment = environment.get("runtime_env", environment)
    if not all(isinstance(k, str) and isinstance(v, str) for k, v in environment.items()):
        raise ValueError("runtime environment must contain string pairs")
    if 0 not in os.sched_getaffinity(0):
        raise ValueError("CPU0 unavailable; do not silently substitute")
    rules = admitted["manifest_rules"]
    if COMMON.FRAME_POINTER in rules:
        raise ValueError("F-specific work exception is forbidden")
    expected_hash = rules["/binary_sha256"]["expected_arm_identity"]
    if digest(binary) != expected_hash:
        raise ValueError("admitted executable hash mismatch")
    args.out.mkdir(parents=True, exist_ok=False)
    arm_cwds = {posture: args.cwd / ("cwd-" + posture) for posture in ("off", "on")}
    for cwd in arm_cwds.values():
        cwd.mkdir(parents=True, exist_ok=False)
    command = ["taskset", "-c", "0", str(binary), COMMON.TEST,
               "--ignored", "--exact", "--nocapture", "--test-threads=1"]
    write(args.out / "protocol.json", {
        "argv": command, "cwd": str(args.cwd.resolve()), "binary_sha256": expected_hash,
        "identity_sha256": digest(args.identity), "environment_sha256": digest(args.environment),
        "collector_sha256": digest(__file__), "common_collector_sha256": digest(COMMON_PATH),
        "warmup_order": list(ORDER), "pair_order": ["off/on", "on/off"] * 3,
        "source_patch_sha256": digest(args.source_patch),
        "timeout_s": args.timeout, "cpu": 0, "no_retry": True})
    schedule = [("warmup", i // 2, posture) for i, posture in enumerate(ORDER)]
    schedule += [("measured", pair, posture) for pair in range(6)
                 for posture in (("off", "on") if pair % 2 == 0 else ("on", "off"))]
    if args.development_only:
        schedule = [("development", 0, "on")]
    population_reference = None
    for ordinal, (kind, pair, posture) in enumerate(schedule):
        env = dict(environment)
        env.update(RUST_MIN_STACK="67108864", CARGO_PROFILE_RELEASE_LTO="false",
                   OPENWEPP_EXPERIMENT_OFES="1", OPENWEPP_EXPERIMENT_REPEATS="1",
                   OPENWEPP_EXPERIMENT_MEMORY="0", OPENWEPP_COMPACT_COST=posture)
        env.pop("OPENWEPP_EXPERIMENT_TRACE_DIR", None)
        log = args.out / f"{ordinal:02d}-{kind}-{pair}-{posture}.stdout"
        stderr = args.out / f"{ordinal:02d}-{kind}-{pair}-{posture}.stderr"
        receipt = {"ordinal": ordinal, "kind": kind, "pair": pair, "posture": posture,
                   "argv": command, "cwd": str(arm_cwds[posture].resolve()),
                   "log": str(log.resolve()), "stderr": str(stderr.resolve()),
                   "started_utc": datetime.now(timezone.utc).isoformat(),
                   "binary_sha256": expected_hash,
                   "source_patch_sha256": digest(args.source_patch),
                   "identity_sha256": digest(args.identity),
                   "environment_sha256": digest(args.environment),
                   "fixture_identity": admitted["common_identity"],
                   "environment_allowlist": {key: env[key] for key in (
                       "RUST_MIN_STACK", "CARGO_PROFILE_RELEASE_LTO",
                       "OPENWEPP_EXPERIMENT_OFES", "OPENWEPP_EXPERIMENT_REPEATS",
                       "OPENWEPP_EXPERIMENT_MEMORY", "OPENWEPP_COMPACT_COST")},
                   "valid": False}
        before = resource.getrusage(resource.RUSAGE_CHILDREN)
        started = time.monotonic_ns()
        failure = None
        child_wall_ns = None
        try:
            with log.open("xb") as output, stderr.open("xb") as errors:
                process = subprocess.run(command, cwd=arm_cwds[posture], env=env,
                                         stdout=output, stderr=errors,
                                         timeout=args.timeout, check=False)
            child_wall_ns = time.monotonic_ns() - started
            receipt["exit_code"] = process.returncode
            if process.returncode != 0:
                raise ValueError("runner nonzero exit")
            records = [json.loads(line.split("STAGE3_CONTROLLED_MECHANISM ", 1)[1])
                       for line in log.read_text().splitlines()
                       if "STAGE3_CONTROLLED_MECHANISM " in line]
            if len(records) != 1:
                raise ValueError("expected exactly one complete runner record")
            record = records[0]
            receipt["record"] = record
            if COMMON.exact_json(COMMON.record_identity(record, rules)) != COMMON.exact_json(admitted["common_identity"]):
                raise ValueError("protected output/control/closure identity mismatch")
            for field in ("carrier_counts", "lse_counts"):
                if COMMON.exact_json(record[field]) != COMMON.exact_json(admitted[field]):
                    raise ValueError("protected work count mismatch: " + field)
            accounting(record, posture)
            if posture == "on":
                population = COMMON.exact_json(record["compact_cost"]["populations"])
                if population_reference is not None and population != population_reference:
                    raise ValueError("runtime population changed across on arms")
                population_reference = population
            if digest(binary) != expected_hash:
                raise ValueError("executable changed during series")
            receipt["valid"] = True
        except Exception as error:
            failure = error
            receipt["error"] = f"{type(error).__name__}: {error}"
        after = resource.getrusage(resource.RUSAGE_CHILDREN)
        receipt.update(process_wall_ns=child_wall_ns,
                       process_cpu_s=(after.ru_utime + after.ru_stime - before.ru_utime - before.ru_stime),
                       ended_utc=datetime.now(timezone.utc).isoformat(),
                       log_sha256=digest(log), stderr_sha256=digest(stderr))
        write(args.out / f"{ordinal:02d}.json", receipt)
        print(json.dumps({k: receipt[k] for k in ("ordinal", "kind", "pair", "posture", "valid")}), flush=True)
        if failure is not None:
            raise failure
    write(args.out / "complete.json", {"status": "PASS", "processes": len(schedule),
                                       "measured_pairs": 0 if args.development_only else 6})


if __name__ == "__main__":
    main()
