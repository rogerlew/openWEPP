#!/usr/bin/env python3
"""Bind final A04/F05/R05 manifests without replacing earlier evidence."""
import json

from freeze_authority import BASE, COMMON, OUTPUT, emit, sha


def main():
    previous = (OUTPUT / 'composition-manifest-03.json').read_bytes()
    result = json.loads(previous)
    checked = {}
    kits = [
        ('A-source-04', BASE, '98ee7bafe471bfc73fec07909b3bb81f16cc55ab4518675fab62d621fc2ae4ae'),
        ('F-source-05', COMMON, '7a41232f6e513fab954244151c74fac5b9426acc0d83cca372defeb19856dc0f'),
        ('R-source-05', COMMON, '28c3310d916593bc941706e36bf6d4188ebd755b50d5ebb8cc356f9459e05027'),
    ]
    for name, expected_base, expected_identity in kits:
        data = (OUTPUT.parent / name / 'source-manifest.json').read_bytes()
        manifest = json.loads(data)
        if manifest['base'] != expected_base or manifest['source_identity'] != expected_identity:
            raise ValueError('unexpected final identity/base for ' + name)
        checked[name] = {'base': expected_base, 'manifest_bytes': len(data),
                         'manifest_sha256': sha(data), 'source_identity': expected_identity}
    for row in result['new_deltas'].values():
        record = row['patch']
        data = (OUTPUT / record['artifact']).read_bytes()
        if len(data) != record['bytes'] or sha(data) != record['sha256']:
            raise ValueError('authority delta changed')
    result['previous_composition_sha256'] = sha(previous)
    result['source_kit_bases_checked'] = checked
    result['supersedes'] = 'composition-manifest-03.json current source-kit binding only; earlier evidence preserved'
    for name in ['A-initial-common', 'A-full-release-01']:
        result['arm_recipe'][name]['runtime_kit'] = 'A-source-04'
    result['arm_recipe']['F-full']['runtime_kit'] = 'F-source-05'
    result['arm_recipe']['R-final']['runtime_kit'] = 'R-source-05'
    result['verification_scope'] = 'final manifest/base/source identity and unchanged authority patch checks; inherited exact-base patch verification, no builds'
    record = emit('composition-manifest-04.json', (json.dumps(result, indent=2, sort_keys=True) + '\n').encode())
    print(json.dumps({'status': 'PASS', 'manifest': record, 'builds_executed': False}))


if __name__ == '__main__':
    main()
