"""One bounded read-only raw-corpus search after exported histories were decoded."""
import hashlib,json,re
from pathlib import Path
ROOT=Path('/workdir/openwepp-experiments/b01-wb14-cadence')
phase=json.loads((ROOT/'corrected-recorder-export-20260916-1/rows/123091.26.json').read_text())
chain=phase['deferred_native_v2_soil_custody']['continuation']['ordered_layer_credit_chain']
targets=[]
for index in range(7,11):
 for layer in chain[index]:
  for op in layer['accepted_operands']:
   if op['source_kind']=='top_boundary':targets.append({'credit_group_index':index,'receipt':op['debit_credit_identity_sha256']})
needles={x['receipt'].encode():x['credit_group_index'] for x in targets}
pattern=re.compile(b'|'.join(needles))
path=ROOT/'corrected-recorder-capture-20260916-1/observations.json'
hash=hashlib.sha256();tail=b'';offset=0;matches=[];seen=set()
with path.open('rb') as f:
 while chunk:=f.read(16*1024**2):
  hash.update(chunk);data=tail+chunk;base=offset-len(tail)
  for m in pattern.finditer(data):
   at=base+m.start()
   if at in seen:continue
   seen.add(at);matches.append({'offset':at,'credit_group_index':needles[m.group()],'receipt':m.group().decode(),'context':data[max(0,m.start()-180):min(len(data),m.end()+180)].decode('utf-8')})
  offset+=len(chunk);tail=data[-512:]
result={'evidence_class':'Ran: complete raw byte search, no model/physics','path':str(path),'bytes':offset,'sha256':hash.hexdigest(),'targets':targets,'matches':matches,'limitation':'Literal occurrences distinguish retained receipt claims only; no missing support inferred from count alone. Byte-encoded publication histories separately decoded in correspondence-search.json.'}
assert result['sha256']=='1555efe1b592081a3dd67639d71a18e4e67f518e24b93a95ddfe18de2023d87c'
(Path(__file__).parent/'raw-credit-occurrences.json').write_text(json.dumps(result,indent=2)+'\n')
print(json.dumps({'bytes':offset,'sha256':hash.hexdigest(),'matches':len(matches),'by_group':{i:sum(x['credit_group_index']==i for x in matches) for i in range(7,11)}},indent=2))
