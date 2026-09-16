"""Preserve and reconstruct the explicitly selected diagnostic source; assigns no acceptance."""
import datetime
import difflib
import hashlib
import importlib.util
import json
from pathlib import Path
import subprocess
import tempfile

HERE = Path(__file__).resolve().parent
SOURCE = Path('/workdir/openwepp-experiments/b01-wb14-cadence/native-context-restoration-reader-20260916')
ACQUISITION = SOURCE.parent / 'snowfree-recorder-candidate-20260915'
SPEC = importlib.util.spec_from_file_location('retained', HERE.parent / 'execution-discretion-20260915/run-recorded.py')
RETAINED = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(RETAINED)


def sha(path):
    with path.open('rb') as stream:
        return hashlib.file_digest(stream, 'sha256').hexdigest()


def main():
    entries, changes, cumulative = RETAINED.snapshot(SOURCE)
    original = RETAINED.snapshot(ACQUISITION)[0]
    original_tree = hashlib.sha256(json.dumps(original, sort_keys=True).encode()).hexdigest()
    assert original_tree == 'ecc48d5235d8d53af496856459e3eaa29cca2dda2511697350a67bac172f9592'
    changed = sorted(p for p in entries.keys() | original.keys() if entries.get(p) != original.get(p))
    incremental = ''
    for name in changed:
        old, new = ACQUISITION / name, SOURCE / name
        incremental += ''.join(difflib.unified_diff(
            old.read_text().splitlines(True) if old.exists() else [],
            new.read_text().splitlines(True) if new.exists() else [],
            fromfile='a/' + name if old.exists() else '/dev/null',
            tofile='b/' + name if new.exists() else '/dev/null'))
    cumulative_sha = hashlib.sha256(cumulative.encode()).hexdigest()
    cumulative_path = HERE / ('source-' + cumulative_sha + '.patch')
    cumulative_path.write_text(cumulative)
    incremental_path = HERE / 'reader-incremental.patch'
    incremental_path.write_text(incremental)
    with tempfile.TemporaryDirectory(prefix='openwepp-event-restore-reconstruction-') as directory:
        root = Path(directory)
        for name in changed:
            old = ACQUISITION / name
            if old.exists():
                target = root / name
                target.parent.mkdir(parents=True, exist_ok=True)
                target.write_bytes(old.read_bytes())
        subprocess.run(['git', 'apply', str(incremental_path)], cwd=root, check=True, capture_output=True)
        for name in changed:
            restored = root / name
            assert (sha(restored) if restored.exists() else None) == entries.get(name), name
    supplemental_path = HERE.parent / 'reference-input-recovery-20260915/input-custody.json'
    assert sha(supplemental_path) == '2e9e27eeb0a355672e7a74c8fd645a2ce95182ae88bf78741bc8f98ecac9b082'
    supplemental = json.loads(supplemental_path.read_text())['added_inputs']
    assert all(sha(SOURCE / name) == expected for name, expected in supplemental.items())
    custody = json.loads((HERE.parent / 'native-context-restoration-20260916/input-custody-before.json').read_text())
    assert all(Path(m['path']).stat().st_size == m['bytes'] and sha(Path(m['path'])) == m['sha256'] for m in custody['entries'])
    frozen = SOURCE.parent / 'scoped-provider-018.frozen'
    assert sha(frozen) == '9741b3972937ac2f04642e57d68308cc891ba39fdf420a5b07403088aa420363'
    assert RETAINED.snapshot(SOURCE)[0] == entries, 'Source changed during preservation'
    result = dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),
                  source=str(SOURCE), exact_base=str(RETAINED.BASE), acquisition_base=str(ACQUISITION),
                  entries=entries, changed_from_acquisition=changed, changes_from_producing_base=changes,
                  tree_sha256=hashlib.sha256(json.dumps(entries, sort_keys=True).encode()).hexdigest(),
                  cumulative_patch=cumulative_path.name, cumulative_patch_sha256=cumulative_sha,
                  incremental_patch=incremental_path.name, incremental_patch_sha256=sha(incremental_path),
                  incremental_reconstruction_pass=True, acquisition_unchanged=True,
                  all_49_export_members_unchanged=True, prior_frozen_binary_unchanged=True,
                  supplemental_inputs_unchanged=True, supplemental_custody_sha256=sha(supplemental_path))
    (HERE / 'terminal-reconciliation.json').write_text(json.dumps(result, indent=2) + '\n')
    print(json.dumps({key: result[key] for key in ['tree_sha256', 'changed_from_acquisition', 'incremental_reconstruction_pass']}))


if __name__ == '__main__':
    main()
