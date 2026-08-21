"""Independent closed Restart V3 resource-custody oracle; imports no production code."""
import base64,copy,hashlib,importlib.util,json,struct
from pathlib import Path
ROOT=Path(__file__).resolve().parent
OWNERS=["vegetation","snow","land_surface_energy","surface_liquid","hydrology","bgc","soil_thermal"]
def sha(b):return hashlib.sha256(b).hexdigest()
def sid(x):return sha(x.encode())
def canonical(x):return json.dumps(x,sort_keys=True,separators=(",",":")).encode()
def derived(domain,x,id_field):return sha(domain.encode()+canonical({k:v for k,v in x.items() if k!=id_field}))
def bits(x):import struct;return struct.pack(">d",x).hex()
def number(x):return struct.unpack(">d",bytes.fromhex(x))[0]
def support(a,b):return {"start_ns":str(a),"end_ns":str(b)}
def candidate(owner,end):
 b=json.dumps({"owner":owner,"ending_bits":bits(end)},sort_keys=True,separators=(",",":")).encode();return base64.b64encode(b).decode(),sha(b)
def debit(name,slab,owner,ofe,tile,occ,resource,basis,use):
 a,b=(0,900) if slab==0 else (900,1800)
 x={"receipt_id":"","parent_transaction_id":sid("parent"),"segment_id":sid(f"seg{slab}"),"accepted_slab_id":sid(f"slab{slab}"),"support":support(a,b),"owner_id":owner,"ofe_id":ofe,"tile_id":tile,"occupancy_id":occ,"layer_id":"layer-1","source_id":resource,"amount_basis":basis,"request_bits":bits(use+1),"authorization_bits":bits(use+.5),"final_use_bits":bits(use)};x["receipt_id"]=derived("v3-debit",x,"receipt_id");return x
def transition(name,slab,owner,resource,basis,begin,end,links,fluxes):
 a,b=(0,900) if slab==0 else (900,1800);raw,digest=candidate(owner,end)
 x={"transition_id":"","parent_transaction_id":sid("parent"),"segment_id":sid(f"seg{slab}"),"accepted_slab_id":sid(f"slab{slab}"),"support":support(a,b),"owner_id":owner,"ofe_id":"ofe-1","layer_id":"layer-1","source_id":resource,"amount_basis":basis,"beginning_bits":bits(begin),"ending_bits":bits(end),"debit_receipt_ids":sorted(links),"other_flux_receipt_ids":sorted(fluxes),"owner_candidate_bytes_base64":raw,"owner_candidate_sha256":digest};x["transition_id"]=derived("v3-transition",x,"transition_id");return x
def flux(name,slab,owner,source,basis,amount):
 a,b=(0,900) if slab==0 else (900,1800);x={"receipt_id":"","parent_transaction_id":sid("parent"),"segment_id":sid(f"seg{slab}"),"accepted_slab_id":sid(f"slab{slab}"),"support":support(a,b),"owner_id":owner,"ofe_id":"ofe-1","layer_id":"layer-1","source_id":source,"amount_basis":basis,"amount_bits":bits(amount),"flux_class":name};x["receipt_id"]=derived("v3-flux",x,"receipt_id");return x
def complete_candidate(owner,slab,components):
 body={"owner_id":owner,"slab_ordinal":slab,"components":sorted(components,key=lambda x:x["component_id"])};raw=canonical(body);a,b=(0,900) if slab==0 else (900,1800);return {"owner_id":owner,"slab_ordinal":slab,"support":support(a,b),"components":body["components"],"state_bytes_base64":base64.b64encode(raw).decode(),"state_sha256":sha(raw)}
