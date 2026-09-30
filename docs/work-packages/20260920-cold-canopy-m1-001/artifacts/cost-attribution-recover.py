"""Recover private attribution source from frozen14/treatment plus a small overlay.
Usage: .venv/bin/python cost-attribution-recover.py NEW_DESTINATION
Requires sibling projection-cache recovery files and cost-attribution metadata.
External support symlinks retain their original pinned targets.
"""
import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys
import tarfile


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def entries(root):
    result = {}
    for base, dirs, files in os.walk(root, followlinks=False):
        dirs[:] = sorted(d for d in dirs if d not in ('target', '.git'))
        for name in sorted(files + [d for d in dirs if (Path(base)/d).is_symlink()]):
            path = Path(base)/name
            result[str(path.relative_to(root))] = {'link': os.readlink(path)} if path.is_symlink() else sha(path)
    return result


def main():
    here = Path(__file__).resolve().parent
    meta = json.loads((here/'cost-attribution-recovery.json').read_text())
    archive = here/'cost-attribution-source-overlay.tar.gz'
    if sha(archive) != meta['overlay_sha256']:
        raise RuntimeError('Overlay identity mismatch')
    destination = Path(sys.argv[1])
    if destination.exists():
        raise RuntimeError('Recovery destination must be new')
    subprocess.run([sys.executable, str(here/'projection-cache-recover.py'), 'frozen14/treatment', str(destination)], check=True, timeout=180)
    with tarfile.open(archive) as overlay:
        overlay.extractall(destination, filter='data')
    actual = entries(destination)
    if actual != meta['entries']:
        raise RuntimeError('Recovered source differs from frozen attribution source')
    print(json.dumps({'verified': True, 'entries': len(actual), 'source_sha256': hashlib.sha256(json.dumps(actual, sort_keys=True).encode()).hexdigest()}))


if __name__ == '__main__':
    main()
