"""Extract mechanical private decoder field lists from the pinned Rust source."""
from pathlib import Path
import re,json
S=Path('/workdir/openwepp-experiments/b01-wb14-cadence/native-context-restoration-reader-20260916/crates/openwepp-hillslope-orchestrator/src')
HERE=Path(__file__).resolve().parent
index={}
for p in S.rglob('*.rs'):
 if 'test' in p.name or 'tests' in p.parts:continue
 s=p.read_text()
 for m in re.finditer(r'pub(?:\([^)]*\))?\s+(struct|enum)\s+(\w+)\s*\{',s):
  depth=1;i=m.end()
  while depth:
   if s[i]=='{':depth+=1
   elif s[i]=='}':depth-=1
   i+=1
  body=s[m.end():i-1]
  fields=re.findall(r'^\s*pub(?:\([^)]*\))?\s+(\w+)\s*:\s*([^,]+(?:<[^;]*?)?),\s*$',body,re.M) if False else re.findall(r'^\s*pub(?:\([^)]*\))?\s+(\w+)\s*:\s*([^\n]+),\s*$',body,re.M)
  index[m[2]]=dict(kind=m[1],path=str(p.relative_to(S)),line=s[:m.start()].count('\n')+1,body=body,fields=fields)
roots=['DirectDayConstructorInputs','DirectSnowRuntimeCarry','DirectFrostRuntimeCarry','DirectGrowthStateSurface','DirectEvapotranspirationStageState','DirectSubsurfaceLayerState','DirectPublicationFrame','DirectTransferBuffers','DirectSnowSolidToLiquidLedger','DirectSnowLiquidDispositionLedger','DirectSnowStage3Outcome']
seen={};todo=roots.copy()
while todo:
 name=todo.pop()
 if name in seen:continue
 if name not in index:raise RuntimeError(name)
 entry=index[name];seen[name]=entry
 for field,ty in entry['fields']:
  todo.extend(n for n in re.findall(r'\b\w+\b',ty) if n in index and n not in seen)
# Sealed bundle uses its existing validating public constructor.
seen.pop('DirectSnowMassTransitionLedgers',None)
(HERE/'constructor-type-map.json').write_text(json.dumps(seen,indent=2)+'\n')
for name,e in sorted(seen.items()):print(name,e['kind'],len(e['fields']),e['path'], 'EMPTY' if not e['fields'] and e['kind']=='struct' else '')
