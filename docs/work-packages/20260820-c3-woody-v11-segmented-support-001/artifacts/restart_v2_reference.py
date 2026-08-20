"""Independent executable V11 restart V2 chronology/admission model."""
import base64,copy,hashlib,json,struct
from pathlib import Path
ROOT=Path(__file__).resolve().parent
OWNERS=["vegetation","snow","land_surface_energy","surface_liquid","hydrology","bgc","soil_thermal"]
OWNER_FIELDS={
 "vegetation":{"state_value"},"snow":{"liquid_bits"},
 "land_surface_energy":{"canopy_air_temperature_bits","canopy_air_humidity_bits"},
 "surface_liquid":{"liquid_bits"},"hydrology":{"water_bits"},
 "bgc":{"nh4_bits","no3_bits","material_bits"},
 "soil_thermal":{"temperature_bits"},
}
def canonical(v): return json.dumps(v,sort_keys=True,separators=(",",":")).encode()
def sha(v): return hashlib.sha256(v).hexdigest()
def enc(v):
 b=canonical(v); return base64.b64encode(b).decode(),sha(b)
def bits(v): return struct.pack(">d",v).hex()
def unbits(v): return struct.unpack(">d",bytes.fromhex(v))[0]
def owner(name,phase,**state):
 body={"schema":"OPENWEPP_V11_"+name.upper()+"_OWNER_STATE_V1","owner_id":name,"phase":phase,"state":state}; b,d=enc(body)
 return {"owner_id":name,"state_canonical_base64":b,"state_sha256":d}
def exact(value,keys,error):
 if set(value)!=set(keys): raise ValueError(error)
def receipt(kind,ordinal,parent,body):
 raw=canonical(body); return {"kind":kind,"ordinal":ordinal,"parent_transaction_id":parent,"payload_canonical_base64":base64.b64encode(raw).decode(),"payload_sha256":sha(raw),"receipt_id":sha(kind.encode()+ordinal.to_bytes(4,"big")+parent.encode()+raw)}
