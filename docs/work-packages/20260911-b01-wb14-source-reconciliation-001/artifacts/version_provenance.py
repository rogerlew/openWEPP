#!/usr/bin/env python3
"""Retain one parent-to-commit diff for every reachable available test version."""
from pathlib import Path
import gzip,hashlib,json,subprocess
R=Path('/workdir/openWEPP'); O=Path('/workdir/openwepp-experiments/b01-wb14-source-reconciliation'); D=O/'version-diffs'; D.mkdir(exist_ok=True)
paths=['tests/integration/land_surface_energy_real_hydrology_shadow_contract/precedence_tests.rs','tests/integration/land_surface_energy_real_hydrology_shadow_contract/raw_hash_tests.rs']
def run(*x): return subprocess.check_output(x,text=True)
def sha(b): return hashlib.sha256(b).hexdigest()
out={}
for path in paths:
 commits=run('git','-C',str(R),'log','--all','--reverse','--format=%H','--',path).splitlines(); rows=[]
 dd=D/path.split('/')[-1]; dd.mkdir(exist_ok=True)
 for c in commits:
  b=subprocess.check_output(['git','-C',str(R),'show',f'{c}:{path}']); parent=run('git','-C',str(R),'rev-parse',f'{c}^').strip()
  diff=subprocess.check_output(['git','-C',str(R),'diff',parent,c,'--',path]); p=dd/f'{c}.patch.gz';packed=gzip.compress(diff,mtime=0);p.write_bytes(packed)
  rows.append({'commit':c,'parent':parent,'subject':run('git','-C',str(R),'show','-s','--format=%s',c).strip(),'file_sha256':sha(b),'transition_diff':str(p.relative_to(O)),'encoding':'gzip','compressed_sha256':sha(packed),'transition_diff_sha256':sha(diff),'transition_diff_lines':diff.count(b'\n')})
 out[path]=rows
(O/'available-version-provenance.json').write_text(json.dumps(out,indent=2,sort_keys=True)+'\n')
print(json.dumps({p:len(v) for p,v in out.items()},sort_keys=True))
