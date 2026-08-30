"""Independent executable oracle; consumes frozen JSON only, never Rust."""
from __future__ import annotations
import hashlib,json,struct,sys,unicodedata
from fractions import Fraction
from pathlib import Path
ROOT=Path(__file__).resolve().parent; UMAX=(1<<128)-1; SAME="a"*64
CLASSES={n:i for i,n in enumerate(["HardBoundary","EventBoundary","OutputBoundary","RestartBoundary","AdaptiveUpperBound"])}
EVENTS={n:i for i,n in enumerate(["OwnershipTransfer","BoundaryModeTransition","RegimeTransition","ScheduledBoundary","DiagnosticMarker"])}
def fail(x): raise ValueError(x)
def tick(x):
    if not isinstance(x,str) or not x or (x!="0" and x[0]=="0") or not x.isascii() or not x.isdigit(): fail("InvalidWireIdentity")
    n=int(x)
    if n>UMAX: fail("InvalidWireIdentity")
    return n
def support(p):
    a,b=map(tick,p)
    if a>=b: fail("InvalidParentSupport")
    return a,b
def be(n,w):
    if n<0 or n>=1<<(8*w): fail("ArithmeticOverflow")
    return n.to_bytes(w,"big")
def value(f):
    t,v=f["type"],f["value"]
    if t=="u32": return be(int(v),4)
    if t=="u128": return be(tick(v),16)
    if t=="sha256":
        r=bytes.fromhex(v)
        if len(r)!=32: fail("InvalidWireIdentity")
        return r
    if t=="utf8": return unicodedata.normalize("NFC",v).encode()
    if t=="bytes": return bytes.fromhex(v)
    if t=="optional-none": return b"\0"
    fail("InvalidWireIdentity")
def identity(c):
    if c.get("enforce_domain",True):
        model=json.loads((ROOT/"model-definition.json").read_text())
        declared=model["identity_domain_fields"].get(c["domain"])
        if declared is None: fail("InvalidIdentityDomain")
        actual=[f'{f["tag"]}:{f["type"]}' for f in c["fields"]]
        if actual!=declared: fail("IdentityFieldSchemaMismatch")
        if c.get("version",1)!=1: fail("IdentityVersionMismatch")
    d=c["domain"].encode(); pre=b"OPENWEPP\0"+be(c.get("version",1),2)+be(len(d),2)+d
    for f in c["fields"]:
        tag=f["tag"].encode(); v=value(f); pre+=be(len(tag),2)+tag+be(len(v),4)+v
    return {"status":"accepted","preimage_hex":pre.hex(),"sha256":hashlib.sha256(pre).hexdigest()}
def quantize(c):
    raw=int(c["seconds_bits"],16); sign=raw>>63; exp=(raw>>52)&0x7ff; frac=raw&((1<<52)-1)
    if exp==0x7ff or (sign and (exp or frac)): fail("InvalidEventProposal")
    m,e=(frac,-1074) if exp==0 else ((1<<52)|frac,exp-1075)
    q=Fraction(m*1_000_000_000,1)*(Fraction(1<<e,1) if e>=0 else Fraction(1,1<<-e)); lo,rem=divmod(q.numerator,q.denominator)
    n=lo+int(rem*2>q.denominator or (rem*2==q.denominator and lo&1))
    if n>UMAX: fail("EventProposalOverflow")
    a,b=support(c["parent"])
    if a+n>UMAX: fail("EventTickOverflow")
    if a+n>b: fail("EventPastParentEnd")
    return {"status":"accepted","tick":str(a+n)}
