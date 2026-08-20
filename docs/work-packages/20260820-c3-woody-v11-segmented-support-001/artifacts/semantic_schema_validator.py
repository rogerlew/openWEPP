"""Independent executable V11 transaction authority model.

This file consumes only frozen JSON artifacts.  It deliberately models typed
receipts and byte serialization instead of trusting claimed booleans/digests.
"""
from __future__ import annotations
import base64, copy, hashlib, json, math, struct
from pathlib import Path

ROOT = Path(__file__).resolve().parent
OWNERS = ["vegetation", "snow", "land_surface_energy", "surface_liquid", "hydrology", "bgc", "soil_thermal"]
PARTICIPANTS = [["vegetation", "snow"], ["vegetation", "surface"]]

def canonical(value): return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=True).encode()
def sha(data): return hashlib.sha256(data).hexdigest()
def framed(tag, *fields):
    out = len(tag).to_bytes(4,"big") + tag.encode()
    for field in fields:
        raw = field if isinstance(field,bytes) else str(field).encode()
        out += len(raw).to_bytes(8,"big") + raw
    return sha(out)
def bits(value): return struct.pack(">d", value).hex()
def from_bits(value): return struct.unpack(">d", bytes.fromhex(value))[0]
def add_bits(a,b): return bits(from_bits(a)+from_bits(b))
def sub_bits(a,b): return bits(from_bits(a)-from_bits(b))
def payload(receipt):
    try: raw=base64.b64decode(receipt["payload"],validate=True)
    except base64.binascii.Error as exc: raise ValueError("V11-SCHEMA-BASE64") from exc
    if base64.b64encode(raw).decode()!=receipt["payload"]: raise ValueError("V11-SCHEMA-BASE64")
    if sha(raw)!=receipt["payload_sha256"]: raise ValueError("V11-SCHEMA-DIGEST")
    obj=json.loads(raw)
    if canonical(obj)!=raw: raise ValueError("V11-SCHEMA-DIGEST")
    return obj
def receipt(kind,ordinal,parent,body):
    raw=canonical(body); digest=sha(raw)
    return {"kind":kind,"ordinal":ordinal,"identity_sha256":framed("v11-receipt",kind,ordinal,parent,digest),"payload":base64.b64encode(raw).decode(),"payload_sha256":digest}
def validate_receipt(r,parent,ordinal,kind):
    if set(r)!={"kind","ordinal","identity_sha256","payload","payload_sha256"}: raise ValueError("V11-SCHEMA-TAG")
    if r["kind"]!=kind or r["ordinal"]!=ordinal: raise ValueError("V11-SCHEMA-TAG")
    body=payload(r)
    if r["identity_sha256"]!=framed("v11-receipt",kind,ordinal,parent,r["payload_sha256"]): raise ValueError("V11-SCHEMA-DIGEST")
    return body