def fixture():
 spec=importlib.util.spec_from_file_location("v2",ROOT/"restart_v2_reference.py");v2=importlib.util.module_from_spec(spec);spec.loader.exec_module(v2);v2_checkpoint=v2.build();parent=v2_checkpoint["parent_transaction_id"]
 d=[debit("w0a",0,"hydrology","ofe-1","tile-a","occ-a","soil_water","kg_m2",1.5),debit("w0b",0,"hydrology","ofe-1","tile-b","occ-b","soil_water","kg_m2",1.0),debit("w1a",1,"hydrology","ofe-1","tile-a","occ-a","soil_water","kg_m2",.75),debit("nha",0,"bgc","ofe-1","tile-a","occ-a","nh4","kg_n_m2",.1),debit("noa",0,"bgc","ofe-1","tile-a","occ-a","no3","kg_n_m2",.2)]
 for x in d:x["parent_transaction_id"]=parent;x["receipt_id"]=derived("v3-debit",x,"receipt_id")
 f=[flux("runon",0,"hydrology","soil_water","kg_m2",.5),flux("mineralization",0,"bgc","mineral_n","kg_n_m2",.1),flux("runon",1,"hydrology","soil_water","kg_m2",.25)]
 for x in f:x["parent_transaction_id"]=parent;x["receipt_id"]=derived("v3-flux",x,"receipt_id")
 t=[transition("wt0",0,"hydrology","soil_water","kg_m2",10,8,[d[0]["receipt_id"],d[1]["receipt_id"]],[f[0]["receipt_id"]]),transition("nt0",0,"bgc","nh4","kg_n_m2",1,.8,[d[3]["receipt_id"]],[f[1]["receipt_id"]]),transition("no0",0,"bgc","no3","kg_n_m2",2,1.7,[d[4]["receipt_id"]],[f[1]["receipt_id"]]),transition("wt1",1,"hydrology","soil_water","kg_m2",8,6.5,[d[2]["receipt_id"]],[f[2]["receipt_id"]])]
 for x in t:x["parent_transaction_id"]=parent
 candidates=[]
 for slab in (0,1):
  for owner in OWNERS:
   components=[]
   for x in t:
    if x["owner_id"]==owner and int(x["support"]["start_ns"])==(0 if slab==0 else 900):components.append({"component_id":x["source_id"],"ending_bits":x["ending_bits"],"debit_receipt_ids":x["debit_receipt_ids"],"other_flux_receipt_ids":x["other_flux_receipt_ids"]})
   candidates.append(complete_candidate(owner,slab,components))
 cmap={(x["owner_id"],x["slab_ordinal"]):x for x in candidates}
 for x in t:
  slab=0 if x["support"]["start_ns"]=="0" else 1;c=cmap[(x["owner_id"],slab)];x["owner_candidate_bytes_base64"]=c["state_bytes_base64"];x["owner_candidate_sha256"]=c["state_sha256"];x["transition_id"]=derived("v3-transition",x,"transition_id")
 terminal=[{"owner_id":o,"state_bytes_base64":cmap[(o,1)]["state_bytes_base64"],"state_sha256":cmap[(o,1)]["state_sha256"]} for o in OWNERS]
 return {"schema":"OPENWEPP_C3_WOODY_V11_RESTART_V3","v2_checkpoint":v2_checkpoint,"parent_transaction_id":parent,"parent_support":support(0,1800),"other_flux_receipts":sorted(f,key=lambda x:x["receipt_id"]),"debit_receipts":sorted(d,key=lambda x:x["receipt_id"]),"shared_owner_transitions":sorted(t,key=lambda x:(int(x["support"]["start_ns"]),x["owner_id"],x["source_id"])),"complete_owner_candidates":candidates,"terminal_complete_owners":terminal}