def constraints(c):
    _,b=support(c["parent"]); cur=tick(c["cursor"]); xs=c["constraints"]
    if not xs: fail("NoStepConstraint")
    for x in xs:
        end=tick(x["end"])
        if end<cur: fail("ConstraintBehindAcceptedTime")
        if end>b: fail("ConstraintPastParentEnd")
        if end==cur and x["class"]!="EventBoundary": fail("ZeroStepWithoutEventTransition")
    end=min(map(lambda x:tick(x["end"]),xs)); tied=[x for x in xs if tick(x["end"])==end]
    if len({(x["parent_id"],x["cursor"],x["calendar_receipt"],x["forcing_receipt"]) for x in tied})!=1: fail("ConflictingEqualTimeConstraints")
    if len({x["compatibility_group"] for x in tied if x["class"]!="AdaptiveUpperBound"})>1: fail("ConflictingEqualTimeConstraints")
    ordered=sorted(tied,key=lambda x:(CLASSES[x["class"]],x["owner"].encode(),x["digest"]))
    return {"status":"accepted","end":str(end),"ordered":[f'{x["class"]}:{x["owner"]}:{x["digest"]}' for x in ordered]}
def events(c):
    xs=sorted(c["events"],key=lambda e:(EVENTS[e["class"]],e["owner"].encode(),e["digest"])); state=c["begin_owner_sha256"]; regime=c.get("begin_regime_sha256"); participants=c.get("begin_participant_sha256"); seen=set(c.get("accepted_event_ids",[])); semantic=set(); receipts=[]
    if len(xs)>c.get("same_tick_event_budget",256): fail("EventBudgetExhausted")
    for e in xs:
        if e["event_id"] in seen: fail("EventReplay")
        if not e["ledger_closed"]: fail("LedgerNotClosed")
        if e["begin_owner_sha256"]!=state: fail("EventCustodyConflict")
        if "mutation_set" in e:
            ledger=e.get("ledger",[])
            if any(x["debit"]!=x["credit"] for x in ledger): fail("LedgerNotClosed")
            physical=(e["end_owner_sha256"]!=state or e["regime_sha256"]!=regime or e["participant_sha256"]!=participants)
            custody=(e["class"]=="OwnershipTransfer" and e["mutation_set"]==[] and len(ledger)>0 and all(x["debit"]!="0" and x["credit"]!="0" and x["lineage"]!="0" for x in ledger))
            if not physical and not custody: fail("EventNoProgressCycle")
        key=(e["tick"],e["end_owner_sha256"],e["regime_sha256"],e["participant_sha256"],tuple(e["pending_event_ids"]))
        if key in semantic: fail("EventNoProgressCycle")
        semantic.add(key); seen.add(e["event_id"]); state=e["end_owner_sha256"]; regime=e["regime_sha256"]; participants=e["participant_sha256"]; receipts.append(f'{e["class"]}:{e["event_id"]}:{state}')
    return {"status":"accepted","event_ordinal":c["event_ordinal"]+len(xs),"end_owner_sha256":state,"receipts":receipts}
def retry(c):
    if c["controller_policy"]!=c["restart_controller_policy"]: fail("ControllerPolicyMismatch")
    accepted=c["accepted_state_sha256"]; ords=[]
    for a in c["attempts"]:
        ords.append(a["ordinal"])
        if a["begin_state_sha256"]!=accepted: fail("BeginningOwnerSetMismatch")
        if a["owner_duration_bits"]!=c["duration_bits"]: fail("DurationBitsMismatch")
        if a["outcome"]=="accept": return {"status":"accepted","attempt_ordinals":ords,"accepted_cursor":a["end"],"accepted_state_sha256":a["end_state_sha256"]}
        if a["end_state_sha256"]!=accepted: fail("RejectedAttemptMutatedState")
    fail("MinimumStepExhaustion")
def candidate(c):
    if sorted(c["complete_owners"])!=c["complete_owners"] or len(set(c["complete_owners"]))!=len(c["complete_owners"]): fail("InvalidCompleteOwnerSet")
    if any(x not in c["complete_owners"] for x in c["participants"]): fail("ParticipantSetMismatch")
    if c["begin_owner_sha256"]!=c["accepted_owner_sha256"]: fail("BeginningOwnerSetMismatch")
    if any(c["begin_bytes"][x]!=c["end_bytes"][x] for x in c["complete_owners"] if x not in c["participants"]): fail("InactiveOwnerMutated")
    if c["clock_writer"]!="CoupledClock": fail("UnauthorizedClockAdvance")
    return {"status":"accepted","end_owner_sha256":c["end_owner_sha256"]}
