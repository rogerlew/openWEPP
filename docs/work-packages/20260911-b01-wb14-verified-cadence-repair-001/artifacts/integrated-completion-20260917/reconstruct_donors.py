"""Join primitive donors to all selected native fixture credits, fail closed.

Extends the prior suffix reconstruction to the accepted six-child snow prefix.
Trial observations are not accepted by support, position, proximity or energy.
Selection requires the exact retained receipt digest and its full phase payload,
then independently recomputed internal source digests and exact credit membership.
This is source correspondence, not full scientific conservation qualification.
"""
import argparse
from collections import Counter
import hashlib
import json
from pathlib import Path
import re

from reconstruct_credits import read_records, unique_object, reject_constant, require, bits, LAYERS
from source_digest_helpers import physical_sources, trial_receipt_digest, probe_child_digest

PHASE_TAG = 'B01_INTEGRATED_ACCEPTED_PHASE_CHAIN'
RESEAL_TAG = 'B01_INTEGRATED_PROBE_RESEAL'
NS = 1_000_000_000
# Prospective inventory for the unchanged fixture's next affected positive run,
# established from v8 observer reach. All trial records remain accountable.
ENDPOINT_SUPPORT_INVENTORY = [
    [(0, 60)], [(60, 120)], [(120, 180)], [(60, 120)], [(120, 180)],
    [(180, 240), (240, 300), (300, 360)], [(180, 240)], [(240, 300)], [(300, 360)],
]


def reseal_records(raw, phases):
    records = []
    pattern = re.compile(r'(?<![A-Z0-9_])' + RESEAL_TAG + r'(?=\s|$)')
    for line in raw.decode().splitlines():
        match = pattern.search(line)
        if match:
            value = json.loads(line[match.end():].strip(), object_pairs_hook=unique_object,
                               parse_constant=reject_constant)
            require(isinstance(value, dict), 'reseal record object')
            records.append(value)
    require(len(records) == 11, 'required eleven canonical reseal records')
    preterminal = 'preterminal accepted replay child physical ordinal reseal'
    endpoint = 'exact endpoint selected carrier-phase physical ordinal reseal'
    require([r['boundary'] for r in records] == [preterminal]*5 + [endpoint]*3 +
            [preterminal]*2 + [endpoint], 'complete reseal boundary order')
    authenticated = {}
    roles = [('Full', 0), ('Retry', 3), ('Full', 0), ('Full', 0), ('Full', 0),
             ('Retry', 3), ('Retry', 3), ('Full', 0), ('Retry', 3), ('Full', 0), ('Full', 0)]
    for record, phase, role in zip(records, phases, roles):
        require(record['record_version'] == 1, 'reseal record version')
        old, new = record['old_identity'], record['new_identity']
        old_digest, new_digest = probe_child_digest(old), probe_child_digest(new)
        require(set(old) == set(new), 'reseal identity field inventory')
        require(all(old[k] == new[k] for k in old if k not in
                    ('physical_child_ordinal', 'receipt_sha256')), 'only physical ordinal changes at reseal')
        require((old['role'], old['role_u8']) == (new['role'], new['role_u8']) == role,
                'native fixture complete probe role inventory')
        require(new['enclosing_parent_support'] == {'start_ns':'0', 'end_ns':str(1800*NS)}, 'reseal parent support')
        require(new['trial_support'] == phase['support'], 'reseal phase support')
        require(new['physical_child_ordinal'] == phase['physical_child_ordinal'], 'reseal phase ordinal')
        require(new_digest == phase['probe_child_receipt_sha256'], 'reseal phase receipt')
        require(record['carrier_ending_joint_sha256'] == phase['carrier_ending_joint_sha256'],
                'reseal selected ending joint')
        require(record['top_boundary_credits_by_lane'] == phase['top_boundary_credits_by_lane'], 'reseal exact credit payload')
        require(record['trial_source_receipts_by_lane'] == phase['terminal_snow_soil_trial_receipts_by_lane'], 'reseal exact source payload')
        require(set(record['trial_source_receipts_by_lane']) == {'1'}, 'reseal single lane')
        trial = record['trial_source_receipts_by_lane']['1']
        digest = trial_receipt_digest(trial)
        require(trial['canonical_source_sha256'] == old_digest, 'original source probe identity')
        require(digest not in authenticated, 'unique resealed primitive receipt')
        authenticated[digest] = dict(old_probe_sha256=old_digest, new_probe_sha256=new_digest,
            carrier_ending_joint_sha256=record['carrier_ending_joint_sha256'],
            parent_transaction_sha256=new['parent_transaction_sha256'])
    require(len({r['parent_transaction_sha256'] for r in authenticated.values()}) == 1,
            'one canonical parent across all reseals')
    return authenticated


