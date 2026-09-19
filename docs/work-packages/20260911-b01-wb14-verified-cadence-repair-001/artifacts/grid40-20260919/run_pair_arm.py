"""Single-use, frozen Grid40 arm launcher. Never retries or selects a solver policy."""
import argparse
import datetime as dt
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import resource
import shutil
import signal
import subprocess
import sys
import time
import uuid

ROOT = Path('/workdir/openWEPP')
SOURCE = Path('/home/roger/openwepp-experiments/b01-wb14-grid40-source-20260919')
RUNS = Path('/home/roger/openwepp-experiments/b01-wb14-grid40-run-evidence-20260919')
FREEZE = RUNS / 'pair-freeze.json'
DEADLINE = dt.datetime.fromisoformat('2026-09-19T09:04:00+00:00')
RESERVE = 1200
ARM_SECONDS = 120
OUTPUT_BYTES = 1024**3
# Eight declared child-output files, each bounded below one eighth of the total.
# The remaining two MiB cover receipts. This stricter mechanical subdivision is
# part of the prospective protocol, not a post-outcome adjustment.
FILE_BYTES = (OUTPUT_BYTES - 2 * 1024**2) // 8
INPUT_SHA = '527208c3e5ad07f1744f1bd92c9e4ac17cf7efd9469968a97d7964c5db482dfd'
SNAPSHOT_FILE = ROOT / 'docs/work-packages/20260911-b01-wb14-verified-cadence-repair-001/artifacts/execution-discretion-20260915/run-recorded.py'


def require(condition, message):
    if not condition:
        raise RuntimeError(message)


def sha(path):
    with Path(path).open('rb') as stream:
        return hashlib.file_digest(stream, 'sha256').hexdigest()


def save(path, data):
    encoded = json.dumps(data, indent=2, allow_nan=False).encode() + b'\n'
    require(len(encoded) < 1024**2, 'receipt exceeds reserved size')
    temp = path.with_suffix(path.suffix + '.tmp')
    temp.write_bytes(encoded)
    temp.replace(path)


def source_hash():
    spec = importlib.util.spec_from_file_location('grid40_snapshot', SNAPSHOT_FILE)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    entries = module.snapshot(SOURCE)[0]
    return hashlib.sha256(json.dumps(entries, sort_keys=True).encode()).hexdigest()


def verify(freeze):
    require(freeze['schema'] == 'openwepp.grid40.pair-freeze.v1', 'freeze schema')
    require(freeze['prerequisites'] == {'correctness': 'GO', 'qa': 'GO', 'focused_checks': 'PASS'}, 'prerequisites incomplete')
    require(sha(__file__) == freeze['recorder_sha256'], 'recorder drift')
    require(sha(SNAPSHOT_FILE) == freeze['snapshot_tool_sha256'], 'snapshot tool drift')
    require(source_hash() == freeze['source_sha256'], 'source drift')
    require(sha(freeze['binary']) == freeze['binary_sha256'], 'binary drift')
    require(sha(freeze['input']) == INPUT_SHA == freeze['input_sha256'], 'input drift')
    for path, digest in freeze['bound_files'].items():
        require(sha(path) == digest, 'bound file drift: ' + path)
    require(len(freeze['external_build_inputs']) == 19, 'external build binding count')
    for path, digest in freeze['external_build_inputs'].items():
        require(sha(path) == digest, 'external build drift: ' + path)
    require(len(freeze['support_links']) == 5, 'support link count')
    for path, target in freeze['support_links'].items():
        require(str((SOURCE / path).resolve()) == target, 'support link drift: ' + path)
    boundary = freeze['write_boundary']
    require(boundary['source_sha256'] == freeze['source_sha256'] and
            boundary['binary_sha256'] == freeze['binary_sha256'], 'write boundary identity')
    require(boundary['entry_arguments'] == freeze['binary_arguments'], 'write boundary entry')
    require(boundary['no_subprocess'] is True and boundary['no_other_destinations'] is True, 'incomplete write boundary')
    require(sha(boundary['review']) == boundary['review_sha256'], 'write boundary review drift')


