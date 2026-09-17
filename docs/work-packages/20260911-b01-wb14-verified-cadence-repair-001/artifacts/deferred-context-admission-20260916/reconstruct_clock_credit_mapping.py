"""Independent retained-operand framing and one-to-many support join; no solver."""
from pathlib import Path
import hashlib
import json
import struct

HERE = Path(__file__).resolve().parent
ROWS = Path('/workdir/openwepp-experiments/b01-wb14-cadence/corrected-recorder-export-20260916-1/rows')
paths = [ROWS/'123091.26.json', ROWS/'123091.12.json', HERE/'retained-subslab-primitives.json', HERE/'retained-soil-trial-primitives.json']
phase = json.loads(paths[0].read_text())
clock = json.loads(bytes(json.loads(paths[1].read_text())))
subslabs = [x['record'] for x in json.loads(paths[2].read_text())['records']]
raw = json.loads(paths[3].read_text())['records']
by_receipt = {}
for entry in raw:
    record = {k:v for k,v in entry['record'].items() if k != 'capture'}
    key = record['receipt']['receipt_sha256']
    if key in by_receipt:
        assert by_receipt[key] == record
    by_receipt[key] = record
children = []
for group in phase['deferred_native_v2_soil_custody']['continuation']['ordered_layer_credit_chain']:
    matches = [by_receipt[op['debit_credit_identity_sha256']]['receipt'] for layer in group for op in layer['accepted_operands'] if op['source_kind'] == 'top_boundary']
    assert len(matches) == 1
    children.append(matches[0])

def frame(domain, fields):
    raw = b'OPENWEPP\0' + struct.pack('>HH', 1, len(domain)) + domain.encode()
    for tag, value in fields:
        raw += struct.pack('>H',len(tag)) + tag.encode() + struct.pack('>I',len(value)) + value
    return hashlib.sha256(raw).hexdigest()

mapping = []
consumed = []
assert len(subslabs) == len(clock['accepted_slab_receipts']) == len(phase['ordered_owner_joins'])
for index, (raw, accepted, phase_join) in enumerate(zip(subslabs, clock['accepted_slab_receipts'], phase['ordered_owner_joins'])):
    assert raw['accepted_slab_sha256'] == accepted['accepted_slab_id'] == phase_join['accepted_slab_sha256']
    assert raw['support'] == accepted['support'] == phase_join['support']
    assert raw['parent_transaction_id'] == accepted['parent_transaction_id'] == clock['parent_transaction_id']
    assert raw['owner_join'] == phase_join['owner_join']
    join = raw['owner_join']
    assert join['beginning_complete_owner_set_sha256'] == accepted['begin_owner_set_sha256']
    assert join['ending_complete_owner_set_sha256'] == accepted['end_owner_set_sha256']
    fields = []
    for name, tag in [('adaptive_terminal_snow_soil_heat_receipts','adaptive_terminal_snow_soil_heat_receipt'),('adaptive_terminal_snow_soil_trial_receipts','adaptive_terminal_snow_soil_trial_receipt'),('terminal_snow_soil_heat_receipts','terminal_snow_soil_heat_receipt')]:
        fields += [(tag, bytes.fromhex(receipt['receipt_sha256'])) for lane,receipt in sorted(raw[name].items(), key=lambda x:int(x[0]))]
    assert frame('covered-terminal-snow-soil-heat-receipt-set-v1', fields) == join['terminal_snow_soil_heat_receipt_set_sha256']
    start,end = (int(raw['support'][key]) for key in ['start_ns','end_ns'])
    selected = [i for i,c in enumerate(children) if start <= int(c['support']['start_ns']) and int(c['support']['end_ns']) <= end]
    assert selected
    assert int(children[selected[0]]['support']['start_ns']) == start
    assert int(children[selected[-1]]['support']['end_ns']) == end
    for before,after in zip(selected,selected[1:]):
        assert children[before]['support']['end_ns'] == children[after]['support']['start_ns']
    if raw['adaptive_terminal_snow_soil_trial_receipts']:
        assert len(selected) == 1
        assert raw['adaptive_terminal_snow_soil_trial_receipts']['1'] == children[selected[0]]
    else:
        final = raw['terminal_snow_soil_heat_receipts']['1']
        assert final['limiting_boundary_receipt_sha256'] == children[selected[-1]]['receipt_sha256']
        assert final['support'] == raw['support']
        event = phase['ordered_event_groups'][0]
        candidate = event['candidates'][0]
        assert candidate['support'] == final['support']
        assert candidate['terminal_snow_soil_trial_receipt_sha256'] == final['limiting_boundary_receipt_sha256']
        assert candidate['terminal_state_sha256'] == final['ending_dormant_snow_owner_sha256']
        assert candidate['event']['accepted_trials'] == len(selected)
        heat = sum((children[i]['snow_heat_j_m2'] for i in selected),0.0)
        assert heat == final['snow_heat_j_m2'] == candidate['event']['snow_soil_heat_energy_j_m2']
        assert event['accepted_event_receipt']['receipt_id'] == clock['accepted_event_receipts'][0]['receipt_id']
        assert event['receipt_sha256'] == clock['accepted_event_receipts'][0]['event_context_sha256']
        assert event['accepted_event_receipt']['begin_owner_set'] == accepted['end_owner_set_sha256']
    consumed.extend(selected)
    mapping.append({'slab':index,'accepted_slab_id':accepted['accepted_slab_id'],'children':selected,'support':raw['support'],'terminal_set_sha256':join['terminal_snow_soil_heat_receipt_set_sha256']})
assert consumed == list(range(len(children)))
result = {'evidence_class':'Ran: independent Python framing/support/receipt join, no physical evolution', 'inputs':{str(p):hashlib.sha256(p.read_bytes()).hexdigest() for p in paths}, 'mapping':mapping, 'limits':'Retained JSON operand join only; canonical Rust receipt validators, semantic negative controls, opaque capability, and live admission are separate obligations.'}
(HERE/'independent-clock-credit-mapping.json').write_text(json.dumps(result,indent=2)+'\n')
print(json.dumps({'slabs':len(mapping),'child_mapping':[m['children'] for m in mapping],'status':'PASS narrow retained operand mapping'}))