def phase_records(raw):
    records = []
    pattern = re.compile(r'(?<![A-Z0-9_])' + PHASE_TAG + r'(?=\s|$)')
    for line in raw.decode().splitlines():
        match = pattern.search(line)
        if match:
            value = json.loads(line[match.end():].strip(), object_pairs_hook=unique_object,
                               parse_constant=reject_constant)
            require(isinstance(value, dict), 'phase record object')
            require(value['record_version'] == 1, 'phase record version')
            require(value['endpoint_finalizer_succeeded'] is True, 'endpoint-local finalizer success')
            require(value['selection_boundary'] == 'endpoint-local terminal finalizer success; global donor membership unresolved', 'endpoint observation boundary')
            records.append(value)
    require(len(records) == 9, 'required nine endpoint phase-chain observations')
    observed = [[(int(p['support']['start_ns']), int(p['support']['end_ns']))
                 for p in r['selected_phase_chain']] for r in records]
    require(observed == [[(a * NS, b * NS) for a, b in group] for group in ENDPOINT_SUPPORT_INVENTORY], 'complete endpoint phase-record inventory/order')
    phases = [p for r in records for p in r['selected_phase_chain']]
    require(len(phases) == 11, 'required eleven observed phase payloads')
    return phases


def op_key(op):
    return (op['ofe_id'], op['layer_id'], op['source_kind'], op['source_owner_id'],
            op['debit_credit_identity_sha256'], op['ordinal'], op['units'], op['basis'],
            bits(op['energy_j_m2_ofe_ground']))