def build():
 parent=sha(b"parent-24"); config=sha(b"configuration"); authority=sha(b"authority-v18"); policy=sha(b"controller")
 begin=[owner("vegetation","begin",state_value=0),owner("snow","begin",liquid_bits=bits(.25)),owner("land_surface_energy","begin",canopy_air_temperature_bits=bits(275),canopy_air_humidity_bits=bits(.002)),owner("surface_liquid","begin",liquid_bits=bits(0)),owner("hydrology","begin",water_bits=bits(3)),owner("bgc","begin",nh4_bits=bits(.3),no3_bits=bits(.7),material_bits=bits(0)),owner("soil_thermal","begin",temperature_bits=bits(274))]
 slab0=receipt("slab",0,parent,{"start_ns":"0","end_ns":"600000000000","duration_s_bits":bits(600.0),"beginning_state":0,"forcing":1,"ending_state":1})
 event=receipt("event",0,parent,{"tick_ns":"600000000000","from_participants":["vegetation","snow"],"to_participants":["vegetation","surface_liquid"],"source_owner":"snow","receiver_owner":"surface_liquid","transfer_bits":bits(.25)})
 resource=[receipt("resource",i,parent,{"resource":name,"beginning_bits":bits(start),"final_use_bits":bits(use),"ending_bits":bits(start-use)}) for i,(name,start,use) in enumerate((("water",3,1),("nh4",.3,.1),("no3",.7,.3)))]
 material=[receipt("material",0,parent,{"source":"vegetation","receiver":"bgc","amount_bits":bits(.01),"slab_ordinal":0})]
 staged=[owner("vegetation","staged",state_value=1),owner("snow","staged",liquid_bits=bits(0)),owner("land_surface_energy","staged",canopy_air_temperature_bits=bits(276),canopy_air_humidity_bits=bits(.003)),owner("surface_liquid","staged",liquid_bits=bits(.25)),owner("hydrology","staged",water_bits=bits(2)),owner("bgc","staged",nh4_bits=bits(.3-.1),no3_bits=bits(.7-.3),material_bits=bits(.01)),owner("soil_thermal","staged",temperature_bits=bits(275))]
 beginning_state={"schema":"OPENWEPP_C3_WOODY_V11_STATE_V1","value":0,"last_parent_transaction_sequence":"23"}; staged_state={"schema":"OPENWEPP_C3_WOODY_V11_STATE_V1","value":1,"last_parent_transaction_sequence":"23"}
 checkpoint={"schema":"OPENWEPP_C3_WOODY_V11_PARENT_CHECKPOINT_V1","parent_transaction_id":parent,"beginning_state":beginning_state,"staged_state":staged_state,"accepted_until_ns":"600000000000","accepted_segments":[{"slab_receipt":slab0,"beginning_state_sha256":sha(canonical(beginning_state)),"ending_state":staged_state,"ending_state_sha256":sha(canonical(staged_state)),"resource_receipts":resource,"material_receipts":material,"ending_complete_owners":staged}],"cumulative_debits":{"water":bits(1),"nh4":bits(.1),"no3":bits(.3)},"beginning_complete_owners":begin,"staged_complete_owners":staged,"finalized":False}
 coupled={"schema":"OPENWEPP_COUPLED_TIME_RESTART_V2","authority_sha256":sha(b"coupled-authority-v2"),"controller_policy_sha256":policy,"configuration_sha256":config,"parent_transaction_id":parent,"parent_start_ns":"0","parent_end_ns":"1800000000000","accepted_until_ns":"600000000000","next_slab_ordinal":1,"next_event_ordinal":1,"active_segment_id":"segment-1","active_participant_ids":["vegetation","surface_liquid"]}
 cb,cd=enc(coupled); pb,pd=enc(checkpoint); op0=sha(canonical({"source_receipt_id":slab0["receipt_id"],"support_start_ns":"0","support_end_ns":"600000000000","value_bits":bits(.2)})); reduction=sha(canonical([op0])); payload=canonical({"value_bits":bits(.2)}); record=sha(reduction.encode()+sha(payload).encode())
 outbox_id=sha(canonical({"parent_transaction_id":parent,"record_id":record,"state":"CommittedUndelivered","delivery_count":0}))
 return {"schema":"OPENWEPP_C3_WOODY_V11_RESTART_V2","authority_sha256":authority,"configuration_sha256":config,"parent_transaction_id":parent,"parent_transaction_sequence":"24","next_parent_transaction_sequence":"25","active_segment_id":"segment-1","active_regime_id":"snow-free","accepted_until_ns":"600000000000","next_slab_ordinal":1,"next_event_ordinal":1,"active_participant_ids":["vegetation","surface_liquid"],"controller_policy_sha256":policy,"coupled_time_v2_canonical_base64":cb,"coupled_time_v2_sha256":cd,"parent_checkpoint_canonical_base64":pb,"parent_checkpoint_sha256":pd,"beginning_complete_owners":begin,"staged_complete_owners":staged,"accepted_event_receipts":[event],"scheduled_execution_keys":[sha(b"parent-24:gsi")],"reduction_operands":[{"operand_id":op0,"source_receipt_id":slab0["receipt_id"],"support_start_ns":"0","support_end_ns":"600000000000","value_bits":bits(.2)}],"pending_publication_records":[{"record_id":record,"source_reduction_id":reduction,"payload_canonical_base64":base64.b64encode(payload).decode(),"payload_sha256":sha(payload)}],"publication_outbox":[{"outbox_id":outbox_id,"record_id":record,"state":"CommittedUndelivered","delivery_count":0}]}
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
  exact(value,{"owner_id","state_canonical_base64","state_sha256"},"V11-V2-OWNERS"); exact(body,{"schema","owner_id","phase","state"},"V11-V2-OWNERS")
  expected_schema="OPENWEPP_V11_"+value["owner_id"].upper()+"_OWNER_STATE_V1"
  if body.get("owner_id")!=value["owner_id"] or body.get("schema")!=expected_schema or not isinstance(body["state"],dict) or set(body["state"])!=OWNER_FIELDS[value["owner_id"]]: raise ValueError("V11-V2-OWNERS")
 return [decode(x["state_canonical_base64"],x["state_sha256"])["state"] for x in values]
