"""Read retained export for credit-receipt occurrences; no admission or physics."""
import hashlib,json,collections
from pathlib import Path
import ijson
ROOT=Path('/workdir/openwepp-experiments/b01-wb14-cadence/corrected-recorder-export-20260916-1')
HERE=Path(__file__).resolve().parent
phase=json.loads((ROOT/'rows/123091.26.json').read_text())
chain=phase['deferred_native_v2_soil_custody']['continuation']['ordered_layer_credit_chain']
targets={o['debit_credit_identity_sha256'] for g in chain for l in g for o in l['accepted_operands']}
results={}
for name in ['123091.19.json','123091.24.json','123091.26.json']:
 p=ROOT/'rows'/name; matches=collections.defaultdict(list); keys=[]
 with p.open('rb') as f:
  for prefix,event,value in ijson.parse(f):
   if prefix=='' and event=='map_key':keys.append(value)
   if event=='string' and value in targets and not prefix.startswith('deferred_native_v2_soil_custody.'):
    matches[value].append(prefix)
 with p.open('rb') as f: digest=hashlib.file_digest(f,'sha256').hexdigest()
 results[name]={'bytes':p.stat().st_size,'sha256':digest,'top_level_keys':keys,'credit_receipt_occurrences':dict(matches)}
result={'evidence_class':'Ran: read-only streaming extraction, not validation','unique_credit_receipts':len(targets),'members':results,'limitation':'String occurrence search only; JSON encoded in byte arrays requires separate decode. A digest occurrence alone does not authenticate a preimage or support correspondence.'}
(HERE/'correspondence-search.json').write_text(json.dumps(result,indent=2)+'\n')
print(json.dumps({k:{'top_level_keys':v['top_level_keys'],'matched_unique':len(v['credit_receipt_occurrences'])} for k,v in results.items()},indent=2))

# Byte-encoded retained histories are separate semantic surfaces; inspect them too.
from ijson.common import ObjectBuilder
matches=collections.defaultdict(list)
selected_preimages=[]
byte_surfaces=[]
def walk(v,path,parent=None):
 if isinstance(v,dict):
  for k,x in v.items():walk(x,path+'/'+k,v)
 elif isinstance(v,list):
  for i,x in enumerate(v):walk(x,path+'/'+str(i),v)
 elif isinstance(v,str) and v in targets:
  matches[v].append(path)
  if isinstance(parent,dict):selected_preimages.append({'path':path,'object':parent})
parent=json.loads(bytes(json.loads((ROOT/'rows/123091.24.json').read_text())))
walk(parent,'parent_checkpoint_decoded')
del parent
wanted={'accepted_publication_rotation','frozen_litter_residents.physical_publication_history.item','frozen_litter_residents.exact_publication_history.item'}
active=None;builder=None;depth=0;ordinal=collections.Counter()
with (ROOT/'rows/123091.19.json').open('rb') as f:
 for prefix,event,value in ijson.parse(f):
  if active is None and prefix in wanted and event=='start_array':
   active=prefix;builder=ObjectBuilder();depth=0
  if active is not None:
   builder.event(event,value)
   depth+=int(event in ('start_array','start_map'))-int(event in ('end_array','end_map'))
   if depth==0:
    raw=bytes(builder.value);builder=None
    decoded=json.loads(raw)
    path=active+'/'+str(ordinal[active]);ordinal[active]+=1
    byte_surfaces.append({'path':path,'bytes':len(raw),'sha256':hashlib.sha256(raw).hexdigest(),'top_level_keys':list(decoded) if isinstance(decoded,dict) else None})
    walk(decoded,path);del decoded,raw
    active=None
result['decoded_byte_surfaces']=byte_surfaces
result['decoded_credit_receipt_occurrences']=dict(matches)
result['decoded_matching_preimages']=selected_preimages
result['limitation']='Read-only string joins and byte-JSON decode; no scientific or semantic admission. Occurrences alone do not prove complete support/event correspondence.'
(HERE/'correspondence-search.json').write_text(json.dumps(result,indent=2)+'\n')
print(json.dumps({'decoded_surfaces':byte_surfaces,'matched_unique':len(matches),'occurrences':dict(matches)},indent=2))