def restart(c):
    cp=c["checkpoint"]; expected=c["expected_identity"]
    for key in ["run_id","calendar_receipt","forcing_receipt","model_definition","constraint_policy","controller_policy"]:
        if cp[key]!=expected[key]: fail("RestartIdentityMismatch")
    if len(cp["complete_owner_state"])!=cp["owner_count"]: fail("OwnerCardinalityMismatch")
    if len({x["owner"] for x in cp["complete_owner_state"]})!=cp["owner_count"]: fail("OwnerCardinalityMismatch")
    if cp["accepted_until_ns"]==cp["event_tick_ns"] and cp["event_applied"] != (cp["event_receipt_id"] in cp["accepted_event_receipts"]): fail("EventReplayStateMismatch")
    continuation={k:cp[k] for k in ["accepted_event_receipts","scheduled_once_receipts","reduction_maximum","publication_outbox","complete_owner_state"]}
    if "expected_continuation" in c and continuation!=c["expected_continuation"]: fail("RestartContinuationMismatch")
    return {"status":"accepted","continuation_sha256":hashlib.sha256(json.dumps(cp,sort_keys=True,separators=(",",":")).encode()).hexdigest(),"scheduled_once_receipts":cp["scheduled_once_receipts"],"reduction_maximum":cp["reduction_maximum"],"publication_outbox":cp["publication_outbox"]}
def restart_equivalence(c):
    keys=["ending_owner_set_sha256","accepted_slab_receipts","accepted_event_receipts","scheduled_once_receipts","reduction_state","publication_outbox"]
    if any(c["uninterrupted"][k]!=c["restarted"][k] for k in keys): fail("RestartContinuationMismatch")
    return {"status":"accepted","equivalence_sha256":hashlib.sha256(json.dumps({k:c["restarted"][k] for k in keys},sort_keys=True,separators=(",",":")).encode()).hexdigest()}
def joins(c):
    if len(c["owner_candidates"])!=c["complete_owner_count"]: fail("OwnerCardinalityMismatch")
    if len({x["owner"] for x in c["owner_candidates"]})!=c["complete_owner_count"]: fail("OwnerCardinalityMismatch")
    if any(x["begin_digest"]!=c["begin_owner_set_sha256"] for x in c["owner_candidates"]): fail("BeginningOwnerSetMismatch")
    if any(not x["ledger_closed"] for x in c["owner_candidates"]): fail("LedgerNotClosed")
    if c["aggregate_ledger_residual"]!=0: fail("LedgerNotClosed")
    return {"status":"accepted","ending_owner_set_sha256":c["ending_owner_set_sha256"],"ledger_digest":c["ledger_digest"]}
def outbox(c):
    state=c["state"]; action=c["action"]
    transitions={("Buffered","parent_commit"):"CommittedUndelivered",("Buffered","parent_rollback"):"Removed",("CommittedUndelivered","deliver"):"DeliveredUnacknowledged",("CommittedUndelivered","crash"):"CommittedUndelivered",("CommittedUndelivered","restart"):"CommittedUndelivered",("DeliveredUnacknowledged","crash"):"DeliveredUnacknowledged",("DeliveredUnacknowledged","redeliver"):"DeliveredUnacknowledged",("DeliveredUnacknowledged","ack"):"Acknowledged",("Acknowledged","crash"):"Acknowledged",("Acknowledged","restart"):"Acknowledged"}
    if (state,action) not in transitions: fail("InvalidOutboxTransition")
    end=transitions[state,action]
    return {"status":"accepted","state":end,"receipt_id":c["receipt_id"],"delivery_count":c["delivery_count"]+(1 if action in ["deliver","redeliver"] else 0)}
def reduction(c):
    accepted=[x["value"] for x in c["operands"] if x["accepted"] and x["phase"]=="accepted_slab"]
    if c["claimed_maximum"]!=max(accepted): fail("ReductionAliasMismatch")
    if c["published_before_commit"]: fail("PublicationBeforeParentCommit")
    return {"status":"accepted","maximum":max(accepted),"accepted_operand_receipts":[x["receipt_id"] for x in c["operands"] if x["accepted"] and x["phase"]=="accepted_slab"]}
