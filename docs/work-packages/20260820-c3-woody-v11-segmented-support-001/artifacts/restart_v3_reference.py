"""Independent Restart V3 accepted-prefix custody oracle; no production imports."""
import base64,copy,hashlib,importlib.util,json,struct
from pathlib import Path
ROOT=Path(__file__).resolve().parent
OWNERS=["vegetation","snow","land_surface_energy","surface_liquid","hydrology","bgc","soil_thermal"]
def sha(b):return hashlib.sha256(b).hexdigest()
def sid(x):return sha(x.encode())
def canon(x):return json.dumps(x,sort_keys=True,separators=(",",":")).encode()
def derive(domain,x,field):return sha(domain.encode()+canon({k:v for k,v in x.items() if k!=field}))
def bits(x):return struct.pack(">d",x).hex()
def num(x):return struct.unpack(">d",bytes.fromhex(x))[0]
def support(a,b):return {"start_ns":str(a),"end_ns":str(b)}
def v2mod():
 s=importlib.util.spec_from_file_location("v2",ROOT/"restart_v2_reference.py");m=importlib.util.module_from_spec(s);s.loader.exec_module(m);return m
def decode(c,k):return json.loads(base64.b64decode(c[k],validate=True))
def projection(c):
 ct=decode(c,"coupled_time_v2_canonical_base64");pc=decode(c,"parent_checkpoint_canonical_base64");slabs=[]
 for seg in pc["accepted_segments"]:
  r=seg["slab_receipt"];p=json.loads(base64.b64decode(r["payload_canonical_base64"],validate=True));slabs.append({"slab_ordinal":r["ordinal"],"receipt_id":r["receipt_id"],"support":support(int(p["start_ns"]),int(p["end_ns"]))})
 return ct,pc,slabs
def debit(parent,segment,slab,sup,owner,occ,source,basis,amount):
 x={"receipt_id":"","parent_transaction_id":parent,"segment_id":segment,"accepted_slab_id":slab,"support":sup,"owner_id":owner,"ofe_id":"ofe-1","tile_id":"tile-a","occupancy_id":occ,"layer_id":"layer-1","source_id":source,"amount_basis":basis,"request_bits":bits(amount),"authorization_bits":bits(amount),"final_use_bits":bits(amount)};x["receipt_id"]=derive("v3-debit",x,"receipt_id");return x
def candidate(owner,ordinal,sup,components,envelope):return {"owner_id":owner,"slab_ordinal":ordinal,"support":sup,"components":sorted(components,key=lambda z:z["component_id"]),"state_bytes_base64":envelope["state_canonical_base64"],"state_sha256":envelope["state_sha256"]}
def transition(parent,segment,slab,sup,owner,source,basis,begin,end,links,c):
 x={"transition_id":"","parent_transaction_id":parent,"segment_id":segment,"accepted_slab_id":slab,"support":sup,"owner_id":owner,"ofe_id":"ofe-1","layer_id":"layer-1","source_id":source,"amount_basis":basis,"beginning_bits":bits(begin),"ending_bits":bits(end),"debit_receipt_ids":sorted(links),"other_flux_receipt_ids":[],"owner_candidate_bytes_base64":c["state_bytes_base64"],"owner_candidate_sha256":c["state_sha256"]};x["transition_id"]=derive("v3-transition",x,"transition_id");return x
