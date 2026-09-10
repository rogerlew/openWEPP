#!/usr/bin/env python3
"""Freeze test executables selected from actual Cargo compiler artifacts."""
import argparse
import hashlib
import json
from pathlib import Path
import shutil

p=argparse.ArgumentParser();p.add_argument('cargo_log',type=Path);p.add_argument('directory',type=Path);p.add_argument('version');a=p.parse_args()
a.directory.mkdir(parents=True,exist_ok=True)
artifacts=[]
for line in a.cargo_log.read_text().splitlines():
    try:row=json.loads(line)
    except ValueError:continue
    if row.get('reason')=='compiler-artifact' and row.get('executable') and row['target']['kind']==['lib']:
        artifacts.append(row)
rows=[]
for target,label in [('openwepp_runner','runner'),('openwepp_hillslope_orchestrator','orchestrator'),('openwepp_land_surface_energy','lse')]:
    matches=[r for r in artifacts if r['target']['name']==target and r['profile']['test']]
    if len(matches)!=1:raise ValueError('ambiguous actual Cargo artifact '+target)
    source=Path(matches[0]['executable']);dest=a.directory/f'final-{label}-v{a.version}'
    if dest.exists() or dest.with_suffix('.cargo.json').exists():raise ValueError('refuse to overwrite frozen executable/receipt')
    before=hashlib.sha256(source.read_bytes()).hexdigest()
    shutil.copy2(source,dest)
    after=hashlib.sha256(source.read_bytes()).hexdigest()
    copied=hashlib.sha256(dest.read_bytes()).hexdigest()
    if not before==after==copied:raise ValueError('executable changed during freeze')
    receipt=dict(cargo_artifact=matches[0],cargo_log_sha256=hashlib.sha256(a.cargo_log.read_bytes()).hexdigest(),
        executable=str(dest.resolve()),sha256=hashlib.sha256(dest.read_bytes()).hexdigest())
    dest.with_suffix('.cargo.json').write_text(json.dumps(receipt,indent=2)+'\n');rows.append(receipt)
print(json.dumps(rows,indent=2))
