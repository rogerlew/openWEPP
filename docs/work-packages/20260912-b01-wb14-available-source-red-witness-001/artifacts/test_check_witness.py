"""Named source-shaped controls for the current captured boundary packet only."""
import copy, hashlib, json, tempfile
from pathlib import Path
from check_witness import WitnessError, check, u128

HERE = Path(__file__).parent
PACKET = HERE / "current-run-boundary-packet.json"
RAW = Path("/workdir/openwepp-experiments/b01-wb14-available-source-red-witness/raw-capture-20260912-1")
BINARY = Path("/workdir/openwepp-experiments/b01-wb14-available-source-red-witness/artifacts/openwepp_runner-identity-stamped.frozen")
LISTING = Path("/workdir/openwepp-experiments/b01-wb14-available-source-red-witness/artifacts/compiled-listing-identity-stamped.stdout")
def row(p, i): return p["records"][i]["record"]
def typed(r, k): return json.loads(bytes(r[k]["Ok"]))
def put(r, k, x): r[k] = {"Ok": list(json.dumps(x, separators=(",", ":")).encode())}
def parent(p): return typed(row(p, 1), "parent_working_typed_bytes")
def edit_parent(p, f):
 r=row(p,1); x=parent(p); f(x); put(r,"parent_working_typed_bytes",x)
def edit_input(p, i, f):
 r=row(p,i); x=typed(r,"input_typed_bytes"); f(x); put(r,"input_typed_bytes",x)
def edit_state(p, name, field, value):
 r=row(p, 0 if name=="beginning" else 1); key="beginning_typed_bytes" if name=="beginning" else name+"_typed_bytes"; x=typed(r,key); x["continuations"][0][field]=value; put(r,key,x)
def reject(name, code, packet_edit=None, receipt_edit=None, run_edit=None, listing_edit=None):
 p=json.loads(PACKET.read_text()); receipt=json.loads((RAW/"receipt.json").read_text()); run=json.loads((RAW/"run.json").read_text())
 if packet_edit: packet_edit(p)
 if receipt_edit: receipt_edit(receipt)
 if run_edit: run_edit(run)
 with tempfile.TemporaryDirectory() as td:
  d=Path(td); pp,rp,up,lp=d/"packet.json",d/"receipt.json",d/"run.json",d/"listing"
  pp.write_text(json.dumps(p,separators=(",",":")))
  if run_edit is None: up.write_text((RAW/"run.json").read_text())
  else:
   up.write_text(json.dumps(run,separators=(",",":")))
   receipt["runner_receipt_sha256"]=hashlib.sha256(up.read_bytes()).hexdigest()
  rp.write_text(json.dumps(receipt,separators=(",",":")))
  lp.write_text(LISTING.read_text() if listing_edit is None else listing_edit(LISTING.read_text()))
  try: result=check(pp,rp,up,BINARY,lp)
  except WitnessError as e: assert e.code==code, f"{name}: {e.code} != {code}"
  else: assert code=="positive", f"{name}: accepted {result}"

# Compact executable case table. Each mutation retains valid JSON and changes the
# named target predicate; synthetic receipt/run/listing wrappers are separate.
reject("current_packet_positive", "positive")
reject("missing_record", "records", lambda p:p["records"].pop())
reject("wrong_boundary_valid_json", "input", lambda p:edit_input(p,0,lambda x:x.__setitem__("day_index",5)))
reject("raw_pair_mismatch", "input_pair", lambda p:edit_input(p,1,lambda x:x.__setitem__("interval_index",23)))
reject("capture_mismatch", "capture", lambda p:p["records"][1]["record"]["capture"].__setitem__("process",0))
reject("overflow_observation_loss", "observation", lambda p:p["top_level_scalars"].__setitem__("overflow",True))
reject("poisoned_observation_loss", "observation", lambda p:p["top_level_scalars"].__setitem__("counts_poisoned",True))
reject("truncated_input", "typed", lambda p:row(p,0)["input_typed_bytes"]["Ok"].pop())
reject("generic_panic", "error", lambda p:row(p,1).__setitem__("error","panic"))
reject("timeout_only", "receipt", receipt_edit=lambda r:r.__setitem__("timeout",True))
reject("wrong_binary", "receipt", receipt_edit=lambda r:r.__setitem__("binary_sha256","0"*64))
reject("zero_exit", "receipt", receipt_edit=lambda r:r.__setitem__("exit_code",0))
reject("infrastructure", "receipt", receipt_edit=lambda r:r.__setitem__("infrastructure_error","x"))
reject("run_hash", "receipt", receipt_edit=lambda r:r.__setitem__("runner_receipt_sha256","0"*64))
reject("artifact_hash", "receipt", receipt_edit=lambda r:r["artifact_sha256"].__setitem__("observations.json","0"*64))
reject("wrong_argv", "selection", receipt_edit=lambda r:r.__setitem__("argv",[]))
reject("zero_listing", "selection", listing_edit=lambda _:"\n0 tests, 0 benchmarks\n")
for name,key in (("beginning","beginning_typed_bytes"),("parent","parent_working_typed_bytes"),("working","working_typed_bytes")):
 reject("missing_"+name,"typed",lambda p,key=key:row(p,0 if name=="beginning" else 1).pop(key))
