#!/usr/bin/env python3
"""Add exact 2b authority deltas without replacing prior e89 evidence."""
import difflib
import json
from pathlib import Path
import subprocess
import tempfile

from freeze_authority import BASE, COMMON, LSE, SNOW, OUTPUT, blob, emit, sha


def main():
    prior_data = (OUTPUT / 'manifest.json').read_bytes()
    prior = json.loads(prior_data)
    deltas = {}
    for name, target in [('F-common-from-2b', 'common-2b'),
                         ('R-final-SG1-from-2b', 'R-final-SG1')]:
        targets = {}
        patch = []
        for path, key in [(LSE, 'lse'), (SNOW, 'snow')]:
            row = prior['cuts'][target][key]
            data = (OUTPUT / row['artifact']).read_bytes()
            if sha(data) != row['sha256'] or len(data) != row['bytes']:
                raise ValueError('snapshot identity mismatch')
            targets[path] = data
            patch.extend(difflib.unified_diff(
                blob(COMMON, path).decode().splitlines(keepends=True),
                data.decode().splitlines(keepends=True),
                fromfile='a/' + path, tofile='b/' + path))
        encoded = ''.join(patch).encode()
        record = emit('patches/' + name + '.patch', encoded)
        with tempfile.TemporaryDirectory(prefix='openwepp-authority-2b-verify-') as directory:
            scratch = Path(directory)
            for path in targets:
                destination = scratch / path
                destination.parent.mkdir(parents=True, exist_ok=True)
                destination.write_bytes(blob(COMMON, path))
            # F is an explicit identity operation, not a patch with invented changes.
            empty = ['--allow-empty'] if not encoded else []
            command = ['git', 'apply', *empty]
            subprocess.run([*command, '--check', str(OUTPUT / record['artifact'])],
                           cwd=scratch, check=True)
            subprocess.run([*command, str(OUTPUT / record['artifact'])],
                           cwd=scratch, check=True)
            if any((scratch / path).read_bytes() != data for path, data in targets.items()):
                raise ValueError('2b reconstruction mismatch')
        deltas[name] = {'base': COMMON, 'target_cut': target, 'patch': record,
                        'identity_operation': not bool(encoded),
                        'git_apply_and_exact_byte_comparison': 'PASS'}
    source_bases = {}
    for name in ['A-source-03', 'F-source-03', 'R-source-03']:
        path = OUTPUT.parent / name / 'source-manifest.json'
        data = path.read_bytes()
        source_bases[name] = {'base': json.loads(data)['base'], 'manifest_sha256': sha(data)}
    if source_bases['A-source-03']['base'] != BASE or any(
            source_bases[name]['base'] != COMMON for name in ['F-source-03', 'R-source-03']):
        raise ValueError('runtime-kit base differs from recipe')
    result = {
        'supersedes': 'manifest.json arm recipe only; prior snapshots/e89 patch evidence preserved',
        'prior_manifest_sha256': sha(prior_data), 'source_kit_bases_checked': source_bases,
        'new_deltas': deltas,
        'arm_recipe': {
            'A-initial-common': {'checkout': BASE, 'authority_patch': 'patches/common-2b.patch'},
            'A-full-release-01': {'checkout': BASE, 'authority_patch': 'patches/A-full-release-01-PC1.patch'},
            'F-full': {'checkout': COMMON, 'authority_patch': 'patches/F-common-from-2b.patch', 'action': 'no-op; verify existing canonical hashes'},
            'R-final': {'checkout': COMMON, 'authority_patch': 'patches/R-final-SG1-from-2b.patch'},
        },
        'builds_executed': False,
    }
    emit('composition-manifest-02.json', (json.dumps(result, indent=2, sort_keys=True) + '\n').encode())
    print(json.dumps({'status': 'PASS', 'new_deltas': deltas, 'builds_executed': False}))


if __name__ == '__main__':
    main()