def fixture():
 m=v2mod();v2=m.build();ct,pc,slabs=projection(v2);s=slabs[0];parent=v2["parent_transaction_id"];segment=v2["active_segment_id"];staged={x["owner_id"]:x for x in v2["staged_complete_owners"]}
 d=[debit(parent,segment,s["receipt_id"],s["support"],"hydrology","occ-water","soil_water","kg_m2",1),debit(parent,segment,s["receipt_id"],s["support"],"bgc","occ-nh4","nh4","kg_n_m2",.1),debit(parent,segment,s["receipt_id"],s["support"],"bgc","occ-no3","no3","kg_n_m2",.3)]
 components={o:[] for o in OWNERS};components["hydrology"]=[{"component_id":"soil_water","ending_bits":bits(2),"debit_receipt_ids":[d[0]["receipt_id"]],"other_flux_receipt_ids":[]}];components["bgc"]=[{"component_id":"nh4","ending_bits":bits(.2),"debit_receipt_ids":[d[1]["receipt_id"]],"other_flux_receipt_ids":[]},{"component_id":"no3","ending_bits":bits(.4),"debit_receipt_ids":[d[2]["receipt_id"]],"other_flux_receipt_ids":[]}]
 cs=[candidate(o,0,s["support"],components[o],staged[o]) for o in OWNERS];cm={x["owner_id"]:x for x in cs}
 ts=[transition(parent,segment,s["receipt_id"],s["support"],"hydrology","soil_water","kg_m2",3,2,[d[0]["receipt_id"]],cm["hydrology"]),transition(parent,segment,s["receipt_id"],s["support"],"bgc","nh4","kg_n_m2",.3,.2,[d[1]["receipt_id"]],cm["bgc"]),transition(parent,segment,s["receipt_id"],s["support"],"bgc","no3","kg_n_m2",.7,.4,[d[2]["receipt_id"]],cm["bgc"])]
 raw=canon(pc["staged_state"])
 return {"schema":"OPENWEPP_C3_WOODY_V11_RESTART_V3","v2_checkpoint":v2,"parent_transaction_id":parent,"parent_support":support(int(ct["parent_start_ns"]),int(ct["parent_end_ns"])),"checkpoint_position":{"accepted_until_ns":ct["accepted_until_ns"],"next_slab_ordinal":ct["next_slab_ordinal"],"next_event_ordinal":ct["next_event_ordinal"],"accepted_slabs":slabs,"accepted_event_receipt_ids":[x["receipt_id"] for x in v2["accepted_event_receipts"]]},"other_flux_receipts":[],"debit_receipts":sorted(d,key=lambda z:z["receipt_id"]),"shared_owner_transitions":sorted(ts,key=lambda z:(z["owner_id"],z["source_id"])),"complete_owner_candidates":cs,"terminal_complete_owners":[{"owner_id":o,"state_bytes_base64":cm[o]["state_bytes_base64"],"state_sha256":cm[o]["state_sha256"]} for o in OWNERS],"terminal_v11_state":{"state_canonical_base64":base64.b64encode(raw).decode(),"state_sha256":sha(raw)}}

# Frozen future input is external to the checkpoint.
SUFFIX={"support":support(600000000000,1800000000000),"forcing_id":sid("future-forcing"),"water_use_bits":bits(.5),"nh4_use_bits":bits(.05),"reduction_bits":bits(7.25),"publication":base64.b64encode(b"future-publication").decode()}
def execute_suffix(c,consume=True):
 if not consume:return {"owners":c["terminal_complete_owners"],"resource_receipts":[x["receipt_id"] for x in c["debit_receipts"]],"slabs":c["checkpoint_position"]["accepted_slabs"],"events":c["checkpoint_position"]["accepted_event_receipt_ids"],"reduction_bits":bits(0),"publication":[]}
 owners=[]
 for x in c["terminal_complete_owners"]:
  raw=canon({"beginning_state_sha256":x["state_sha256"],"forcing_id":SUFFIX["forcing_id"],"owner_id":x["owner_id"],"support":SUFFIX["support"]});owners.append({"owner_id":x["owner_id"],"state_bytes_base64":base64.b64encode(raw).decode(),"state_sha256":sha(raw)})
 future=[sid("future-water-receipt"),sid("future-nh4-receipt")]
 return {"owners":owners,"resource_receipts":[x["receipt_id"] for x in c["debit_receipts"]]+future,"slabs":c["checkpoint_position"]["accepted_slabs"]+[{"slab_ordinal":c["checkpoint_position"]["next_slab_ordinal"],"receipt_id":sid("future-slab"),"support":SUFFIX["support"]}],"events":c["checkpoint_position"]["accepted_event_receipt_ids"],"reduction_bits":SUFFIX["reduction_bits"],"publication":[{"record_id":sid("future-publication"),"payload_base64":SUFFIX["publication"]}]}
