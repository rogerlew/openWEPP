#!/usr/bin/env python3
"""Hash and compare every regular file and every symlink target in two source snapshots."""
from __future__ import annotations
import hashlib,json,os
from pathlib import Path
OUT=Path('/workdir/openwepp-experiments/b01-wb14-source-reconciliation')
FRESH=OUT/'reconstructed-available145'
RETAINED=Path('/workdir/openwepp-experiments/b01-wb14-cadence/recovery-inputs')
def sha(p):
 h=hashlib.sha256()
 with p.open('rb') as f:
  for b in iter(lambda:f.read(1024*1024),b''): h.update(b)
 return h.hexdigest()
def census(root):
 files={}; links={}
 for base,dirs,names in os.walk(root,followlinks=False):
  for n in dirs+names:
   p=Path(base)/n; r=str(p.relative_to(root))
   if p.is_symlink(): links[r]=os.readlink(p)
   elif p.is_file(): files[r]=sha(p)
 return files,links
def digest(x): return hashlib.sha256(json.dumps(x,sort_keys=True,separators=(',',':')).encode()).hexdigest()
ff,fl=census(FRESH); rf,rl=census(RETAINED)
allf=sorted(set(ff)|set(rf)); alll=sorted(set(fl)|set(rl))
result={'fresh_root':str(FRESH),'retained_root':str(RETAINED),'fresh_regular_file_sha256':ff,'retained_regular_file_sha256':rf,'fresh_symlink_targets':fl,'retained_symlink_targets':rl,'regular_file_digest':digest(rf),'symlink_target_digest':digest(rl),'regular_file_mismatches':[{'path':p,'fresh':ff.get(p),'retained':rf.get(p)} for p in allf if ff.get(p)!=rf.get(p)],'symlink_mismatches':[{'path':p,'fresh':fl.get(p),'retained':rl.get(p)} for p in alll if fl.get(p)!=rl.get(p)]}
result['external_symlink_resolutions']=[{'path':p,'target':t,'resolved':str((RETAINED/p).parent.joinpath(t).resolve(strict=False))} for p,t in rl.items() if not str((RETAINED/p).parent.joinpath(t).resolve(strict=False)).startswith(str(RETAINED))]
(OUT/'fresh-recipe-whole-source-comparison.json').write_text(json.dumps(result,indent=2,sort_keys=True)+'\n')
print(json.dumps({'regular_files':len(rf),'symlinks':len(rl),'regular_mismatches':len(result['regular_file_mismatches']),'symlink_mismatches':len(result['symlink_mismatches']),'regular_digest':result['regular_file_digest'],'symlink_digest':result['symlink_target_digest']},sort_keys=True))