def validate(c):
 if set(c)!={"schema","v2_checkpoint","parent_transaction_id","parent_support","other_flux_receipts","debit_receipts","shared_owner_transitions","complete_owner_candidates","terminal_complete_owners"} or c["schema"]!="OPENWEPP_C3_WOODY_V11_RESTART_V3":raise ValueError("V3-SCHEMA")
 spec=importlib.util.spec_from_file_location("v2",ROOT/"restart_v2_reference.py");v2=importlib.util.module_from_spec(spec);spec.loader.exec_module(v2);v2.validate(c["v2_checkpoint"]);suffix=v2.restore_suffix(c["v2_checkpoint"])
 if c["parent_transaction_id"]!=c["v2_checkpoint"]["parent_transaction_id"]:raise ValueError("V3-V2")
 fluxes=c["other_flux_receipts"]
 if fluxes!=sorted(fluxes,key=lambda x:x["receipt_id"]) or len({x["receipt_id"] for x in fluxes})!=len(fluxes):raise ValueError("V3-ORDER")
 for f in fluxes:
  if f["receipt_id"]!=derived("v3-flux",f,"receipt_id"):raise ValueError("V3-FLUX")
 flux_ids={x["receipt_id"] for x in fluxes}
 debits=c["debit_receipts"]
 if debits!=sorted(debits,key=lambda x:x["receipt_id"]) or len({x["receipt_id"] for x in debits})!=len(debits):raise ValueError("V3-DEBIT")
 byid={x["receipt_id"]:x for x in debits};linked=[];prior={};last={};debit_keys=set()
 for d in debits:
  if set(d)!={"receipt_id","parent_transaction_id","segment_id","accepted_slab_id","support","owner_id","ofe_id","tile_id","occupancy_id","layer_id","source_id","amount_basis","request_bits","authorization_bits","final_use_bits"}:raise ValueError("V3-DEBIT")
  if d["receipt_id"]!=derived("v3-debit",d,"receipt_id"):raise ValueError("V3-DEBIT")
  key=(d["parent_transaction_id"],d["segment_id"],d["accepted_slab_id"],d["owner_id"],d["ofe_id"],d["tile_id"],d["occupancy_id"],d["layer_id"],d["source_id"],d["amount_basis"])
  if key in debit_keys or d["parent_transaction_id"]!=c["parent_transaction_id"]:raise ValueError("V3-DEBIT")
  request,authorization,use=map(number,(d["request_bits"],d["authorization_bits"],d["final_use_bits"]))
  if not 0<=use<=authorization<=request:raise ValueError("V3-DEBIT")
  debit_keys.add(key)
 transition_keys=set()
 candidates=c["complete_owner_candidates"]
 if len(candidates)!=14 or [(x["slab_ordinal"],x["owner_id"]) for x in candidates]!=[(s,o) for s in (0,1) for o in OWNERS]:raise ValueError("V3-CANDIDATE")
 cmap={}
 for x in candidates:
  if x["components"]!=sorted(x["components"],key=lambda y:y["component_id"]) or len({y["component_id"] for y in x["components"]})!=len(x["components"]):raise ValueError("V3-CANDIDATE")
  raw=base64.b64decode(x["state_bytes_base64"],validate=True)
  if sha(raw)!=x["state_sha256"] or json.loads(raw)!={"components":x["components"],"owner_id":x["owner_id"],"slab_ordinal":x["slab_ordinal"]}:raise ValueError("V3-CANDIDATE")
  cmap[(x["owner_id"],x["slab_ordinal"])]=x
 for t in c["shared_owner_transitions"]:
  if set(t)!={"transition_id","parent_transaction_id","segment_id","accepted_slab_id","support","owner_id","ofe_id","layer_id","source_id","amount_basis","beginning_bits","ending_bits","debit_receipt_ids","other_flux_receipt_ids","owner_candidate_bytes_base64","owner_candidate_sha256"}:raise ValueError("V3-SCHEMA")
  a,b=map(int,(t["support"]["start_ns"],t["support"]["end_ns"]));pa,pb=map(int,(c["parent_support"]["start_ns"],c["parent_support"]["end_ns"]))
  if not pa<=a<b<=pb:raise ValueError("V3-SUPPORT")
  ordinal=0 if a==0 else 1 if a==900 else None
  if ordinal is None or t["parent_transaction_id"]!=c["parent_transaction_id"] or t["segment_id"]!=sid(f"seg{ordinal}") or t["accepted_slab_id"]!=sid(f"slab{ordinal}"):raise ValueError("V3-SUPPORT")
  transition_key=(t["owner_id"],t["ofe_id"],t["layer_id"],t["source_id"],t["amount_basis"],a,b)
  if transition_key in transition_keys:raise ValueError("V3-ORDER")
  transition_keys.add(transition_key)
 expected=sorted(c["shared_owner_transitions"],key=lambda x:(int(x["support"]["start_ns"]),x["owner_id"],x["source_id"]))
 if c["shared_owner_transitions"]!=expected:return (_ for _ in ()).throw(ValueError("V3-ORDER"))
 for t in expected:
  if not t["debit_receipt_ids"] or t["debit_receipt_ids"]!=sorted(set(t["debit_receipt_ids"])):raise ValueError("V3-LINK")
  if t["other_flux_receipt_ids"]!=sorted(set(t["other_flux_receipt_ids"])) or any(x not in flux_ids for x in t["other_flux_receipt_ids"]):raise ValueError("V3-FLUX")
  raw=base64.b64decode(t["owner_candidate_bytes_base64"],validate=True)
  if sha(raw)!=t["owner_candidate_sha256"]:raise ValueError("V3-CANDIDATE")
  slab=0 if t["support"]["start_ns"]=="0" else 1;complete=cmap[(t["owner_id"],slab)]
  if t["owner_candidate_sha256"]!=complete["state_sha256"] or t["owner_candidate_bytes_base64"]!=complete["state_bytes_base64"]:raise ValueError("V3-CANDIDATE")
  component=next((x for x in complete["components"] if x["component_id"]==t["source_id"]),None)
  if component!={"component_id":t["source_id"],"ending_bits":t["ending_bits"],"debit_receipt_ids":t["debit_receipt_ids"],"other_flux_receipt_ids":t["other_flux_receipt_ids"]}:raise ValueError("V3-CANDIDATE")
  if t["transition_id"]!=derived("v3-transition",t,"transition_id"):raise ValueError("V3-TRANSITION")
  key=(t["owner_id"],t["ofe_id"],t["layer_id"],t["source_id"],t["amount_basis"])
  if key in prior and prior[key]!=t["beginning_bits"]:raise ValueError("V3-CHAIN")
  authorization=0.0;final_use=0.0
  for rid in t["debit_receipt_ids"]:
   if rid not in byid:raise ValueError("V3-LINK")
   d=byid[rid]
   if (d["parent_transaction_id"],d["segment_id"],d["accepted_slab_id"],d["support"],d["owner_id"],d["ofe_id"],d["layer_id"],d["source_id"],d["amount_basis"])!=(t["parent_transaction_id"],t["segment_id"],t["accepted_slab_id"],t["support"],t["owner_id"],t["ofe_id"],t["layer_id"],t["source_id"],t["amount_basis"]):raise ValueError("V3-ALIAS")
   linked.append(rid)
   authorization+=number(d["authorization_bits"]);final_use+=number(d["final_use_bits"])
  admitted_inflow=sum(number(x["amount_bits"]) for x in fluxes if x["receipt_id"] in t["other_flux_receipt_ids"] and x["owner_id"]==t["owner_id"])
  if authorization>number(t["beginning_bits"])+admitted_inflow or final_use>number(t["beginning_bits"])+admitted_inflow:raise ValueError("V3-OVERBOOK")
  prior[key]=t["ending_bits"];last[t["owner_id"]]=t
 if sorted(linked)!=sorted(byid) or len(linked)!=len(set(linked)):raise ValueError("V3-LINK")
 if [x["owner_id"] for x in c["terminal_complete_owners"]]!=OWNERS:raise ValueError("V3-TERMINAL")
 for o in c["terminal_complete_owners"]:
  raw=base64.b64decode(o["state_bytes_base64"],validate=True)
  if sha(raw)!=o["state_sha256"]:raise ValueError("V3-TERMINAL")
 for owner in OWNERS:
  t=cmap[(owner,1)]
  o=next(x for x in c["terminal_complete_owners"] if x["owner_id"]==owner)
  if o["state_sha256"]!=t["state_sha256"] or o["state_bytes_base64"]!=t["state_bytes_base64"]:raise ValueError("V3-TERMINAL")
 return {"water_ending_bits":prior[("hydrology","ofe-1","layer-1","soil_water","kg_m2")],"debit_count":len(debits),"transition_count":len(expected),"complete_suffix_sha256":suffix["complete_continuation_sha256"]}
