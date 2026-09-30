"""Restore a frozen arm from the accepted archive and small experiment overlay.
Usage: python projection-cache-recover.py frozen14/treatment DESTINATION
The accepted archive, overlay and recovery JSON must be beside this script.
External support symlinks are retained; authenticate their targets with recorded pins.
"""
import hashlib
import json
import os
from pathlib import Path
import sys
import tarfile


def sha(p):
    return hashlib.sha256(Path(p).read_bytes()).hexdigest()


def main():
    here=Path(__file__).resolve().parent
    meta=json.loads((here/'projection-cache-recovery.json').read_text())
    cut,destination=sys.argv[1],Path(sys.argv[2])
    if destination.exists():
        raise RuntimeError('Destination must be new')
    for name,key in [('bvls05-terminal09-source.tar.gz','base_archive_sha256'),
                     ('projection-cache-source-overlay.tar.gz','overlay_sha256')]:
        if sha(here/name)!=meta[key]:
            raise RuntimeError('Archive hash mismatch: '+name)
    entries=meta['cuts'][cut]['entries']
    destination.mkdir(parents=True)
    with tarfile.open(here/'bvls05-terminal09-source.tar.gz') as base:
        # This authenticated base contains regular source files only.
        base.extractall(destination,filter='data')
    with tarfile.open(here/'projection-cache-source-overlay.tar.gz') as overlay:
        for name,value in entries.items():
            p=destination/name
            if isinstance(value,dict):
                p.parent.mkdir(parents=True,exist_ok=True)
                if p.exists() or p.is_symlink(): p.unlink()
                p.symlink_to(value['link'])
            elif not p.is_file() or sha(p)!=value:
                item=overlay.extractfile(cut+'/'+name)
                if item is None: raise RuntimeError('Missing overlay '+name)
                p.parent.mkdir(parents=True,exist_ok=True)
                p.write_bytes(item.read())
    actual={}
    for root,dirs,files in os.walk(destination,followlinks=False):
        dirs[:]=sorted(d for d in dirs if d not in ('target','.git'))
        for name in sorted(files+[d for d in dirs if (Path(root)/d).is_symlink()]):
            p=Path(root)/name
            actual[str(p.relative_to(destination))]={'link':os.readlink(p)} if p.is_symlink() else sha(p)
    if actual!=entries:
        raise RuntimeError('Recovered tree differs from frozen source')
    print(json.dumps({'cut':cut,'entries':len(actual),'source_sha256':hashlib.sha256(json.dumps(actual,sort_keys=True).encode()).hexdigest(),'verified':True}))


if __name__=='__main__': main()
