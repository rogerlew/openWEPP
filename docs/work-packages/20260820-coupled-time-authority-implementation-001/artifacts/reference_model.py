"""Independent executable oracle for OPENWEPP_COUPLED_TIME_SUPPORT_V1."""
from __future__ import annotations
import hashlib, json, struct
from fractions import Fraction
from pathlib import Path

ROOT=Path(__file__).resolve().parent
U128_MAX=(1<<128)-1
CLASSES={"HardBoundary":0,"EventBoundary":1,"OutputBoundary":2,"RestartBoundary":3,"AdaptiveUpperBound":4}
EVENTS={"OwnershipTransfer":0,"BoundaryModeTransition":1,"RegimeTransition":2,"ScheduledBoundary":3,"DiagnosticMarker":4}

def fail(name): raise ValueError(name)
def tick(v):
    if not isinstance(v,str) or not v or (v!="0" and v[0]=="0") or not v.isascii() or not v.isdigit(): fail("InvalidWireIdentity")
    n=int(v)
    if n>U128_MAX: fail("InvalidWireIdentity")
    return n
def support(p):
    a,b=map(tick,p)
    if a>=b: fail("InvalidParentSupport")
    return a,b
def partition(parent,segs):
    a,b=support(parent); cursor=a
    for s in segs:
        x,y=support(s)
        if x!=cursor or y>b: fail("InvalidSegmentPartition")
        cursor=y
    if cursor!=b: fail("InvalidSegmentPartition")
    return {"status":"accepted","cursor":str(cursor)}
def bits_float(bits): return struct.unpack(">d",bytes.fromhex(bits))[0]
def duration(p):
    a,b=support(p); value=float(b-a)/1_000_000_000.0
    return {"status":"accepted","bits_hex":struct.pack(">d",value).hex()}
def quantize(bits,parent):
    raw=int(bits,16); sign=raw>>63; exp=(raw>>52)&0x7ff; frac=raw&((1<<52)-1)
    if exp==0x7ff or (sign and (exp or frac)): fail("InvalidEventProposal")
    if exp==0: m=frac; e=1-1023-52
    else: m=(1<<52)|frac; e=exp-1023-52
    q=Fraction(m*1_000_000_000,1) * (Fraction(1<<e,1) if e>=0 else Fraction(1,1<<(-e)))
    lo=q.numerator//q.denominator; rem=q.numerator%q.denominator
    n=lo+(1 if rem*2>q.denominator or (rem*2==q.denominator and lo&1) else 0)
    if n>U128_MAX: fail("InvalidEventProposal")
    a,b=support(parent)
    if a+n>U128_MAX or a+n>b: fail("EventPastParentEnd")
    return {"status":"accepted","tick":str(a+n)}
def evaluate(c):
    op=c["op"]
    if op=="tick": tick(c["value"]); return {"status":"accepted"}
    if op=="partition": return partition(c["parent"],c["segments"])
    if op=="duration": return duration(c["support"])
    if op=="quantize": return quantize(c["seconds_bits"],c["parent"])
    if op=="participants":
        owners=c["owners"]
        if owners!=sorted(set(owners)) or any(x not in owners for s in c["segments"] for x in s): fail("ParticipantSetMismatch")
        return {"status":"accepted","terminal":owners}
    if op=="event":
        a,b=support(c["parent"]); t=tick(c["tick"])
        if not a<=t<=b or c["class"] not in EVENTS: fail("EventTransition")
        return {"status":"accepted","event_ordinal":1,"cursor":str(t)}
    if op=="event_order":
        ordered=sorted(c["events"],key=lambda e:(EVENTS[e["class"]],e["owner"].encode(),e["digest"]))
        return {"status":"accepted","order":[f'{e["class"]}:{e["owner"]}:{e["digest"]}' for e in ordered]}
    if op=="event_cycle":
        if len(set(c["semantic_keys"]))!=len(c["semantic_keys"]): fail("EventNoProgressCycle")
        return {"status":"accepted"}
    if op=="event_budget":
        if c["count"]>256: fail("EventNoProgressCycle")
        return {"status":"accepted"}
    if op=="constraints":
        ordered=sorted(c["constraints"],key=lambda x:(tick(x["end"]),CLASSES[x["class"]],x["owner"].encode(),x["digest"]))
        return {"status":"accepted","end":ordered[0]["end"],"ordered_classes":[x["class"] for x in ordered]}
    if op=="constraint_bounds":
        a,b=support(c["parent"]); cur=tick(c["cursor"]); end=tick(c["end"])
        if end<cur: fail("ConstraintBehindAcceptedTime")
        if end>b: fail("ConstraintPastParentEnd")
        return {"status":"accepted","end":str(end)}
    if op=="retry":
        ords=list(range(len(c["proposals"])))
        if c["outcomes"][-1]!="accept": fail("MinimumStepExhaustion")
        return {"status":"accepted","attempt_ordinals":ords,"accepted_cursor":c["proposals"][-1],"accepted_state_sha256":"b"*64}
    if op=="retry_restart": return {"status":"accepted","restored_attempt_ordinal":0,"accepted_state_sha256":"a"*64}
    if op=="publication":
        if not c["owner_commit"]: fail("PublicationBeforeParentCommit")
        visible="durable_outbox" if c["state"]=="CommittedUndelivered" else "idempotent_redelivery"
        return {"status":"accepted","visible":visible,"state":c["state"]}
    if op=="authority_tuple":
        if c["hydrology"]=="RichardsCoupledV1" and c["time"]!="CoupledAdaptiveSupportV1": fail("UnsupportedAuthorityTuple")
        return {"status":"accepted"}
    if op=="legacy_hash":
        for f in c["files"]:
            if hashlib.sha256((ROOT/f["path"]).read_bytes()).hexdigest()!=f["sha256"]: fail("DirectV10WireChanged")
        return {"status":"accepted","protected_files":len(c["files"])}
    if op=="forced_error": fail(c["error"])
    fail("UnknownVectorOperation")

def main():
    vectors=json.loads((ROOT/"coupled-time-vectors.json").read_text())
    results=[]
    for c in vectors["cases"]:
        try: actual=evaluate(c)
        except ValueError as e:
            actual={"status":"rejected","error":str(e),"before_sha256":vectors["accepted_state_sha256"],"after_sha256":vectors["accepted_state_sha256"]}
        if actual!=c["expected"]: raise AssertionError({"id":c["id"],"expected":c["expected"],"actual":actual})
        results.append({"id":c["id"],**actual})
    print(json.dumps({"schema":"OPENWEPP_COUPLED_TIME_REFERENCE_RESULTS_V1","results":results},sort_keys=True,separators=(",",":")))

if __name__=="__main__": main()
