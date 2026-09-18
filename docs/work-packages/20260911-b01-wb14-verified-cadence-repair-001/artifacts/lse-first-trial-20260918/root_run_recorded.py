"""Record one explicitly chosen bounded command; no retry or acceptance policy."""
import argparse
import datetime as dt
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import resource
import signal
import shutil
import subprocess
import time

HERE = Path(__file__).resolve().parent
SOURCE = Path('/home/roger/openwepp-experiments/b01-wb14-lse-first-trial-source-20260918')
LOGS = Path('/home/roger/openwepp-experiments/b01-wb14-lse-first-trial-evidence-20260918')
DEADLINE = dt.datetime.fromisoformat('2026-09-19T00:48:45.275000+00:00')
SPEC = importlib.util.spec_from_file_location(
    'snapshot_tool',
    Path('/workdir/openWEPP/docs/work-packages/20260911-b01-wb14-verified-cadence-repair-001/artifacts/execution-discretion-20260915/run-recorded.py'),
)
SNAPSHOT = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(SNAPSHOT)


def sha(path):
    with Path(path).open('rb') as stream:
        return hashlib.file_digest(stream, 'sha256').hexdigest()


def save(path, value):
    path.write_text(json.dumps(value, indent=2) + '\n')


def limits():
    resource.setrlimit(resource.RLIMIT_AS, (16 * 1024**3, 16 * 1024**3))
    resource.setrlimit(resource.RLIMIT_FSIZE, (1024**3, 1024**3))
    resource.setrlimit(resource.RLIMIT_CORE, (0, 0))


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('label')
    parser.add_argument('--timeout', type=int, required=True)
    parser.add_argument('--binary', type=Path)
    parser.add_argument('command', nargs=argparse.REMAINDER)
    args = parser.parse_args()
    command = args.command[1:] if args.command[:1] == ['--'] else args.command
    if not command or args.timeout <= 0:
        parser.error('explicit command and positive timeout required; options precede label')
    LOGS.mkdir(exist_ok=True)
    receipt_path = LOGS / (args.label + '.json')
    if receipt_path.exists():
        raise SystemExit('Attempt identity already exists')
    before = SNAPSHOT.snapshot(SOURCE)[0]
    source_hash = hashlib.sha256(json.dumps(before, sort_keys=True).encode()).hexdigest()
    freeze = json.loads((LOGS / 'root-freeze.json').read_text())
    assert source_hash == freeze['source_sha256']
    assert all(sha(path) == digest for path, digest in freeze['external_build_inputs'].items())
    assert all(str((SOURCE / name).resolve()) == target for name, target in freeze['support_links'].items())
    source_receipt = LOGS / 'root-freeze.json'
    pinned = {str(path): sha(path) for path in (Path(__file__), Path('/workdir/openWEPP/flake.nix'), Path('/workdir/openWEPP/flake.lock'), Path('/workdir/openWEPP/tools/dev/openwepp-env'))}
    timeout = min(args.timeout, 3600, (DEADLINE - dt.datetime.now(dt.timezone.utc)).total_seconds() - 900)
    if timeout <= 0:
        raise SystemExit('Review/preservation reserve reached')
    executed = ['/usr/bin/time', '-v', '-o', str(LOGS / (args.label + '.resources')), *command]
    receipt = dict(label=args.label, argv=command, executed_argv=executed, cwd=str(SOURCE), source_tree_sha256=source_hash,
                   source_receipt=str(source_receipt), source_receipt_sha256=sha(source_receipt), external_build_inputs=freeze['external_build_inputs'], support_links=freeze['support_links'], pinned_files=pinned, start_utc=dt.datetime.now(dt.timezone.utc).isoformat(),
                   timeout_seconds=timeout, address_space_bytes=16 * 1024**3, output_file_limit_bytes=1024**3,
                   automatic_retries=0, binary=str(args.binary) if args.binary else None,
                   binary_sha256=sha(args.binary) if args.binary else None,
                   environment_policy='Inherited environment excluding OPENWEPP_*; explicit env assignments in argv',
                   input_scope='Source-defined local component fixtures only; no original corpus inputs')
    save(receipt_path, receipt)
    env = {k: v for k, v in os.environ.items() if not k.startswith('OPENWEPP_')}
    env['TMPDIR'] = str(LOGS / 'tmp')
    env['RUST_BACKTRACE'] = '1'
    receipt['explicit_environment'] = {'TMPDIR': env['TMPDIR'], 'RUST_BACKTRACE': '1'}
    assert shutil.disk_usage(LOGS).free > 20 * 1024**3
    start = time.monotonic()
    with (LOGS / (args.label + '.stdout')).open('xb') as out, (LOGS / (args.label + '.stderr')).open('xb') as err:
        process = subprocess.Popen(executed, cwd=SOURCE, env=env, stdout=out, stderr=err, preexec_fn=limits, start_new_session=True)
        receipt['process_group'] = process.pid
        save(receipt_path, receipt)
        try:
            disk_stop = False
            while process.poll() is None:
                if time.monotonic() - start >= timeout:
                    raise subprocess.TimeoutExpired(command, timeout)
                if shutil.disk_usage(LOGS).free < 10 * 1024**3:
                    disk_stop = True
                    raise subprocess.TimeoutExpired(command, timeout)
                try:
                    process.wait(timeout=min(1, max(0.01, timeout - (time.monotonic() - start))))
                except subprocess.TimeoutExpired:
                    pass
            code = process.returncode
            timed_out = False
        except subprocess.TimeoutExpired:
            timed_out = True
            os.killpg(process.pid, signal.SIGTERM)
            try:
                process.wait(timeout=5)
            except subprocess.TimeoutExpired:
                os.killpg(process.pid, signal.SIGKILL)
                process.wait()
            code = 124
    receipt.update(exit_code=code, timed_out=timed_out, disk_stop=disk_stop, elapsed_seconds=time.monotonic() - start,
                   end_utc=dt.datetime.now(dt.timezone.utc).isoformat(), source_unchanged=SNAPSHOT.snapshot(SOURCE)[0] == before,
                   binary_unchanged=sha(args.binary) == receipt['binary_sha256'] if args.binary else None,
                   pinned_files_unchanged=all(sha(path) == digest for path, digest in pinned.items()),
                   external_build_inputs_unchanged=all(sha(path) == digest for path, digest in freeze['external_build_inputs'].items()),
                   support_links_unchanged=all(str((SOURCE / name).resolve()) == target for name, target in freeze['support_links'].items()),
                   stdout_sha256=sha(LOGS / (args.label + '.stdout')), stderr_sha256=sha(LOGS / (args.label + '.stderr')))
    save(receipt_path, receipt)
    print(json.dumps({key: receipt[key] for key in ('label', 'exit_code', 'elapsed_seconds', 'source_unchanged', 'binary_unchanged')}))
    raise SystemExit(code)


if __name__ == '__main__':
    main()