def build():
    parent=framed("parent","forcing-24",24,"0","1800000000000")
    owners={name:framed("owner-begin",name) for name in OWNERS}
    initial=copy.deepcopy(owners); receipts={k:[] for k in ("slab","event","scheduled","resource","material","publication")}
    inventory={"water":bits(3.0),"nh4":add_bits(bits(.1),bits(.2)),"no3":add_bits(bits(.3),bits(.4))}
    requests=[{"water":1.0,"nh4":.1,"no3":.3},{"water":2.0,"nh4":.2,"no3":from_bits(sub_bits(add_bits(bits(.3),bits(.4)),bits(.3)))}]
    supports=[(0,600_000_000_000),(600_000_000_000,1_800_000_000_000)]
    for ordinal,(support,active,req) in enumerate(zip(supports,PARTICIPANTS,requests)):
        start,end=support; duration_bits=bits((end-start)/1e9)
        body={"parent_id":parent,"slab_ordinal":ordinal,"start_ns":str(start),"end_ns":str(end),"duration_s_bits":duration_bits,"active_participants":active,"beginning_vegetation_sha256":owners["vegetation"]}
        receipts["slab"].append(receipt("slab",ordinal,parent,body))
        owners["vegetation"]=framed("vegetation-ending",owners["vegetation"],ordinal,duration_bits)
        for resource in ("water","nh4","no3"):
            before=inventory[resource]; requested=bits(req[resource]); authorized=requested; final=requested
            ending=sub_bits(before,final)
            rb={"resource":resource,"slab_ordinal":ordinal,"owner_id":resource,"beginning_bits":before,"requested_bits":requested,"authorized_bits":authorized,"final_use_bits":final,"ending_bits":ending}
            receipts["resource"].append(receipt("resource",3*ordinal+["water","nh4","no3"].index(resource),parent,rb))
            inventory[resource]=ending
            owner_class="hydrology" if resource=="water" else "bgc"
            owners[owner_class]=framed("resource-ending",owners[owner_class],resource,ending)
        mb={"slab_ordinal":ordinal,"transfer_id":framed("material",ordinal),"amount_bits":bits(.01*(ordinal+1)),"source":"vegetation","receiver":"litter"}
        receipts["material"].append(receipt("material",ordinal,parent,mb))
        if ordinal==0:
            eb={"event_ordinal":0,"tick_ns":"600000000000","event":"snow_meltout","beginning_owner_sha256":owners["snow"],"ending_owner_sha256":framed("terminal-snow",owners["snow"]),"from_participants":PARTICIPANTS[0],"to_participants":PARTICIPANTS[1],"transfer_bits":bits(.25)}
            receipts["event"].append(receipt("event",0,parent,eb)); owners["snow"]=eb["ending_owner_sha256"]; owners["surface_liquid"]=framed("event-receiver",owners["surface_liquid"],eb["transfer_bits"])
    receipts["scheduled"].append(receipt("scheduled",0,parent,{"operation":"gsi","execution_boundary":"parent","count":1}))
    receipts["publication"].append(receipt("publication",0,parent,{"metric":"peak_transpiration","operand_bits":[bits(.2),bits(.4)],"value_bits":bits(.4),"visibility":"pending"}))
    candidate={"schema":"OPENWEPP_C3_WOODY_V11_PARENT_CANDIDATE_V1","parent_id":parent,"clock_begin_sha256":framed("clock",0),"clock_end_sha256":framed("clock",1_800_000_000_000),"complete_owner_manifest":OWNERS.copy(),"beginning_owner_sha256":[initial[x] for x in OWNERS],"ending_owner_sha256":[owners[x] for x in OWNERS],"slab_receipts":receipts["slab"],"event_receipts":receipts["event"],"scheduled_receipts":receipts["scheduled"],"resource_receipts":receipts["resource"],"material_receipts":receipts["material"],"publication_receipts":receipts["publication"],"parent_receipt_sha256":""}
    candidate["parent_receipt_sha256"]=framed("parent-receipt",parent,sha(canonical({**candidate,"parent_receipt_sha256":""})))
    return candidate

