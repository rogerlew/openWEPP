#!/usr/bin/env python3
"""Fail-closed, offline checker for the one extracted B01 WB14 failure packet."""
import argparse, hashlib, json, math, struct
from pathlib import Path

TEST = "hillslope::tests::stage3_snow_accuracy_case"
ERROR = "SURFACELIQUID-E-008 IngressCandidate: WB14 day or interval continuation mismatch"
SOURCE_SHA = "7a944817acf43be2ba018f670efbd3490a0cf1f3629b0011ae26449de2332ba9"
BINARY_SHA = "958e61764d373a2c183764e7f7ff41607a32ef74a2af5cd7b3f299bf1123906f"

class WitnessError(ValueError):
    def __init__(self, code): self.code = code; super().__init__(code)
def fail(code): raise WitnessError(code)
def sha(path):
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for chunk in iter(lambda: f.read(1024 * 1024), b""): h.update(chunk)
    return h.hexdigest()
def need(x, key, code="typed"):
    if not isinstance(x, dict) or key not in x: fail(code)
    return x[key]
def u128(x, code):
    if type(x) is int and 0 <= x < 2**128: return x
    if type(x) is str and x.isdecimal() and int(x) < 2**128: return int(x)
    fail(code)
def raw(record, key):
    value = need(record, key)
    if set(value) != {"Ok"} or not isinstance(value["Ok"], list): fail("typed")
    if any(type(b) is not int or not 0 <= b <= 255 for b in value["Ok"]): fail("typed")
    try: return bytes(value["Ok"]), json.loads(bytes(value["Ok"]))
    except (UnicodeDecodeError, json.JSONDecodeError): fail("typed")
def a32(x, key):
    value = need(x, key, "identity")
    if not isinstance(value, list) or len(value) != 32 or any(type(b) is not int or not 0 <= b <= 255 for b in value): fail("identity")
    return value
def f64(value, code="cumulative"):
    if type(value) not in (int, float) or not math.isfinite(value): fail(code)
    return struct.pack(">d", float(value))
def state(value, code, ofe="ofe-1"):
    cs = need(value, "continuations", code)
    if not isinstance(cs, list) or len(cs) != 1: fail(code)
    c = cs[0]
    if need(c, "ofe_id", code) != ofe or u128(need(c,"day_index",code),code) != 0 or u128(need(c,"next_interval_index",code),code) != 0: fail(code)
    for field in ("cumulative_supply_m", "cumulative_infiltration_m"):
        if f64(need(c,field,code)) != b"\x00" * 8: fail("cumulative")
    return c
def equal_state(left, right, code):
    if left != right: fail(code)
def hex32(text, code="identity"):
    if not isinstance(text, str) or len(text) != 64:
        fail(code)
    try: return list(bytes.fromhex(text))
    except ValueError: fail(code)

