#!/usr/bin/env python3
"""Stream only the retained historical pair for checker fixture provenance."""
import hashlib,json,tarfile
from pathlib import Path
import ijson
ARCHIVE=Path('docs/work-packages/20260910-stage3-b01-snow-cycle-integration-001/artifacts/raw-stop-point145.tar.gz')
MEMBER='correction145-warm-tail7/observations.json'
OUT=Path('docs/work-packages/20260912-b01-wb14-available-source-red-witness-001/artifacts/historical-correction145-warm-tail7-pair.json')
class Hashing:
 def __init__(self,f):self.f=f;self.h=hashlib.sha256()
 def read(self,n=-1):
  b=self.f.read(n);self.h.update(b);return b
with tarfile.open(ARCHIVE,'r:gz') as t:
 f=Hashing(t.extractfile(MEMBER)); rows=[]
 for ordinal,row in enumerate(ijson.items(f,'physical.item')):
  if row.get('kind') in {'surface_liquid_wb14_cadence_failure','surface_liquid_wb14_cadence_caller_failure'}:rows.append({'ordinal':ordinal,'row':row})
 if len(rows)!=2:raise SystemExit(f'expected pair, found {len(rows)}')
 OUT.write_text(json.dumps({'provenance':{'archive':str(ARCHIVE),'archive_sha256':'5365a6e47b67d23663f668cbe5c1d6c1b063cf599abcd8be0c37115135c38985','member':MEMBER,'member_sha256':f.h.hexdigest(),'historical_schema_only':True},'records':rows},indent=2)+'\n')
