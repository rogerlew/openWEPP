"""Explicit prospective finite section unions, not a dependency engine or telemetry."""
from pathlib import Path
import json
import re
ROOT=Path(__file__).resolve().parents[4]
P=ROOT/'docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001'
BASE=264863
COMMON=['variables-and-units-using-canonical-symbols-first','algorithm-state-surfaces','algorithm-specification-with-step-sequence','branch-and-guard-table','symbol-alias-map','constants-and-parameters-with-provenance-anchors','unit-governance-map','tolerance-and-numeric-notes','calibration-and-identifiability','test-vector-obligations']
ORDER='ordered-numerical-algorithm-active-branches-and-error-precedence'
MAP=['validated-in-memory-lse-custody-handoff-amendment','snow-free-final-receipt-reseal-amendment','covered-nonfinal-physical-only-map-amendment']

class Selection:
    def __init__(self):self.rows={}
    def add(self,name,anchor=None):
        path=P.with_suffix('.md') if name=='entry' else P/(name+'.md')
        lines=path.read_text().splitlines(keepends=True)
        idx=self.rows.setdefault(path,set())
        if anchor is None:idx.update(range(len(lines)));return
        a=next(i for i,l in enumerate(lines) if f'<a id="{anchor}"></a>' in l)
        if anchor.startswith(('INV-','OBL-')):
            idx.add(a)
            h=max(i for i,l in enumerate(lines[:a]) if l.startswith('| Invariant ID | Statement') or l.startswith('| Obligation ID | Statement'))
            idx.update([h,h+1])
        else:
            h=next(i for i in range(a,len(lines)) if re.match(r'^#{1,6} ',lines[i]))
            level=len(lines[h])-len(lines[h].lstrip('#'))
            b=next((i for i in range(h+1,len(lines)) if re.match(r'^#{1,'+str(level)+r'} ',lines[i])),len(lines))
            if b<len(lines) and lines[b-1].startswith('<a id='):b-=1
            idx.update(range(a,b))
        # Dependency header and chapter introduction are always inspected.
        first=next(i for i,l in enumerate(lines) if l.startswith('# '))
        after=next((i for i in range(first+1,len(lines)) if lines[i].startswith('<a id=')),len(lines))
        idx.update(range(after))
    def inv(self,name,nums):
        for n in nums:self.add(name,f'INV-LANDSURFACEENERGY-{n:03}')
    def obl(self,name,nums):
        for n in nums:self.add(name,f'OBL-LANDSURFACEENERGY-C-{n:03}')
    def result(self):
        rows=[]
        for path,indices in self.rows.items():
            lines=path.read_bytes().splitlines(keepends=True); spans=[]
            for i in sorted(indices):
                if spans and spans[-1][1]==i:spans[-1][1]=i+1
                else:spans.append([i+1,i+1])
            rows.append({'path':str(path.relative_to(ROOT)),'ranges':spans,'bytes':sum(len(lines[i]) for i in indices)})
        size=sum(r['bytes'] for r in rows)
        return {'lse_bytes':size,'reduction_percent':100*(1-size/BASE),'readings':rows}

out={}
for task in range(1,5):
    s=Selection();s.add('entry');s.add('interface')
    if task!=3:s.add('common-details')
    if task in [1,4]:
        for n in ['surface-energy','soil-coupling','water-vapor','litter-phase','soil-custody','surface-custody','solve-boundary','terminal-support','map-custody']:s.add(n)
        s.add('nonlinear-solve')
        # Include actual surface08 full-method expansion prospectively.
        if task==1:s.add('numerical-methods')
        else:s.add('numerical-methods','canonical-invariants')
        if task==4:s.add('qualification')
    elif task==2:
        s.add('nonlinear-solve')
        for n in ['numerical-methods','solve-boundary','dependency-replay','replay-evidence','surface-energy','soil-coupling','terminal-support','map-custody']:s.add(n)
        # Conservative preflight includes the full water chapter after actual
        # readers chose it twice; no smaller theoretical substitute for exposure.
        s.add('water-vapor')
        for version in [28,29,30]:s.add('binding-index',f'profile-v{version}')
    else:s.add('replay-evidence')
    out[f'exercise{task:02}']=s.result()
result={'kind':'prospective explicit source selection; sufficiency not yet qualified; not observed tokens','baseline_lse_bytes':BASE,'always_read_bytes':P.with_suffix('.md').stat().st_size+(P/'interface.md').stat().st_size,'tasks':out}
(Path(__file__).parent/'predicted-reading.json').write_text(json.dumps(result,indent=2)+'\n')
print({k:(v['lse_bytes'],round(v['reduction_percent'],1)) for k,v in out.items()})