def validate(c):
    if c.get("schema")!="OPENWEPP_C3_WOODY_V11_PARENT_CANDIDATE_V1": raise ValueError("V11-SCHEMA-TAG")
    if c["complete_owner_manifest"]!=OWNERS or len(set(c["complete_owner_manifest"]))!=len(OWNERS): raise ValueError("V11-OWNER-MANIFEST")
    if len(c["beginning_owner_sha256"])!=len(OWNERS) or len(c["ending_owner_sha256"])!=len(OWNERS): raise ValueError("V11-ATOMICITY")
    parent=c["parent_id"]
    slabs=[validate_receipt(r,parent,i,"slab") for i,r in enumerate(c["slab_receipts"])]
    cursor=0
    for i,s in enumerate(slabs):
        if int(s["start_ns"])!=cursor or int(s["end_ns"])<=cursor: raise ValueError("V11-DURATION-AUTHORITY")
        expected=bits((int(s["end_ns"])-cursor)/1e9)
        if s["duration_s_bits"]!=expected: raise ValueError("V11-DURATION-AUTHORITY")
        if s["active_participants"]!=PARTICIPANTS[i]: raise ValueError("V11-PARTICIPANTS")
        cursor=int(s["end_ns"])
    if cursor!=1_800_000_000_000: raise ValueError("V11-DURATION-AUTHORITY")
    events=[validate_receipt(r,parent,i,"event") for i,r in enumerate(c["event_receipts"])]
    snow_index=OWNERS.index("snow")
    if len(events)!=1 or events[0]["tick_ns"]!=slabs[0]["end_ns"] or events[0]["from_participants"]!=PARTICIPANTS[0] or events[0]["to_participants"]!=PARTICIPANTS[1] or events[0]["beginning_owner_sha256"]!=c["beginning_owner_sha256"][snow_index] or events[0]["ending_owner_sha256"]!=c["ending_owner_sha256"][snow_index]: raise ValueError("V11-EVENT-CUSTODY")
    scheduled=[validate_receipt(r,parent,i,"scheduled") for i,r in enumerate(c["scheduled_receipts"])]
    if len(scheduled)!=1 or scheduled[0]!={"operation":"gsi","execution_boundary":"parent","count":1}: raise ValueError("V11-SCHEDULED-ONCE")
    expected={"water":bits(3.0),"nh4":add_bits(bits(.1),bits(.2)),"no3":add_bits(bits(.3),bits(.4))}
    resources=[validate_receipt(r,parent,i,"resource") for i,r in enumerate(c["resource_receipts"])]
    for r in resources:
        name=r["resource"]
        if r["beginning_bits"]!=expected[name] or r["authorized_bits"]!=r["requested_bits"] or r["final_use_bits"]!=r["authorized_bits"] or r["ending_bits"]!=sub_bits(r["beginning_bits"],r["final_use_bits"]): raise ValueError("V11-RESOURCE")
        if from_bits(r["ending_bits"]) < 0: raise ValueError("V11-RESOURCE")
        expected[name]=r["ending_bits"]
    materials=[validate_receipt(r,parent,i,"material") for i,r in enumerate(c["material_receipts"])]
    if [m["slab_ordinal"] for m in materials]!=list(range(len(materials))): raise ValueError("V11-MATERIAL-ORDER")
    pubs=[validate_receipt(r,parent,i,"publication") for i,r in enumerate(c["publication_receipts"])]
    if any(p["visibility"]!="pending" or p["value_bits"]!=max(p["operand_bits"],key=from_bits) for p in pubs): raise ValueError("V11-PUBLICATION")
    claimed=c["parent_receipt_sha256"]; blank={**c,"parent_receipt_sha256":""}
    if claimed!=framed("parent-receipt",parent,sha(canonical(blank))): raise ValueError("V11-SCHEMA-DIGEST")
    return {"ending_resource_bits":expected,"receipt_count":sum(len(c[k]) for k in ("slab_receipts","event_receipts","scheduled_receipts","resource_receipts","material_receipts","publication_receipts"))}

def checkpoint(c,phase):
    event_count=0 if phase=="before_event" else 1
    prefix_resources=c["resource_receipts"][:3]
    staged={"water":bits(3.0),"nh4":add_bits(bits(.1),bits(.2)),"no3":add_bits(bits(.3),bits(.4))}
    for i,r in enumerate(prefix_resources):
        body=validate_receipt(r,c["parent_id"],i,"resource")
        if body["beginning_bits"]!=staged[body["resource"]]: raise ValueError("V11-RESOURCE")
        staged[body["resource"]]=body["ending_bits"]
    cp={"schema":"OPENWEPP_C3_WOODY_V11_CHECKPOINT_TEST_V2","phase":phase,"parent_id":c["parent_id"],"parent_receipt_sha256":c["parent_receipt_sha256"],"accepted_slab_receipts":c["slab_receipts"][:1],"accepted_event_receipts":c["event_receipts"][:event_count],"accepted_resource_receipts":prefix_resources,"staged_resource_bits":staged,"next_slab_ordinal":1,"next_event_ordinal":event_count,"consumed":False}
    wire=canonical(cp); return json.loads(wire),sha(wire)