def receipt_body(value,kind,ordinal,parent):
 exact(value,{"kind","ordinal","parent_transaction_id","payload_canonical_base64","payload_sha256","receipt_id"},"V11-V2-RECEIPT")
 if value.get("kind")!=kind or value.get("ordinal")!=ordinal or value.get("parent_transaction_id")!=parent: raise ValueError("V11-V2-RECEIPT")
 body=decode(value["payload_canonical_base64"],value["payload_sha256"])
 expected=sha(kind.encode()+ordinal.to_bytes(4,"big")+parent.encode()+canonical(body))
 if value.get("receipt_id")!=expected: raise ValueError("V11-V2-RECEIPT")
 return body
def ordered_unique(values,key,error):
 ids=[key(value) for value in values]
 if len(ids)!=len(set(ids)) or ids!=sorted(ids): raise ValueError(error)
def validate(v):
 exact(v,{"schema","authority_sha256","configuration_sha256","parent_transaction_id","parent_transaction_sequence","next_parent_transaction_sequence","active_segment_id","active_regime_id","accepted_until_ns","next_slab_ordinal","next_event_ordinal","active_participant_ids","controller_policy_sha256","coupled_time_v2_canonical_base64","coupled_time_v2_sha256","parent_checkpoint_canonical_base64","parent_checkpoint_sha256","beginning_complete_owners","staged_complete_owners","accepted_event_receipts","scheduled_execution_keys","reduction_operands","pending_publication_records","publication_outbox"},"V11-V2-CANONICAL")
 if v.get("schema")!="OPENWEPP_C3_WOODY_V11_RESTART_V2" or v.get("authority_sha256")!=sha(b"authority-v18"): raise ValueError("V11-V2-AUTHORITY")
 coupled=decode(v["coupled_time_v2_canonical_base64"],v["coupled_time_v2_sha256"]); cp=decode(v["parent_checkpoint_canonical_base64"],v["parent_checkpoint_sha256"])
 exact(coupled,{"schema","authority_sha256","controller_policy_sha256","configuration_sha256","parent_transaction_id","parent_start_ns","parent_end_ns","accepted_until_ns","next_slab_ordinal","next_event_ordinal","active_segment_id","active_participant_ids"},"V11-V2-JOIN")
 exact(cp,{"schema","parent_transaction_id","beginning_state","staged_state","accepted_until_ns","accepted_segments","cumulative_debits","beginning_complete_owners","staged_complete_owners","finalized"},"V11-V2-CHECKPOINT")
 if coupled.get("schema")!="OPENWEPP_COUPLED_TIME_RESTART_V2" or coupled.get("authority_sha256")!=sha(b"coupled-authority-v2"): raise ValueError("V11-V2-JOIN")
 for key in ("configuration_sha256","parent_transaction_id","accepted_until_ns","next_slab_ordinal","next_event_ordinal","active_segment_id","active_participant_ids","controller_policy_sha256"):
  if coupled.get(key)!=v.get(key): raise ValueError("V11-V2-JOIN")
 exact(cp["beginning_state"],{"schema","value","last_parent_transaction_sequence"},"V11-V2-CHECKPOINT"); exact(cp["staged_state"],{"schema","value","last_parent_transaction_sequence"},"V11-V2-CHECKPOINT")
 if v["parent_transaction_sequence"]!="24" or v["next_parent_transaction_sequence"]!="25" or cp["beginning_state"]["last_parent_transaction_sequence"]!="23" or cp["staged_state"]["last_parent_transaction_sequence"]!="23" or v["next_parent_transaction_sequence"]!=str(int(v["parent_transaction_sequence"])+1) or cp.get("parent_transaction_id")!=v["parent_transaction_id"] or cp.get("accepted_until_ns")!=v["accepted_until_ns"] or len(cp.get("accepted_segments",[]))!=v["next_slab_ordinal"] or cp.get("finalized") is not False: raise ValueError("V11-V2-JOIN")
 begin_states=owners(v["beginning_complete_owners"]); staged_states=owners(v["staged_complete_owners"])
 if cp.get("beginning_complete_owners")!=v["beginning_complete_owners"] or cp.get("staged_complete_owners")!=v["staged_complete_owners"]: raise ValueError("V11-V2-OWNERS")
 predecessor_state=cp["beginning_state"]; predecessor_end="0"
 for ordinal, candidate in enumerate(cp["accepted_segments"]):
  exact(candidate,{"slab_receipt","beginning_state_sha256","ending_state","ending_state_sha256","resource_receipts","material_receipts","ending_complete_owners"},"V11-V2-CHECKPOINT")
  slab_candidate=receipt_body(candidate["slab_receipt"],"slab",ordinal,v["parent_transaction_id"])
  if candidate["beginning_state_sha256"]!=sha(canonical(predecessor_state)) or candidate["ending_state_sha256"]!=sha(canonical(candidate["ending_state"])) or slab_candidate.get("start_ns")!=predecessor_end: raise ValueError("V11-V2-CHECKPOINT")
  predecessor_state=candidate["ending_state"]; predecessor_end=slab_candidate.get("end_ns")
 segment=cp["accepted_segments"][0]; slab=receipt_body(segment["slab_receipt"],"slab",0,v["parent_transaction_id"])
 exact(slab,{"start_ns","end_ns","duration_s_bits","beginning_state","forcing","ending_state"},"V11-V2-RECEIPT")
 if slab!={"start_ns":"0","end_ns":"600000000000","duration_s_bits":bits(600.0),"beginning_state":0,"forcing":1,"ending_state":1} or cp["beginning_state"]["value"]!=0 or cp["staged_state"]["value"]!=1 or predecessor_state!=cp["staged_state"] or predecessor_end!=cp["accepted_until_ns"] or segment["ending_complete_owners"]!=cp["staged_complete_owners"] or segment["ending_complete_owners"]!=v["staged_complete_owners"]: raise ValueError("V11-V2-CHECKPOINT")
 if len(segment["resource_receipts"])!=3 or len(segment["material_receipts"])!=1: raise ValueError("V11-V2-CHECKPOINT")
 resource_bodies=[]
 for i,r in enumerate(segment["resource_receipts"]):
  body=receipt_body(r,"resource",i,v["parent_transaction_id"]); exact(body,{"resource","beginning_bits","final_use_bits","ending_bits"},"V11-V2-RECEIPT")
  if unbits(body["ending_bits"])!=unbits(body["beginning_bits"])-unbits(body["final_use_bits"]): raise ValueError("V11-V2-CUSTODY")
  resource_bodies.append(body)
 material_bodies=[]
 for i,r in enumerate(segment["material_receipts"]):
  body=receipt_body(r,"material",i,v["parent_transaction_id"]); exact(body,{"source","receiver","amount_bits","slab_ordinal"},"V11-V2-RECEIPT"); material_bodies.append(body)
 expected_debits={x["resource"]:x["final_use_bits"] for x in resource_bodies}
 if cp["cumulative_debits"]!=expected_debits: raise ValueError("V11-V2-CUSTODY")
 if begin_states[0]!={"state_value":0} or staged_states[0]!={"state_value":1} or begin_states[1]!={"liquid_bits":bits(.25)} or staged_states[1]!={"liquid_bits":bits(0)} or begin_states[3]!={"liquid_bits":bits(0)} or staged_states[3]!={"liquid_bits":bits(.25)}: raise ValueError("V11-V2-CUSTODY")
 if begin_states[4]!={"water_bits":resource_bodies[0]["beginning_bits"]} or staged_states[4]!={"water_bits":resource_bodies[0]["ending_bits"]}: raise ValueError("V11-V2-CUSTODY")
 if begin_states[5]!={"nh4_bits":resource_bodies[1]["beginning_bits"],"no3_bits":resource_bodies[2]["beginning_bits"],"material_bits":bits(0)} or staged_states[5]!={"nh4_bits":resource_bodies[1]["ending_bits"],"no3_bits":resource_bodies[2]["ending_bits"],"material_bits":material_bodies[0]["amount_bits"]}: raise ValueError("V11-V2-CUSTODY")
 events=[receipt_body(r,"event",i,v["parent_transaction_id"]) for i,r in enumerate(v["accepted_event_receipts"])]
 for body in events: exact(body,{"tick_ns","from_participants","to_participants","source_owner","receiver_owner","transfer_bits"},"V11-V2-EVENT")
 expected_event={"tick_ns":v["accepted_until_ns"],"from_participants":["vegetation","snow"],"to_participants":v["active_participant_ids"],"source_owner":"snow","receiver_owner":"surface_liquid","transfer_bits":bits(.25)}
 if len(events)!=v["next_event_ordinal"] or events!=[expected_event] or begin_states[1]["liquid_bits"]!=events[0]["transfer_bits"] or staged_states[1]["liquid_bits"]!=bits(0) or begin_states[3]["liquid_bits"]!=bits(0) or staged_states[3]["liquid_bits"]!=events[0]["transfer_bits"] or v["active_regime_id"]!="snow-free": raise ValueError("V11-V2-EVENT")
 if len(v["scheduled_execution_keys"])!=len(set(v["scheduled_execution_keys"])) or v["scheduled_execution_keys"]!=sorted(v["scheduled_execution_keys"]) or v["scheduled_execution_keys"]!=[sha(b"parent-24:gsi")]: raise ValueError("V11-V2-SCHEDULED")
 ordered_unique(v["reduction_operands"],lambda x:x.get("operand_id"),"V11-V2-REDUCTION")
 values=[]
 for operand in v["reduction_operands"]:
  exact(operand,{"operand_id","source_receipt_id","support_start_ns","support_end_ns","value_bits"},"V11-V2-REDUCTION")
  expected_id=sha(canonical({k:operand[k] for k in ("source_receipt_id","support_start_ns","support_end_ns","value_bits")}))
  if operand["operand_id"]!=expected_id or int(operand["support_end_ns"])>int(v["accepted_until_ns"]) or operand["source_receipt_id"]!=segment["slab_receipt"]["receipt_id"]: raise ValueError("V11-V2-REDUCTION")
  values.append(unbits(operand["value_bits"]))
 if not values or max(values)!=.2: raise ValueError("V11-V2-REDUCTION")
 reduction_id=sha(canonical([x["operand_id"] for x in v["reduction_operands"]])); records={}
 ordered_unique(v["pending_publication_records"],lambda x:x.get("record_id"),"V11-V2-PUBLICATION")
 for record in v["pending_publication_records"]:
  exact(record,{"record_id","source_reduction_id","payload_canonical_base64","payload_sha256"},"V11-V2-PUBLICATION")
  body=decode(record["payload_canonical_base64"],record["payload_sha256"])
  expected_record=sha(reduction_id.encode()+record["payload_sha256"].encode())
  if body!={"value_bits":bits(max(values))} or record["source_reduction_id"]!=reduction_id or record["record_id"]!=expected_record: raise ValueError("V11-V2-PUBLICATION")
  if record["record_id"] in records: raise ValueError("V11-V2-PUBLICATION")
  records[record["record_id"]]=record
 ordered_unique(v["publication_outbox"],lambda x:x.get("outbox_id"),"V11-V2-PUBLICATION")
 bound_records=set()
 for row in v["publication_outbox"]:
  exact(row,{"outbox_id","record_id","state","delivery_count"},"V11-V2-PUBLICATION")
  expected_outbox=sha(canonical({"parent_transaction_id":v["parent_transaction_id"],"record_id":row["record_id"],"state":row["state"],"delivery_count":row["delivery_count"]}))
  if row["outbox_id"]!=expected_outbox or row["record_id"] not in records or row["record_id"] in bound_records or (row["state"]=="CommittedUndelivered" and row["delivery_count"]!=0) or (row["state"]!="CommittedUndelivered" and row["delivery_count"]==0): raise ValueError("V11-V2-PUBLICATION")
  bound_records.add(row["record_id"])
 if bound_records!=set(records): raise ValueError("V11-V2-PUBLICATION")
 return cp
