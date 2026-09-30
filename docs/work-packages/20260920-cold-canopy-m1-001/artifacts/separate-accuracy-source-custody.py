"""Pack, authenticate, and recover the detached diagnostic source, without links traversal."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import tarfile


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def entries(root):
    result = {}
    for directory, dirs, files in os.walk(root, followlinks=False):
        dirs[:] = sorted(d for d in dirs if d not in {'.git', 'target'})
        for name in sorted(files + [d for d in dirs if (Path(directory) / d).is_symlink()]):
            path = Path(directory) / name
            key = str(path.relative_to(root))
            result[key] = {'link': os.readlink(path)} if path.is_symlink() else sha(path)
    return result


def identity(value):
    return hashlib.sha256(json.dumps(value, sort_keys=True).encode()).hexdigest()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('action', choices=['pack', 'check', 'recover'])
    parser.add_argument('root', type=Path)
    parser.add_argument('archive', type=Path)
    parser.add_argument('receipt', type=Path)
    args = parser.parse_args()
    if args.action == 'pack':
        if args.archive.exists() or args.receipt.exists():
            raise RuntimeError('Use a new cut; never overwrite frozen source evidence')
        before = entries(args.root)
        with tarfile.open(args.archive, 'w:gz') as archive:
            for name, value in sorted(before.items()):
                if isinstance(value, str):
                    archive.add(args.root / name, arcname=name, recursive=False)
        after = entries(args.root)
        if after != before:
            raise RuntimeError('Source changed during archive creation')
        result = {'schema': 1, 'source_sha256': identity(before),
                  'archive_sha256': sha(args.archive), 'entries': before,
                  'scope': 'Detached source only; external symlink targets and resulting binary need separate build input pins.'}
        args.receipt.write_text(json.dumps(result, indent=2, sort_keys=True) + '\n')
    else:
        result = json.loads(args.receipt.read_text())
        if sha(args.archive) != result['archive_sha256']:
            raise RuntimeError('Archive identity mismatch')
        if identity(result['entries']) != result['source_sha256']:
            raise RuntimeError('Entry identity mismatch')
        if args.action == 'recover':
            if args.root.exists():
                raise RuntimeError('Recovery destination must not exist')
            args.root.mkdir(parents=True)
            with tarfile.open(args.archive) as archive:
                for member in archive.getmembers():
                    if not member.isfile():
                        raise RuntimeError('Only regular files belong in source archive')
                archive.extractall(args.root, filter='data')
            for name, value in result['entries'].items():
                if isinstance(value, dict):
                    path = args.root / name
                    if Path(name).is_absolute() or '..' in Path(name).parts:
                        raise RuntimeError('Invalid link path')
                    path.parent.mkdir(parents=True, exist_ok=True)
                    path.symlink_to(value['link'])
        if entries(args.root) != result['entries']:
            raise RuntimeError('Actual source differs from receipt')
    print(json.dumps({'action': args.action, 'source_sha256': result['source_sha256'],
                      'entries': len(result['entries']), 'verified': True}))


if __name__ == '__main__':
    main()
