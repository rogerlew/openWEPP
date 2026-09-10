#!/usr/bin/env python3
"""Verify and restore this packet; never overwrite changed existing evidence."""
import argparse
import hashlib
import json
from pathlib import Path,PurePosixPath
import tarfile

def main():
    p=argparse.ArgumentParser();p.add_argument('archive',type=Path);p.add_argument('--root',type=Path,required=True);p.add_argument('--verify-only',action='store_true');a=p.parse_args()
    root=a.root.resolve();restored=existing=0
    with tarfile.open(a.archive,'r:gz') as tar:
        members={}
        for member in tar.getmembers():
            if member.name in members or not member.isfile():raise ValueError('duplicate/non-file archive member')
            members[member.name]=member
        index=json.load(tar.extractfile(members['index.json']))
        if index['schema']!='snow_accuracy_evidence_archive_v1':raise ValueError('archive schema')
        entries=index['entries'];seen=set();required=set();objects={}
        for entry in entries:
            relative=PurePosixPath(entry['path'])
            if relative.is_absolute() or '..' in relative.parts or not relative.parts or relative.parts[0]!='tmp' or str(relative) in seen:
                raise ValueError('invalid/duplicate restore path')
            seen.add(str(relative));required.add(entry['sha256'])
        if set(members)!={'index.json'}|{'objects/'+sha for sha in required}:raise ValueError('archive object coverage')
        for sha in sorted(required):
            data=tar.extractfile(members['objects/'+sha]).read()
            if hashlib.sha256(data).hexdigest()!=sha:raise ValueError('corrupt object '+sha)
            objects[sha]=data
        for entry in entries:
            data=objects[entry['sha256']]
            if len(data)!=entry['size']:raise ValueError('object size mismatch')
            if a.verify_only:continue
            destination=root/entry['path']
            if not destination.resolve().is_relative_to(root) or not destination.resolve().is_relative_to((root/'tmp').resolve()):raise ValueError('restore path escapes temporary root')
            if destination.exists():
                if not destination.is_file() or hashlib.sha256(destination.read_bytes()).hexdigest()!=entry['sha256']:
                    raise ValueError('refuse to overwrite changed existing evidence '+str(destination))
                existing+=1;continue
            destination.parent.mkdir(parents=True,exist_ok=True)
            with destination.open('xb') as stream:stream.write(data)
            destination.chmod(entry['mode']&0o777);restored+=1
    print(json.dumps(dict(verified_files=len(entries),verified_objects=len(objects),restored_files=restored,unchanged_existing=existing,verify_only=a.verify_only)))
if __name__=='__main__':main()