reject("missing_ofe","ofe",lambda p:edit_parent(p,lambda x:x.__setitem__("per_ofe_authorities",{})))
reject("extra_ofe","ofe",lambda p:edit_parent(p,lambda x:x["per_ofe_authorities"].__setitem__("other",copy.deepcopy(x["per_ofe_authorities"]["ofe-1"]))))
reject("wrong_ofe","ofe",lambda p:row(p,0).__setitem__("ofe_id","other"))
for state in ("beginning","working"):
 for field in ("cumulative_supply_m","cumulative_infiltration_m"):
  reject(state+"_"+field,"cumulative",lambda p,s=state,f=field:edit_state(p,s,f,1.0))
reject("cumulative_nonfinite","cumulative",lambda p:edit_state(p,"beginning","cumulative_supply_m",float("inf")))
reject("cumulative_nonnumeric","cumulative",lambda p:edit_state(p,"beginning","cumulative_supply_m","zero"))
reject("cumulative_negative_zero","cumulative",lambda p:edit_state(p,"beginning","cumulative_supply_m",-0.0))
for field in ("cumulative_supply_m","cumulative_infiltration_m"):
 reject("persistent_"+field,"cumulative",lambda p,f=field:edit_parent(p,lambda x:x["persistent_beginning_state"]["continuations"][0].__setitem__(f,1.0)))
 reject("candidate_"+field,"cumulative",lambda p,f=field:edit_parent(p,lambda x:x["candidate_state"]["continuations"][0].__setitem__(f,1.0)))
 reject("cursor_"+field,"cumulative",lambda p,f=field:edit_parent(p,lambda x:x["per_ofe_authorities"]["ofe-1"]["beginning_cursor"].__setitem__(f,1.0)))
 reject("scalar_"+field,"cumulative",lambda p,f=field:edit_parent(p,lambda x:x["per_ofe_authorities"]["ofe-1"]["working"].__setitem__(f,1.0)))
reject("support","support",lambda p:row(p,1)["coupled_binding"].__setitem__("parent_support_end_ns","387000000000001"))
reject("ordinal","ordinal",lambda p:edit_parent(p,lambda x:x["per_ofe_authorities"]["ofe-1"]["working"].__setitem__("next_child_ordinal",1)))
reject("prior_receipt","ordinal",lambda p:edit_parent(p,lambda x:x["per_ofe_authorities"]["ofe-1"].__setitem__("receipts",[{"x":1}])))
reject("prefix_identity","identity",lambda p:edit_parent(p,lambda x:x["per_ofe_authorities"]["ofe-1"]["authority"]["inactive_prefix"]["coupled_parent_transaction_sha256"].__setitem__(0,0)))
reject("upper_bound","support",lambda p:row(p,1)["coupled_binding"].__setitem__("proposed_upper_bound_s_bits",0))
reject("u128_out_of_domain","input",lambda p:edit_input(p,0,lambda x:x.__setitem__("transaction_id",str(2**128))))
assert u128(str(2**127 + 17), "u128") == 2**127 + 17
print("PASS current-packet positive plus named finite negative controls")
