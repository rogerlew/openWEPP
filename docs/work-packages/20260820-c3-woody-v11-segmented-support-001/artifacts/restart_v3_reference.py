"""Independent closed Restart V3 resource-custody oracle; imports no production code."""
import base64,copy,hashlib,json,struct
from pathlib import Path
ROOT=Path(__file__).resolve().parent
OWNERS=["vegetation","snow","land_surface_energy","surface_liquid","hydrology","bgc","soil_thermal"]
def sha(b):return hashlib.sha256(b).hexdigest()
def sid(x):return sha(x.encode())
def bits(x):import struct;return struct.pack(">d",x).hex()
def number(x):return struct.unpack(">d",bytes.fromhex(x))[0]
def support(a,b):return {"start_ns":str(a),"end_ns":str(b)}
def candidate(owner,end):
 b=json.dumps({"owner":owner,"ending_bits":bits(end)},sort_keys=True,separators=(",",":")).encode();return base64.b64encode(b).decode(),sha(b)
def debit(name,slab,owner,ofe,tile,occ,resource,basis,use):
 a,b=(0,900) if slab==0 else (900,1800)
 return {"receipt_id":sid(name),"parent_transaction_id":sid("parent"),"segment_id":sid(f"seg{slab}"),"accepted_slab_id":sid(f"slab{slab}"),"support":support(a,b),"owner_id":owner,"ofe_id":ofe,"tile_id":tile,"occupancy_id":occ,"layer_id":"layer-1","source_id":resource,"amount_basis":basis,"request_bits":bits(use+1),"authorization_bits":bits(use+.5),"final_use_bits":bits(use)}
def transition(name,slab,owner,resource,basis,begin,end,links,fluxes):
 a,b=(0,900) if slab==0 else (900,1800);raw,digest=candidate(owner,end)
 return {"transition_id":sid(name),"parent_transaction_id":sid("parent"),"segment_id":sid(f"seg{slab}"),"accepted_slab_id":sid(f"slab{slab}"),"support":support(a,b),"owner_id":owner,"ofe_id":"ofe-1","layer_id":"layer-1","source_id":resource,"amount_basis":basis,"beginning_bits":bits(begin),"ending_bits":bits(end),"debit_receipt_ids":sorted(links),"other_flux_receipt_ids":sorted(fluxes),"owner_candidate_bytes_base64":raw,"owner_candidate_sha256":digest}
def fixture():
 d=[debit("w0a",0,"hydrology","ofe-1","tile-a","occ-a","soil_water","kg_m2",1.5),debit("w0b",0,"hydrology","ofe-1","tile-b","occ-b","soil_water","kg_m2",1.0),debit("w1a",1,"hydrology","ofe-1","tile-a","occ-a","soil_water","kg_m2",.75),debit("nha",0,"bgc","ofe-1","tile-a","occ-a","nh4","kg_n_m2",.1),debit("noa",0,"bgc","ofe-1","tile-a","occ-a","no3","kg_n_m2",.2)]
 flux=[sid("runon"),sid("mineralization")]
 t=[transition("wt0",0,"hydrology","soil_water","kg_m2",10,8,[d[0]["receipt_id"],d[1]["receipt_id"]],[flux[0]]),transition("nt0",0,"bgc","nh4","kg_n_m2",1,.8,[d[3]["receipt_id"]],[flux[1]]),transition("no0",0,"bgc","no3","kg_n_m2",2,1.7,[d[4]["receipt_id"]],[flux[1]]),transition("wt1",1,"hydrology","soil_water","kg_m2",8,6.5,[d[2]["receipt_id"]],[flux[0]])]
 terminal=[]
 last={x:(b"{}",sha(b"{}")) for x in OWNERS}
 for x in t:last[x["owner_id"]]=(base64.b64decode(x["owner_candidate_bytes_base64"]),x["owner_candidate_sha256"])
 for owner in OWNERS:
  raw,digest=last[owner];terminal.append({"owner_id":owner,"state_bytes_base64":base64.b64encode(raw).decode(),"state_sha256":digest})
 return {"schema":"OPENWEPP_C3_WOODY_V11_RESTART_V3","parent_transaction_id":sid("parent"),"parent_support":support(0,1800),"admitted_other_flux_receipt_ids":sorted(flux),"debit_receipts":sorted(d,key=lambda x:x["receipt_id"]),"shared_owner_transitions":sorted(t,key=lambda x:(int(x["support"]["start_ns"]),x["owner_id"],x["source_id"])),"terminal_complete_owners":terminal}
