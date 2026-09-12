#!/usr/bin/env python3
"""Synthetic negative checks; these never execute the Rust runner."""
import hashlib
import json
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).parent
CHECK = ROOT / "check_witness.py"

def raw(obj): return list(json.dumps(obj, separators=(",", ":")).encode())
def fixture(binary):
    inp={"day_index":4,"interval_index":22,"transaction_id":"255","interval_s":60}
    bind={"parent_support_start_ns":"385200000000000","parent_support_end_ns":"387000000000000","child_support_start_ns":"385920000000000","child_support_end_ns":"385980000000000"}
    rows=[{"kind":"surface_liquid_wb14_cadence_failure","input_typed_bytes":raw(inp),"actual_day":4,"actual_interval":22,"parent_child_mode":True,"finalize_parent_interval":False},{"kind":"surface_liquid_wb14_cadence_caller_failure","input_typed_bytes":raw(inp),"error":"SURFACELIQUID-E-008 continuation","coupled_binding":bind}]
    return {"schema":"snow_accuracy_observation_v2","physical_enabled":True,"overflow":False,"counts_poisoned":False,"physical":rows},{"timeout":False,"binary_unchanged":True,"execution_valid":False,"runner_execution":"FAIL","argv":["taskset",str(binary),"hillslope::tests::stage3_snow_accuracy_case"],"binary_sha256":hashlib.sha256(binary.read_bytes()).hexdigest()}
with tempfile.TemporaryDirectory() as temp:
    temp=Path(temp); binary=temp/'binary'; binary.write_bytes(b'frozen')
    obs, receipt=fixture(binary)
    def run(label, mutate, ok):
        o=json.loads(json.dumps(obs)); r=json.loads(json.dumps(receipt)); mutate(o,r)
        op=temp/f'{label}.json'; rp=temp/f'{label}-receipt.json'; op.write_text(json.dumps(o)); rp.write_text(json.dumps(r))
        got=subprocess.run([sys.executable,str(CHECK),str(op),str(rp),str(binary)],capture_output=True,text=True).returncode == 0
        assert got == ok, (label, got)
    run('valid',lambda o,r:None,True)
    run('missing-record',lambda o,r:o.__setitem__('physical',o['physical'][:1]),False)
    run('wrong-boundary',lambda o,r:o['physical'][0].__setitem__('actual_day',3),False)
    run('wrong-binary',lambda o,r:r.__setitem__('binary_sha256','0'*64),False)
    run('observer-loss',lambda o,r:o.__setitem__('counts_poisoned',True),False)
    run('truncated-input',lambda o,r:o['physical'][0].__setitem__('input_typed_bytes',[123]),False)
    run('generic-panic-timeout',lambda o,r:(o['physical'][1].__setitem__('error','panic'),r.__setitem__('timeout',True)),False)
print('checker synthetic cases: PASS (1 positive, 6 required negative)')
