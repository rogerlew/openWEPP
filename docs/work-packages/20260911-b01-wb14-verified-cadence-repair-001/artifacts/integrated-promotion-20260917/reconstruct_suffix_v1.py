"""Independent selected-child physical operand and installed-state comparison."""
import argparse
from collections import Counter
import hashlib
import json
from pathlib import Path
import struct


def f64(bits):
    return struct.unpack('>d', int(bits).to_bytes(8, 'big'))[0]


def bits(value):
    return struct.pack('>d', float(value)).hex()


def key(ofe, layer, kind, value):
    return ofe, layer, kind, bits(value)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('stdout', type=Path)
    parser.add_argument('output', type=Path)
    args = parser.parse_args()
    raw = args.stdout.read_bytes()
    records = {}
    for line in raw.decode().splitlines():
        for tag in ('B01_INTEGRATED_LINEAGE', 'B01_INTEGRATED_INSTALLED'):
            if tag + ' {' in line:
                records.setdefault(tag, []).append(json.loads(line.split(tag + ' ', 1)[1]))
    assert len(records['B01_INTEGRATED_LINEAGE']) == len(records['B01_INTEGRATED_INSTALLED']) == 1
    lineage = records['B01_INTEGRATED_LINEAGE'][0]
    installed = records['B01_INTEGRATED_INSTALLED'][0]
    expected = []
    for tile in lineage['pre_ingress_soil_thermal']:
        for layer in tile['layers']:
            assert layer['infiltration_enthalpy_credit_bits'] == 0
            delta = f64(layer['ending_enthalpy_bits']) - f64(layer['beginning_enthalpy_bits'])
            expected.append(key(tile['ofe_id'], layer['layer_id'], 'soil_internal', delta))
    internal_count = len(expected)
    for receipt in lineage['surface_ingress_receipts']:
        if receipt['disposition'] == 'infiltration':
            recipient = receipt['recipient']
            assert recipient['recipient_kind'] == 'soil_infiltration'
            assert recipient['ofe_id'] == receipt['recipient_store_key']['ofe_id']
            expected.append(key(recipient['ofe_id'], recipient['soil_thermal_layer_id'], 'infiltration', receipt['enthalpy_j_m2_basis_ofe_ground']))
    trial = lineage['physical_trial']
    actual = [key(op['ofe_id'], op['layer_id'], op['source_kind'], op['energy_j_m2_ofe_ground']) for row in trial['layer_credits'] for op in row['accepted_operands']]
    suffix_equal = Counter(expected) == Counter(actual)
    latest = installed['soil_latest_accepted']
    owner = installed['soil_owner']
    receipt = latest['credit_receipt']
    source_equal = latest['expected_sources'] == lineage['accumulated_operands']
    selected_equal = owner['state'] == trial['selected_ending_state']
    receipt_owner_equal = receipt['ending_owner_state_sha256'] == owner['state']['state_sha256']
    support_equal = (owner['support_start_ns'], owner['support_end_ns']) == (receipt['support_start_ns'], receipt['support_end_ns'])
    wb14 = json.loads(bytes(lineage['wb14_child_replay_bytes']))
    child_receipts = []
    def collect(value):
        if isinstance(value, dict):
            if 'ordinal' in value and 'support_start_ns' in value and 'accepted_coupled_slab_sha256' in value:
                child_receipts.append({k:value[k] for k in ('ordinal','support_start_ns','support_end_ns')})
            for child in value.values():collect(child)
        elif isinstance(value, list):
            for child in value:collect(child)
    collect(wb14)
    first_physical = bool(child_receipts) and all(row['ordinal'] == 0 and row['support_start_ns'] == trial['support_start_ns'] and row['support_end_ns'] == trial['support_end_ns'] for row in child_receipts)
    result = {'input':str(args.stdout), 'input_sha256':hashlib.sha256(raw).hexdigest(),
              'soil_internal_operands':internal_count, 'infiltration_operands':len(expected)-internal_count,
              'reported_selected_operands':len(actual), 'selected_suffix_energy_bits_equal':suffix_equal,
              'accumulated_operand_count':len(lineage['accumulated_operands']),
              'credit_chain_groups':len(lineage['ordered_layer_credit_chain']),
              'installed_expected_sources_equal_accumulated':source_equal,
              'installed_state_equal_selected_ending':selected_equal,
              'receipt_ending_equal_installed_state':receipt_owner_equal, 'owner_receipt_support_equal':support_equal,
              'installed_support_ns':[owner['support_start_ns'],owner['support_end_ns']],
              'installed_transaction_id':owner['transaction_id'],
              'selected_trial_support_ns':[trial['support_start_ns'],trial['support_end_ns']],
              'selected_trial_transaction_id':trial['transaction_id'],
              'wb14_first_physical_receipts':child_receipts, 'first_physical_child_support_equal':first_physical,
              'wb14_parent_replay_present':lineage['wb14_parent_replay_bytes'] is not None,
              'pass':all((suffix_equal,source_equal,selected_equal,receipt_owner_equal,support_equal,first_physical)),
              'limits':['Only selected child physical primitives are available; prior covered-prefix donor outputs are absent.',
                        'Numerical/group-key suffix equality does not independently authenticate every debit-credit source digest.',
                        'This partial-child replay is not parent-final WB14 replay, negative coverage, rollback, or full qualification.']}
    args.output.write_text(json.dumps(result,indent=2)+'\n')
    print(json.dumps(result))
    raise SystemExit(0 if result['pass'] else 1)


if __name__ == '__main__':
    main()
