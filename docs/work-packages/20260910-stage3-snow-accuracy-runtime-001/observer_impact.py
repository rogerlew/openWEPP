#!/usr/bin/env python3
"""Serial full/minimal observation pairs; output/work neutrality, no subtraction."""
import argparse
import datetime
import json
from pathlib import Path
from collect import collect,digest,PACKAGE
p=argparse.ArgumentParser();p.add_argument('binary',type=Path);p.add_argument('source',type=Path);p.add_argument('directory',type=Path);a=p.parse_args()
a.directory.mkdir(parents=True,exist_ok=False)
jobs=[(policy,physical) for policy in ['R0','C600','B01'] for physical in [True,False]]
frozen={str(p.resolve()):digest(p) for p in [a.binary,a.source,Path(__file__),PACKAGE/'collect.py',PACKAGE/'artifacts/cases.json',PACKAGE/'artifacts/metrics.json']}
(a.directory/'protocol.json').write_text(json.dumps(dict(created_utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),
    jobs=jobs,frozen_files=frozen,scope='one diagnostic pair per policy, full then minimal; no timing subtraction or statistical observer estimate'),indent=2)+'\n')
rows=[]
for policy,physical in jobs:
    if any(digest(Path(p))!=sha for p,sha in frozen.items()):raise ValueError('frozen observer protocol changed')
    directory=a.directory/f"{policy}-{'full' if physical else 'minimal'}"
    r=collect(a.binary,directory,policy,'continuity',physical=physical)
    rows.append(dict(policy=policy,physical=physical,directory=str(directory),receipt_sha256=digest(directory/'receipt.json'),receipt=r))
    print(json.dumps(dict(policy=policy,physical=physical,valid=r['execution_valid'],runner_wall_s=r.get('runner_wall_s'))),flush=True)
comparisons=[]
for policy in ['R0','C600','B01']:
    f,m=[r for r in rows if r['policy']==policy];valid=f['receipt']['execution_valid'] and m['receipt']['execution_valid']
    row=dict(policy=policy,both_execution_valid=valid,full_s=f['receipt'].get('runner_wall_s'),minimal_s=m['receipt'].get('runner_wall_s'))
    if valid:
        read=lambda r,name:json.loads((Path(r['directory'])/name).read_text())
        fr,mr=read(f,'run.json'),read(m,'run.json')
        row.update(input_files_equal=read(f,'inputs.json')['files']==read(m,'inputs.json')['files'],
            output_files_equal=read(f,'outputs.json')['files']==read(m,'outputs.json')['files'],
            carrier_work_equal=fr['carrier']==mr['carrier'],lse_work_equal=fr['lse']==mr['lse'],
            full_minus_minimal_s=row['full_s']-row['minimal_s'])
        fo,mo=read(f,'observations.json'),read(m,'observations.json')
        parents=lambda run:[{k:v for k,v in parent.items() if k!='parent_seconds'} for parent in run['parents']]
        row.update(snapshot_equal=fr['snapshot']==mr['snapshot'],parent_work_equal=parents(fr)==parents(mr),
            method_counts_equal=fo['counts']==mo['counts'],minimal_has_no_physical=not mo.get('physical') and not mo.get('physical_ledgers'))
    checks=['input_files_equal','output_files_equal','carrier_work_equal','lse_work_equal','snapshot_equal',
        'parent_work_equal','method_counts_equal','minimal_has_no_physical']
    row['neutrality']='PASS' if valid and all(row.get(k) is True for k in checks) else 'FAIL_OR_UNRESOLVED'
    comparisons.append(row)
(a.directory/'summary.json').write_text(json.dumps(dict(samples=rows,comparisons=comparisons),indent=2)+'\n')
