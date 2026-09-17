"""Independent producer-tuple hash reconstruction from retained raw state preimages.
No solver, owner mutation, or capability construction. Rust typed checks remain separate.
"""
from pathlib import Path
import hashlib
import json

HERE = Path(__file__).resolve().parent
ROWS = Path('/workdir/openwepp-experiments/b01-wb14-cadence/corrected-recorder-export-20260916-1/rows')
paths = [HERE/'retained-soil-trial-primitives.json', HERE/'retained-subslab-primitives.json', ROWS/'123091.26.json']
trials = json.loads(paths[0].read_text())['records']
subslabs = json.loads(paths[1].read_text())['records']
phase = json.loads(paths[2].read_text())

def layer(value):
    result = {k: value[k] for k in ('layer_id', 'temperature_k', 'enthalpy_hi_j_m2_ofe_ground')}
    result['enthalpy_carry'] = {k: value['enthalpy_carry'][k] for k in ('sign', 'coefficient_hex', 'exponent2')}
    result['last_accepted_transaction_id'] = value['last_accepted_transaction_id']
    return result

def digest(value):
    # This corpus's candidate tuples have no exponent-form floats; the separately
    # reconstructed OFE likewise uses fixed-format temperatures/high terms.
    return hashlib.sha256(json.dumps(value, separators=(',', ':'), ensure_ascii=False).encode()).hexdigest()

results = {}
for entry in trials:
    raw = entry['record']
    ending = layer(raw['ending_soil']['v2'])
    assert ending['last_accepted_transaction_id'] == raw['transaction_id']
    preimage = ['OPENWEPP_TERMINAL_FIRST_SOIL_CANDIDATE_V2', raw['beginning_soil_owner_id'], raw['beginning_soil_state_sha256'], raw['transaction_id'], ending]
    actual = digest(preimage)
    assert actual == raw['receipt']['ending_soil_candidate_sha256'], (raw['transaction_id'], actual)
    results[raw['receipt']['receipt_sha256']] = {'physical_transaction_id': raw['transaction_id'], 'candidate_sha256': actual}
assert len(results) == 12
last = subslabs[-1]['record']
ending_state = last['ending_soil']['v2']
custody = phase['deferred_native_v2_soil_custody']
# Locate the source-defined final state without deriving it from receipt digests.
assert ending_state == custody['candidate']['ending_state']
ofe = ending_state['ofes'][0]
assert len(ending_state['ofes']) == 1
preimage = {'ofe_id': ofe['ofe_id'], 'ordered_layers': [layer(x) for x in ofe['ordered_layers']]}
actual = digest(preimage)
assert actual == last['terminal_snow_soil_heat_receipts']['1']['ending_soil_owner_sha256'], actual
result = {'evidence_class': 'Ran: independent retained-state producer-tuple hashing; no physical work', 'inputs': {str(p): hashlib.sha256(p.read_bytes()).hexdigest() for p in paths}, 'trial_candidates': results, 'terminal_ending_ofe_sha256': actual, 'final_native_transaction_id': ending_state['last_accepted_transaction_id'], 'limits': 'Independent operand hash joins only; typed canonical validators, semantic controls and composite admission remain separate.'}
(HERE/'independent-candidate-state-hashes.json').write_text(json.dumps(result, indent=2)+'\n')
print(json.dumps({'trial_candidates': len(results), 'terminal_ending_ofe_sha256': actual, 'status': 'PASS retained state hash joins'}))
