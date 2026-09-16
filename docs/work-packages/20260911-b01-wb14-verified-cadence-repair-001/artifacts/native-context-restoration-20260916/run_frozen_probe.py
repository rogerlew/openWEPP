"""Record one explicitly selected file-only diagnostic; never launch a model."""
import argparse
import datetime
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import resource
import shutil
import subprocess
import time

HERE = Path(__file__).resolve().parent
EXPERIMENT = Path('/workdir/openwepp-experiments/b01-wb14-cadence')
SOURCE = EXPERIMENT / 'native-context-restoration-reader-20260916'
EXPORT = EXPERIMENT / 'corrected-recorder-export-20260916-1'
DEADLINE = datetime.datetime.fromisoformat('2026-09-16T06:32:15.218169+00:00')
SPEC = importlib.util.spec_from_file_location(
    'retained_snapshot', HERE.parent / 'execution-discretion-20260915/run-recorded.py')
SNAPSHOT = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(SNAPSHOT)


def digest(path):
    with path.open('rb') as stream:
        return hashlib.file_digest(stream, 'sha256').hexdigest()


def write(path, value):
    path.write_text(json.dumps(value, indent=2) + '\n')


def limits():
    resource.setrlimit(resource.RLIMIT_AS, (16 * 1024**3, 16 * 1024**3))
    resource.setrlimit(resource.RLIMIT_FSIZE, (1024**3, 1024**3))
    resource.setrlimit(resource.RLIMIT_CORE, (0, 0))


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('label')
    parser.add_argument('binary', type=Path)
    parser.add_argument('selector')
    args = parser.parse_args()
    if not args.selector.startswith('snow_stage3_v11_current_context_capture::'):
        raise SystemExit('Only the inspected diagnostic test namespace is allowed')
    receipt_path = HERE / (args.label + '.json')
    frozen = EXPERIMENT / (args.label + '.frozen')
    if receipt_path.exists() or frozen.exists():
        raise SystemExit('Attempt names are immutable; select a new recorded attempt')
    before, changes, patch = SNAPSHOT.snapshot(SOURCE)
    source_sha = hashlib.sha256(json.dumps(before, sort_keys=True).encode()).hexdigest()
    patch_sha = hashlib.sha256(patch.encode()).hexdigest()
    patch_path = HERE / ('source-' + patch_sha + '.patch')
    patch_path.write_text(patch)
    write(HERE / (args.label + '-source.json'), {
        'source': str(SOURCE), 'producing_base': str(SNAPSHOT.BASE),
        'tree_sha256': source_sha, 'entry_count': len(before),
        'patch': patch_path.name, 'patch_sha256': patch_sha,
        'entries': before, 'changes': changes})
    shutil.copy2(args.binary, frozen)
    frozen.chmod(0o555)
    binary_sha = digest(frozen)
    if digest(args.binary) != binary_sha:
        raise SystemExit('Binary changed while freezing')
    custody = json.loads((HERE / 'input-custody-before.json').read_text())
    for member in custody['entries']:
        path = Path(member['path'])
        if path.stat().st_size != member['bytes'] or digest(path) != member['sha256']:
            raise SystemExit('Input identity changed: ' + str(path))
    seed = HERE / 'pinned-original-seed.json'
    if digest(seed) != 'd1c467265dc5c718cc6d1a18fd1fbf71289e3ad5760967f9556f33109283f515':
        raise SystemExit('Pinned original seed changed')
    now = datetime.datetime.now(datetime.timezone.utc)
    timeout = min(900.0, (DEADLINE - now).total_seconds() - 180.0)
    if timeout <= 0:
        raise SystemExit('Preservation reserve reached')
    env = {k: v for k, v in os.environ.items() if not k.startswith('OPENWEPP_')}
    env.update(OPENWEPP_B01_MEMBER_EXPORT=str(EXPORT),
               OPENWEPP_B01_PINNED_SEED=str(seed), RUST_MIN_STACK='67108864')
    argv = [str(frozen), args.selector, '--exact', '--nocapture', '--test-threads=1']
    executed = ['/usr/bin/time', '-v', '-o', str(HERE / (args.label + '.resources')),
                '/usr/bin/timeout', '--signal=TERM', '--kill-after=5s',
                str(timeout) + 's', *argv]
    receipt = dict(label=args.label, start_utc=now.isoformat(), argv=argv,
                   executed_argv=executed, cwd=str(SOURCE), binary=str(frozen),
                   binary_sha256=binary_sha, source_tree_sha256=source_sha,
                   patch_sha256=patch_sha, seed_sha256=digest(seed),
                   input_manifest='input-custody-before.json',
                   environment={k: env[k] for k in
                       ('OPENWEPP_B01_MEMBER_EXPORT', 'OPENWEPP_B01_PINNED_SEED', 'RUST_MIN_STACK')},
                   timeout_seconds=timeout, address_space_bytes=16 * 1024**3,
                   output_file_limit_bytes=1024**3, automatic_retries=0)
    write(receipt_path, receipt)
    start = time.monotonic()
    with (HERE / (args.label + '.stdout')).open('wb') as out, \
            (HERE / (args.label + '.stderr')).open('wb') as err:
        result = subprocess.run(executed, cwd=SOURCE, env=env,
                                stdout=out, stderr=err, preexec_fn=limits)
    after = SNAPSHOT.snapshot(SOURCE)[0]
    receipt.update(exit_code=result.returncode, elapsed_seconds=time.monotonic() - start,
                   end_utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),
                   source_unchanged=before == after, frozen_binary_unchanged=digest(frozen) == binary_sha,
                   inputs_unchanged=all(Path(m['path']).stat().st_size == m['bytes']
                                        and digest(Path(m['path'])) == m['sha256']
                                        for m in custody['entries']))
    write(receipt_path, receipt)
    print(json.dumps({k: receipt[k] for k in ('label', 'exit_code', 'elapsed_seconds',
                    'source_unchanged', 'frozen_binary_unchanged', 'inputs_unchanged')}))
    raise SystemExit(result.returncode)


if __name__ == '__main__':
    main()
