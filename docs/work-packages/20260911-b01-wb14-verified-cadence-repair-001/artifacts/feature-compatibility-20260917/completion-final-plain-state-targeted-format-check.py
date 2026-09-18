from pathlib import Path
from difflib import SequenceMatcher
import subprocess, tempfile, shutil
root=Path('/home/roger/openwepp-experiments/b01-wb14-feature-compatibility-20260917/crates/openwepp-hillslope-orchestrator/src')
paths='''direct_runtime/00_core_frames.rs direct_runtime/01_publication.rs direct_runtime/02_state_reports.rs direct_runtime/decomposition.rs direct_runtime/evapotranspiration.rs direct_runtime/growth.rs direct_runtime/subsurface.rs hydrology/08_snow_albedo.rs hydrology/support_helpers_mod/snow_mass_transition.rs winter_column.rs'''.split()
for rel in paths:
 p=root/rel; old=p.read_text().splitlines(keepends=True)
 ranges=[(i-3,i+3) for i,line in enumerate(old,1) if line.strip()=='test,']
 with tempfile.TemporaryDirectory() as d:
  q=Path(d)/p.name;shutil.copy2(p,q);subprocess.run(['rustfmt','--edition','2024','--config','skip_children=true',str(q)],check=True,stdout=subprocess.DEVNULL)
  new=q.read_text().splitlines(keepends=True)
  for tag,i1,i2,j1,j2 in SequenceMatcher(None,old,new).get_opcodes():
   if tag!='equal' and any(i1+1<=b and max(i2,i1+1)>=a for a,b in ranges): raise SystemExit(f'unformatted introduced cfg {rel} {i1+1}:{i2}')
print('plain-state introduced cfg ranges rustfmt-clean')