def environment(freeze, arm, directory, permitted):
    cleared = {'LLVM_PROFILE_FILE', 'LLVM_PROFILE_MERGE_POOL_SIZE', 'GCOV_PREFIX',
               'GCOV_PREFIX_STRIP', 'RUSTFLAGS', 'RUSTDOCFLAGS', 'CARGO_ENCODED_RUSTFLAGS',
               'LD_PRELOAD', 'LD_AUDIT', 'RUST_TEST_THREADS'}
    env = {key: value for key, value in os.environ.items()
           if not key.startswith('OPENWEPP_') and key not in cleared}
    explicit = {key: value.format(arm=arm, output=str(directory), input=freeze['input'])
                for key, value in freeze['environment'].items()}
    require(not (set(explicit) & cleared), 'forbidden profiling/build environment')
    env.update(explicit)
    for key, filename in freeze['write_boundary']['output_environment'].items():
        require(filename in permitted, 'unlisted writer filename')
        require(key in explicit and Path(explicit[key]).resolve() == (directory / filename).resolve(),
                'writer environment escapes frozen output path: ' + key)
    env['RUST_MIN_STACK'] = str(64 * 1024**2)
    return env


def limits():
    resource.setrlimit(resource.RLIMIT_AS, (16 * 1024**3, 16 * 1024**3))
    resource.setrlimit(resource.RLIMIT_STACK, (64 * 1024**2, 64 * 1024**2))
    resource.setrlimit(resource.RLIMIT_FSIZE, (FILE_BYTES, FILE_BYTES))
    resource.setrlimit(resource.RLIMIT_CORE, (0, 0))


def output_inventory(directory, permitted):
    total = 0
    for path in directory.rglob('*'):
        require(not path.is_symlink(), 'unexpected output symlink')
        if path.is_file():
            require(path.relative_to(directory).as_posix() in permitted, 'unexpected output file: ' + str(path))
            total += path.stat().st_size
    require(total <= OUTPUT_BYTES, 'aggregate output budget')
    return total


