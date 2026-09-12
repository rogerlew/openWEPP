#!/usr/bin/env python3
"""Hash all relevant source/config inputs in fresh recipe replay and retained source."""
from __future__ import annotations
import hashlib,json,sys
from pathlib import Path
OUT=Path('/workdir/openwepp-experiments/b01-wb14-source-reconciliation')
RETAINED=Path('/workdir/openwepp-experiments/b01-wb14-cadence/recovery-inputs')
FRESH=Path(sys.argv[1])
def sha(p): return hashlib.sha256(p.read_bytes()).hexdigest()
def relevant(root):
 paths=[]
 for p in root.rglob('*'):
  if not p.is_file() or p.is_symlink(): continue
  r=str(p.relative_to(root))
  if r in ('Cargo.toml','Cargo.lock','.config/nextest.toml','.cargo/config.toml') or r.startswith('crates/') or r.startswith('tests/integration/land_surface_energy_real_hydrology_shadow_contract'):
   paths.append(r)
 return {r:sha(root/r) for r in sorted(paths)}
def digest(d): return hashlib.sha256(json.dumps(d,sort_keys=True,separators=(',',':')).encode()).hexdigest()
fresh,retn=relevant(FRESH),relevant(RETAINED)
allpaths=sorted(set(fresh)|set(retn)); mismatches=[{'path':p,'fresh':fresh.get(p),'retained':retn.get(p)} for p in allpaths if fresh.get(p)!=retn.get(p)]
result={'fresh_root':str(FRESH),'retained_root':str(RETAINED),'relevant_path_definition':'root Cargo/config plus every regular non-symlink crates/** path and target test root/module paths','fresh_path_sha256':fresh,'retained_path_sha256':retn,'fresh_digest':digest(fresh),'retained_digest':digest(retn),'paths':len(allpaths),'mismatches':mismatches}
(OUT/'fresh-recipe-relevant-input-comparison.json').write_text(json.dumps(result,indent=2,sort_keys=True)+'\n')
print(json.dumps({'paths':len(allpaths),'mismatches':len(mismatches),'fresh_digest':digest(fresh),'retained_digest':digest(retn)},sort_keys=True))
