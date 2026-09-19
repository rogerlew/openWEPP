"""One recorded preparation sequence; never selects or executes the original replay."""
import datetime as dt
import difflib
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import shutil
import subprocess

SOURCE = Path('/home/roger/openwepp-experiments/b01-wb14-grid40-source-20260919')
FROZEN = Path('/home/roger/openwepp-experiments/b01-wb14-lse-first-trial-source-20260918')
EVIDENCE = Path(__file__).parent
ROOT = Path('/workdir/openWEPP')
SNAPSHOT = ROOT / 'docs/work-packages/20260911-b01-wb14-verified-cadence-repair-001/artifacts/execution-discretion-20260915/run-recorded.py'
spec = importlib.util.spec_from_file_location('snapshot', SNAPSHOT)
module = importlib.util.module_from_spec(spec)
spec.loader.exec_module(module)
PREFIX = ['nix', 'develop', str(ROOT), '--command', 'env',
          'CARGO_TARGET_DIR=/home/roger/openwepp-experiments/cache/impl4-target',
          'TMPDIR=/home/roger/openwepp-experiments/tmp-grid40-impl4',
          'CARGO_BUILD_JOBS=2', 'RUST_MIN_STACK=67108864', 'cargo']
TEST_VECTOR = 'grid40_treatment_extension_vectors_cover_b21_and_b40'


def sha(path):
    with Path(path).open('rb') as stream:
        return hashlib.file_digest(stream, 'sha256').hexdigest()


def snapshot():
    entries = module.snapshot(SOURCE)[0]
    return entries, hashlib.sha256(json.dumps(entries, sort_keys=True).encode()).hexdigest()


def save(name, value):
    with (EVIDENCE / name).open('x') as stream:
        stream.write(json.dumps(value, indent=2) + '\n')


def patch_from_frozen(name):
    entries, digest = snapshot()
    patch = []
    changes = []
    for rel in sorted(set(entries) | set(module.snapshot(FROZEN)[0])):
        before = (FROZEN / rel).read_bytes() if (FROZEN / rel).is_file() else b''
        after = (SOURCE / rel).read_bytes() if (SOURCE / rel).is_file() else b''
        if before != after:
            changes.append(rel)
            patch.extend(difflib.unified_diff(before.decode().splitlines(True),
                                             after.decode().splitlines(True),
                                             fromfile='a/' + rel, tofile='b/' + rel))
    with (EVIDENCE / (name + '.patch')).open('x') as stream:
        stream.write(''.join(patch))
    save(name + '.json', {'utc': dt.datetime.now(dt.timezone.utc).isoformat(),
                         'source': str(SOURCE), 'source_sha256': digest,
                         'source_entries': entries, 'changed_paths': changes,
                         'base': str(FROZEN), 'patch_sha256': sha(EVIDENCE / (name + '.patch'))})
    return digest


def run(name, arguments, expected=0):
    before, digest = snapshot()
    argv = PREFIX + [arguments[0], '--manifest-path', str(SOURCE / 'Cargo.toml'),
                     *arguments[1:]]
    env = {key: value for key, value in os.environ.items()
           if not key.startswith('OPENWEPP_') and key not in
           {'LLVM_PROFILE_FILE', 'LLVM_PROFILE_MERGE_POOL_SIZE', 'LD_PRELOAD', 'LD_AUDIT'}}
    receipt = {'start_utc': dt.datetime.now(dt.timezone.utc).isoformat(),
               'argv': argv, 'cwd': str(SOURCE), 'source_sha256': digest,
               'expected_exit': expected, 'original_case_execution': False}
    with (EVIDENCE / (name + '.stdout')).open('x') as stdout, (EVIDENCE / (name + '.stderr')).open('x') as stderr:
        result = subprocess.run(argv, cwd=SOURCE, env=env, stdout=stdout, stderr=stderr, timeout=300)
    receipt.update(end_utc=dt.datetime.now(dt.timezone.utc).isoformat(),
                   exit=result.returncode, source_unchanged=snapshot()[0] == before)
    save(name + '.json', receipt)
    print(json.dumps({'name': name, 'exit': result.returncode,
                      'source_sha256': digest, 'source_unchanged': receipt['source_unchanged']}), flush=True)
    if result.returncode != expected or not receipt['source_unchanged']:
        raise RuntimeError('preparation command failed: ' + name)


