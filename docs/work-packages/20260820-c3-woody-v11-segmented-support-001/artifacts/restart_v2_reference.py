"""Independent executable V11 restart V2 chronology/admission model."""
import base64,copy,hashlib,json,struct
from pathlib import Path
ROOT=Path(__file__).resolve().parent
OWNERS=["vegetation","snow","land_surface_energy","surface_liquid","hydrology","bgc","soil_thermal"]
def canonical(v): return json.dumps(v,sort_keys=True,separators=(",",":")).encode()
def sha(v): return hashlib.sha256(v).hexdigest()
def enc(v):
 b=canonical(v); return base64.b64encode(b).decode(),sha(b)
def bits(v): return struct.pack(">d",v).hex()
def unbits(v): return struct.unpack(">d",bytes.fromhex(v))[0]
def owner(name,phase,value=0):
 body={"schema":"OPENWEPP_V11_OWNER_STATE_V1","owner_id":name,"phase":phase,"value":value}; b,d=enc(body)
 return {"owner_id":name,"state_canonical_base64":b,"state_sha256":d}
def receipt(kind,ordinal,parent,body):
 raw=canonical(body); return {"kind":kind,"ordinal":ordinal,"parent_transaction_id":parent,"payload_canonical_base64":base64.b64encode(raw).decode(),"payload_sha256":sha(raw),"receipt_id":sha(kind.encode()+ordinal.to_bytes(4,"big")+parent.encode()+raw)}
def build():
 parent=sha(b"parent-24"); config=sha(b"configuration"); authority=sha(b"authority-v16"); policy=sha(b"controller")
 begin=[owner(x,"begin",0) for x in OWNERS]
 slab0=receipt("slab",0,parent,{"start_ns":"0","end_ns":"600000000000","duration_s_bits":bits(600.0),"beginning_state":0,"forcing":1,"ending_state":1})
 event=receipt("event",0,parent,{"tick_ns":"600000000000","from_participants":["vegetation","snow"],"to_participants":["vegetation","surface_liquid"],"source_owner":"snow","receiver_owner":"surface_liquid","transfer_bits":bits(.25)})
 resource=[receipt("resource",i,parent,{"resource":name,"beginning_bits":bits(start),"final_use_bits":bits(use),"ending_bits":bits(start-use)}) for i,(name,start,use) in enumerate((("water",3,1),("nh4",.3,.1),("no3",.7,.3)))]
 material=[receipt("material",0,parent,{"source":"vegetation","receiver":"bgc","amount_bits":bits(.01),"slab_ordinal":0})]
 staged=[owner(x,"staged",1 if x in ("vegetation","snow","surface_liquid","hydrology","bgc") else 0) for x in OWNERS]
 checkpoint={"schema":"OPENWEPP_C3_WOODY_V11_PARENT_CHECKPOINT_V1","parent_transaction_id":parent,"beginning_state":{"schema":"OPENWEPP_C3_WOODY_V11_STATE_V1","value":0,"last_parent_transaction_sequence":"23"},"staged_state":{"schema":"OPENWEPP_C3_WOODY_V11_STATE_V1","value":1,"last_parent_transaction_sequence":"23"},"accepted_until_ns":"600000000000","accepted_segments":[{"slab_receipt":slab0,"beginning_state_sha256":sha(b"state-0"),"ending_state_value":1,"resource_receipts":resource,"material_receipts":material,"ending_complete_owners":staged}],"cumulative_debits":{"water":bits(1),"nh4":bits(.1),"no3":bits(.3)},"beginning_complete_owners":begin,"staged_complete_owners":staged,"finalized":False}
 coupled={"schema":"OPENWEPP_COUPLED_TIME_RESTART_V2","authority_sha256":sha(b"coupled-authority-v2"),"controller_policy_sha256":policy,"configuration_sha256":config,"parent_transaction_id":parent,"parent_start_ns":"0","parent_end_ns":"1800000000000","accepted_until_ns":"600000000000","next_slab_ordinal":1,"next_event_ordinal":1,"active_segment_id":"segment-1","active_participant_ids":["vegetation","surface_liquid"]}
 cb,cd=enc(coupled); pb,pd=enc(checkpoint); op0=sha(b"operand-0"); reduction=sha(b"maximum-reduction"); record=sha(b"record")
 return {"schema":"OPENWEPP_C3_WOODY_V11_RESTART_V2","authority_sha256":authority,"configuration_sha256":config,"parent_transaction_id":parent,"parent_transaction_sequence":"24","next_parent_transaction_sequence":"25","active_segment_id":"segment-1","active_regime_id":"snow-free","accepted_until_ns":"600000000000","next_slab_ordinal":1,"next_event_ordinal":1,"active_participant_ids":["vegetation","surface_liquid"],"controller_policy_sha256":policy,"coupled_time_v2_canonical_base64":cb,"coupled_time_v2_sha256":cd,"parent_checkpoint_canonical_base64":pb,"parent_checkpoint_sha256":pd,"beginning_complete_owners":begin,"staged_complete_owners":staged,"accepted_event_receipts":[event],"scheduled_execution_keys":[sha(b"parent-24:gsi")],"reduction_operands":[{"operand_id":op0,"source_receipt_id":slab0["receipt_id"],"support_start_ns":"0","support_end_ns":"600000000000","value_bits":bits(.2)}],"pending_publication_records":[{"record_id":record,"source_reduction_id":reduction,"payload_canonical_base64":base64.b64encode(canonical({"value_bits":bits(.2)})).decode(),"payload_sha256":sha(canonical({"value_bits":bits(.2)}))}],"publication_outbox":[]}