def rebuild_candidate(x,t):
 slab=0 if t["support"]["start_ns"]=="0" else 1;c=next(c for c in x["complete_owner_candidates"] if c["owner_id"]==t["owner_id"] and c["slab_ordinal"]==slab);component=next(z for z in c["components"] if z["component_id"]==t["source_id"]);component["debit_receipt_ids"]=t["debit_receipt_ids"];component["other_flux_receipt_ids"]=t["other_flux_receipt_ids"];raw=canonical({"owner_id":c["owner_id"],"slab_ordinal":slab,"components":c["components"]});c["state_bytes_base64"]=base64.b64encode(raw).decode();c["state_sha256"]=sha(raw);t["owner_candidate_bytes_base64"]=c["state_bytes_base64"];t["owner_candidate_sha256"]=c["state_sha256"];t["transition_id"]=derived("v3-transition",t,"transition_id")
 if slab==1:
  o=next(o for o in x["terminal_complete_owners"] if o["owner_id"]==t["owner_id"]);o["state_bytes_base64"]=c["state_bytes_base64"];o["state_sha256"]=c["state_sha256"]
def apply_poison(x,name):
 if name=="forged_candidate":x["complete_owner_candidates"][0]["state_sha256"]="0"*64
 elif name=="arbitrary_self_chain":
  t=x["shared_owner_transitions"][-1];t["beginning_bits"]=t["ending_bits"];t["transition_id"]=derived("v3-transition",t,"transition_id")
 elif name=="unknown_other_flux":x["shared_owner_transitions"][0]["other_flux_receipt_ids"]=[sid("unknown")]
 elif name=="reversed_links":next(t for t in x["shared_owner_transitions"] if len(t["debit_receipt_ids"])>1)["debit_receipt_ids"].reverse()
 elif name=="duplicate_transition":x["shared_owner_transitions"].append(copy.deepcopy(x["shared_owner_transitions"][0]))
 elif name=="out_of_support":x["shared_owner_transitions"][0]["support"]={"start_ns":"1800","end_ns":"1900"}
 elif name=="coordinated_debit_link":
  d=x["debit_receipts"][0];old=d["receipt_id"];d["final_use_bits"]=bits(number(d["final_use_bits"])/2);d["receipt_id"]=derived("v3-debit",d,"receipt_id");t=next(t for t in x["shared_owner_transitions"] if old in t["debit_receipt_ids"]);t["debit_receipt_ids"]=sorted(d["receipt_id"] if z==old else z for z in t["debit_receipt_ids"]);t["transition_id"]=derived("v3-transition",t,"transition_id")
  x["debit_receipts"].sort(key=lambda z:z["receipt_id"])
 elif name=="transition_id_reframe":x["shared_owner_transitions"][0]["transition_id"]="0"*64
 elif name=="coordinated_flux_link":
  f=x["other_flux_receipts"][0];old=f["receipt_id"];f["amount_bits"]=bits(number(f["amount_bits"])+1);f["receipt_id"]=derived("v3-flux",f,"receipt_id");t=next(t for t in x["shared_owner_transitions"] if old in t["other_flux_receipt_ids"]);t["other_flux_receipt_ids"]=[f["receipt_id"]];t["transition_id"]=derived("v3-transition",t,"transition_id")
  x["other_flux_receipts"].sort(key=lambda z:z["receipt_id"])
 elif name=="v2_field_omission":x["v2_checkpoint"].pop("coupled_time_v2_sha256")
 elif name=="v2_field_forgery":x["v2_checkpoint"]["next_parent_transaction_sequence"]="99"
 elif name=="two_100_vs_10":
  t=next(t for t in x["shared_owner_transitions"] if t["owner_id"]=="hydrology" and t["support"]["start_ns"]=="0");new=[]
  for rid in list(t["debit_receipt_ids"]):
   d=next(d for d in x["debit_receipts"] if d["receipt_id"]==rid);d["request_bits"]=bits(100);d["authorization_bits"]=bits(100);d["final_use_bits"]=bits(100);d["receipt_id"]=derived("v3-debit",d,"receipt_id");new.append(d["receipt_id"])
  t["debit_receipt_ids"]=sorted(new);rebuild_candidate(x,t)
  x["debit_receipts"].sort(key=lambda z:z["receipt_id"])
 elif name=="terminal_join_forgery":x["terminal_complete_owners"][4]["state_sha256"]="0"*64
 elif name=="missing_debit_link":x["shared_owner_transitions"][0]["debit_receipt_ids"].pop()
def main():
 base=fixture();accepted=validate(base);poisons=[]
 for p in json.loads((ROOT/"restart-v3-poisons.json").read_text()):
  x=copy.deepcopy(base);apply_poison(x,p["mutation"])
  try:validate(x);raise SystemExit("accepted poison "+p["id"])
  except ValueError as e:
   if str(e)!=p["error"]:raise
   poisons.append({"id":p["id"],"error":str(e)})
 print(json.dumps({"schema":"OPENWEPP_C3_WOODY_V11_RESTART_V3_RESULTS_V1","accepted":accepted,"poisons":poisons},sort_keys=True,separators=(",",":")))
if __name__=="__main__":main()
