#!/usr/bin/env python3
"""Serialize authorized canonical input cuts; no source edits or builds."""
import difflib
import hashlib
import json
from pathlib import Path
import subprocess
import tempfile

BASE = 'e89befa4678eadec039b3e7f7fe0a176af8e9dc5'
COMMON = '2b56e6ebc9c8e8073fb68b89626d4d4e2b4602e3'
ROOT = Path(__file__).resolve().parents[6]
OUTPUT = Path(__file__).resolve().parent
PREFIX = 'docs/specifications/science-contracts/contracts/'
LSE = PREFIX + 'SC-LANDSURFACEENERGY-001.md'
SNOW = PREFIX + 'SC-SNOWENERGY-001.md'


def sha(data):
    return hashlib.sha256(data).hexdigest()


def blob(commit, path):
    return subprocess.check_output(['git', '-C', str(ROOT), 'show', commit + ':' + path])


def emit(path, data):
    destination = OUTPUT / path
    destination.parent.mkdir(parents=True, exist_ok=True)
    with destination.open('xb') as stream:
        stream.write(data)
    return {'artifact': path, 'bytes': len(data), 'sha256': sha(data)}


def main():
    common_lse, snow = blob(COMMON, LSE), blob(COMMON, SNOW)
    current = (ROOT / LSE).read_bytes()
    marker = b'\n### EXP-STAGE3-20260906-R-SG1:'
    if current.count(marker) != 1:
        raise ValueError('SG1 boundary not unique')
    pc1 = current.split(marker)[0].rstrip(b'\n') + b'\n'
    expected = {
        'pc1': 'c993020e96aec0dac934acd5ccfa3050f8173a90bcd7f860b88b8da7405418cd',
        'sg1': '7a002bcac2ad640716b52f4bd59326f241d5fd3084d26e56f6a703eb496ed61d',
        'snow': '94eb7a73ed2d0de50f5a354a200467809154c2e4ecfa0792677942343cc4943d',
    }
    for name, data in [('pc1', pc1), ('sg1', current), ('snow', snow)]:
        if sha(data) != expected[name]:
            raise ValueError('unexpected authorized hash: ' + name)
    if snow != (ROOT / SNOW).read_bytes():
        raise ValueError('common/current snow authority differs')
    cuts = {
        'common-2b': (common_lse, 'exact Git blob ' + COMMON + ':' + LSE),
        'A-full-release-01-PC1': (pc1, 'current SG1 prefix, independently matched frozen PC1 SHA-256'),
        'R-final-SG1': (current, 'current canonical exact dual-reviewed SG1 SHA-256'),
    }
    records = {}
    snow_record = emit('snapshots/SC-SNOWENERGY-001.md', snow)
    for name, (lse, provenance) in cuts.items():
        record = emit('snapshots/' + name + '/SC-LANDSURFACEENERGY-001.md', lse)
        chunks = []
        for path, data in [(LSE, lse), (SNOW, snow)]:
            chunks.extend(difflib.unified_diff(
                blob(BASE, path).decode().splitlines(keepends=True),
                data.decode().splitlines(keepends=True),
                fromfile='a/' + path, tofile='b/' + path))
        patch = ''.join(chunks).encode()
        patch_record = emit('patches/' + name + '.patch', patch)
        with tempfile.TemporaryDirectory(prefix='openwepp-authority-verify-') as temporary:
            scratch = Path(temporary)
            for path in (LSE, SNOW):
                target = scratch / path
                target.parent.mkdir(parents=True, exist_ok=True)
                target.write_bytes(blob(BASE, path))
            subprocess.run(['git', 'apply', '--check', str(OUTPUT / patch_record['artifact'])],
                           cwd=scratch, check=True)
            subprocess.run(['git', 'apply', str(OUTPUT / patch_record['artifact'])],
                           cwd=scratch, check=True)
            if (scratch / LSE).read_bytes() != lse or (scratch / SNOW).read_bytes() != snow:
                raise ValueError('authority patch reconstruction differs: ' + name)
        records[name] = {'lse': record, 'snow': snow_record, 'patch': patch_record,
                         'provenance': provenance, 'patch_reconstruction': 'PASS'}
    manifest = {
        'base': BASE, 'common_commit': COMMON,
        'paths': [LSE, SNOW], 'cuts': records,
        'arm_mapping': {'A-initial-common': 'common-2b',
                        'A-full-release-01': 'A-full-release-01-PC1',
                        'F-full': 'common-2b', 'R-final': 'R-final-SG1'},
        'builds_executed': False,
        'scope': 'two changed canonical adjuncts; all other authority/docs come from FULL base checkout',
    }
    emit('manifest.json', (json.dumps(manifest, indent=2, sort_keys=True) + '\n').encode())
    print(json.dumps({'status': 'PASS', 'cuts': list(records), 'builds_executed': False}))


if __name__ == '__main__':
    main()
