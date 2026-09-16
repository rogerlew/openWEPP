"""Read retained B01 members; report object joins, never mint admission authority."""
import hashlib
import json
from pathlib import Path

EXPORT = Path('/workdir/openwepp-experiments/b01-wb14-cadence/corrected-recorder-export-20260916-1')
HERE = Path(__file__).resolve().parent
identities = {}


def member(name):
    path = EXPORT / 'rows' / name
    data = path.read_bytes()
    digest = hashlib.sha256(data).hexdigest()
    ordinal = name.split('.')[0]
    events = [json.loads(line) for line in
              (EXPORT / 'rows' / f'{ordinal}.events.jsonl').read_text().splitlines()]
    recorded = [row for row in events if row['event'] == 'external_json_end'
                and row.get('file') == f'rows/{name}']
    assert len(recorded) == 1
    assert recorded[0]['sha256'] == digest and recorded[0]['bytes'] == len(data)
    identities[name] = {'sha256': digest, 'bytes': len(data)}
    return json.loads(data)


resident = member('123091.23.json')
bootstrap = member('108033.9.json')
phase = member('123091.26.json')
clock = json.loads(bytes(member('123091.12.json')))
clock_projection = member('123091.6.json')
clock_soil = json.loads(bytes(clock_projection['soil_thermal']))
deferred = phase['deferred_native_v2_soil_custody']
continuation = deferred['continuation']
original = continuation['original_beginning_owner']
trial = deferred['candidate']
chain = continuation['ordered_layer_credit_chain']


def metadata(owner):
    return {key: owner[key] for key in ('transaction_id',
            'expected_predecessor_transaction_id', 'support_start_ns',
            'support_end_ns', 'receipt_chain_sha256', 'run_id')}


result = {
    'evidence_class': 'Ran: read-only JSON extraction and exact object comparisons; no Rust or physics',
    'export': str(EXPORT),
    'members': identities,
    'resident': metadata(resident['owner']),
    'resident_has_latest_accepted': resident['latest_accepted'] is not None,
    'bootstrap': metadata(bootstrap['owner']),
    'clock_accepted_until_ns': clock['accepted_until_ns'],
    'clock_next_slab_ordinal': clock['next_slab_ordinal'],
    'accepted_slab_receipt_count': len(clock['accepted_slab_receipts']),
    'clock_projection_soil_owner_equals_resident_owner': clock_soil['owner'] == resident['owner'],
    'deferred_posture': deferred['posture'],
    'original_prepared_owner': metadata(original),
    'original_state_equals_resident_state': original['state'] == resident['owner']['state'],
    'original_receipt_chain_equals_resident': original['receipt_chain_sha256'] == resident['owner']['receipt_chain_sha256'],
    'deferred_candidate_equals_continuation_trial': trial == continuation['physical_trial'],
    'deferred_trial': {key: trial[key] for key in ('transaction_id',
        'predecessor_transaction_id', 'support_start_ns', 'support_end_ns',
        'unpublished_predecessor_trial_sha256', 'unpublished_trial_sha256')},
    'ordered_credit_groups': len(chain),
    'accumulated_operand_count': len(continuation['accumulated_operands']),
    'deferred_ending_differs_from_resident': trial['ending_state'] != resident['owner']['state'],
    'limitation': 'These direct object comparisons do not revalidate credits, authenticate a restoration capability, or establish successor success. Prior canonical diagnostic validation is reused.',
}
assert result['clock_projection_soil_owner_equals_resident_owner']
assert result['original_state_equals_resident_state']
assert result['original_receipt_chain_equals_resident']
assert result['deferred_candidate_equals_continuation_trial']
assert result['deferred_ending_differs_from_resident']
(HERE / 'soil-operands.json').write_text(json.dumps(result, indent=2) + '\n')
print(json.dumps(result, indent=2))