def decode(text,digest,error="V11-V2-DIGEST"):
 try: raw=base64.b64decode(text,validate=True)
 except Exception as e: raise ValueError("V11-V2-CANONICAL") from e
 if base64.b64encode(raw).decode()!=text or sha(raw)!=digest: raise ValueError(error)
 try: value=json.loads(raw)
 except Exception as e: raise ValueError("V11-V2-CANONICAL") from e
 if canonical(value)!=raw: raise ValueError("V11-V2-CANONICAL")
 return value
def owners(values):
 if len(values)!=7 or [x.get("owner_id") for x in values]!=OWNERS: raise ValueError("V11-V2-OWNERS")
 for value in values:
  body=decode(value["state_canonical_base64"],value["state_sha256"])
  if body.get("owner_id")!=value["owner_id"] or body.get("schema")!="OPENWEPP_V11_OWNER_STATE_V1": raise ValueError("V11-V2-OWNERS")
def receipt_body(value,kind,ordinal,parent):
 if value.get("kind")!=kind or value.get("ordinal")!=ordinal or value.get("parent_transaction_id")!=parent: raise ValueError("V11-V2-RECEIPT")
 body=decode(value["payload_canonical_base64"],value["payload_sha256"])
 expected=sha(kind.encode()+ordinal.to_bytes(4,"big")+parent.encode()+canonical(body))
 if value.get("receipt_id")!=expected: raise ValueError("V11-V2-RECEIPT")
 return body
def validate(v):
 if v.get("schema")!="OPENWEPP_C3_WOODY_V11_RESTART_V2" or v.get("authority_sha256")!=sha(b"authority-v16"): raise ValueError("V11-V2-AUTHORITY")
 coupled=decode(v["coupled_time_v2_canonical_base64"],v["coupled_time_v2_sha256"]); cp=decode(v["parent_checkpoint_canonical_base64"],v["parent_checkpoint_sha256"])
 if coupled.get("schema")!="OPENWEPP_COUPLED_TIME_RESTART_V2" or coupled.get("authority_sha256")!=sha(b"coupled-authority-v2"): raise ValueError("V11-V2-JOIN")
 for key in ("configuration_sha256","parent_transaction_id","accepted_until_ns","next_slab_ordinal","next_event_ordinal","active_segment_id","active_participant_ids","controller_policy_sha256"):
  if coupled.get(key)!=v.get(key): raise ValueError("V11-V2-JOIN")
 if v["next_parent_transaction_sequence"]!=str(int(v["parent_transaction_sequence"])+1) or cp.get("parent_transaction_id")!=v["parent_transaction_id"] or cp.get("accepted_until_ns")!=v["accepted_until_ns"] or len(cp.get("accepted_segments",[]))!=v["next_slab_ordinal"] or cp.get("finalized") is not False: raise ValueError("V11-V2-JOIN")
 owners(v["beginning_complete_owners"]); owners(v["staged_complete_owners"])
 if cp.get("beginning_complete_owners")!=v["beginning_complete_owners"] or cp.get("staged_complete_owners")!=v["staged_complete_owners"]: raise ValueError("V11-V2-OWNERS")
 segment=cp["accepted_segments"][0]; slab=receipt_body(segment["slab_receipt"],"slab",0,v["parent_transaction_id"])
 if slab!={"start_ns":"0","end_ns":"600000000000","duration_s_bits":bits(600.0),"beginning_state":0,"forcing":1,"ending_state":1} or cp["beginning_state"]["value"]!=0 or cp["staged_state"]["value"]!=1 or segment["ending_state_value"]!=1: raise ValueError("V11-V2-CHECKPOINT")
 if len(segment["resource_receipts"])!=3 or len(segment["material_receipts"])!=1: raise ValueError("V11-V2-CHECKPOINT")
 for i,r in enumerate(segment["resource_receipts"]): receipt_body(r,"resource",i,v["parent_transaction_id"])
 for i,r in enumerate(segment["material_receipts"]): receipt_body(r,"material",i,v["parent_transaction_id"])
 events=[receipt_body(r,"event",i,v["parent_transaction_id"]) for i,r in enumerate(v["accepted_event_receipts"])]
 if len(events)!=v["next_event_ordinal"] or events[0]["tick_ns"]!=v["accepted_until_ns"] or events[0]["to_participants"]!=v["active_participant_ids"]: raise ValueError("V11-V2-EVENT")
 if v["scheduled_execution_keys"]!=[sha(b"parent-24:gsi")]: raise ValueError("V11-V2-SCHEDULED")
 values=[]
 for operand in v["reduction_operands"]:
  if int(operand["support_end_ns"])>int(v["accepted_until_ns"]) or operand["source_receipt_id"]!=segment["slab_receipt"]["receipt_id"]: raise ValueError("V11-V2-REDUCTION")
  values.append(unbits(operand["value_bits"]))
 if not values or max(values)!=.2: raise ValueError("V11-V2-REDUCTION")
 records={}
 for record in v["pending_publication_records"]:
  body=decode(record["payload_canonical_base64"],record["payload_sha256"])
  if body!={"value_bits":bits(max(values))} or record["source_reduction_id"]!=sha(b"maximum-reduction"): raise ValueError("V11-V2-PUBLICATION")
  records[record["record_id"]]=record
 for row in v["publication_outbox"]:
  if row["record_id"] not in records or (row["state"]=="CommittedUndelivered" and row["delivery_count"]!=0) or (row["state"]!="CommittedUndelivered" and row["delivery_count"]==0): raise ValueError("V11-V2-PUBLICATION")
 return cp