def validate(c,consume=True):
 req={"schema","v2_checkpoint","parent_transaction_id","parent_support","checkpoint_position","other_flux_receipts","debit_receipts","shared_owner_transitions","complete_owner_candidates","terminal_complete_owners","terminal_v11_state"}
 if set(c)!=req or c["schema"]!="OPENWEPP_C3_WOODY_V11_RESTART_V3":raise ValueError("V3-SCHEMA")
 m=v2mod();m.validate(c["v2_checkpoint"]);ct,pc,slabs=projection(c["v2_checkpoint"])
 if c["parent_transaction_id"]!=c["v2_checkpoint"]["parent_transaction_id"] or c["parent_support"]!=support(int(ct["parent_start_ns"]),int(ct["parent_end_ns"])):raise ValueError("V3-V2")
 expected={"accepted_until_ns":ct["accepted_until_ns"],"next_slab_ordinal":ct["next_slab_ordinal"],"next_event_ordinal":ct["next_event_ordinal"],"accepted_slabs":slabs,"accepted_event_receipt_ids":[x["receipt_id"] for x in c["v2_checkpoint"]["accepted_event_receipts"]]}
 if c["checkpoint_position"]!=expected:raise ValueError("V3-POSITION")
 n=len(slabs);accepted={x["receipt_id"]:(x["slab_ordinal"],x["support"]) for x in slabs};cs=c["complete_owner_candidates"]
 if len(cs)!=7*n or [(x["slab_ordinal"],x["owner_id"]) for x in cs]!=[(i,o) for i in range(n) for o in OWNERS]:raise ValueError("V3-CANDIDATE-CARDINALITY")
 cm={}
 for x in cs:
  if x["components"]!=sorted(x["components"],key=lambda z:z["component_id"]) or x["support"]!=slabs[x["slab_ordinal"]]["support"]:raise ValueError("V3-CANDIDATE")
  if sha(base64.b64decode(x["state_bytes_base64"],validate=True))!=x["state_sha256"]:raise ValueError("V3-CANDIDATE")
  cm[(x["owner_id"],x["slab_ordinal"])]=x
 if c["other_flux_receipts"]!=sorted(c["other_flux_receipts"],key=lambda z:z["receipt_id"]):raise ValueError("V3-ORDER")
 fluxids=set()
 for f in c["other_flux_receipts"]:
  if f["receipt_id"]!=derive("v3-flux",f,"receipt_id") or f["accepted_slab_id"] not in accepted:raise ValueError("V3-FLUX")
  fluxids.add(f["receipt_id"])
 ds=c["debit_receipts"]
 if ds!=sorted(ds,key=lambda z:z["receipt_id"]) or len({x["receipt_id"] for x in ds})!=len(ds):raise ValueError("V3-DEBIT")
 byid={x["receipt_id"]:x for x in ds}
 for d in ds:
  if d["receipt_id"]!=derive("v3-debit",d,"receipt_id") or d["accepted_slab_id"] not in accepted or d["support"]!=accepted[d["accepted_slab_id"]][1]:raise ValueError("V3-DEBIT")
  if not 0<=num(d["final_use_bits"])<=num(d["authorization_bits"])<=num(d["request_bits"]):raise ValueError("V3-DEBIT")
 ts=c["shared_owner_transitions"]
 if ts!=sorted(ts,key=lambda z:(z["owner_id"],z["source_id"])):raise ValueError("V3-ORDER")
 linked=[];prior={}
 for t in ts:
  if t["accepted_slab_id"] not in accepted:raise ValueError("V3-SUPPORT")
  ordinal,sup=accepted[t["accepted_slab_id"]]
  if t["support"]!=sup:raise ValueError("V3-SUPPORT")
  cc=cm[(t["owner_id"],ordinal)]
  if (t["owner_candidate_bytes_base64"],t["owner_candidate_sha256"])!=(cc["state_bytes_base64"],cc["state_sha256"]):raise ValueError("V3-CANDIDATE")
  comp={"component_id":t["source_id"],"ending_bits":t["ending_bits"],"debit_receipt_ids":t["debit_receipt_ids"],"other_flux_receipt_ids":t["other_flux_receipt_ids"]}
  if comp not in cc["components"]:raise ValueError("V3-CANDIDATE")
  if t["transition_id"]!=derive("v3-transition",t,"transition_id"):raise ValueError("V3-TRANSITION")
  auth=use=0.0
  for rid in t["debit_receipt_ids"]:
   d=byid.get(rid)
   if not d:raise ValueError("V3-LINK")
   if (d["accepted_slab_id"],d["support"],d["owner_id"],d["ofe_id"],d["layer_id"],d["source_id"],d["amount_basis"])!=(t["accepted_slab_id"],t["support"],t["owner_id"],t["ofe_id"],t["layer_id"],t["source_id"],t["amount_basis"]):raise ValueError("V3-ALIAS")
   linked.append(rid);auth+=num(d["authorization_bits"]);use+=num(d["final_use_bits"])
  inflow=sum(num(f["amount_bits"]) for f in c["other_flux_receipts"] if f["receipt_id"] in t["other_flux_receipt_ids"])
  if auth>num(t["beginning_bits"])+inflow or use>num(t["beginning_bits"])+inflow:raise ValueError("V3-OVERBOOK")
  key=(t["owner_id"],t["ofe_id"],t["layer_id"],t["source_id"],t["amount_basis"])
  if key in prior and prior[key]!=t["beginning_bits"]:raise ValueError("V3-CHAIN")
  prior[key]=t["ending_bits"]
 if sorted(linked)!=sorted(byid):raise ValueError("V3-LINK")
 staged={x["owner_id"]:x for x in c["v2_checkpoint"]["staged_complete_owners"]}
 if [x["owner_id"] for x in c["terminal_complete_owners"]]!=OWNERS:raise ValueError("V3-TERMINAL")
 for o in c["terminal_complete_owners"]:
  cc=cm[(o["owner_id"],n-1)];v=staged[o["owner_id"]]
  if (o["state_bytes_base64"],o["state_sha256"])!=(cc["state_bytes_base64"],cc["state_sha256"]):raise ValueError("V3-TERMINAL")
  if (o["state_bytes_base64"],o["state_sha256"])!=(v["state_canonical_base64"],v["state_sha256"]):raise ValueError("V3-V2-TERMINAL")
 raw=base64.b64decode(c["terminal_v11_state"]["state_canonical_base64"],validate=True)
 if sha(raw)!=c["terminal_v11_state"]["state_sha256"] or json.loads(raw)!=pc["staged_state"]:raise ValueError("V3-V11-STATE")
 restored=execute_suffix(c,consume);full=execute_suffix(c,True)
 if canon(restored)!=canon(full):raise ValueError("V3-SUFFIX")
 return {"accepted_prefix_slabs":n,"candidate_count":len(cs),"transition_count":len(ts),"complete_suffix_sha256":sha(canon(full))}