def reconstruct(raw):
    records = read_records(raw)
    lineage = records['B01_INTEGRATED_LINEAGE']
    installed = records['B01_INTEGRATED_INSTALLED']
    chain = lineage['ordered_layer_credit_chain']
    require(len(chain) == 7, 'seven selected credit groups')
    phases = phase_records(raw)
    reseals = reseal_records(raw, phases)
    parent_record = read_records(raw, ('B01_INTEGRATED_PARENT',))['B01_INTEGRATED_PARENT']
    parent_clock = json.loads(bytes(parent_record['parent']['beginning_clock_canonical_json']),
                             object_pairs_hook=unique_object, parse_constant=reject_constant)
    require(all(row['parent_transaction_sha256'] == parent_clock['parent_transaction_id']
                for row in reseals.values()), 'all donor reseals join actual accepted parent')
    by_receipt = {}
    all_rows = []
    for phase in phases:
        require(set(phase['top_boundary_credits_by_lane']) == {'1'}, 'single-lane credit inventory')
        require(set(phase['terminal_snow_soil_trial_receipts_by_lane']) == {'1'}, 'single-lane trial inventory')
        trial = phase['terminal_snow_soil_trial_receipts_by_lane']['1']
        credit = phase['top_boundary_credits_by_lane']['1']
        digest = trial_receipt_digest(trial)
        require(digest not in by_receipt, 'duplicate primitive trial receipt')
        # Canonical replay reseals physical ordinals; authenticate both namespaces.
        start, end = int(phase['support']['start_ns']), int(phase['support']['end_ns'])
        require(trial['support'] == phase['support'], 'trial phase support')
        require((credit['support_start_ns'], credit['support_end_ns']) == (start, end), 'credit source support')
        require(credit['snow_soil_heat_receipt_sha256'] == digest, 'credit source receipt identity')
        require(credit['ofe_id'] == trial['ofe_id'] == 'ofe-1' and credit['lane_id'] == trial['lane_id'] == 1, 'donor lane/OFE')
        require(credit['first_layer_id'] == LAYERS[0], 'top boundary layer')
        require(bits(trial['soil_heat_j_m2']) == bits(credit['soil_thermal_credit_j_m2_ofe_ground']) ==
                bits(credit['accepted_positive_downward_j_m2_ofe_ground']), 'donor credit exact sign and energy')
        for tile in phase['pre_ingress_soil_thermal']:
            require(tile['beginning_state_sha256'] == credit['beginning_state_sha256'], 'soil beginning identity')
            require(tile['owner_id'] == credit['beginning_owner_id'], 'soil beginning owner')
            require(tile['beginning_identity']['V2']['configuration_sha256'] == credit['beginning_configuration_sha256'], 'soil configuration')
        by_receipt[digest] = phase
        all_rows.append(dict(receipt_sha256=digest, support_ns=[start, end], trial_receipt_digest_verified=True,
                             physical_source_sha256=trial['canonical_source_sha256'],
                             endpoint_probe_sha256=phase['probe_child_receipt_sha256'],
                             source_equals_endpoint_probe=trial['canonical_source_sha256']==phase['probe_child_receipt_sha256']))
    selected_digests = []
    selected_rows = []
    for index, group in enumerate(chain[:6]):
        require(len(group) == 6, 'six layer rows per prefix group')
        expected = [op for row in group for op in row['accepted_operands']]
        tops = [op for op in expected if op['source_kind'] == 'top_boundary']
        require(len(tops) == 1, 'one selected top source per group')
        top = tops[0]
        digest = top['debit_credit_identity_sha256']
        require(digest in by_receipt, 'selected primitive source absent')
        phase = by_receipt[digest]
        selected_digests.append(digest)
        start, end = int(phase['support']['start_ns']), int(phase['support']['end_ns'])
        require((start, end) == (index * 60 * NS, (index + 1) * 60 * NS), 'selected exact prefix partition')
        require(phase['physical_child_ordinal'] == index, 'selected physical phase ordinal')
        actual = physical_sources(phase, phase['child_transaction_id'], start, end)
        trial = phase['terminal_snow_soil_trial_receipts_by_lane']['1']
        actual.append(dict(ofe_id='ofe-1', layer_id=LAYERS[0], source_kind='top_boundary',
            source_owner_id='snow', debit_credit_identity_sha256=digest, ordinal=0,
            units='J m^-2 OFE-ground', basis='ofe_ground', energy_j_m2_ofe_ground=trial['soil_heat_j_m2']))
        require(Counter(map(op_key, actual)) == Counter(map(op_key, expected)), 'primitive prefix source/credit exact membership')
        selected_rows.append(dict(group=index, support_ns=[start,end], source_transaction_id=phase['child_transaction_id'],
                                  top_source_receipt_sha256=digest, internal_operands=12, top_operands=1,
                                  reseal=reseals[digest]))
    require(len(set(selected_digests)) == 6, 'unique selected source receipts')
    # Suffix source transaction is the actual ingress receipt transaction, independently
    # checked against the full internal digest preimage and canonical target identity.
    receipts = lineage['surface_ingress_receipts']
    require(len(receipts) == 2, 'two selected infiltration source receipts')
    source_tx = receipts[0]['transaction_id']
    trial = lineage['physical_trial']
    suffix = physical_sources(lineage, source_tx, trial['support_start_ns'], trial['support_end_ns'])
    require((trial['support_start_ns'],trial['support_end_ns']) == (360*NS,420*NS), 'selected physical suffix support')
    expected = [op for row in chain[-1] for op in row['accepted_operands']]
    require(Counter(map(op_key,suffix)) == Counter(map(op_key,expected)), 'primitive suffix source/credit exact membership')
    require(installed['soil_latest_accepted']['expected_sources']['accepted_operands'] == lineage['accumulated_operands'], 'installed full source sequence')
    return dict(passed=True, input_sha256=hashlib.sha256(raw).hexdigest(),
                observed_endpoint_records=9, observed_phase_payloads=11,
                authenticated_selected_phase_count=6, unselected_phase_count=5,
                probe_reseal_authenticated=True, authenticated_reseal_records=len(reseals),
                complete_selected_primitive_count=92, internal_operands=84,
                top_boundary_operands=6, infiltration_operands=2,
                selected_prefix=selected_rows, all_observed_sources=all_rows,
                scope='Complete primitive-source to selected-credit correspondence including canonical probe reseals and selected ending-joint joins; receiver exact arithmetic separately reconstructed; no full scientific/constitutive conservation qualification.')


def main():
    parser=argparse.ArgumentParser()
    parser.add_argument('stdout',type=Path)
    parser.add_argument('output',type=Path)
    args=parser.parse_args()
    try:
        result=reconstruct(args.stdout.read_bytes())
    except (ValueError,KeyError,TypeError,OverflowError,AssertionError) as error:
        result=dict(passed=False,error=f'{type(error).__name__}: {error}')
    result['input']=str(args.stdout)
    args.output.write_text(json.dumps(result,indent=2)+'\n')
    print(json.dumps({key:value for key,value in result.items() if key not in ('selected_prefix','all_observed_sources')}))
    raise SystemExit(0 if result['passed'] else 1)


if __name__=='__main__':
    main()
