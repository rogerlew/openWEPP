"""Independent V11 authority oracle; imports no production code."""
from __future__ import annotations
import hashlib,json,math,struct
from fractions import Fraction
from pathlib import Path
ROOT=Path(__file__).resolve().parent
U128_MAX=(1<<128)-1
def b2f(x): return struct.unpack(">d",bytes.fromhex(x))[0]
def f2b(x): return struct.pack(">d",x).hex()
def reject(e,**kw): return {"status":"rejected","error":e,**kw}
def migrate(bits):
 v=b2f(bits)
 if not math.isfinite(v) or v<=0:return reject("VEG-E-121")
 q,r=divmod((Fraction.from_float(v)*1_000_000_000).numerator,(Fraction.from_float(v)*1_000_000_000).denominator)
 n=q+int(2*r>(Fraction.from_float(v)*1_000_000_000).denominator or (2*r==(Fraction.from_float(v)*1_000_000_000).denominator and q&1))
 if n<=0 or n>U128_MAX or f2b(float(n)/1e9)!=bits:return reject("VEG-E-121")
 return {"status":"accepted","nominal_cadence_ns":str(n)}
def chronology(c,parent_end):
 if c.get("attempt_rejected"):return reject("VEG-E-123",state_unchanged=True,publication_visible=False)
 cursor=0
 for a,b in c["supports"]:
  a,b=int(a),int(b)
  if a!=cursor:return reject("VEG-E-123")
  if b<=a:return reject("VEG-E-122")
  cursor=b
 if cursor!=parent_end:return reject("VEG-E-123")
 expected=[f"slab-{i}" for i in range(len(c["supports"]))]
 if c.get("slab_receipts",expected)!=expected or c.get("replayed_slab") is not None or c.get("replayed_event") is not None:return reject("VEG-E-127")
 if c.get("participant_mismatch") or (c.get("participant_sets") and len(c["participant_sets"])!=len(c["supports"])) or c.get("segment_starts_from_parent"):return reject("VEG-E-123")
 if c.get("duration_alias"):return reject("VEG-E-122")
 if c.get("per_segment_commit") or c.get("requested_increments",1)!=1:return reject("VEG-E-126")
 receipts=c.get("scheduled_receipts",[])
 if len(receipts)!=len(set(receipts)):return reject("VEG-E-125")
 events=c.get("events",[]); seen=set()
 for e in events:
  tick=int(e["tick"])
  if tick<0 or tick>parent_end or e["id"] in seen:return reject("VEG-E-127")
  seen.add(e["id"])
  if e.get("integrates_rate"):return reject("VEG-E-122")
 if c.get("zero_remainder_skip") and events and int(events[-1]["tick"])!=parent_end:return reject("VEG-E-123")
 inv=c.get("inventories",{}); debits=c.get("resource_debits",{}); totals={}; endings={}
 for resource in ("water","nh4","no3"):
  total=0.0; available=inv.get(resource,"Infinity")
  staged=float(available) if available!="Infinity" else math.inf
  declared_endings=c.get("resource_endings",{}).get(resource,[])
  for ordinal,encoded in enumerate(debits.get(resource,[])):
   value=float(encoded)
   if not math.isfinite(value) or value<0:return reject("VEG-E-124",resource=resource)
   if value>staged:return reject("VEG-E-124",resource=resource)
   staged=staged-value
   if ordinal<len(declared_endings) and f2b(staged)!=f2b(float(declared_endings[ordinal])):return reject("VEG-E-124",resource=resource)
   total+=value
   if not math.isfinite(total):return reject("VEG-E-124",resource=resource)
  if available!="Infinity" and total>float(available):return reject("VEG-E-124",resource=resource)
  if declared_endings and len(declared_endings)!=len(debits.get(resource,[])):return reject("VEG-E-124",resource=resource)
  if c.get("regrouped_ending_alias") and available!="Infinity" and f2b(float(available)-total)!=f2b(staged):return reject("VEG-E-124",resource=resource)
  totals[resource]=str(total)
  if declared_endings:endings[resource]=str(staged)
 forcing=c.get("forcing",[]); state=Fraction(c.get("beginning_state","0"))
 if forcing:
  if len(forcing)!=len(c["supports"]):return reject("VEG-E-123")
  for i,x in enumerate(forcing):state=state*2+Fraction(x)+i
 restart=c.get("restart")
 if restart:
  if restart.get("replays_event") or restart.get("replays_slab"):return reject("VEG-E-127")
  if restart.get("restored_digest")!=restart.get("uninterrupted_digest"):return reject("VEG-E-128")
 if c.get("abort_parent"):return {"status":"aborted","owner_unchanged":True,"publication_visible":False}
 if c.get("publish_before_commit"):return reject("VEG-E-126")
 pub=hashlib.sha256(json.dumps(c.get("accepted_publications",[]),separators=(",",":")).encode()).hexdigest()
 result={"status":"accepted","resource_totals":totals,"ending_state":str(float(state)),"event_count":len(events),"increments":1,"atomic_commits":1,"publication_sha256":pub}
 if endings:result["resource_endings"]=endings
 return result
def main():
 m=json.loads((ROOT/"v10-v11-migration-vectors.json").read_text());s=json.loads((ROOT/"segmented-support-vectors.json").read_text());out=[]
 for c in m["cases"]:
  a=migrate(c["duration_bits"])
  if a!=c["expected"]:raise SystemExit(f"migration mismatch {c['id']}: {a} != {c['expected']}")
  out.append({"id":c["id"],"actual":a})
 for c in s["cases"]:
  a=chronology(c,int(s["parent_end_ns"]))
  if a!=c["expected"]:raise SystemExit(f"chronology mismatch {c['id']}: {a} != {c['expected']}")
  out.append({"id":c["id"],"actual":a})
 print(json.dumps({"schema":"OPENWEPP_C3_WOODY_V11_REFERENCE_RESULTS_V3","results":out},sort_keys=True,separators=(",",":")))
if __name__=="__main__":main()