def poison(x,name):
 if name=="grafted_valid_v3_v2_mismatch":x["parent_support"]["end_ns"]="1800000000001"
 elif name=="support_scale":x["checkpoint_position"]["accepted_slabs"][0]["support"]["end_ns"]="600"
 elif name=="cursor":x["checkpoint_position"]["accepted_until_ns"]="600000000001"
 elif name=="prefix_substitution":x["debit_receipts"][0]["accepted_slab_id"]=sid("future-slab")
 elif name=="terminal_owner":x["terminal_complete_owners"][4]["state_sha256"]="0"*64
 elif name=="missing_candidate":x["complete_owner_candidates"].pop()
 elif name=="extra_candidate":x["complete_owner_candidates"].append(copy.deepcopy(x["complete_owner_candidates"][-1]))
 elif name=="suffix_not_consumed":return False
 elif name=="forged_candidate":x["complete_owner_candidates"][0]["state_sha256"]="0"*64
 elif name=="missing_debit_link":x["shared_owner_transitions"][0]["debit_receipt_ids"].pop()
 return True
def main():
 base=fixture();accepted=validate(base);out=[]
 for p in json.loads((ROOT/"restart-v3-poisons.json").read_text()):
  x=copy.deepcopy(base);consume=poison(x,p["mutation"])
  try:validate(x,consume);raise SystemExit("accepted poison "+p["id"])
  except ValueError as e:
   if str(e)!=p["error"]:raise
   out.append({"id":p["id"],"error":str(e)})
 print(json.dumps({"schema":"OPENWEPP_C3_WOODY_V11_RESTART_V3_RESULTS_V2","accepted":accepted,"poisons":out},sort_keys=True,separators=(",",":")))
if __name__=="__main__":main()