def restore_suffix(v):
 cp=validate(v); staged=cp["staged_state"]["value"]
 ending=staged*2+4+1
 uninterrupted=(0*2+1+0)*2+4+1
 if ending!=uninterrupted: raise ValueError("V11-V2-CHECKPOINT")
 return {"ending_state":ending,"uninterrupted_state":uninterrupted,"suffix_slabs":1}
def reframe(v,key,obj):
 b,d=enc(obj); v[key+"_canonical_base64"]=b; v[key+"_sha256"]=d
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
 elif name in ("coupled_wrong_authority","coupled_wrong_configuration"):
  coupled=decode(x["coupled_time_v2_canonical_base64"],x["coupled_time_v2_sha256"]); coupled["authority_sha256"]="0"*64 if name=="coupled_wrong_authority" else coupled["authority_sha256"]
  if name=="coupled_wrong_configuration": coupled["configuration_sha256"]="0"*64
  reframe(x,"coupled_time_v2",coupled)
 elif name in ("checkpoint_forged_staged_state","checkpoint_missing_resource"):
  cp=decode(x["parent_checkpoint_canonical_base64"],x["parent_checkpoint_sha256"])
  if name=="checkpoint_forged_staged_state": cp["staged_state"]["value"]=99
  else: cp["accepted_segments"][0]["resource_receipts"].pop()
  reframe(x,"parent_checkpoint",cp)
 elif name=="event_wrong_tick":
  body=receipt_body(x["accepted_event_receipts"][0],"event",0,x["parent_transaction_id"]); body["tick_ns"]="42"; x["accepted_event_receipts"][0]=receipt("event",0,x["parent_transaction_id"],body)
 elif name=="scheduled_wrong_key": x["scheduled_execution_keys"]=[sha(b"wrong")]
 elif name=="reduction_wrong_source": x["reduction_operands"][0]["source_receipt_id"]="0"*64
 elif name=="publication_wrong_payload":
  raw=canonical({"value_bits":bits(9.0)}); x["pending_publication_records"][0]["payload_canonical_base64"]=base64.b64encode(raw).decode(); x["pending_publication_records"][0]["payload_sha256"]=sha(raw)
 elif name=="outbox_impossible_count": x["publication_outbox"]=[{"outbox_id":sha(b"outbox"),"record_id":x["pending_publication_records"][0]["record_id"],"state":"CommittedUndelivered","delivery_count":1}]
 return x
def main():
 base=build(); accepted=restore_suffix(base); cases=json.loads((ROOT/"restart-v2-poisons.json").read_text())["cases"]; results=[]
 for case in cases:
  try: restore_suffix(mutate(base,case["mutation"]))
  except ValueError as e:
   if str(e)!=case["error"]: raise SystemExit(f"{case['id']}: {e} != {case['error']}")
   results.append({"id":case["id"],"error":str(e)})
  else: raise SystemExit(f"{case['id']}: accepted")
 print(json.dumps({"schema":"OPENWEPP_C3_WOODY_V11_RESTART_V2_RESULTS_V2","accepted":accepted,"poisons":results},sort_keys=True,separators=(",",":")))
if __name__=="__main__": main()
