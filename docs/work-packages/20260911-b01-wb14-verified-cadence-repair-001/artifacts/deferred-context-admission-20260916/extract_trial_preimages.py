"""Extract authentic retained soil-trial records by selected credit receipt identity."""
import hashlib,json,re
from pathlib import Path
ROOT=Path('/workdir/openwepp-experiments/b01-wb14-cadence')
HERE=Path(__file__).resolve().parent
phase=json.loads((ROOT/'corrected-recorder-export-20260916-1/rows/123091.26.json').read_text())
chain=phase['deferred_native_v2_soil_custody']['continuation']['ordered_layer_credit_chain']
targets={op['debit_credit_identity_sha256'].encode():i for i,group in enumerate(chain) for layer in group for op in layer['accepted_operands'] if op['source_kind']=='top_boundary'}
assert len(targets)==12
pattern=re.compile(b'|'.join(targets));path=ROOT/'corrected-recorder-capture-20260916-1/observations.json'
hash=hashlib.sha256();tail=b'';offset=0;matches=set()
with path.open('rb') as f:
 while chunk:=f.read(16*1024**2):
  hash.update(chunk);data=tail+chunk;base=offset-len(tail)
  for m in pattern.finditer(data):matches.add((base+m.start(),targets[m.group()]))
  offset+=len(chunk);tail=data[-64:]
assert hash.hexdigest()=='1555efe1b592081a3dd67639d71a18e4e67f518e24b93a95ddfe18de2023d87c'
records={};unmatched=[];decoder=json.JSONDecoder()
with path.open('rb') as f:
 for at,group in sorted(matches):
  start=max(0,at-32768);f.seek(start);window=f.read(65536);found=False
  for m in re.finditer(rb'\{"beginning_soil"\s*:',window):
   if start+m.start()>at:break
   try:record,end=decoder.raw_decode(window[m.start():].decode())
   except (json.JSONDecodeError,UnicodeDecodeError):continue
   stop=start+m.start()+len(window[m.start():].decode()[:end].encode())
   if start+m.start()<=at<stop and record.get('kind')=='soil_trial_primitive' and record.get('receipt',{}).get('receipt_sha256','').encode() in targets:
    raw=window[m.start():].decode()[:end].encode()
    assert json.loads(raw)==record
    records[start+m.start()]={'source_start':start+m.start(),'source_end':stop,'source_record_sha256':hashlib.sha256(raw).hexdigest(),'credit_group_index':group,'record':record,'source_canonical_json':raw.decode()}
    found=True;break
  if not found:unmatched.append({'offset':at,'credit_group_index':group})
result={'evidence_class':'Ran: exact raw-record extraction; no physics or admission','source':str(path),'source_bytes':offset,'source_sha256':hash.hexdigest(),'literal_occurrences':len(matches),'records':list(records.values()),'occurrences_outside_extracted_primitives':unmatched,'limitation':'Trial-only records require selection by committed credit receipt and complete lineage; a record alone is not accepted-work authority.'}
(HERE/'retained-soil-trial-primitives.json').write_text(json.dumps(result,indent=2)+'\n')
print(json.dumps({'literal_occurrences':len(matches),'extracted_records':len(records),'groups':sorted({x['credit_group_index'] for x in records.values()}),'unmatched_occurrences':len(unmatched)},indent=2))
