"""Reconstruct the native soil trial-to-installed transformation and seals.

Source recipes: LSE owner_envelope.rs/exact_dyadic_enthalpy.rs, orchestrator
v9_real_consumer_shadow_soil_thermal.rs. No trial identity is normalized away:
the prescribed transaction substitution is constructed and its hash recomputed.
This does not certify restart execution or constitutive conservation.
"""
import argparse
import copy
import hashlib
import json
from pathlib import Path

from reconstruct_credits import read_records, require, LAYERS
from source_digest_helpers import digest_json


def ordered(value, names):
    names = names.split()
    require(set(value) == set(names), 'canonical field inventory: ' + ' '.join(names))
    return {k: value[k] for k in names}


def dyadic(value):
    return ordered(value, 'sign coefficient_hex exponent2')


def operand(value):
    return ordered(value, 'ofe_id layer_id source_kind source_owner_id debit_credit_identity_sha256 ordinal units basis energy_j_m2_ofe_ground')


def credit(value):
    result = ordered(value, 'ofe_id layer_id beginning_enthalpy_hi_j_m2_ofe_ground beginning_enthalpy_carry beginning_temperature_k ending_enthalpy_hi_j_m2_ofe_ground ending_enthalpy_carry ending_temperature_k heat_capacity_j_m2_k accepted_operands')
    for name in ('beginning_enthalpy_carry', 'ending_enthalpy_carry'):
        result[name] = dyadic(result[name])
    result['accepted_operands'] = [operand(op) for op in result['accepted_operands']]
    return result


def state_digest(state, envelope):
    ordered(state, 'owner_id configuration_sha256 state_sha256 last_accepted_transaction_id ofes')
    body = {k: envelope[k] for k in ('owner_tag', 'schema_sha256', 'exact_carry_definition_sha256')}
    body.update({k: state[k] for k in ('owner_id', 'configuration_sha256', 'last_accepted_transaction_id')})
    ofes = []
    for ofe in state['ofes']:
        ordered(ofe, 'ofe_id ordered_layers')
        layers = []
        for layer in ofe['ordered_layers']:
            row = ordered(layer, 'layer_id temperature_k enthalpy_hi_j_m2_ofe_ground enthalpy_carry last_accepted_transaction_id')
            row['enthalpy_carry'] = dyadic(row['enthalpy_carry'])
            layers.append(row)
        ofes.append(dict(ofe_id=ofe['ofe_id'], ordered_layers=layers))
    body['ofes'] = ofes
    return digest_json(body)


