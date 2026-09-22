"""Nonphysical discriminating controls for the resumption recorder changes."""
import copy
import datetime as dt
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import subprocess

HERE = Path(__file__).resolve().parent
SPEC = importlib.util.spec_from_file_location('recorder', HERE / 'run_recorded.py')
REC = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(REC)
ROOT = Path('/home/roger/openwepp-experiments/cold-canopy-m1-owner-validation-support')
SCRATCH = Path('/home/roger/openwepp-experiments/cold-canopy-m1-recorder-controls-20260922')
SCRATCH.mkdir(exist_ok=True)
MARKER = SCRATCH / 'must-not-launch'
assert not MARKER.exists()
LINK_INPUT = SCRATCH / 'selected-input.txt'
LINK_INPUT.write_text('immutable recorder control input\n')
LINK = SCRATCH / 'selected-link'
if not LINK.is_symlink():
    LINK.symlink_to(LINK_INPUT)

previous = json.loads((HERE / 'owner-txn-terminal-focused-01.json').read_text())
pins = {}
for name, digest in previous['pinned_files'].items():
    if name == str(HERE / 'run_recorded.py'):
        continue
    path = Path(name)
    if path.is_relative_to(REC.SOURCE):
        path = ROOT / path.relative_to(REC.SOURCE)
    if path.exists():
        pins[str(path)] = REC.sha(path)
for directory, pattern in (('.config', '*'), ('src', '*.rs'), ('tests', '*.rs')):
    for path in (ROOT / directory).rglob(pattern):
        if path.is_file():
            pins[str(path)] = REC.sha(path)
pins[str(LINK_INPUT)] = REC.sha(LINK_INPUT)
source = REC.SNAPSHOT.snapshot(ROOT)[0]
manifest = dict(source_root=str(ROOT),
                source_tree_sha256=hashlib.sha256(json.dumps(source, sort_keys=True).encode()).hexdigest(),
                authority_root='/workdir/openWEPP', input_root=str(HERE),
                material_environment={name: os.environ.get(name) for name in REC.MATERIAL_ENVIRONMENT},
                files=[dict(path=name, sha256=digest) for name, digest in sorted(pins.items())],
                support_links={str(LINK): str(LINK_INPUT)},
                link_target_files={str(LINK): [str(LINK_INPUT)]},
                scope='Nonphysical recorder controls only; not scientific source qualification')
results = []


def run(case, mutate=None, options=None, label=None, success=False, omit=None, collision=False):
    selected = copy.deepcopy(manifest)
    selected.update(intended_argv=['/usr/bin/pwd'] if success else ['/usr/bin/touch', str(MARKER)],
                    declared_timeout_seconds=int(options[1]) if options else 10,
                    physical=bool(options and '--physical' in options))
    if mutate:
        mutate(selected)
    record = HERE / ('resumption-recorder-control-' + case + '-input-02.json')
    record.write_text(json.dumps(selected, indent=2) + '\n')
    selected_label = label or ('resumption-recorder-control-' + case + '-02')
    argv = ['/workdir/openWEPP/.venv/bin/python', str(HERE / 'run_recorded.py')]
    argv += options or ['--timeout', '10']
    if omit != 'source':
        argv += ['--source-root', str(ROOT)]
    if omit != 'support':
        argv += ['--support-record', str(record)]
    argv += [selected_label, '--', '/usr/bin/pwd'] if success else [selected_label, '--', '/usr/bin/touch', str(MARKER)]
    occupied = HERE / (selected_label + '.stderr') if collision else None
    if occupied:
        with occupied.open('x') as stream:
            stream.write('pre-existing evidence\n')
    before = {str(path): REC.sha(path) for path in HERE.glob(selected_label + '*') if path.is_file()}
    result = subprocess.run(argv, text=True, capture_output=True, timeout=30)
    after = {str(path): REC.sha(path) for path in HERE.glob(selected_label + '*') if path.is_file()}
    valid = result.returncode == 0 if success else result.returncode != 0 and before == after
    assert valid and not MARKER.exists(), (case, result.stdout, result.stderr)
    results.append(dict(case=case, argv=argv, exit_code=result.returncode,
                        stdout=result.stdout, stderr=result.stderr, marker_absent=not MARKER.exists(),
                        rejection_preserves_outputs=before == after if not success else None))


run('bound', options=['--timeout', '99999999'])
run('physical', options=['--timeout', '181', '--physical'])
run('unpaired-source', omit='support')
run('unpaired-support', omit='source')
run('root', lambda m: m.update(source_root='/tmp'))
run('tree', lambda m: m.update(source_tree_sha256='0' * 64))
run('authority-root', lambda m: m.update(authority_root=str(ROOT)))
run('authority-input', lambda m: next(x for x in m['files'] if x['path'].endswith('SC-VEGETATION-001.md')).update(sha256='0' * 64))
run('input', lambda m: next(x for x in m['files'] if x['path'].endswith('m1-original60-12.stderr')).update(sha256='0' * 64))
run('link-target', lambda m: next(x for x in m['files'] if x['path'] == str(LINK_INPUT)).update(sha256='0' * 64))
run('link-coverage', lambda m: m['link_target_files'].update({str(LINK): []}))
run('environment', lambda m: m['material_environment'].update(PATH='/wrong'))
run('unsafe-label', label='../recorder-escape-must-not-exist-02')
run('collision', collision=True)
run('command', lambda m: m.update(intended_argv=['/usr/bin/false']))
run('timeout', lambda m: m.update(declared_timeout_seconds=11))
run('physical-policy', lambda m: m.update(physical=True))
run('cwd', success=True)
receipt = json.loads((HERE / 'resumption-recorder-control-cwd-02.json').read_text())
assert (HERE / 'resumption-recorder-control-cwd-02.stdout').read_text().strip() == str(ROOT)
assert receipt['cwd'] == str(ROOT) and receipt['source_unchanged'] and receipt['pinned_files_unchanged']
assert receipt['deadline_utc'] == REC.DEADLINE.isoformat()
output = dict(observed_utc=dt.datetime.now(dt.timezone.utc).isoformat(), recorder_sha256=REC.sha(HERE / 'run_recorded.py'),
              deadline_utc=REC.DEADLINE.isoformat(), results=results, scope='Nonphysical launch/custody controls only')
(HERE / 'resumption-recorder-controls-02.json').write_text(json.dumps(output, indent=2) + '\n')
print(f'{len(results)} recorder controls PASS; no physical execution')
