"""Independent canonical V11 restart V2 admission model; imports no Rust."""
import base64,copy,hashlib,json
from pathlib import Path
ROOT=Path(__file__).resolve().parent
OWNERS=["vegetation","snow","land_surface_energy","surface_liquid","hydrology","bgc","soil_thermal"]
def canonical(v): return json.dumps(v,sort_keys=True,separators=(",",":")).encode()
def sha(v): return hashlib.sha256(v).hexdigest()
def enc(v):
 b=canonical(v); return base64.b64encode(b).decode(),sha(b)
def owner(name,phase):
 b=canonical({"owner":name,"phase":phase}); return {"owner_id":name,"state_canonical_base64":base64.b64encode(b).decode(),"state_sha256":sha(b)}
def build():
 parent=sha(b"parent-24"); begin=[owner(x,"begin") for x in OWNERS]; staged=[owner(x,"staged") for x in OWNERS]
 coupled={"schema":"OPENWEPP_COUPLED_TIME_RESTART_V2","parent_transaction_id":parent,"accepted_until_ns":"600000000000","next_slab_ordinal":1,"next_event_ordinal":1,"active_participant_ids":["vegetation","surface_liquid"]}
 checkpoint={"schema":"OPENWEPP_C3_WOODY_V11_PARENT_CHECKPOINT_V1","parent_transaction_id":parent,"accepted_until_ns":"600000000000","accepted_slab_count":1,"beginning_complete_owners":begin,"staged_complete_owners":staged,"finalized":False}
 cb,cd=enc(coupled); pb,pd=enc(checkpoint); reduction=sha(b"reduction"); record=sha(b"record")
 return {"schema":"OPENWEPP_C3_WOODY_V11_RESTART_V2","authority_sha256":sha(b"authority-v16"),"configuration_sha256":sha(b"configuration"),"parent_transaction_id":parent,"parent_transaction_sequence":"24","next_parent_transaction_sequence":"25","active_segment_id":"segment-1","active_regime_id":"snow-free","accepted_until_ns":"600000000000","next_slab_ordinal":1,"next_event_ordinal":1,"active_participant_ids":["vegetation","surface_liquid"],"controller_policy_sha256":sha(b"controller"),"coupled_time_v2_canonical_base64":cb,"coupled_time_v2_sha256":cd,"parent_checkpoint_canonical_base64":pb,"parent_checkpoint_sha256":pd,"beginning_complete_owners":begin,"staged_complete_owners":staged,"accepted_event_receipts":[],"scheduled_execution_keys":[],"reduction_operands":[{"operand_id":reduction,"source_receipt_id":sha(b"slab-0"),"support_start_ns":"0","support_end_ns":"600000000000","value_bits":"3fd999999999999a"}],"pending_publication_records":[{"record_id":record,"source_reduction_id":reduction,"payload_canonical_base64":base64.b64encode(b"{}").decode(),"payload_sha256":sha(b"{}") }],"publication_outbox":[]}
def decode(text,digest):
 try: raw=base64.b64decode(text,validate=True)
 except Exception as e: raise ValueError("V11-V2-CANONICAL") from e
 if base64.b64encode(raw).decode()!=text or sha(raw)!=digest: raise ValueError("V11-V2-DIGEST")
 try: value=json.loads(raw)
 except Exception as e: raise ValueError("V11-V2-CANONICAL") from e
 if canonical(value)!=raw: raise ValueError("V11-V2-CANONICAL")
 return value
def validate_owner_set(values):
 if len(values)!=7 or [x.get("owner_id") for x in values]!=OWNERS: raise ValueError("V11-V2-OWNERS")
 for value in values:
  raw=base64.b64decode(value["state_canonical_base64"],validate=True)
  if base64.b64encode(raw).decode()!=value["state_canonical_base64"] or sha(raw)!=value["state_sha256"]: raise ValueError("V11-V2-DIGEST")
