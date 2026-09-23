"""Record one explicitly chosen bounded command; no retry or acceptance policy."""
import argparse
import datetime as dt
import difflib
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import re
import resource
import signal
import subprocess
import time

HERE = Path(__file__).resolve().parent
SOURCE = Path('/home/roger/openwepp-experiments/cold-canopy-m1-20260920')
BASE = Path('/home/roger/openwepp-experiments/b01-wb14-observer-source-cut02-20260918')
LOGS = Path('/workdir/openWEPP/docs/work-packages/20260920-cold-canopy-m1-001/artifacts')
DEADLINE = dt.datetime.fromisoformat('2026-09-23T06:56:00+00:00')
SPEC = importlib.util.spec_from_file_location(
    'snapshot_tool',
    Path('/workdir/openWEPP/docs/work-packages/20260911-b01-wb14-verified-cadence-repair-001/artifacts/execution-discretion-20260915/run-recorded.py'),
)
if hashlib.sha256(Path(SPEC.origin).read_bytes()).hexdigest() != '90fd8f6d218ac1e2234e527c2700a1e50eafea6631f78b62107ec484fe2f9e7d':
    raise SystemExit('Snapshot helper differs from the reviewed helper')
SNAPSHOT = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(SNAPSHOT)
MATERIAL_ENVIRONMENT = (
    'PATH', 'HOME', 'USER', 'LOGNAME', 'SHELL', 'LANG', 'LC_ALL', 'LC_CTYPE',
    'TMPDIR', 'TERM', 'NIX_PATH', 'NIX_PROFILES', 'NIX_CONFIG', 'NIX_SSL_CERT_FILE',
    'IN_NIX_SHELL', 'SSL_CERT_FILE', 'SSL_CERT_DIR', 'XDG_CONFIG_HOME',
    'XDG_CACHE_HOME', 'XDG_DATA_DIRS', 'XDG_RUNTIME_DIR', 'LD_LIBRARY_PATH',
    'CARGO_HOME', 'CARGO_TARGET_DIR', 'CARGO_BUILD_JOBS', 'CARGO_INCREMENTAL',
    'RUSTFLAGS', 'CARGO_ENCODED_RUSTFLAGS', 'RUSTDOCFLAGS', 'RUSTUP_HOME',
    'RUSTUP_TOOLCHAIN', 'RUST_MIN_STACK', 'RUSTC', 'RUSTDOC', 'RUSTC_WRAPPER',
    'RUSTC_WORKSPACE_WRAPPER', 'CC', 'CXX', 'AR', 'CFLAGS', 'CXXFLAGS',
    'LDFLAGS', 'PKG_CONFIG_PATH', 'SOURCE_DATE_EPOCH',
)

# Existing package Critical regression, explicitly retained at 900 seconds by
# the adopted provider/parent authorization. This is not a generic cap override.
REQUIRED_FULL_VALIDATION_ARGV = [
    'nix', 'develop', '/workdir/openWEPP', '--command', 'env',
    'CARGO_TARGET_DIR=/tmp/openwepp-cold-canopy-m1-target', 'CARGO_BUILD_JOBS=2',
    'cargo', 'nextest', 'run', '--workspace', '--profile', 'full', '--no-fail-fast',
]