def scheduled_output(c):
    ids=[x["scheduled_receipt_id"] for x in c["records"]]
    if len(ids)!=len(set(ids)): fail("DuplicateScheduledOutput")
    return {"status":"accepted","publication_order":[x["output_receipt_id"] for x in c["records"]]}
def authority_tuple(c):
    h,t,l,r=c["hydrology"],c["time"],c.get("lane_d","WholeDayNonpersistent"),c.get("legacy_r4l_mutation",True)
    valid=(h=="LegacyWb14Wb18Wb19" and t=="LegacyFixedSchedule") or (h=="RichardsCoupledV1" and t=="CoupledAdaptiveSupportV1" and l=="Persistent" and not r)
    if not valid: fail("UnsupportedAuthorityTuple")
    return {"status":"accepted","tuple":f"{h}:{t}:{l}:{str(r).lower()}"}
def evaluate(c):
    op=c["op"]
    if op=="tick": tick(c["value"]); return {"status":"accepted"}
    if op=="partition":
        a,b=support(c["parent"]); cursor=a
        for p in c["segments"]:
            x,y=support(p)
            if x!=cursor or y>b: fail("InvalidSegmentPartition")
            cursor=y
        if cursor!=b: fail("InvalidSegmentPartition")
        return {"status":"accepted","cursor":str(cursor)}
    if op=="participants":
        owners=c["owners"]
        if owners!=sorted(set(owners)) or any(x not in owners for s in c["segments"] for x in s): fail("ParticipantSetMismatch")
        return {"status":"accepted","terminal":owners}
    if op=="identity": return identity(c)
    if op=="duration":
        a,b=support(c["support"]); return {"status":"accepted","bits_hex":struct.pack(">d",float(b-a)/1e9).hex()}
    if op=="quantize": return quantize(c)
    if op=="constraints": return constraints(c)
    if op=="events": return events(c)
    if op=="retry": return retry(c)
    if op=="candidate": return candidate(c)
    if op=="scheduled_once":
        if c["receipt_id"] in c["accepted_receipts"]: fail("ScheduledOnceReplay")
        return {"status":"accepted","accepted_receipts":sorted(c["accepted_receipts"]+[c["receipt_id"]])}
    if op=="publication":
        if not c["parent_committed"]: fail("PublicationBeforeParentCommit")
        vals=[x["value"] for x in c["samples"] if x["accepted"]]
        if not vals: fail("NoAcceptedReductionOperand")
        return {"status":"accepted","maximum":max(vals),"publication_order":[x["receipt_id"] for x in c["records"]]}
    if op=="transaction_successor":
        n=tick(c["sequence"])
        if n==UMAX: fail("TransactionSequenceOverflow")
        return {"status":"accepted","successor":str(n+1)}
    if op=="restart": return restart(c)
    if op=="restart_equivalence": return restart_equivalence(c)
    if op=="joins": return joins(c)
    if op=="outbox": return outbox(c)
    if op=="reduction": return reduction(c)
    if op=="scheduled_output": return scheduled_output(c)
    if op=="legacy_hash":
        for f in c["files"]:
            if hashlib.sha256((ROOT/f["path"]).read_bytes()).hexdigest()!=f["sha256"]: fail("DirectV10WireChanged")
        return {"status":"accepted","protected_files":len(c["files"])}
    if op=="authority_tuple": return authority_tuple(c)
    fail("UnknownVectorOperation")
def main():
    vs=json.loads((ROOT/"coupled-time-vectors.json").read_text()); results=[]
    for c in vs["cases"]:
        try: actual=evaluate(c)
        except ValueError as e: actual={"status":"rejected","error":str(e),"before_sha256":SAME,"after_sha256":SAME}
        if "--emit" not in sys.argv and actual!=c["expected"]: raise AssertionError({"id":c["id"],"expected":c["expected"],"actual":actual})
        results.append({"id":c["id"],**actual})
    print(json.dumps({"schema":"OPENWEPP_COUPLED_TIME_REFERENCE_RESULTS_V1","results":results},sort_keys=True,separators=(",",":")))
if __name__=="__main__": main()