def validate(v):
 if v["schema"]!="OPENWEPP_C3_WOODY_V11_RESTART_V2" or v["authority_sha256"]!=sha(b"authority-v16"): raise ValueError("V11-V2-AUTHORITY")
 coupled=decode(v["coupled_time_v2_canonical_base64"],v["coupled_time_v2_sha256"]); checkpoint=decode(v["parent_checkpoint_canonical_base64"],v["parent_checkpoint_sha256"])
 if v["next_parent_transaction_sequence"]!=str(int(v["parent_transaction_sequence"])+1): raise ValueError("V11-V2-JOIN")
 for key in ("parent_transaction_id","accepted_until_ns","next_slab_ordinal","next_event_ordinal","active_participant_ids"):
  if coupled[key]!=v[key]: raise ValueError("V11-V2-JOIN")
 if checkpoint["parent_transaction_id"]!=v["parent_transaction_id"] or checkpoint["accepted_until_ns"]!=v["accepted_until_ns"] or checkpoint["accepted_slab_count"]!=v["next_slab_ordinal"] or checkpoint["finalized"]: raise ValueError("V11-V2-JOIN")
 validate_owner_set(v["beginning_complete_owners"]); validate_owner_set(v["staged_complete_owners"])
 if checkpoint["beginning_complete_owners"]!=v["beginning_complete_owners"] or checkpoint["staged_complete_owners"]!=v["staged_complete_owners"]: raise ValueError("V11-V2-OWNERS")
 if any(int(x["support_end_ns"])>int(v["accepted_until_ns"]) for x in v["reduction_operands"]): raise ValueError("V11-V2-REDUCTION")
 records={x["record_id"] for x in v["pending_publication_records"]}
 if any(x["record_id"] not in records for x in v["publication_outbox"]): raise ValueError("V11-V2-PUBLICATION")
 return {"owner_count":7,"accepted_until_ns":v["accepted_until_ns"],"next_sequence":v["next_parent_transaction_sequence"]}
def mutate(v,name):
 x=copy.deepcopy(v)
 if name=="wrong_authority": x["authority_sha256"]="0"*64
 elif name=="wrong_parent": x["parent_transaction_id"]="0"*64
 elif name=="wrong_cursor": x["accepted_until_ns"]="42"
 elif name=="wrong_slab_ordinal": x["next_slab_ordinal"]=2
 elif name=="wrong_event_ordinal": x["next_event_ordinal"]=2
 elif name=="wrong_participants": x["active_participant_ids"]=["bogus"]
 elif name=="bad_coupled_digest": x["coupled_time_v2_sha256"]="0"*64
 elif name=="bad_checkpoint_digest": x["parent_checkpoint_sha256"]="0"*64
 elif name=="noncanonical_checkpoint": x["parent_checkpoint_canonical_base64"]=base64.b64encode(b'{ "x": 1 }').decode(); x["parent_checkpoint_sha256"]=sha(b'{ "x": 1 }')
 elif name=="missing_begin_owner": x["beginning_complete_owners"].pop()
 elif name=="reordered_stage_owner": x["staged_complete_owners"][0],x["staged_complete_owners"][1]=x["staged_complete_owners"][1],x["staged_complete_owners"][0]
 elif name=="forged_owner_bytes": x["staged_complete_owners"][0]["state_canonical_base64"]=base64.b64encode(b'{}').decode()
 elif name=="bad_successor": x["next_parent_transaction_sequence"]="26"
 elif name=="future_reduction": x["reduction_operands"][0]["support_end_ns"]="700000000000"
 elif name=="orphan_outbox": x["publication_outbox"]=[{"outbox_id":sha(b"outbox"),"record_id":sha(b"orphan"),"state":"CommittedUndelivered","delivery_count":0}]
 return x
def main():
 base=build(); accepted=validate(base); cases=json.loads((ROOT/"restart-v2-poisons.json").read_text())["cases"]; results=[]
 for case in cases:
  try: validate(mutate(base,case["mutation"]))
  except ValueError as e:
   if str(e)!=case["error"]: raise SystemExit(f"{case['id']}: {e}")
   results.append({"id":case["id"],"error":str(e)})
  else: raise SystemExit(f"{case['id']}: accepted")
 print(json.dumps({"schema":"OPENWEPP_C3_WOODY_V11_RESTART_V2_RESULTS_V1","accepted":accepted,"poisons":results},sort_keys=True,separators=(",",":")))
if __name__=="__main__": main()
