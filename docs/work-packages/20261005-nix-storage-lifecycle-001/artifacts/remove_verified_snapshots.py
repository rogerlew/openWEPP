#!/usr/bin/env python3
"""One-time exact-path removal; default validates only. Never invokes global GC."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess

HERE = Path(__file__).resolve().parent
DEST = Path('/workdir/openwepp-recovery/nix-snapshots-20261005')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--apply', action='store_true')
    args = parser.parse_args()
    inventory = json.loads((HERE / 'legacy-snapshot-inventory.json').read_text())
    receipts = json.loads((HERE / 'legacy-snapshot-recovery.json').read_text())
    if not isinstance(inventory, list) or not all(isinstance(r, dict) and isinstance(r.get('path'), str) for r in inventory):
        raise RuntimeError('invalid inventory rows')
    inventory_paths = [r['path'] for r in inventory]
    if len(inventory_paths) != len(set(inventory_paths)) or len(inventory_paths) != len(receipts):
        raise RuntimeError('inventory must be unique and match receipt count')
    by_path = {r['source']: r for r in receipts}
    if len(by_path) != len(receipts) or set(by_path) != {r['path'] for r in inventory}:
        raise RuntimeError('complete unique recovery receipts required')
    pins = json.loads((HERE / 'legacy-toolchain-pins.json').read_text())['pins']
    for pin in pins:
        root = Path(pin['root'])
        if not root.is_symlink() or str(root.resolve(strict=True)) != pin['store_path']:
            raise RuntimeError('required legacy toolchain GC root missing or changed')
        roots = subprocess.check_output(['nix-store', '--query', '--roots', pin['store_path']], text=True)
        pairs = [line.split(' -> ', 1) for line in roots.splitlines()]
        if [pin['root'], pin['store_path']] not in pairs:
            raise RuntimeError('required legacy toolchain root is not registered')
    before = shutil.disk_usage('/')
    for item in inventory:
        source = Path(item['path'])
        if source.parent != Path('/nix/store') or not source.name.endswith('-source') or source.is_symlink():
            raise RuntimeError(f'invalid source path: {source}')
        receipt = by_path[str(source)]
        archive = DEST / (source.name + '.nar.zst')
        if receipt['archive'] != str(archive) or receipt['restored_and_verified'] is not True:
            raise RuntimeError('recovery receipt mismatch')
        if any(p.is_symlink() for p in [archive, *archive.parents]):
            raise RuntimeError('archive must not traverse a symlink')
        if archive.stat().st_size != receipt['archive_bytes']:
            raise RuntimeError('archive size mismatch')
        with archive.open('rb') as handle:
            actual = hashlib.file_digest(handle, 'sha256').hexdigest()
        if actual != receipt['archive_sha256']:
            raise RuntimeError('archive corruption; no deletion allowed')
        current = subprocess.check_output(['nix-store', '--query', '--hash', str(source)], text=True).strip()
        if current != receipt['original_nar_hash']:
            raise RuntimeError('source identity changed; no deletion allowed')
    print(f'All {len(inventory)} archives rehashed and source identities checked.', flush=True)
    if not args.apply:
        print('Validation only; no deletion requested.', flush=True)
        return
    outcomes = []
    for item in inventory:
        source = item['path']
        result = subprocess.run(['nix-store', '--delete', source], capture_output=True, text=True)
        outcomes.append({'path': source, 'exit': result.returncode, 'stdout': result.stdout,
                         'stderr': result.stderr, 'absent_after': not Path(source).exists()})
        record = HERE / 'legacy-snapshot-removal.json'
        temporary = record.with_suffix('.tmp')
        with temporary.open('w') as handle:
            handle.write(json.dumps(outcomes, indent=2)+'\n')
            handle.flush()
            os.fsync(handle.fileno())
        temporary.replace(record)
        print(f'{len(outcomes)}/{len(inventory)} exit={result.returncode} {source}', flush=True)
        if result.returncode:
            raise RuntimeError('Nix refused removal; inspect before any further deletion')
    after = shutil.disk_usage('/')
    (HERE / 'legacy-snapshot-space.json').write_text(json.dumps({
        'before_used_bytes': before.used, 'after_used_bytes': after.used,
        'recovered_bytes': before.used-after.used,
        'removed_paths': len(outcomes), 'scope': 'Only inventoried, restored-and-verified openWEPP source snapshots'
    }, indent=2)+'\n')
    print(f'Recovered {(before.used-after.used)/2**30:.3f} GiB', flush=True)


if __name__ == '__main__':
    main()