def main():
    if dt.datetime.now(dt.timezone.utc) >= dt.datetime.fromisoformat('2026-09-19T08:35:00+00:00'):
        raise RuntimeError('preparation reserve reached')
    solve = SOURCE / 'crates/openwepp-land-surface-energy/src/solver_covered_solve.rs'
    treatment = solve.read_text()
    pairs = [('for exponent in covered_strict_decrease_exponents() {',
              'for exponent in 0..=MAX_BACKTRACKING_HALVINGS {'),
             ('+ covered_strict_decrease_failure_increment(),',
              '+ MAX_BACKTRACKING_HALVINGS,')]
    control = treatment
    for old, new in pairs:
        if control.count(old) != 1:
            raise RuntimeError('unexpected actual-callsite replacement count')
        control = control.replace(old, new)
    save('root-terminal-start.json', {'utc': dt.datetime.now(dt.timezone.utc).isoformat(),
                                     'initial_t_source_sha256': snapshot()[1],
                                     'control_semantics': 'Treatment helper/envelope remains40; actual covered search and exhaustion hard20; no original measurement'})
    try:
        solve.write_text(control)
        patch_from_frozen('root-terminal-c')
        run('root-terminal-c-preservation', ['test', '-p', 'openwepp-land-surface-energy',
                                           '--', '--test-threads=1', '--skip', TEST_VECTOR])
        run('root-terminal-c-expected-red', ['test', '-p', 'openwepp-land-surface-energy',
                                           TEST_VECTOR, '--', '--test-threads=1'], expected=101)
    finally:
        solve.write_text(treatment)
    with (EVIDENCE / 'root-terminal-c-to-t.patch').open('x') as stream:
        stream.write(''.join(difflib.unified_diff(control.splitlines(True), treatment.splitlines(True),
                     fromfile='a/crates/openwepp-land-surface-energy/src/solver_covered_solve.rs',
                     tofile='b/crates/openwepp-land-surface-energy/src/solver_covered_solve.rs')))
    patch_from_frozen('root-terminal-t')
    run('root-terminal-t-default-tests', ['test', '-p', 'openwepp-land-surface-energy', '--', '--test-threads=1'])
    run('root-terminal-t-test-support-tests', ['test', '-p', 'openwepp-land-surface-energy', '--features', 'test-support', '--', '--test-threads=1'])
    run('root-terminal-t-fmt', ['fmt', '--all', '--', '--check'])
    run('root-terminal-t-default-check', ['check', '-p', 'openwepp-land-surface-energy'])
    run('root-terminal-t-test-support-check', ['check', '-p', 'openwepp-land-surface-energy', '--features', 'test-support', '--tests'])
    for feature, arguments in [('default', []), ('test-support', ['--features', 'test-support'])]:
        run('root-terminal-t-clippy-' + feature,
            ['clippy', '-p', 'openwepp-land-surface-energy', '--all-targets', *arguments,
             '--message-format=json', '--', '-D', 'warnings'], expected=101)
    run('root-terminal-t-binary', ['test', '-p', 'openwepp-land-surface-energy', '--lib', '--no-run', '--message-format=json'])
    binaries = []
    for line in (EVIDENCE / 'root-terminal-t-binary.stdout').read_text().splitlines():
        if line.startswith('{'):
            row = json.loads(line)
            if row.get('reason') == 'compiler-artifact' and row.get('executable') and row['target']['name'] == 'openwepp_land_surface_energy':
                binaries.append(Path(row['executable']))
    if len(binaries) != 1:
        raise RuntimeError('test binary identity ambiguous')
    frozen_binary = EVIDENCE / 'root-terminal-grid40-tests.frozen'
    if frozen_binary.exists():
        raise RuntimeError('refuse to overwrite retained binary')
    shutil.copy2(binaries[0], frozen_binary)
    frozen_binary.chmod(0o444)
    save('root-terminal-binary.json', {'utc': dt.datetime.now(dt.timezone.utc).isoformat(),
                                     'source_sha256': snapshot()[1], 'built_binary': str(binaries[0]),
                                     'frozen_binary': str(frozen_binary), 'binary_sha256': sha(frozen_binary),
                                     'bytes': frozen_binary.stat().st_size,
                                     'original_replay_executions': 0, 'launch_approval': False})


if __name__ == '__main__':
    main()