def validate_declared_bound(command, timeout, physical, required_full_validation,
                            remaining_for_command):
    if command == REQUIRED_FULL_VALIDATION_ARGV and not physical:
        raise SystemExit('Required full validation includes physical execution')
    if required_full_validation and (
        command != REQUIRED_FULL_VALIDATION_ARGV or timeout != 900 or not physical
    ):
        raise SystemExit('Required full validation needs exact established argv, physical classification and 900-second bound')
    if physical and timeout > 180 and not required_full_validation:
        raise SystemExit('Physical command declared bound exceeds 180 seconds')
    if timeout > remaining_for_command:
        raise SystemExit('Full declared command bound cannot fit before review/preservation reserve')


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
    parser.add_argument('--physical', action='store_true')
    parser.add_argument('--required-full-validation', action='store_true')
    parser.add_argument('--source-root', type=Path, required=True)
    parser.add_argument('--support-record', type=Path, required=True)
    parser.add_argument('command', nargs=argparse.REMAINDER)
    args = parser.parse_args()
    source = args.source_root.resolve()
    environment_root = Path('/workdir/openWEPP')
    input_root = HERE
    command = args.command[1:] if args.command[:1] == ['--'] else args.command
    if not command or args.timeout <= 0:
        parser.error('explicit command and positive timeout required; options precede label')
    if not re.fullmatch(r'[A-Za-z0-9][A-Za-z0-9_-]*', args.label):
        parser.error('label must be a basename containing only letters, digits, underscores and hyphens')
    remaining_for_command = (DEADLINE - dt.datetime.now(dt.timezone.utc)).total_seconds() - 1800
    validate_declared_bound(command, args.timeout, args.physical,
                            args.required_full_validation, remaining_for_command)
    LOGS.mkdir(exist_ok=True)
    receipt_path = LOGS / (args.label + '.json')
    output_paths = [LOGS / (args.label + suffix) for suffix in
                    ('.json', '-source.json', '-from-observer-cut02.patch', '.resources', '.stdout', '.stderr')]
    if any(path.exists() or path.is_symlink() for path in output_paths):
        raise SystemExit('Attempt identity already exists')
    support_record = json.loads(args.support_record.read_text())
    assert command == support_record['intended_argv'], 'selected command mismatch'
    assert args.timeout == support_record['declared_timeout_seconds'], 'selected timeout mismatch'
    assert args.physical == support_record['physical'], 'selected physical policy mismatch'
    assert args.required_full_validation == support_record.get('required_full_validation', False), 'selected required-full validation policy mismatch'
    assert Path(support_record['source_root']).resolve() == source, 'source root mismatch'
    assert support_record['authority_root'] == str(environment_root), 'canonical authority root mismatch'
    assert support_record['input_root'] == str(input_root), 'canonical input root mismatch'
    material_environment = {name: os.environ.get(name) for name in MATERIAL_ENVIRONMENT}
    assert support_record['material_environment'] == material_environment, 'material environment mismatch'
    before = SNAPSHOT.snapshot(source)[0]
    original = SNAPSHOT.snapshot(BASE)[0]
    assert hashlib.sha256(json.dumps(original, sort_keys=True).encode()).hexdigest() == '85b8314efcd53ccb8111aa97af1f752ed49a99f46b805e754db5a1b2c7059963'
    source_hash = hashlib.sha256(json.dumps(before, sort_keys=True).encode()).hexdigest()
    assert source_hash == support_record['source_tree_sha256'], 'selected source tree mismatch'
    patch = []
    for name in sorted(set(before) | set(original)):
        old = (BASE / name).read_text() if original.get(name) is not None else ''
        new = (source / name).read_text() if before.get(name) is not None else ''
        if old != new:
            patch.extend(difflib.unified_diff(old.splitlines(True), new.splitlines(True), fromfile='a/' + name, tofile='b/' + name))
    patch_path = LOGS / (args.label + '-from-observer-cut02.patch')
    source_receipt = LOGS / (args.label + '-source.json')
    pinned = {str(path): sha(path) for path in (Path(__file__), environment_root / 'flake.nix', environment_root / 'flake.lock', environment_root / 'tools/dev/openwepp-env')}
    # These source/configuration supports are outside the detached crate snapshot.
    # Runtime datasets still require explicit per-campaign input binding.
    snapshot_path = Path(SPEC.origin)
    pinned[str(snapshot_path)] = sha(snapshot_path)
    for support_root, pattern in (('.config', '*'), ('src', '*.rs'), ('tests', '*.rs')):
        for support_path in sorted((source / support_root).rglob(pattern)):
            if support_path.is_file():
                pinned[str(support_path)] = sha(support_path)
    authority_root = environment_root / 'docs/specifications/science-contracts/contracts'
    for name in (
        'SC-VEGETATION-001.md',
        'SC-LANDSURFACEENERGY-001.md',
        'SC-LANDSURFACEENERGY-001/interface.md',
        'SC-LANDSURFACEENERGY-001/water-vapor.md',
        'SC-LANDSURFACEENERGY-001/binding-index.md',
        'SC-LANDSURFACEENERGY-001/nonlinear-solve.md',
        'SC-LANDSURFACEENERGY-001/numerical-methods.md',
        'SC-LANDSURFACEENERGY-001/solve-boundary.md',
        'SC-LANDSURFACEENERGY-001/terminal-support.md',
        'SC-VEGETATIONTRANSACTION-001.md',
        'SC-SURFACELIQUID-001.md',
    ):
        authority_path = authority_root / name
        pinned[str(authority_path)] = sha(authority_path)
    support_record = json.loads(args.support_record.read_text())
    assert Path(support_record['source_root']).resolve() == source
    pinned[str(args.support_record.resolve())] = sha(args.support_record)
    for item in support_record['files']:
        assert sha(item['path']) == item['sha256'], item['path']
        pinned[item['path']] = item['sha256']
    support_links = support_record['support_links']
    assert all(Path(name).is_symlink() and os.readlink(name) == target
               for name, target in support_links.items())
    selected_files = {str(Path(item['path']).resolve()) for item in support_record['files']}
    for name, target in support_links.items():
        target_path = Path(name).resolve()
        selected_targets = support_record['link_target_files'][name]
        assert selected_targets, f'no selected target content for {name}'
        for selected in selected_targets:
            selected_path = Path(selected).resolve()
            assert str(selected_path) in selected_files, f'unpinned link input {selected}'
            assert selected_path == target_path or target_path in selected_path.parents, f'wrong link target {selected}'
    # Retained diagnostic operands and independent oracle used by drainage controls.
    for name in (
        'm1-original60-12.stderr',
        'm1-original60-12-linear-reconstruction.json',
        'structural-drainage-correctness-reconstruction.py',
        'structural-drainage-correctness-reconstruction.json',
    ):
        input_path = input_root / name
        pinned[str(input_path)] = sha(input_path)
    component_inventory_path = input_root / 'm1-frozen-component-inventory.json'
    component_inventory = json.loads(component_inventory_path.read_text())
    pinned[str(component_inventory_path)] = sha(component_inventory_path)
    for name, expected_sha256 in component_inventory['files'].items():
        input_path = input_root / name
        assert sha(input_path) == expected_sha256, str(input_path)
        pinned[str(input_path)] = expected_sha256
    remaining_for_command = (DEADLINE - dt.datetime.now(dt.timezone.utc)).total_seconds() - 1800
    validate_declared_bound(command, args.timeout, args.physical,
                            args.required_full_validation, remaining_for_command)
    timeout = args.timeout
    expected_pins = {item['path']: item['sha256'] for item in support_record['files']}
    for path, digest in pinned.items():
        if path not in (str(Path(__file__)), str(args.support_record.resolve())):
            assert expected_pins.get(path) == digest, f'input absent from selected manifest: {path}'
    # Reserve all identities together only after preflight. Never overwrite another attempt.
    reserved = []
    try:
        for path in output_paths:
            with path.open('xb'):
                pass
            reserved.append(path)
    except FileExistsError:
        for path in reserved:
            path.unlink()
        raise SystemExit('Attempt output identity collision')
    patch_path.write_text(''.join(patch))
    save(source_receipt, dict(source=str(source), entries=before, tree_sha256=source_hash, base=str(BASE), incremental_patch=str(patch_path), incremental_patch_sha256=sha(patch_path)))
    executed = ['/usr/bin/time', '-v', '-o', str(LOGS / (args.label + '.resources')), *command]
    receipt = dict(label=args.label, argv=command, executed_argv=executed, cwd=str(source), source_tree_sha256=source_hash,
                   source_receipt=str(source_receipt), pinned_files=pinned, support_links=support_links,
                   frozen_support_record=str(args.support_record) if args.support_record else None,
                   environment_root=str(environment_root),
                   material_inherited_environment=material_environment,
                   deadline_utc=DEADLINE.isoformat(), reserve_seconds=1800,
                   start_utc=dt.datetime.now(dt.timezone.utc).isoformat(),
                   timeout_seconds=timeout, address_space_bytes=16 * 1024**3, output_file_limit_bytes=1024**3,
                   physical=args.physical, required_full_validation=args.required_full_validation,
                   automatic_retries=0, binary=str(args.binary) if args.binary else None,
                   binary_sha256=sha(args.binary) if args.binary else None,
                   environment_policy='Only manifest-bound MATERIAL_ENVIRONMENT values; explicit env assignments in argv',
                   input_scope='Source-defined local component fixtures only; no original corpus inputs')
    save(receipt_path, receipt)
    env = {k: v for k, v in material_environment.items() if v is not None}
    start = time.monotonic()
    with (LOGS / (args.label + '.stdout')).open('wb') as out, (LOGS / (args.label + '.stderr')).open('wb') as err:
        process = subprocess.Popen(executed, cwd=source, env=env, stdout=out, stderr=err, preexec_fn=limits, start_new_session=True)
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
                   end_utc=dt.datetime.now(dt.timezone.utc).isoformat(), source_unchanged=SNAPSHOT.snapshot(source)[0] == before,
                   observer_base_unchanged=SNAPSHOT.snapshot(BASE)[0] == original,
                   binary_unchanged=sha(args.binary) == receipt['binary_sha256'] if args.binary else None,
                   pinned_files_unchanged=all(sha(path) == digest for path, digest in pinned.items()),
                   support_links_unchanged=all(
                       (Path(name).is_symlink() and os.readlink(name) == target) if args.support_record
                       else str(Path(name).resolve()) == str(Path(target).resolve())
                       for name, target in support_links.items()),
                   stdout_sha256=sha(LOGS / (args.label + '.stdout')), stderr_sha256=sha(LOGS / (args.label + '.stderr')))
    save(receipt_path, receipt)
    print(json.dumps({key: receipt[key] for key in ('label', 'exit_code', 'elapsed_seconds', 'source_unchanged', 'observer_base_unchanged')}))
    raise SystemExit(code)


if __name__ == '__main__':
    main()
