"""Prepare, then consume the one newly authorized capture. Never use for status."""
import argparse
import datetime
import importlib.util
import json
import os
from pathlib import Path
import shutil
import subprocess

import ijson

HERE = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location('build_record', HERE / 'build.py')
build = importlib.util.module_from_spec(spec)
spec.loader.exec_module(build)
ROOT = Path('/workdir/openWEPP')
DURABLE = Path('/workdir/openwepp-experiments/b01-wb14-cadence')
OUTPUT = DURABLE / 'corrected-recorder-capture-20260916-1'
FIXTURE = DURABLE / 'corrected-recorder-fixture-20260916-1'
COLLECTOR = ROOT / 'docs/work-packages/20260910-stage3-b01-snow-cycle-integration-001/collect.py'
CASES = COLLECTOR.parent / 'artifacts/gradual-warm-tail7-cases.json'
DEADLINE = datetime.datetime.fromisoformat('2026-09-16T05:39:08.596115+00:00')


def now():
    return datetime.datetime.now(datetime.timezone.utc)


def preflight():
    listing = json.loads((HERE / 'compiled-list.json').read_text())
    before = json.loads((HERE / 'source-before-build.json').read_text())
    assert build.custody.identity(build.SOURCE) == before
    assert build.sha(build.FROZEN) == listing['binary_sha256']
    assert build.sha(CASES) == 'c48cc560dff31102085ebcf852225b2063a1629ad96cfa9f7283364df44fbf3f'
    assert len(json.loads(CASES.read_text())['cases']['gradual_warm_tail7']['forcing']) == 7
    assert not OUTPUT.exists() and not FIXTURE.exists()
    assert 0 in os.sched_getaffinity(0)
    assert ijson.__version__ == '3.4.0'
    remaining = (DEADLINE - now()).total_seconds()
    # 3600 child + 600 collector finish + 1200 offline passes + 900 review/publication.
    assert remaining >= 6300, ('insufficient reserved execution/collection allowance', remaining)
    old = DURABLE / 'authentic-raw-capture-20260914-1'
    prior_bytes = sum((old / name).stat().st_size for name in ['observations.json', 'physical-rows.jsonpart'])
    storage = {'free_durable_bytes': shutil.disk_usage(DURABLE).free,
               'prior_raw_and_spool_bytes': prior_bytes, 'additional_context_and_duplicate_spool_bytes': 4 * 1024**3,
               'export_budget_bytes': 4 * 1024**3, 'headroom_bytes': 4 * 1024**3,
               'free_build_bytes': shutil.disk_usage('/tmp').free}
    storage['required_durable_bytes'] = prior_bytes + 12 * 1024**3
    assert storage['free_durable_bytes'] >= storage['required_durable_bytes'], storage
    original = json.loads((HERE.parent / 'authentic-raw-collection-20260914/preflight.json').read_text())
    checked = []
    for entry in original['inputs']:
        path = Path(entry['path'])
        actual = build.sha(path)
        assert actual == entry['sha256'], str(path)
        checked.append({'path': str(path), 'sha256': actual})
    argv = [str(ROOT / '.venv/bin/python'), str(COLLECTOR), str(build.FROZEN), str(OUTPUT),
            '--policy', 'B01', '--case', 'gradual_warm_tail7', '--ofes', '1', '--cases', str(CASES),
            '--cpu', '0', '--timeout', '3600']
    return {'evidence_class': 'Ran: current launch identity/capacity checks; source activation independently inspected',
            'prepared_utc': now().isoformat(), 'source': str(build.SOURCE), 'source_tree_sha256': before['tree_sha256'],
            'patch_sha256': before['patch_sha256'], 'supplemental_custody_sha256': before['supplemental_custody_sha256'],
            'binary': str(build.FROZEN), 'binary_sha256': listing['binary_sha256'], 'build_inputs_sha256': build.sha(HERE / 'build-inputs.json'),
            'argv': argv, 'cwd': str(ROOT), 'inputs': checked, 'storage': storage,
            'environment_overrides': {'TMPDIR': str(FIXTURE), 'PYTHONDONTWRITEBYTECODE': '1'},
            'ijson_version': ijson.__version__, 'child_timeout_seconds': 3600, 'automatic_retries': 0,
            'remaining_seconds_at_check': remaining, 'reserved_total_seconds': 6300,
            'target': {'day_index': 4, 'interval_index': 22, 'phase': 'snow_free',
                       'child_support_ns': [385920000000000, 385980000000000],
                       'historical_transaction_255_is_not_arrival_proof': True},
            'observer': {'OPENWEPP_ACCURACY_PHYSICAL': '1', 'begin_spooled_before_model': True,
                         'finish_to_file_after_result': True, 'production_constructor_retention': 'physical_enabled'},
            'capture_allowance': {'name': 'CORRECTED-RECORDER-20260916', 'used': 0, 'maximum': 1},
            'old_capture': '1/1 consumed', 'old_witness': '1/1 exhausted', 'authentic_probe': '0/1 RESERVED AND PROHIBITED'}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('action', choices=['prepare', 'launch'])
    action = parser.parse_args().action
    assert not (HERE / 'raw-launch-start.json').exists(), 'New capture already consumed; read records for status.'
    current = preflight()
    if action == 'prepare':
        build.write('preflight.json', current)
        print(json.dumps({'preflight': 'PASS', 'remaining_seconds': current['remaining_seconds_at_check'],
                          'storage': current['storage']}), flush=True)
        return
    prepared = json.loads((HERE / 'preflight.json').read_text())
    for key in ['argv', 'binary_sha256', 'source_tree_sha256', 'patch_sha256', 'inputs', 'build_inputs_sha256']:
        assert prepared[key] == current[key], key
    FIXTURE.mkdir()
    env = os.environ.copy()
    env.update(current['environment_overrides'])
    receipt = dict(current)
    receipt.update(start_utc=now().isoformat(), wrapper_pid=os.getpid(), state='LAUNCHING',
                   capture_allowance={'name': 'CORRECTED-RECORDER-20260916', 'used': 1, 'maximum': 1})
    build.write('raw-launch-start.json', receipt)
    with (HERE / 'collector.stdout').open('xb') as out, (HERE / 'collector.stderr').open('xb') as err:
        process = subprocess.Popen(current['argv'], cwd=ROOT, env=env, stdout=out, stderr=err)
        receipt.update(collector_pid=process.pid, state='RUNNING')
        build.write('raw-launch-start.json', receipt)
        collector_exit = process.wait()
    terminal = {'collector_exit': collector_exit, 'end_utc': now().isoformat(), 'wrapper_pid': os.getpid(),
                'collector_pid': process.pid, 'output': str(OUTPUT), 'fixture': str(FIXTURE),
                'source_and_inputs_unchanged': build.custody.identity(build.SOURCE) == json.loads((HERE / 'source-before-build.json').read_text()),
                'binary_unchanged': build.sha(build.FROZEN) == current['binary_sha256'],
                'receipt_present': (OUTPUT / 'receipt.json').is_file()}
    if terminal['receipt_present']:
        run = json.loads((OUTPUT / 'receipt.json').read_text())
        terminal['runner'] = {key: run.get(key) for key in ['exit_code', 'timeout', 'infrastructure_error', 'runner_execution', 'execution_valid', 'observation_complete', 'physical_rows', 'wall_s']}
    build.write('raw-launch-terminal.json', terminal)
    print(json.dumps(terminal), flush=True)


if __name__ == '__main__':
    main()
