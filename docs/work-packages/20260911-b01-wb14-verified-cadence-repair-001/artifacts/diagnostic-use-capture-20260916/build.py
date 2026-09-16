"""Build and freeze the explicitly authorized recorder executable; no model run."""
import datetime
import hashlib
import importlib.util
import json
from pathlib import Path
import shutil
import subprocess

HERE = Path(__file__).resolve().parent
ROOT = Path('/workdir/openWEPP')
SOURCE = Path('/workdir/openwepp-experiments/b01-wb14-cadence/snowfree-recorder-candidate-20260915')
TARGET = Path('/tmp/openwepp-b01-wb14-cadence-targets/diagnostic-use-capture-20260916')
FROZEN = Path('/workdir/openwepp-experiments/b01-wb14-cadence/corrected-recorder-20260916.frozen')
spec = importlib.util.spec_from_file_location('custody', HERE.parent / 'complete-lint-diagnostics-20260916/run-collection.py')
custody = importlib.util.module_from_spec(spec)
spec.loader.exec_module(custody)


def write(name, value):
    (HERE / name).write_text(json.dumps(value, indent=2) + '\n')


def sha(path):
    with path.open('rb') as file:
        return hashlib.file_digest(file, 'sha256').hexdigest()


def main():
    assert not TARGET.exists() and not FROZEN.exists()
    before = custody.identity(SOURCE)
    assert before['tree_sha256'] == 'ecc48d5235d8d53af496856459e3eaa29cca2dda2511697350a67bac172f9592'
    assert before['patch_sha256'] == '4559c7d21fbcdb9314845e6a5c19940d051d0ef861e48c7cb2a4ae7e51020c5d'
    assert before['supplemental_custody_sha256'] == '2e9e27eeb0a355672e7a74c8fd645a2ce95182ae88bf78741bc8f98ecac9b082'
    assert not before['supplemental_mismatches']
    write('source-before-build.json', before)
    historical = json.loads((HERE.parent / 'authentic-raw-collection-20260914/build-command.json').read_text())
    argv = historical['argv'].copy()
    oldsource = historical['cwd']
    argv = [str(SOURCE / 'Cargo.toml') if x == oldsource + '/Cargo.toml' else x for x in argv]
    argv = [f'CARGO_TARGET_DIR={TARGET}' if x.startswith('CARGO_TARGET_DIR=') else x for x in argv]
    argv = [f"OPENWEPP_B01_SOURCE_SHA256={before['tree_sha256']}" if x.startswith('OPENWEPP_B01_SOURCE_SHA256=') else x for x in argv]
    argv = [x for x in argv if not x.startswith('OPENWEPP_B01_BUILD_INPUTS_SHA256=')]
    old_inputs = json.loads((HERE.parent / 'authentic-raw-collection-20260914/preflight.json').read_text())['inputs']
    inputs = []
    for item in old_inputs:
        path = Path(item['path'])
        actual = sha(path)
        assert actual == item['sha256'], str(path)
        inputs.append({'path': str(path), 'sha256': actual})
    build_inputs = {'argv_without_self_stamp': argv, 'cwd': str(SOURCE), 'source_tree_sha256': before['tree_sha256'],
                    'source_namespace': '741 entries; supplemental execution inputs separately bound',
                    'supplemental_custody_sha256': before['supplemental_custody_sha256'], 'inputs': inputs,
                    'cargo_lock_sha256': sha(SOURCE / 'Cargo.lock')}
    write('build-inputs.json', build_inputs)
    stamp = sha(HERE / 'build-inputs.json')
    argv.insert(argv.index('cargo'), f'OPENWEPP_B01_BUILD_INPUTS_SHA256={stamp}')
    receipt = {'argv': argv, 'cwd': str(SOURCE), 'source_stamp': before['tree_sha256'], 'build_stamp': stamp,
               'start_utc': datetime.datetime.now(datetime.timezone.utc).isoformat(), 'tests_executed': 0,
               'timeout_seconds': 1800, 'executed_argv': ['/usr/bin/timeout', '--signal=TERM', '--kill-after=5s', '1800s', *argv]}
    write('build-command.json', receipt)
    with (HERE / 'build.stdout').open('wb') as out, (HERE / 'build.stderr').open('wb') as err:
        process = subprocess.Popen(receipt['executed_argv'], cwd=SOURCE, stdout=out, stderr=err)
        receipt['pid'] = process.pid
        write('build-command.json', receipt)
        receipt['exit'] = process.wait()
    receipt['end_utc'] = datetime.datetime.now(datetime.timezone.utc).isoformat()
    receipt['source_and_inputs_unchanged'] = custody.identity(SOURCE) == before
    write('build-command.json', receipt)
    assert receipt['exit'] == 0 and receipt['source_and_inputs_unchanged'], receipt
    artifacts = []
    for line in (HERE / 'build.stdout').read_text().splitlines():
        try:
            record = json.loads(line)
        except json.JSONDecodeError:
            continue
        if record.get('reason') == 'compiler-artifact' and record.get('executable') and record['target']['name'] == 'openwepp_runner':
            artifacts.append(record)
    assert len(artifacts) == 1 and artifacts[0]['profile']['test']
    artifact = artifacts[0]
    shutil.copy2(artifact['executable'], FROZEN)
    FROZEN.chmod(0o555)
    binary_sha = sha(FROZEN)
    assert sha(Path(artifact['executable'])) == binary_sha
    argv = [str(FROZEN), 'hillslope::tests::stage3_snow_accuracy_case', '--ignored', '--exact', '--list']
    result = subprocess.run(argv, capture_output=True, cwd=SOURCE)
    (HERE / 'compiled-list.stdout').write_bytes(result.stdout)
    (HERE / 'compiled-list.stderr').write_bytes(result.stderr)
    write('compiled-list.json', {'argv': argv, 'exit': result.returncode, 'cwd': str(SOURCE),
          'binary': str(FROZEN), 'binary_sha256': binary_sha, 'bytes': FROZEN.stat().st_size,
          'compiler_artifact': artifact, 'listing_only': True, 'tests_executed': 0,
          'utc': datetime.datetime.now(datetime.timezone.utc).isoformat()})
    assert result.returncode == 0 and b'hillslope::tests::stage3_snow_accuracy_case: test' in result.stdout
    assert b'1 test, 0 benchmarks' in result.stdout
    print(json.dumps({'build_exit': receipt['exit'], 'binary_sha256': binary_sha, 'listing_exit': result.returncode}), flush=True)


if __name__ == '__main__':
    main()