def check(packet_path, receipt_path, run_path, binary_path, listing_path):
    packet = json.loads(Path(packet_path).read_text())
    receipt = json.loads(Path(receipt_path).read_text())
    run = json.loads(Path(run_path).read_text())
    if packet.get("schema") != "current_run_boundary_packet_v1": fail("packet")
    source = need(packet,"source","packet")
    if source.get("sha256") != SOURCE_SHA or source.get("bytes") != 3699473934: fail("packet")
    if packet.get("top_level_scalars") != {"schema":"snow_accuracy_observation_v2","physical_enabled":True,"overflow":False,"counts_poisoned":False}: fail("observation")
    if packet.get("completeness_flags") != {"physical_enabled":True,"overflow":False,"counts_poisoned":False}: fail("observation")
    if packet.get("receipt_completeness") != {"observation_complete":True,"files_complete":False,"days_complete":False}: fail("observation")
    stream = need(packet,"stream","packet")
    if stream.get("array_prefix") != "physical.item" or stream.get("matching_records") != 2 or stream.get("trailing_validation") != "ijson complete stream parse": fail("records")
    rows = need(packet,"records","records")
    if not isinstance(rows,list) or len(rows) != 2: fail("records")
    g, c = (need(rows[0],"record","records"), need(rows[1],"record","records"))
    if rows[1].get("physical_ordinal") != rows[0].get("physical_ordinal",-2) + 1: fail("records")
    if (g.get("kind"),c.get("kind")) != ("surface_liquid_wb14_cadence_failure","surface_liquid_wb14_cadence_caller_failure"): fail("records")
    gc, cc = need(g,"capture","capture"), need(c,"capture","capture")
    if any(gc.get(k) != cc.get(k) for k in ("process","session","thread")) or cc.get("ordinal") != gc.get("ordinal",-2)+1: fail("capture")
    gb, gi = raw(g,"input_typed_bytes"); cb, ci = raw(c,"input_typed_bytes")
    if gb != cb: fail("input_pair")
    if [u128(need(gi,k,"input"),"input") for k in ("day_index","interval_index","transaction_id")] != [4,22,255] or f64(need(gi,"interval_s","input"),"input") != struct.pack(">d",60.0): fail("input")
    if (g.get("actual_day"),g.get("actual_interval"),g.get("ofe_id"),g.get("initial"),g.get("expected_day"),g.get("expected_interval"),g.get("accepted_parent_local_projection"),g.get("state_mutated")) != (4,22,"ofe-1",True,0,0,False,False): fail("guard")
    if c.get("error") != ERROR or ERROR not in str(run.get("error")): fail("error")
    if any(r.get("parent_child_mode") is not True or r.get("finalize_parent_interval") is not False for r in (g,c)): fail("guard")
    bb, beginning = raw(g,"beginning_typed_bytes"); bb2, caller_beginning = raw(c,"beginning_typed_bytes")
    if bb != bb2: fail("beginning")
    begin = state(beginning,"state"); state(caller_beginning,"state")
    _, parent = raw(c,"parent_working_typed_bytes"); _, working = raw(c,"working_typed_bytes")
    work = state(working,"state")
    if parent.get("schema") != "OPENWEPP_DIRECT_WB14_PARENT_WORKING_STATE_V2": fail("parent")
    if [u128(need(parent,k,"parent"),"parent") for k in ("parent_day_index","parent_interval_index")] != [4,22]: fail("parent")
    binding = need(c,"coupled_binding","support")
    bounds = [u128(need(binding,k,"support"),"support") for k in ("parent_support_start_ns","parent_support_end_ns","child_support_start_ns","child_support_end_ns")]
    if bounds != [385200000000000,387000000000000,385920000000000,385980000000000] or not bounds[2] < bounds[3] <= bounds[1]: fail("support")
    if u128(need(parent,"accepted_until_ns","parent"),"parent") != bounds[2] or [u128(need(parent,k,"parent"),"parent") for k in ("parent_support_start_ns","parent_support_end_ns")] != bounds[:2]: fail("support")
    if struct.unpack(">d",u128(need(binding,"proposed_upper_bound_s_bits","support"),"support").to_bytes(8,"big"))[0] != 60.0: fail("support")
    persistent = need(parent,"persistent_beginning_state","parent"); candidate = need(parent,"candidate_state","parent")
    equal_state(persistent, beginning, "parent"); equal_state(candidate, beginning, "parent"); equal_state(working, beginning, "parent")
    for supplied in (state(persistent,"state"), state(candidate,"state"), begin, work):
        for field in ("cumulative_supply_m","cumulative_infiltration_m"):
            if f64(supplied[field]) != b"\x00"*8: fail("cumulative")
    if parent.get("parameters") != gi.get("wb14_parameters") or parent.get("surface_liquid_configuration_sha256") != beginning.get("configuration_sha256"): fail("identity")
    authorities = need(parent,"per_ofe_authorities","ofe")
    if set(authorities) != {"ofe-1"}: fail("ofe")
    auth = authorities["ofe-1"]; authority, cursor, scalar = need(auth,"authority","identity"), need(auth,"beginning_cursor","state"), need(auth,"working","state")
    state({"continuations":[dict(cursor,ofe_id="ofe-1")]},"state")
    if u128(need(scalar,"accepted_until_ns","support"),"support") != bounds[2] or u128(need(scalar,"next_child_ordinal","ordinal"),"ordinal") != 0 or auth.get("receipts") != []: fail("ordinal")
    for field in ("cumulative_supply_m","cumulative_infiltration_m"):
        if f64(need(scalar,field,"cumulative")) != b"\x00"*8: fail("cumulative")
    prefix = need(authority,"inactive_prefix","identity")
    for x in (authority,prefix,binding):
        for key in x:
            if key.endswith("sha256"): a32(x,key)
    if authority.get("coupled_parent_transaction_sha256") != prefix.get("coupled_parent_transaction_sha256") != binding.get("coupled_parent_transaction_sha256"): fail("identity")
    if authority.get("parent_beginning_owner_sha256") != prefix.get("parent_beginning_owner_sha256") or prefix.get("prefix_ending_owner_sha256") != binding.get("parent_beginning_complete_owner_set_sha256"): fail("identity")
    if [u128(need(prefix,k,"support"),"support") for k in ("parent_support_start_ns","parent_support_end_ns")] != bounds[:2] or u128(need(prefix,"prefix_end_ns","support"),"support") != bounds[2]: fail("support")
    if [u128(need(authority,k,"identity"),"identity") for k in ("parent_day_index","parent_interval_index")] != [0,0]: fail("identity")
    if hex32(parent["surface_liquid_configuration_sha256"]) != authority.get("surface_liquid_configuration_sha256") or hex32(parent["wb14_configuration_sha256"]) != authority.get("wb14_configuration_sha256") or hex32(parent["wb14_model_definition_sha256"]) != authority.get("wb14_model_definition_sha256"): fail("identity")
    if receipt.get("timeout") or receipt.get("infrastructure_error") or receipt.get("exit_code",0) == 0 or receipt.get("execution_valid") is not False or receipt.get("runner_execution") != "FAIL" or receipt.get("binary_unchanged") is not True or receipt.get("runner_receipt_present") is not True or run.get("execution") != "FAIL": fail("receipt")
    if receipt.get("runner_receipt_sha256") != sha(run_path) or receipt.get("binary_sha256") != BINARY_SHA or sha(binary_path) != BINARY_SHA: fail("receipt")
    if receipt.get("case") != "gradual_warm_tail7" or receipt.get("policy") != "B01" or receipt.get("ofes") != 1 or receipt.get("physical") is not True or receipt.get("observation_complete") is not True or receipt.get("files_complete") is not False or receipt.get("days_complete") is not False or receipt.get("physical_rows") != stream.get("records_scanned"): fail("receipt")
    if receipt.get("artifact_sha256",{}).get("observations.json") != SOURCE_SHA: fail("receipt")
    argv = ["taskset","-c","0",str(binary_path),TEST,"--ignored","--exact","--nocapture","--test-threads=1"]
    if receipt.get("argv") != argv or Path(listing_path).read_text().splitlines() != [TEST+": test","","1 test, 0 benchmarks"]: fail("selection")
    return {"witness_verdict":"LIMITED_CHECKER_PASS_PENDING_REVIEW","runner_verdict":"FAIL","observation_sha256":SOURCE_SHA,"failure_row":rows[0]["physical_ordinal"],"caller_row":rows[1]["physical_ordinal"]}

if __name__ == "__main__":
    p=argparse.ArgumentParser()
    for name in ("packet","receipt","run","binary","listing"): p.add_argument(name,type=Path)
    p.add_argument("--output",type=Path); a=p.parse_args(); result=check(a.packet,a.receipt,a.run,a.binary,a.listing)
    if a.output: a.output.write_text(json.dumps(result,indent=2)+"\n")
    else: print(json.dumps(result))
