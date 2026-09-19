"""Isolated tooling tests: Python dummy children only; never runs a Rust solver."""
import contextlib
import datetime as dt
import importlib.util
import io
import json
from pathlib import Path
import sys
import tempfile

HERE = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location('synthetic_grid40_recorder', HERE / 'run_pair_arm.py')
recorder = importlib.util.module_from_spec(spec)
spec.loader.exec_module(recorder)
root = Path(tempfile.mkdtemp(prefix='grid40-recorder-synthetic-final-', dir='/home/roger/openwepp-experiments'))
recorder.SOURCE = root / 'synthetic-source'
recorder.SOURCE.mkdir()
recorder.source_hash = lambda: 'synthetic-tooling-source'
recorder.ARM_SECONDS = 0.2
external, links = {}, {}
for i in range(19):
    path = root / ('external-' + str(i))
    path.write_text(str(i))
    external[str(path)] = recorder.sha(path)
for i in range(5):
    path = recorder.SOURCE / ('link-' + str(i))
    target = root / ('external-' + str(i))
    path.symlink_to(target)
    links[path.name] = str(target)
review = root / 'synthetic-only-review.txt'
review.write_text('Synthetic tooling fixture only; not independent scientific admission.\n')
checks = []
cases = [('success', 'print("synthetic tooling only")', False, 0),
         ('failure', 'raise SystemExit(3)', False, 3),
         ('timeout', 'import time; time.sleep(2)', False, -9),
         ('expired', 'raise SystemExit(99)', True, None)]
for label, code, expired, expected_exit in cases:
    recorder.RUNS = root / label
    recorder.RUNS.mkdir()
    recorder.FREEZE = recorder.RUNS / 'pair-freeze.json'
    recorder.DEADLINE = dt.datetime.now(dt.timezone.utc) + dt.timedelta(seconds=1 if expired else 2000)
    binary = str(Path(sys.executable).resolve())
    binary_sha = recorder.sha(binary)
    arguments = ['-c', code]
    freeze = dict(schema='openwepp.grid40.pair-freeze.v1',
                  prerequisites={'correctness': 'GO', 'qa': 'GO', 'focused_checks': 'PASS'},
                  recorder_sha256=recorder.sha(HERE / 'run_pair_arm.py'),
                  snapshot_tool_sha256=recorder.sha(recorder.SNAPSHOT_FILE),
                  source_sha256='synthetic-tooling-source', binary=binary, binary_sha256=binary_sha,
                  input=str(HERE.parent / 'lse-first-trial-20260918/final-first-failure-trace.json'),
                  input_sha256=recorder.INPUT_SHA, bound_files={}, external_build_inputs=external,
                  support_links=links, output_filenames=['stdout', 'stderr', 'trace.json', 'trace.partial',
                                                       'sidecar.json', 'events.jsonl', 'reserved-1', 'reserved-2'],
                  environment={}, binary_arguments=arguments,
                  write_boundary={'source_sha256': 'synthetic-tooling-source', 'binary_sha256': binary_sha,
                                  'entry_arguments': arguments, 'no_subprocess': True,
                                  'no_other_destinations': True, 'review': str(review),
                                  'review_sha256': recorder.sha(review), 'output_environment': {}})
    recorder.FREEZE.write_text(json.dumps(freeze, indent=2) + '\n')
    sys.argv = ['synthetic_grid40_recorder_test', 'baseline']
    error = None
    try:
        with contextlib.redirect_stdout(io.StringIO()) as output:
            recorder.entry()
    except (RuntimeError, FileExistsError) as failure:
        error = str(failure)
    (recorder.RUNS / 'test-error.txt').write_text(str(error))
    receipt = recorder.RUNS / 'baseline/receipt.json'
    if expired:
        passed = not receipt.exists() and error is not None
    else:
        observed = json.loads(receipt.read_text())
        passed = (observed['process_exit'] == expected_exit and observed['integrity_unchanged']
                  and ((error is None) == (label == 'success')))
    if error is not None:
        passed = passed and bool(list((recorder.RUNS / 'launcher-refusals').glob('*.json')))
    if not passed:
        raise RuntimeError((label, error))
    checks.append({'case': label, 'pass': True, 'expected_process_exit': expected_exit,
                   'receipt': str(receipt) if receipt.exists() else None})
    if label == 'success':
        try:
            with contextlib.redirect_stdout(io.StringIO()):
                recorder.entry()
            raise RuntimeError('duplicate arm admitted')
        except FileExistsError:
            checks.append({'case': 'duplicate claim refused and recorded', 'pass': True})
result = {'evidence_class': 'Ran: isolated Python tooling children; no Rust/evaluator/scientific arm',
          'recorder_sha256': recorder.sha(HERE / 'run_pair_arm.py'),
          'driver_sha256': recorder.sha(__file__), 'raw_directory': str(root),
          'overrides': 'Synthetic source hash, review fixture, run paths, deadline and 0.2-second time cap; original input only hashed, never evaluated.',
          'checks': checks, 'nonzero_child_outcomes': 2}
(HERE / 'arm-recorder-final-process-checks.json').write_text(json.dumps(result, indent=2) + '\n')
print(json.dumps(result, indent=2))
