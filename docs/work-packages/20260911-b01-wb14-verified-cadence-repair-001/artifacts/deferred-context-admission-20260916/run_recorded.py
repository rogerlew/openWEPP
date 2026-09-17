"""Record one explicitly supplied bounded command; no selection, retry, or acceptance logic."""
import argparse
import datetime
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import resource
import signal
import subprocess
import time

HERE = Path(__file__).resolve().parent
SOURCE = Path('/workdir/openwepp-experiments/b01-wb14-cadence/deferred-context-admission-20260916')
SPEC = importlib.util.spec_from_file_location('retained', HERE.parent / 'execution-discretion-20260915/run-recorded.py')
RETAINED = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(RETAINED)
DEADLINE = datetime.datetime.fromisoformat('2026-09-17T00:17:45+00:00')


def sha(path):
    with path.open('rb') as stream:
        return hashlib.file_digest(stream, 'sha256').hexdigest()


def save(path, value):
    path.write_text(json.dumps(value, indent=2) + '\n')


def limits():
    resource.setrlimit(resource.RLIMIT_AS, (16 * 1024**3, 16 * 1024**3))
    resource.setrlimit(resource.RLIMIT_FSIZE, (1024**3, 1024**3))
    resource.setrlimit(resource.RLIMIT_CORE, (0, 0))


def main():
    global SOURCE
    parser = argparse.ArgumentParser()
    parser.add_argument('label')
    parser.add_argument('--source', type=Path, default=SOURCE)
    parser.add_argument('--timeout', type=int, default=900)
    parser.add_argument('--binary', type=Path)
    parser.add_argument('--env', action='append', default=[])
    parser.add_argument('command', nargs=argparse.REMAINDER)
    args = parser.parse_args()
    SOURCE = args.source.resolve()
    if SOURCE.name not in {"deferred-context-admission-20260916", "native-context-restoration-reader-20260916"}:
        parser.error("source must be bounded R or S")
    command = args.command
    if command and command[0] == '--':
        command = command[1:]
    if not command:
        parser.error('explicit command required')
    receipt_path = HERE / (args.label + '.json')
    if receipt_path.exists():
        raise SystemExit('Attempt identity already exists; choose a distinct name')
    before, changes, patch = RETAINED.snapshot(SOURCE)
    patch_sha = hashlib.sha256(patch.encode()).hexdigest()
    patch_path = HERE / ('source-' + patch_sha + '.patch')
    if not patch_path.exists():
        patch_path.write_text(patch)
    source_identity = dict(source=str(SOURCE), base=str(RETAINED.BASE), entries=before,
                           changes=changes, tree_sha256=hashlib.sha256(json.dumps(before, sort_keys=True).encode()).hexdigest(),
                           patch=patch_path.name, patch_sha256=patch_sha)
    save(HERE / (args.label + '-source.json'), source_identity)
    custody_path = HERE.parent / 'native-context-restoration-20260916/input-custody-before.json'
    custody = json.loads(custody_path.read_text())
    assert all(Path(m['path']).stat().st_size == m['bytes'] and sha(Path(m['path'])) == m['sha256'] for m in custody['entries'])
    env = {k: v for k, v in os.environ.items() if not k.startswith('OPENWEPP_')}
    explicit_env = dict(pair.split('=', 1) for pair in args.env)
    env.update(explicit_env)
    pinned_files = [Path(__file__).resolve(), Path('/workdir/openWEPP/flake.nix'), Path('/workdir/openWEPP/flake.lock')]
    if 'OPENWEPP_B01_MEMBER_EXPORT' in explicit_env:
        summary = Path(explicit_env['OPENWEPP_B01_MEMBER_EXPORT']) / 'summary.json'
        assert sha(summary) == custody['accepted_summary_sha256']
        pinned_files.append(summary)
    if 'OPENWEPP_B01_PINNED_SEED' in explicit_env:
        seed = Path(explicit_env['OPENWEPP_B01_PINNED_SEED'])
        assert sha(seed) == 'd1c467265dc5c718cc6d1a18fd1fbf71289e3ad5760967f9556f33109283f515'
        pinned_files.append(seed)
    if 'OPENWEPP_B01_ORIGINAL_CLIMATE' in explicit_env:
        climate = Path(explicit_env['OPENWEPP_B01_ORIGINAL_CLIMATE'])
        assert sha(climate) == '5af208853205300ab2cc606070407fccccba3b4c8d770f5e242613033fa2508a'
        pinned_files.append(climate)
    if 'OPENWEPP_B01_GSI_AUTH_EXPECTED_JSON' in explicit_env:
        pinned_files.append(Path(explicit_env['OPENWEPP_B01_GSI_AUTH_EXPECTED_JSON']))
    for name in ('OPENWEPP_B01_RETAINED_SOIL_TRIAL_PRIMITIVES', 'OPENWEPP_B01_AUTHENTICATED_GSI_PREIMAGES', 'OPENWEPP_B01_RETAINED_SUBSLAB_PRIMITIVES'):
        if name in explicit_env:
            pinned_files.append(Path(explicit_env[name]))
    pinned_hashes = {str(path): sha(path) for path in pinned_files}
    now = datetime.datetime.now(datetime.timezone.utc)
    timeout = min(args.timeout, (DEADLINE - now).total_seconds() - 180)
    if timeout <= 0:
        raise SystemExit('Preservation reserve reached')
    executed = ['/usr/bin/time', '-v', '-o', str(HERE / (args.label + '.resources')), *command]
    receipt = dict(label=args.label, argv=command, executed_argv=executed, cwd=str(SOURCE),
                   start_utc=now.isoformat(), source_tree_sha256=source_identity['tree_sha256'],
                   patch_sha256=patch_sha, source_receipt=args.label + '-source.json',
                   input_manifest=str(custody_path), input_manifest_sha256=sha(custody_path),
                   environment=explicit_env, pinned_files=pinned_hashes, timeout_seconds=timeout, address_space_bytes=16 * 1024**3,
                   output_file_limit_bytes=1024**3, automatic_retries=0,
                   binary=str(args.binary) if args.binary else None,
                   binary_sha256=sha(args.binary) if args.binary else None)
    save(receipt_path, receipt)
    start = time.monotonic()
    with (HERE / (args.label + '.stdout')).open('wb') as out, (HERE / (args.label + '.stderr')).open('wb') as err:
        process = subprocess.Popen(executed, cwd=SOURCE, env=env, stdout=out, stderr=err,
                                   preexec_fn=limits, start_new_session=True)
        receipt['process_group'] = process.pid
        save(receipt_path, receipt)
        try:
            code = process.wait(timeout=timeout)
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
    receipt.update(exit_code=code, timed_out=timed_out, elapsed_seconds=time.monotonic() - start,
                   end_utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),
                   source_unchanged=RETAINED.snapshot(SOURCE)[0] == before,
                   binary_unchanged=sha(args.binary) == receipt['binary_sha256'] if args.binary else None,
                   pinned_files_unchanged=all(sha(Path(path)) == digest for path, digest in pinned_hashes.items()),
                   inputs_unchanged=all(Path(m['path']).stat().st_size == m['bytes'] and sha(Path(m['path'])) == m['sha256'] for m in custody['entries']))
    save(receipt_path, receipt)
    print(json.dumps({key: receipt[key] for key in ('label', 'exit_code', 'elapsed_seconds', 'source_unchanged', 'inputs_unchanged')}))
    raise SystemExit(code)


if __name__ == '__main__':
    main()
