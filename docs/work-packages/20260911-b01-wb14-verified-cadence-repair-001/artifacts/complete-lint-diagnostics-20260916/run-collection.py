"""Record the four owner-authorized diagnostic-only commands; execute no tests."""
import datetime
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import subprocess
import sys

HERE = Path(__file__).resolve().parent
PRIOR = HERE.parent / 'execution-discretion-20260915'
spec = importlib.util.spec_from_file_location('prior_recorder', PRIOR / 'run-recorded.py')
prior = importlib.util.module_from_spec(spec)
spec.loader.exec_module(prior)
DEADLINE = datetime.datetime.fromisoformat('2026-09-16T04:58:17.700175+00:00')


def sha(data):
    return hashlib.sha256(data).hexdigest()


def identity(source):
    entries, changes, patch = prior.snapshot(source)
    custody = HERE.parent / 'reference-input-recovery-20260915/input-custody.json'
    expected = json.loads(custody.read_text())['added_inputs']
    # Added recovery inputs are separate from the historical 741-entry namespace.
    inputs = {rel: sha((source / rel).read_bytes()) if (source / rel).is_file() else None
              for rel in expected}
    return dict(entry_count=len(entries), tree_sha256=sha(json.dumps(entries, sort_keys=True).encode()),
                patch_sha256=sha(patch.encode()), changes=changes,
                supplemental_custody_sha256=sha(custody.read_bytes()),
                supplemental_inputs=inputs,
                supplemental_mismatches=[rel for rel in expected if inputs[rel] != expected[rel]])


def run(label):
    side, package = label.split('-')
    assert side in ('baseline', 'candidate') and package in ('orchestrator', 'runner')
    if side == 'baseline':
        old = json.loads((PRIOR / 'baseline-lint-reuse.json').read_text())[package]
    else:
        old = json.loads((PRIOR / f'candidate-{package}-clippy-final.json').read_text())
    argv = old['argv'].copy()
    source = Path(argv[2])
    target = Path('/tmp/openwepp-b01-wb14-cadence-targets') / f'complete-lint-20260916-{side}'
    argv[3] = str(target)
    argv.insert(argv.index('--'), '-vv')
    argv.extend(['--cap-lints', 'warn'])
    before = identity(source)
    assert before['tree_sha256'] == old['source_tree_sha256'], before
    assert before['supplemental_custody_sha256'] == '2e9e27eeb0a355672e7a74c8fd645a2ce95182ae88bf78741bc8f98ecac9b082'
    if side == 'candidate':
        assert before['patch_sha256'] == '4559c7d21fbcdb9314845e6a5c19940d051d0ef861e48c7cb2a4ae7e51020c5d'
        assert not before['supplemental_mismatches'], before['supplemental_mismatches']
    now = datetime.datetime.now(datetime.timezone.utc)
    remaining = (DEADLINE - now).total_seconds() - 90
    assert remaining > 0, 'preservation reserve reached'
    receipt_path = HERE / (label + '.json')
    assert not receipt_path.exists(), 'Preserve existing receipts; choose an explicit rerun identity.'
    receipt = dict(label=label, purpose='supplemental diagnostic collection; not strict Clippy acceptance',
                   argv=argv, cwd=str(source), target=str(target), target_existed_before=target.exists(),
                   source_before=before, start_utc=now.isoformat(),
                   inherited_environment={key: os.environ.get(key) for key in
                       ('RUSTFLAGS', 'CARGO_ENCODED_RUSTFLAGS', 'RUSTC_WRAPPER', 'RUSTC_WORKSPACE_WRAPPER', 'RUSTUP_TOOLCHAIN')},
                   execution_environment_files={str(path): sha(path.read_bytes()) for path in
                       [Path(argv[1]), Path('/workdir/openWEPP/flake.nix'), Path('/workdir/openWEPP/flake.lock'),
                        Path('/workdir/openWEPP/tools/dev/openwepp-env')]})
    receipt['executed_argv'] = ['/usr/bin/timeout', '--signal=TERM', '--kill-after=5s', f'{remaining}s', *argv]
    receipt_path.write_text(json.dumps(receipt, indent=2) + '\n')
    with (HERE / (label + '.stdout')).open('wb') as out, (HERE / (label + '.stderr')).open('wb') as err:
        result = subprocess.run(receipt['executed_argv'], cwd=source, stdout=out, stderr=err)
    after = identity(source)
    receipt.update(end_utc=datetime.datetime.now(datetime.timezone.utc).isoformat(), exit=result.returncode,
                   source_unchanged_during_command=after == before,
                   stdout_sha256=sha((HERE / (label + '.stdout')).read_bytes()),
                   stderr_sha256=sha((HERE / (label + '.stderr')).read_bytes()))
    receipt_path.write_text(json.dumps(receipt, indent=2) + '\n')
    print(json.dumps({key: receipt[key] for key in ('label', 'start_utc', 'end_utc', 'exit', 'source_unchanged_during_command')}), flush=True)
    assert after == before, 'Source/input drift: stop'
    return result.returncode


if __name__ == '__main__':
    sys.exit(run(sys.argv[1]))