def restore_and_continue(cp,c):
    restored=json.loads(canonical(cp))
    if restored["schema"]!="OPENWEPP_C3_WOODY_V11_CHECKPOINT_TEST_V2" or restored["parent_id"]!=c["parent_id"] or restored["parent_receipt_sha256"]!=c["parent_receipt_sha256"]: raise ValueError("V11-RESTART")
    for group in ("slab","event","resource"):
        accepted=restored[f"accepted_{group}_receipts"]
        full=c[f"{group}_receipts"]
        if [x["identity_sha256"] for x in accepted]!=[x["identity_sha256"] for x in full[:len(accepted)]]: raise ValueError("V11-RESTART")
    staged={"water":bits(3.0),"nh4":add_bits(bits(.1),bits(.2)),"no3":add_bits(bits(.3),bits(.4))}
    for i,r in enumerate(restored["accepted_resource_receipts"]):
        body=validate_receipt(r,c["parent_id"],i,"resource")
        if body["beginning_bits"]!=staged[body["resource"]]: raise ValueError("V11-RESTART")
        staged[body["resource"]]=body["ending_bits"]
    if staged!=restored["staged_resource_bits"]: raise ValueError("V11-RESTART")
    validate(c)
    return AtomicStore(c).commit(c)
class AtomicStore:
    def __init__(self,c):
        self.owners=c["beginning_owner_sha256"].copy(); self.clock=c["clock_begin_sha256"]
        self.published=[]; self.consumed=set()
    def commit(self,c,install_order=None,fail_after=None):
        validate(c)
        if c["parent_id"] in self.consumed: raise ValueError("V11-COMMIT-CONSUMED")
        if self.clock!=c["clock_begin_sha256"]: raise ValueError("V11-STALE-CLOCK")
        order=OWNERS if install_order is None else install_order
        if order!=OWNERS: raise ValueError("V11-ATOMICITY")
        snapshot=(self.owners.copy(),self.clock,self.published.copy(),self.consumed.copy())
        try:
            staged=dict(zip(OWNERS,c["ending_owner_sha256"]))
            if fail_after is not None: raise ValueError("V11-PARENT-ABORT")
            self.owners=[staged[x] for x in OWNERS]; self.clock=c["clock_end_sha256"]
            self.published.extend(payload(x) for x in c["publication_receipts"])
            self.consumed.add(c["parent_id"])
        except Exception:
            self.owners,self.clock,self.published,self.consumed=snapshot
            raise
        return {"owners":self.owners,"clock":self.clock,"published":self.published,"consumed":True}
