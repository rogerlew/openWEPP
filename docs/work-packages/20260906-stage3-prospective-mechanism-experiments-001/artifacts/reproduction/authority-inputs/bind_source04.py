#!/usr/bin/env python3
"""Bind current F/R source-04 manifests to unchanged exact-base authority deltas."""
import json

from freeze_authority import BASE, COMMON, OUTPUT, emit, sha


def main():
    previous = (OUTPUT / 'composition-manifest-02.json').read_bytes()
    result = json.loads(previous)
    checked = {}
    for name, expected in [('A-source-03', BASE), ('F-source-04', COMMON), ('R-source-04', COMMON)]:
        data = (OUTPUT.parent / name / 'source-manifest.json').read_bytes()
        manifest = json.loads(data)
        if manifest['base'] != expected:
            raise ValueError('unexpected exact base for ' + name)
        checked[name] = {'base': expected, 'manifest_bytes': len(data),
                         'manifest_sha256': sha(data),
                         'source_identity': manifest['source_identity']}
    for row in result['new_deltas'].values():
        record = row['patch']
        data = (OUTPUT / record['artifact']).read_bytes()
        if len(data) != record['bytes'] or sha(data) != record['sha256']:
            raise ValueError('authority delta changed')
    result['previous_composition_sha256'] = sha(previous)
    result['source_kit_bases_checked'] = checked
    result['supersedes'] = 'composition-manifest-02.json current source-kit binding only; preserve all earlier evidence'
    result['arm_recipe']['A-initial-common']['runtime_kit'] = 'A-source-03'
    result['arm_recipe']['A-full-release-01']['runtime_kit'] = 'A-source-03'
    result['arm_recipe']['F-full']['runtime_kit'] = 'F-source-04'
    result['arm_recipe']['R-final']['runtime_kit'] = 'R-source-04'
    result['verification_scope'] = 'current manifest/base and unchanged authority patch hash checks; inherited exact-base patch verification, no builds'
    record = emit('composition-manifest-03.json', (json.dumps(result, indent=2, sort_keys=True) + '\n').encode())
    print(json.dumps({'status': 'PASS', 'manifest': record, 'builds_executed': False}))


if __name__ == '__main__':
    main()
