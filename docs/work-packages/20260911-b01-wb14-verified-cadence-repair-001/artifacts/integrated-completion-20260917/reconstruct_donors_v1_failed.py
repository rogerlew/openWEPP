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
from source_digest_helpers import physical_sources, trial_receipt_digest

PHASE_TAG = 'B01_INTEGRATED_ACCEPTED_PHASE_CHAIN'
NS = 1_000_000_000
# Prospective inventory for the unchanged fixture's next affected positive run,
# established from v8 observer reach. All trial records remain accountable.
ENDPOINT_SUPPORT_INVENTORY = [
    [(0, 60)], [(60, 120)], [(120, 180)], [(60, 120)], [(120, 180)],
    [(180, 240), (240, 300), (300, 360)], [(180, 240)], [(240, 300)], [(300, 360)],
]


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
            records.append(value)
    require(len(records) == 9, 'required nine endpoint phase-chain observations')
    observed = [[(int(p['support']['start_ns']) // NS, int(p['support']['end_ns']) // NS)
                 for p in r['selected_phase_chain']] for r in records]
    require(observed == ENDPOINT_SUPPORT_INVENTORY, 'complete endpoint phase-record inventory/order')
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
    by_receipt = {}
    all_rows = []
    for phase in phases:
        require(set(phase['top_boundary_credits_by_lane']) == {'1'}, 'single-lane credit inventory')
        require(set(phase['terminal_snow_soil_trial_receipts_by_lane']) == {'1'}, 'single-lane trial inventory')
        trial = phase['terminal_snow_soil_trial_receipts_by_lane']['1']
        credit = phase['top_boundary_credits_by_lane']['1']
        digest = trial_receipt_digest(trial)
        require(digest not in by_receipt, 'duplicate primitive trial receipt')
        require(trial['canonical_source_sha256'] == phase['probe_child_receipt_sha256'], 'producer phase/source receipt identity')
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
        all_rows.append(dict(receipt_sha256=digest, support_ns=[start, end], source_digest_verified=True))
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
                                  top_source_receipt_sha256=digest, internal_operands=12, top_operands=1))
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
                complete_selected_primitive_count=92, internal_operands=84,
                top_boundary_operands=6, infiltration_operands=2,
                selected_prefix=selected_rows, all_observed_sources=all_rows,
                scope='Complete primitive-source to selected-credit correspondence; receiver exact arithmetic separately reconstructed; no full scientific/constitutive conservation qualification.')


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
