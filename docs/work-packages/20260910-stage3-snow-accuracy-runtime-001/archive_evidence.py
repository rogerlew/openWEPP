#!/usr/bin/env python3
"""Retain raw scratch evidence and referenced fixtures as verified unique blobs."""
import argparse
import gzip
import hashlib
import io
import json
import os
from pathlib import Path
import tarfile

EXCLUDE={'R0','common','candidate','target','reproduction-check','.git','.venv','__pycache__'}
def files(root):
    for directory,dirs,names in os.walk(root):
        dirs[:]=sorted(d for d in dirs if d not in EXCLUDE)
        for name in sorted(names):
            path=Path(directory)/name
            if path.is_file():yield path

def main():
    p=argparse.ArgumentParser();p.add_argument('scratch',type=Path);p.add_argument('archive',type=Path);p.add_argument('index',type=Path);a=p.parse_args()
    receipt_path=a.archive.with_suffix(a.archive.suffix+'.json')
    if a.archive.exists() or a.index.exists() or receipt_path.exists():raise ValueError('refuse archive/index/receipt overwrite')
    paths=list(files(a.scratch.resolve()));fixtures=set()
    for path in paths:
        if path.name=='fixture-path.txt':
            fixture=Path(path.read_text().strip())
            if not fixture.is_absolute() or not fixture.is_relative_to('/tmp') or not fixture.is_dir():
                raise ValueError('missing or foreign fixture path '+str(fixture))
            fixtures.add(fixture)
    for fixture in sorted(fixtures):paths.extend(files(fixture))
    entries=[];objects={};omitted=[]
    for path in sorted(set(paths)):
        if not path.is_relative_to('/tmp'):raise ValueError('raw path outside /tmp')
        with path.open('rb') as stream:
            if stream.read(4)==b'\x7fELF':omitted.append(str(path));continue
        data=path.read_bytes();sha=hashlib.sha256(data).hexdigest();objects.setdefault(sha,path)
        entries.append(dict(path=str(path.relative_to('/')),sha256=sha,size=len(data),mode=path.stat().st_mode&0o777))
    index=dict(schema='snow_accuracy_evidence_archive_v1',entries=entries,fixture_roots=[str(f) for f in sorted(fixtures)],
        omitted_elf_executables=omitted,excluded_directory_names=sorted(EXCLUDE),unique_object_count=len(objects),
        file_count=len(entries),logical_bytes=sum(r['size'] for r in entries),
        scope='raw experiment logs/receipts/projections/source proofs and actual referenced fixture inputs/outputs; source copies, Cargo targets and ELF executables excluded; final source patches/scripts/authority live beside archive in repository')
    encoded=(json.dumps(index,indent=2)+'\n').encode();a.index.write_bytes(encoded)
    a.archive.parent.mkdir(parents=True,exist_ok=True)
    with a.archive.open('xb') as raw,gzip.GzipFile(filename='',fileobj=raw,mode='wb',mtime=0) as zipped,tarfile.open(fileobj=zipped,mode='w|') as tar:
        def add(name,data):
            member=tarfile.TarInfo(name);member.size=len(data);member.mode=0o600;member.mtime=0
            tar.addfile(member,io.BytesIO(data))
        add('index.json',encoded)
        for sha,path in sorted(objects.items()):
            data=path.read_bytes()
            if hashlib.sha256(data).hexdigest()!=sha:raise ValueError('raw file changed while archiving '+str(path))
            add('objects/'+sha,data)
    receipt=dict(archive=str(a.archive),archive_sha256=hashlib.sha256(a.archive.read_bytes()).hexdigest(),
        archive_bytes=a.archive.stat().st_size,index_sha256=hashlib.sha256(encoded).hexdigest(),
        file_count=len(entries),unique_object_count=len(objects),fixture_count=len(fixtures))
    receipt_path.write_text(json.dumps(receipt,indent=2)+'\n');print(json.dumps(receipt))
if __name__=='__main__':main()