def stop(process):
    if process.poll() is None:
        os.killpg(process.pid, signal.SIGKILL)
        process.wait()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('arm', choices=['baseline', 'treatment'])
    args = parser.parse_args()
    freeze = json.loads(FREEZE.read_bytes())
    verify(freeze)
    require((DEADLINE - dt.datetime.now(dt.timezone.utc)).total_seconds() > RESERVE + ARM_SECONDS, 'expired launch / preservation reserve')
    require(shutil.disk_usage(RUNS).free >= 20 * 1024**3, 'insufficient output disk')
    mem = dict(line.split(':', 1) for line in Path('/proc/meminfo').read_text().splitlines())
    require(int(mem['MemAvailable'].split()[0]) * 1024 >= 16 * 1024**3, 'insufficient currently available host memory')
    if args.arm == 'treatment':
        baseline = json.loads((RUNS / 'baseline' / 'receipt.json').read_bytes())
        comparison = json.loads((RUNS / 'baseline-comparison.json').read_bytes())
        admission = json.loads((RUNS / 'baseline-admission.json').read_bytes())
        require(admission['disposition'] == 'MATCHED_BASELINE_GO', 'baseline sidecar/cost admission missing')
        require(admission['baseline_receipt_sha256'] == sha(RUNS / 'baseline' / 'receipt.json'), 'baseline admission binding')
        require(admission['comparison_sha256'] == sha(RUNS / 'baseline-comparison.json'), 'comparison admission binding')
        require(baseline['process_exit'] == 0 and baseline['integrity_unchanged'] and baseline['stop_reason'] is None, 'baseline process invalid')
        require(comparison['ordinary_record_values_and_order_match'] is True and comparison['ordinary_record_count'] == 95, 'baseline mismatch')
        require(comparison['actual_trace_sha256'] == sha(RUNS / 'baseline' / freeze['trace_filename']), 'baseline comparison binding')
        require(baseline['freeze_sha256'] == sha(FREEZE), 'pair freeze changed')
        require(baseline['elapsed_seconds'] <= ARM_SECONDS, 'baseline exceeded wall budget')
    directory = RUNS / args.arm
    permitted = set(freeze['output_filenames'])
    require(len(freeze['output_filenames']) == len(permitted) == 8 and
            {'stdout', 'stderr'} <= permitted, 'output partition must contain eight unique slots')
    require(all(Path(name).name == name for name in permitted), 'output filenames must be local')
    require(not ({'receipt.json', 'receipt.json.tmp'} & permitted), 'child output aliases parent receipt')
    env = environment(freeze, args.arm, directory, permitted)
    # Atomic claim: even a failed spawn consumes this arm identity; no replacement.
    directory.mkdir()
    receipt_path = directory / 'receipt.json'
    permitted |= {'receipt.json', 'receipt.json.tmp'}
    argv = [freeze['binary'], *freeze['binary_arguments']]
    receipt = dict(schema='openwepp.grid40.arm-receipt.v1', arm=args.arm,
                   freeze_sha256=sha(FREEZE), argv=argv, environment=freeze['environment'],
                   source_sha256=freeze['source_sha256'], binary_sha256=freeze['binary_sha256'],
                   input_sha256=INPUT_SHA, start_utc=dt.datetime.now(dt.timezone.utc).isoformat(),
                   limits=dict(seconds=ARM_SECONDS, address_space=16*1024**3, stack=64*1024**2,
                               output=OUTPUT_BYTES, per_file=FILE_BYTES), automatic_retries=0,
                   stop_reason=None, process_exit=None, integrity_unchanged=False)
    save(receipt_path, receipt)
    started = time.monotonic()
    process = None
    try:
        with (directory / 'stdout').open('xb') as stdout, (directory / 'stderr').open('xb') as stderr:
            process = subprocess.Popen(argv, cwd=SOURCE, env=env, stdout=stdout, stderr=stderr,
                                       start_new_session=True, preexec_fn=limits)
            receipt['process_group'] = process.pid
            save(receipt_path, receipt)
            while process.poll() is None:
                elapsed = time.monotonic() - started
                if elapsed >= ARM_SECONDS:
                    receipt['stop_reason'] = 'process_wall_budget'
                    stop(process)
                    break
                output_inventory(directory, permitted)
                if shutil.disk_usage(RUNS).free < 10 * 1024**3:
                    receipt['stop_reason'] = 'host_disk_stop'
                    stop(process)
                    break
                try:
                    process.wait(timeout=min(0.05, max(0.001, ARM_SECONDS - elapsed)))
                except subprocess.TimeoutExpired:
                    pass
            receipt['process_exit'] = process.returncode
    except BaseException as error:
        receipt['stop_reason'] = 'recorder_or_spawn_failure: ' + repr(error)
        if process is not None:
            stop(process)
            receipt['process_exit'] = process.returncode
    finally:
        receipt['elapsed_seconds'] = time.monotonic() - started
        receipt['end_utc'] = dt.datetime.now(dt.timezone.utc).isoformat()
        try:
            verify(freeze)
            require(sha(FREEZE) == receipt['freeze_sha256'], 'freeze drift during arm')
            receipt['output_bytes'] = output_inventory(directory, permitted)
            receipt['integrity_unchanged'] = True
        except BaseException as error:
            receipt['integrity_error'] = repr(error)
        receipt['files'] = {p.name: {'bytes': p.stat().st_size, 'sha256': sha(p)}
                            for p in directory.iterdir() if p.is_file() and not p.is_symlink() and p.name not in {'receipt.json', 'receipt.json.tmp'}}
        save(receipt_path, receipt)
    print(json.dumps(receipt, indent=2))
    require(receipt['process_exit'] == 0 and receipt['stop_reason'] is None and receipt['integrity_unchanged'], 'arm incomplete/invalid; identity consumed')


def entry():
    try:
        main()
    except BaseException as error:
        if isinstance(error, SystemExit) and error.code == 0:
            raise
        # Independent of the arm claim: preserve expired, malformed, reused and
        # admission-refused launches without replacing any existing arm receipt.
        refusals = RUNS / 'launcher-refusals'
        refusals.mkdir(parents=True, exist_ok=True)
        refusal = refusals / (dt.datetime.now(dt.timezone.utc).strftime('%Y%m%dT%H%M%S') + '-' + uuid.uuid4().hex + '.json')
        save(refusal, {'evidence_class': 'Launcher refusal/failure; inspect arm receipt to determine whether a child started',
                      'utc': dt.datetime.now(dt.timezone.utc).isoformat(),
                      'arguments': sys.argv[1:], 'error': repr(error),
                      'freeze_sha256': sha(FREEZE) if FREEZE.is_file() else None,
                      'existing_arm_receipts': {arm: str(RUNS / arm / 'receipt.json')
                                               for arm in ('baseline', 'treatment')
                                               if (RUNS / arm / 'receipt.json').is_file()}})
        raise


if __name__ == '__main__':
    entry()