def reconstruct(raw):
    records = read_records(raw)
    lineage, installed = records['B01_INTEGRATED_LINEAGE'], records['B01_INTEGRATED_INSTALLED']
    original = lineage['original_prepared_owner']
    trial = lineage['physical_trial']
    selected = trial['selected_ending_state']
    owner = installed['soil_owner']
    latest = installed['soil_latest_accepted']
    ordered(latest, 'credit_receipt expected_sources predecessor seals')
    envelope_fields = 'owner_tag schema_sha256 exact_carry_definition_sha256 parent_v1_state_sha256 contract_version model_version model_definition_sha256 run_id transaction_id expected_predecessor_transaction_id support_start_ns support_end_ns receipt_chain_sha256 state'
    ordered(original, envelope_fields)
    ordered(owner, envelope_fields)
    require(latest['predecessor'] == original, 'installed exact original prepared predecessor')
    require(original['transaction_id'] == owner['transaction_id'] == 41 and trial['transaction_id'] == 47,
            'distinct original/install41 and selected47 namespaces')
    require(selected['last_accepted_transaction_id'] == 47, 'selected trial state transaction')
    for state in (original['state'], selected, owner['state']):
        require(state_digest(state, original) == state['state_sha256'], 'canonical original/trial/installed state digest')
        require([o['ofe_id'] for o in state['ofes']] == ['ofe-1'], 'complete installed OFE inventory')
        require([l['layer_id'] for l in state['ofes'][0]['ordered_layers']] == list(LAYERS), 'complete installed layer inventory')
    transformed = copy.deepcopy(selected)
    transformed['last_accepted_transaction_id'] = 41
    for layer in transformed['ofes'][0]['ordered_layers']:
        require(layer['last_accepted_transaction_id'] == 47, 'selected layer transaction')
        layer['last_accepted_transaction_id'] = 41
    transformed['state_sha256'] = state_digest(transformed, original)
    require(transformed == owner['state'], 'complete canonical selected-to-installed state transformation')
    require(owner['state'] != original['state'], 'successful install changes complete soil state')
    expected_owner = copy.deepcopy(original)
    expected_owner['state'] = transformed
    receipt = latest['credit_receipt']
    expected_owner['receipt_chain_sha256'] = receipt['receipt_sha256']
    require(expected_owner == owner, 'complete installed envelope transformation')
    fields = 'receipt_tag schema_sha256 exact_carry_definition_sha256 contract_version model_version model_definition_sha256 configuration_sha256 run_id soil_thermal_owner_id transaction_id predecessor_transaction_id support_start_ns support_end_ns beginning_owner_state_sha256 ending_owner_state_sha256 predecessor_receipt_chain_sha256 layer_credits'
    body = ordered({k:v for k,v in receipt.items() if k != 'receipt_sha256'}, fields)
    body['layer_credits'] = [credit(row) for row in body['layer_credits']]
    require(digest_json(body) == receipt['receipt_sha256'], 'canonical complete credit receipt digest')
    for name in ('schema_sha256', 'exact_carry_definition_sha256', 'contract_version', 'model_version', 'model_definition_sha256', 'run_id'):
        require(receipt[name] == original[name], 'receipt original metadata ' + name)
    require(receipt['configuration_sha256'] == original['state']['configuration_sha256'] and
            receipt['soil_thermal_owner_id'] == original['state']['owner_id'], 'receipt configuration/owner join')
    require(receipt['beginning_owner_state_sha256'] == original['state']['state_sha256'] and
            receipt['ending_owner_state_sha256'] == owner['state']['state_sha256'], 'receipt beginning/ending state join')
    require(receipt['transaction_id'] == 41 and receipt['predecessor_transaction_id'] == original['expected_predecessor_transaction_id'] and
            receipt['predecessor_receipt_chain_sha256'] == original['receipt_chain_sha256'], 'receipt original lineage')
    require((receipt['support_start_ns'],receipt['support_end_ns']) == (original['support_start_ns'],original['support_end_ns']) == (0,420_000_000_000), 'aggregate install support')
    require(receipt['layer_credits'] == [row for group in lineage['ordered_layer_credit_chain'] for row in group], 'installed full ordered credit receipt')
    expected = latest['expected_sources']
    ordered(expected, 'accepted_operands temperature_projections expected_set_sha256')
    require(expected['accepted_operands'] == lineage['accumulated_operands'], 'expected full accumulated source membership')
    projections = [ordered(p, 'ofe_id layer_id heat_capacity_j_m2_k ending_temperature_k') for p in expected['temperature_projections']]
    require([(p['ofe_id'],p['layer_id']) for p in projections] == [('ofe-1',l) for l in LAYERS], 'temperature projection inventory')
    expected_hash = digest_json(['OPENWEPP_SOIL_THERMAL_EXPECTED_ACCEPTED_OPERAND_SET_V2', [operand(op) for op in expected['accepted_operands']], projections])
    require(expected_hash == expected['expected_set_sha256'], 'canonical expected operand/temperature set seal')
    seals = latest['seals']
    ordered(seals, 'restart checkpoint latest_credit_receipt_sha256 expected_operand_set_sha256 orchestrator_seal_sha256')
    common = 'owner_tag schema_sha256 exact_carry_definition_sha256 parent_v1_state_sha256 owner_state_sha256 last_accepted_transaction_id receipt_chain_sha256'.split()
    for kind in ('restart', 'checkpoint'):
        seal = seals[kind]
        for name in common:
            value = owner['state']['state_sha256'] if name == 'owner_state_sha256' else owner['state']['last_accepted_transaction_id'] if name == 'last_accepted_transaction_id' else owner[name]
            require(seal[name] == value, kind + ' full owner lineage')
        require(digest_json(['OPENWEPP_SOIL_THERMAL_OWNER_' + kind.upper() + '_V2'] + [seal[k] for k in common]) == seal[kind+'_sha256'], 'canonical ' + kind + ' digest')
    require(seals['latest_credit_receipt_sha256'] == receipt['receipt_sha256'] and seals['expected_operand_set_sha256'] == expected_hash, 'orchestrator receipt/source joins')
    seal_hash = digest_json(['OPENWEPP_SOIL_THERMAL_ORCHESTRATOR_SEALS_V2',
        ordered(seals['restart'], ' '.join(common) + ' restart_sha256'),
        ordered(seals['checkpoint'], ' '.join(common) + ' checkpoint_sha256'), receipt['receipt_sha256'], expected_hash])
    require(seal_hash == seals['orchestrator_seal_sha256'], 'canonical orchestrator seal')
    return dict(passed=True, input_sha256=hashlib.sha256(raw).hexdigest(), selected_transaction=47, installed_transaction=41,
        complete_physical_inventory_preserved=True, canonical_state_transformation=True, complete_credit_receipt_digest=True,
        expected_operand_set_digest=True, restart_checkpoint_and_orchestrator_seals=True,
        scope='Exact native soil selected-state transformation and installed receipt/seals; no runtime restart, full V3/V4 installation, or scientific qualification claim.')


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('stdout', type=Path)
    parser.add_argument('output', type=Path)
    args = parser.parse_args()
    try:
        result = reconstruct(args.stdout.read_bytes())
    except (ValueError, KeyError, TypeError, OverflowError, AssertionError) as error:
        result = dict(passed=False, error=f'{type(error).__name__}: {error}')
    result['input'] = str(args.stdout)
    args.output.write_text(json.dumps(result, indent=2)+'\n')
    print(json.dumps(result))
    raise SystemExit(0 if result['passed'] else 1)


if __name__ == '__main__':
    main()