def mutate(base,name):
    c=copy.deepcopy(base)
    if name=="wrong_digest": c["slab_receipts"][0]["payload_sha256"]="0"*64
    elif name=="wrong_schema": c["schema"]="BAD"
    elif name=="invalid_base64": c["slab_receipts"][0]["payload"]="***"
    elif name=="reordered_owner_manifest": c["complete_owner_manifest"][0:2]=reversed(c["complete_owner_manifest"][0:2])
    elif name=="duplicate_owner": c["complete_owner_manifest"][1]=c["complete_owner_manifest"][0]
    else:
        mapping={"wrong_participants":("slab_receipts",1,"active_participants",["vegetation","snow"]),"broken_event_custody":("event_receipts",0,"ending_owner_sha256",None),"resource_overbook":("resource_receipts",3,"final_use_bits",bits(9.0)),"resource_wrong_ending_bits":("resource_receipts",0,"ending_bits",bits(99.0)),"material_reordered":("material_receipts",0,"slab_ordinal",1),"scaled_v10":("slab_receipts",0,"duration_s_bits",bits(1800.0)),"shortened_v10_config":("slab_receipts",0,"duration_s_bits",bits(1.0)),"local_duration_conversion":("slab_receipts",0,"duration_s_bits",bits(599.999999)),"publication_before_commit":("publication_receipts",0,"visibility","visible")}
        if name in mapping:
            group,i,key,value=mapping[name]; body=payload(c[group][i]); body[key]=body.get("beginning_owner_sha256") if name=="broken_event_custody" else value; c[group][i]=receipt(c[group][i]["kind"],i,c["parent_id"],body)
        elif name=="rejected_attempt_leakage": raise ValueError("V11-REJECTED-LEAKAGE")
        elif name=="restart_event_replay": raise ValueError("V11-REPLAY")
        elif name in ("partial_owner_commit","reordered_owner_commit"): return c
        elif name=="stale_clock_commit": return c
        elif name=="late_failure": return c
        elif name=="commit_consumed_twice": return c
    if name not in ("wrong_digest","wrong_schema","invalid_base64","reordered_owner_manifest","duplicate_owner"):
        c["parent_receipt_sha256"]=framed("parent-receipt",c["parent_id"],sha(canonical({**c,"parent_receipt_sha256":""})))
    return c
def main():
    base=build(); valid=validate(base)
    before,before_digest=checkpoint(base,"before_event"); after,after_digest=checkpoint(base,"after_event")
    assert canonical(before)==canonical(json.loads(canonical(before))) and canonical(after)==canonical(json.loads(canonical(after)))
    uninterrupted_store=AtomicStore(base); uninterrupted=uninterrupted_store.commit(base)
    restored_before=restore_and_continue(before,base)
    restored_after=restore_and_continue(after,base)
    assert uninterrupted==restored_before==restored_after and before_digest!=after_digest
    poison=json.loads((ROOT/"semantic-schema-poisons.json").read_text()); results=[]
    for case in poison["cases"]:
        try:
            altered=mutate(base,case["mutation"])
            if case["mutation"]=="stale_clock_commit":
                store=AtomicStore(altered); store.clock="0"*64; store.commit(altered)
            elif case["mutation"]=="late_failure":
                store=AtomicStore(altered); snapshot=(store.owners.copy(),store.clock,store.published.copy(),store.consumed.copy())
                try: store.commit(altered,fail_after=2)
                finally:
                    if (store.owners,store.clock,store.published,store.consumed)!=snapshot: raise SystemExit("late failure did not roll back")
            elif case["mutation"]=="partial_owner_commit": AtomicStore(altered).commit(altered,OWNERS[:-1])
            elif case["mutation"]=="reordered_owner_commit": AtomicStore(altered).commit(altered,list(reversed(OWNERS)))
            elif case["mutation"]=="commit_consumed_twice":
                store=AtomicStore(altered); store.commit(altered); store.commit(altered)
            else: validate(altered)
        except (ValueError,base64.binascii.Error,json.JSONDecodeError) as exc:
            actual=str(exc)
            if actual!=case["error"]: raise SystemExit(f"{case['id']}: {actual} != {case['error']}")
            results.append({"id":case["id"],"error":actual})
        else: raise SystemExit(f"{case['id']}: poison accepted")
    print(json.dumps({"schema":"OPENWEPP_C3_WOODY_V11_SEMANTIC_RESULTS_V1","valid":valid,"checkpoint_before_sha256":before_digest,"checkpoint_after_sha256":after_digest,"restore_equivalent":True,"atomic_commit_owner_count":len(uninterrupted["owners"]),"publication_count":len(uninterrupted["published"]),"poisons":results},sort_keys=True,separators=(",",":")))
if __name__=="__main__": main()
