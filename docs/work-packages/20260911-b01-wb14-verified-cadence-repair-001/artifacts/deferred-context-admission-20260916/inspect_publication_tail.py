"""Decode retained publication history, preserving exact bytes separately."""
import hashlib,json
from pathlib import Path
import ijson
ROOT=Path('/workdir/openwepp-experiments/b01-wb14-cadence/corrected-recorder-export-20260916-1')
OUT=Path('/home/roger/openwepp-experiments/b01-wb14-deferred-context-admission-20260916')
OUT.mkdir(parents=True,exist_ok=True)
with (ROOT/'rows/123091.19.json').open('rb') as f:
 raw=bytes(next(ijson.items(f,'accepted_publication_rotation')))
p=OUT/'accepted-publication-rotation.json';p.write_bytes(raw)
history=json.loads(raw)
rows=history['resident_supports']
selected=rows[660:668]
p=OUT/'publication-tail-660-667.json';p.write_text(json.dumps(selected,indent=2)+'\n')
result={'evidence_class':'Ran: retained byte-array decode only','rotation_bytes':len(raw),'rotation_sha256':hashlib.sha256(raw).hexdigest(),'resident_support_count':len(rows),'selected_path':str(p),'selected_sha256':hashlib.sha256(p.read_bytes()).hexdigest(),'selected_keys':list(selected[0]),'selected_noncontainer_fields':[{k:v for k,v in row.items() if not isinstance(v,(list,dict))} for row in selected],'selected_container_summaries':{k: {'type':type(v).__name__,'size':len(v),'keys':list(v) if isinstance(v,dict) else None} for k,v in selected[0].items() if isinstance(v,(list,dict))}}
(Path(__file__).parent/'publication-tail-inspection.json').write_text(json.dumps(result,indent=2)+'\n')
print(json.dumps(result,indent=2))
