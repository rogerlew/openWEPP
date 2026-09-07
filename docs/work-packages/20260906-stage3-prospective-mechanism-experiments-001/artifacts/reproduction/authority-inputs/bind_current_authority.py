#!/usr/bin/env python3
"""Separate current arm authority recipes from preserved historical labels."""
import json
from pathlib import Path

from freeze_authority import BASE, COMMON, LSE, SNOW, ROOT, OUTPUT, emit, sha


def main():
    previous = (OUTPUT / 'composition-manifest-04.json').read_bytes()
    result = json.loads(previous)
    original = json.loads((OUTPUT / 'manifest.json').read_bytes())
    mapping = [
        ('current-A-SG1', 'A-source-04', BASE, 'R-final-SG1', 'patches/R-final-SG1.patch', ROOT),
        ('current-F-common', 'F-source-05', COMMON, 'common-2b', None,
         Path('/tmp/openwepp-controlled-mechanisms-Hb6uS2/F')),
        ('current-R-SG1', 'R-source-05', COMMON, 'R-final-SG1',
         'patches/R-final-SG1-from-2b.patch', Path('/tmp/openwepp-controlled-mechanisms-Hb6uS2/R')),
    ]
    recipes = {}
    for label, kit, base, cut, patch, checkout in mapping:
        canonical = {}
        for path, key in [(LSE, 'lse'), (SNOW, 'snow')]:
            expected = original['cuts'][cut][key]
            data = (checkout / path).read_bytes()
            if sha(data) != expected['sha256'] or len(data) != expected['bytes']:
                raise ValueError('current canonical mismatch: ' + label + '/' + key)
            canonical[path] = {'bytes': len(data), 'sha256': sha(data)}
        record = {'runtime_kit': kit, 'checkout': base, 'target_cut': cut,
                  'canonical_files': canonical, 'authority_patch': patch}
        if patch:
            data = (OUTPUT / patch).read_bytes()
            record['authority_patch_bytes'] = len(data)
            record['authority_patch_sha256'] = sha(data)
        else:
            record['action'] = 'no-op; verify existing common canonical hashes'
        recipes[label] = record
    result['previous_composition_sha256'] = sha(previous)
    result['current_arm_recipe'] = recipes
    result['historical_recipe_note'] = ('arm_recipe retains earlier authority labels only; '
        'use original run source-kit receipts/earlier compositions for exact historical reproduction; '
        'CURRENT kits use current_arm_recipe exclusively')
    result['supersedes'] = 'composition-manifest-04.json current authority selection only; historical inputs preserved'
    record = emit('composition-manifest-05.json', (json.dumps(result, indent=2, sort_keys=True) + '\n').encode())
    print(json.dumps({'status': 'PASS', 'manifest': record, 'builds_executed': False}))


if __name__ == '__main__':
    main()