def restore_suffix(v):
 cp=validate(v); staged=cp["staged_state"]["value"]; staged_values=owners(v["staged_complete_owners"])
 ending=staged*2+4+1
 uninterrupted=(0*2+1+0)*2+4+1
 if ending!=uninterrupted: raise ValueError("V11-V2-CHECKPOINT")
 suffix_states={"vegetation":{"state_value":ending},"snow":staged_values[1],"land_surface_energy":{"canopy_air_temperature_bits":bits(unbits(staged_values[2]["canopy_air_temperature_bits"])+1),"canopy_air_humidity_bits":bits(unbits(staged_values[2]["canopy_air_humidity_bits"])+.001)},"surface_liquid":staged_values[3],"hydrology":{"water_bits":bits(unbits(staged_values[4]["water_bits"])-.5)},"bgc":{"nh4_bits":bits(unbits(staged_values[5]["nh4_bits"])-.05),"no3_bits":bits(unbits(staged_values[5]["no3_bits"])-.05),"material_bits":bits(unbits(staged_values[5]["material_bits"])+.01)},"soil_thermal":{"temperature_bits":bits(unbits(staged_values[6]["temperature_bits"])+1)}}
 final_owners=[owner(name,"ending",**suffix_states[name]) for name in OWNERS]
 final={"ending_complete_owners":final_owners,"accepted_event_receipts":v["accepted_event_receipts"],"scheduled_execution_keys":v["scheduled_execution_keys"],"reduction_operands":v["reduction_operands"],"pending_publication_records":v["pending_publication_records"],"publication_outbox":v["publication_outbox"],"prefix_resource_receipts":cp["accepted_segments"][0]["resource_receipts"],"prefix_material_receipts":cp["accepted_segments"][0]["material_receipts"],"next_parent_transaction_sequence":v["next_parent_transaction_sequence"]}
 expected_states={"vegetation":{"state_value":7},"snow":{"liquid_bits":bits(0)},"land_surface_energy":{"canopy_air_temperature_bits":bits(276+1),"canopy_air_humidity_bits":bits(.003+.001)},"surface_liquid":{"liquid_bits":bits(.25)},"hydrology":{"water_bits":bits(2-.5)},"bgc":{"nh4_bits":bits(.3-.1-.05),"no3_bits":bits(.7-.3-.05),"material_bits":bits(.01+.01)},"soil_thermal":{"temperature_bits":bits(275+1)}}
 uninterrupted_final={"ending_complete_owners":[owner(name,"ending",**expected_states[name]) for name in OWNERS],"accepted_event_receipts":build()["accepted_event_receipts"],"scheduled_execution_keys":build()["scheduled_execution_keys"],"reduction_operands":build()["reduction_operands"],"pending_publication_records":build()["pending_publication_records"],"publication_outbox":build()["publication_outbox"],"prefix_resource_receipts":decode(build()["parent_checkpoint_canonical_base64"],build()["parent_checkpoint_sha256"])["accepted_segments"][0]["resource_receipts"],"prefix_material_receipts":decode(build()["parent_checkpoint_canonical_base64"],build()["parent_checkpoint_sha256"])["accepted_segments"][0]["material_receipts"],"next_parent_transaction_sequence":"25"}
 if canonical(final)!=canonical(uninterrupted_final): raise ValueError("V11-V2-CHECKPOINT")
 return {"owner_count":7,"ending_state":ending,"uninterrupted_state":uninterrupted,"suffix_slabs":1,"complete_continuation_sha256":sha(canonical(final))}
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
 elif name=="joint_sequence_forgery":
  x["parent_transaction_sequence"]="25"; x["next_parent_transaction_sequence"]="26"; cp=decode(x["parent_checkpoint_canonical_base64"],x["parent_checkpoint_sha256"]); cp["beginning_state"]["last_parent_transaction_sequence"]="24"; cp["staged_state"]["last_parent_transaction_sequence"]="24"; reframe(x,"parent_checkpoint",cp)
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
 elif name in ("zero_segment_predecessor","changed_terminal_vegetation_owner"):
  cp=decode(x["parent_checkpoint_canonical_base64"],x["parent_checkpoint_sha256"]); segment=cp["accepted_segments"][0]
  if name=="zero_segment_predecessor": segment["beginning_state_sha256"]="0"*64
  else:
   row=segment["ending_complete_owners"][0]; body=decode(row["state_canonical_base64"],row["state_sha256"]); body["state"]["state_value"]=99; b,d=enc(body); row["state_canonical_base64"]=b; row["state_sha256"]=d
  reframe(x,"parent_checkpoint",cp)
 elif name=="event_wrong_tick":
  body=receipt_body(x["accepted_event_receipts"][0],"event",0,x["parent_transaction_id"]); body["tick_ns"]="42"; x["accepted_event_receipts"][0]=receipt("event",0,x["parent_transaction_id"],body)
 elif name in ("event_wrong_source","event_wrong_receiver","event_wrong_from_participants"):
  body=receipt_body(x["accepted_event_receipts"][0],"event",0,x["parent_transaction_id"])
  if name=="event_wrong_source": body["source_owner"]="bgc"
  elif name=="event_wrong_receiver": body["receiver_owner"]="hydrology"
  else: body["from_participants"]=["vegetation","bgc"]
  x["accepted_event_receipts"][0]=receipt("event",0,x["parent_transaction_id"],body)
 elif name=="scheduled_wrong_key": x["scheduled_execution_keys"]=[sha(b"wrong")]
 elif name=="duplicate_scheduled_key": x["scheduled_execution_keys"].append(x["scheduled_execution_keys"][0])
 elif name=="duplicate_event_receipt": x["accepted_event_receipts"].append(copy.deepcopy(x["accepted_event_receipts"][0]))
 elif name=="reduction_wrong_source": x["reduction_operands"][0]["source_receipt_id"]="0"*64
 elif name=="publication_wrong_payload":
  raw=canonical({"value_bits":bits(9.0)}); x["pending_publication_records"][0]["payload_canonical_base64"]=base64.b64encode(raw).decode(); x["pending_publication_records"][0]["payload_sha256"]=sha(raw)
 elif name=="outbox_impossible_count": x["publication_outbox"]=[{"outbox_id":sha(b"outbox"),"record_id":x["pending_publication_records"][0]["record_id"],"state":"CommittedUndelivered","delivery_count":1}]
 elif name=="outbox_zero_id": x["publication_outbox"][0]["outbox_id"]="0"*64
 elif name=="outbox_duplicate": x["publication_outbox"].append(copy.deepcopy(x["publication_outbox"][0]))
 elif name in ("duplicate_reduction_operand","duplicate_publication_record","reordered_reduction_operands"):
  if name=="duplicate_reduction_operand": x["reduction_operands"].append(copy.deepcopy(x["reduction_operands"][0]))
  elif name=="duplicate_publication_record": x["pending_publication_records"].append(copy.deepcopy(x["pending_publication_records"][0]))
  else:
   extra=copy.deepcopy(x["reduction_operands"][0]); extra["value_bits"]=bits(.1); extra["operand_id"]=sha(canonical({k:extra[k] for k in ("source_receipt_id","support_start_ns","support_end_ns","value_bits")})); x["reduction_operands"]=[extra,x["reduction_operands"][0]]
 elif name=="wrong_active_regime": x["active_regime_id"]="snow-covered"
 elif name in ("resource_final_use_reframed","cumulative_debit_reframed","material_amount_reframed","unknown_checkpoint_field","unknown_receipt_field"):
  cp=decode(x["parent_checkpoint_canonical_base64"],x["parent_checkpoint_sha256"]); segment=cp["accepted_segments"][0]
  if name=="resource_final_use_reframed":
   r=segment["resource_receipts"][0]; body=receipt_body(r,"resource",0,x["parent_transaction_id"]); body["final_use_bits"]=bits(.5); segment["resource_receipts"][0]=receipt("resource",0,x["parent_transaction_id"],body)
  elif name=="cumulative_debit_reframed": cp["cumulative_debits"]["water"]=bits(.5)
  elif name=="material_amount_reframed":
   r=segment["material_receipts"][0]; body=receipt_body(r,"material",0,x["parent_transaction_id"]); body["amount_bits"]=bits(.5); segment["material_receipts"][0]=receipt("material",0,x["parent_transaction_id"],body)
  elif name=="unknown_checkpoint_field": cp["unknown"]=1
  else: segment["slab_receipt"]["unknown"]=1
  reframe(x,"parent_checkpoint",cp)
 elif name in ("duplicate_resource_receipt","duplicate_material_receipt"):
  cp=decode(x["parent_checkpoint_canonical_base64"],x["parent_checkpoint_sha256"]); key="resource_receipts" if name=="duplicate_resource_receipt" else "material_receipts"; cp["accepted_segments"][0][key].append(copy.deepcopy(cp["accepted_segments"][0][key][0])); reframe(x,"parent_checkpoint",cp)
 elif name=="reduction_operand_id_forged": x["reduction_operands"][0]["operand_id"]="0"*64
 elif name=="publication_record_id_forged": x["pending_publication_records"][0]["record_id"]="0"*64
 elif name=="unknown_top_field": x["unknown"]=1
 elif name=="unknown_coupled_field":
  coupled=decode(x["coupled_time_v2_canonical_base64"],x["coupled_time_v2_sha256"]); coupled["unknown"]=1; reframe(x,"coupled_time_v2",coupled)
 elif name in ("unknown_beginning_state_field","unknown_event_field","unknown_lse_state_field","forged_lse_state","forged_soil_thermal_state"):
  if name=="unknown_event_field":
   body=receipt_body(x["accepted_event_receipts"][0],"event",0,x["parent_transaction_id"]); body["unknown"]=1; x["accepted_event_receipts"][0]=receipt("event",0,x["parent_transaction_id"],body)
  elif name=="unknown_beginning_state_field":
   cp=decode(x["parent_checkpoint_canonical_base64"],x["parent_checkpoint_sha256"]); cp["beginning_state"]["unknown"]=1; reframe(x,"parent_checkpoint",cp)
  elif name=="unknown_lse_state_field":
   row=x["staged_complete_owners"][2]; body=decode(row["state_canonical_base64"],row["state_sha256"]); body["state"]["unknown"]=1; b,d=enc(body); row["state_canonical_base64"]=b; row["state_sha256"]=d
  else:
   index=2 if name=="forged_lse_state" else 6; row=x["staged_complete_owners"][index]; body=decode(row["state_canonical_base64"],row["state_sha256"]); field="canopy_air_temperature_bits" if index==2 else "temperature_bits"; body["state"][field]=bits(999); b,d=enc(body); replacement={"owner_id":row["owner_id"],"state_canonical_base64":b,"state_sha256":d}; x["staged_complete_owners"][index]=replacement
   cp=decode(x["parent_checkpoint_canonical_base64"],x["parent_checkpoint_sha256"]); cp["staged_complete_owners"][index]=replacement; cp["accepted_segments"][0]["ending_complete_owners"][index]=replacement; reframe(x,"parent_checkpoint",cp)
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
