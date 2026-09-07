#!/usr/bin/env python3
"""Record transparent executable input identity and complete local delta custody."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import subprocess
import tarfile

BASE = 'e89befa4678eadec039b3e7f7fe0a176af8e9dc5'

def git(root, *args):
    return subprocess.check_output(['git', '-C', str(root), *args])

def executable_input(path):
    return (path.startswith(('crates/', 'src/', 'tests/', '.cargo/', '.config/', 'tools/'))
            or path in ('Cargo.toml', 'Cargo.lock', 'flake.nix', 'flake.lock',
                        'rust-toolchain', 'rust-toolchain.toml', 'build.rs'))

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--root', type=Path, required=True)
    parser.add_argument('--out', type=Path, required=True)
    parser.add_argument('--name', required=True)
    parser.add_argument('--base', default=BASE)
    args = parser.parse_args()
    args.out.mkdir(parents=True, exist_ok=False)
    paths = sorted(set(git(args.root, 'ls-files', '-z').decode().split('\0')) |
                   set(git(args.root, 'ls-files', '--others', '--exclude-standard', '-z').decode().split('\0')))
    rows = []
    for path in paths:
        if not executable_input(path):
            continue
        full = args.root / path
        if not full.exists():
            rows.append({'path':path, 'state':'deleted'})
        elif full.is_file():
            data = full.read_bytes()
            rows.append({'path':path, 'state':'present', 'bytes':len(data),
                         'sha256':hashlib.sha256(data).hexdigest(),
                         'mode':oct(full.stat().st_mode & 0o777),
                         'symlink':os.readlink(full) if full.is_symlink() else None})
    encoded = json.dumps(rows, sort_keys=True, separators=(',', ':')).encode()
    (args.out / 'path-content-manifest.json').write_bytes(encoded + b'\n')
    patch = git(args.root, 'diff', '--binary', args.base, '--', 'crates', 'src', 'tests',
                '.cargo', '.config', 'tools', 'Cargo.toml', 'Cargo.lock', 'flake.nix',
                'flake.lock', 'rust-toolchain', 'rust-toolchain.toml', 'build.rs')
    (args.out / 'tracked-source.patch').write_bytes(patch)
    untracked = [p for p in git(args.root, 'ls-files', '--others', '--exclude-standard', '-z').decode().split('\0')
                 if executable_input(p) and (args.root/p).is_file()]
    with tarfile.open(args.out / 'new-source-files.tar.gz', 'w:gz') as archive:
        for path in sorted(untracked):
            archive.add(args.root/path, arcname=path, recursive=False)
    changed = git(args.root, 'diff', '--name-only', '-z', args.base).decode().split('\0')
    with tarfile.open(args.out / 'changed-source-snapshot.tar.gz', 'w:gz') as archive:
        for path in sorted(set(changed+untracked)):
            if executable_input(path) and (args.root/path).is_file():
                archive.add(args.root/path, arcname=path, recursive=False)
    manifest = {'name':args.name, 'base':args.base,
                'checkout':git(args.root, 'rev-parse', 'HEAD').decode().strip(),
                'source_identity':hashlib.sha256(encoded).hexdigest(),
                'identity_inputs':'path-content-manifest.json',
                'patch_sha256':hashlib.sha256(patch).hexdigest(),
                'untracked_paths':sorted(untracked),
                'reproduce':['checkout exact base detached', 'git apply tracked-source.patch',
                             'extract new-source-files.tar.gz at root',
                             'verify every path-content row before building'],
                'documentary_output_excluded':True}
    (args.out/'source-manifest.json').write_text(json.dumps(manifest,indent=2)+'\n')
    print(json.dumps(manifest))

if __name__ == '__main__':
    main()