def validate(c):
 if set(c)!={"schema","parent_transaction_id","parent_support","admitted_other_flux_receipt_ids","debit_receipts","shared_owner_transitions","terminal_complete_owners"} or c["schema"]!="OPENWEPP_C3_WOODY_V11_RESTART_V3":raise ValueError("V3-SCHEMA")
 if c["admitted_other_flux_receipt_ids"]!=sorted(set(c["admitted_other_flux_receipt_ids"])):raise ValueError("V3-ORDER")
 debits=c["debit_receipts"]
 if debits!=sorted(debits,key=lambda x:x["receipt_id"]) or len({x["receipt_id"] for x in debits})!=len(debits):raise ValueError("V3-DEBIT")
 byid={x["receipt_id"]:x for x in debits};linked=[];prior={};last={};debit_keys=set()
 for d in debits:
  if set(d)!={"receipt_id","parent_transaction_id","segment_id","accepted_slab_id","support","owner_id","ofe_id","tile_id","occupancy_id","layer_id","source_id","amount_basis","request_bits","authorization_bits","final_use_bits"}:raise ValueError("V3-DEBIT")
  key=(d["parent_transaction_id"],d["segment_id"],d["accepted_slab_id"],d["owner_id"],d["ofe_id"],d["tile_id"],d["occupancy_id"],d["layer_id"],d["source_id"],d["amount_basis"])
  if key in debit_keys or d["parent_transaction_id"]!=c["parent_transaction_id"]:raise ValueError("V3-DEBIT")
  request,authorization,use=map(number,(d["request_bits"],d["authorization_bits"],d["final_use_bits"]))
  if not 0<=use<=authorization<=request:raise ValueError("V3-DEBIT")
  debit_keys.add(key)
 transition_keys=set()
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
  if t["other_flux_receipt_ids"]!=sorted(set(t["other_flux_receipt_ids"])) or any(x not in c["admitted_other_flux_receipt_ids"] for x in t["other_flux_receipt_ids"]):raise ValueError("V3-FLUX")
  raw=base64.b64decode(t["owner_candidate_bytes_base64"],validate=True)
  if sha(raw)!=t["owner_candidate_sha256"]:raise ValueError("V3-CANDIDATE")
  body=json.loads(raw)
  if body!={"ending_bits":t["ending_bits"],"owner":t["owner_id"]}:raise ValueError("V3-CANDIDATE")
  key=(t["owner_id"],t["ofe_id"],t["layer_id"],t["source_id"],t["amount_basis"])
  if key in prior and prior[key]!=t["beginning_bits"]:raise ValueError("V3-CHAIN")
  for rid in t["debit_receipt_ids"]:
   if rid not in byid:raise ValueError("V3-LINK")
   d=byid[rid]
   if (d["parent_transaction_id"],d["segment_id"],d["accepted_slab_id"],d["support"],d["owner_id"],d["ofe_id"],d["layer_id"],d["source_id"],d["amount_basis"])!=(t["parent_transaction_id"],t["segment_id"],t["accepted_slab_id"],t["support"],t["owner_id"],t["ofe_id"],t["layer_id"],t["source_id"],t["amount_basis"]):raise ValueError("V3-ALIAS")
   linked.append(rid)
  prior[key]=t["ending_bits"];last[t["owner_id"]]=t
 if sorted(linked)!=sorted(byid) or len(linked)!=len(set(linked)):raise ValueError("V3-LINK")
 if [x["owner_id"] for x in c["terminal_complete_owners"]]!=OWNERS:raise ValueError("V3-TERMINAL")
 for o in c["terminal_complete_owners"]:
  raw=base64.b64decode(o["state_bytes_base64"],validate=True)
  if sha(raw)!=o["state_sha256"]:raise ValueError("V3-TERMINAL")
 for owner,t in last.items():
  o=next(x for x in c["terminal_complete_owners"] if x["owner_id"]==owner)
  if o["state_sha256"]!=t["owner_candidate_sha256"] or o["state_bytes_base64"]!=t["owner_candidate_bytes_base64"]:raise ValueError("V3-TERMINAL")
 return {"water_ending_bits":prior[("hydrology","ofe-1","layer-1","soil_water","kg_m2")],"debit_count":len(debits),"transition_count":len(expected)}
def main():
 base=fixture();accepted=validate(base);poisons=[]
 for p in json.loads((ROOT/"restart-v3-poisons.json").read_text()):
  x=copy.deepcopy(base);env={"x":x,"sid":sid,"bits":bits};exec(p["mutation"],env,env)
  try:validate(x);raise SystemExit("accepted poison "+p["id"])
  except ValueError as e:
   if str(e)!=p["error"]:raise
   poisons.append({"id":p["id"],"error":str(e)})
 print(json.dumps({"schema":"OPENWEPP_C3_WOODY_V11_RESTART_V3_RESULTS_V1","accepted":accepted,"poisons":poisons},sort_keys=True,separators=(",",":")))
if __name__=="__main__":main()
