"""Independently reconstruct retained binary64 + dyadic receiver credits.

Reads raw producer JSON only. This does not certify donor correspondence,
promotion guards, accepted-parent installation, or full scientific closure.
"""
import argparse
from fractions import Fraction
import hashlib
import json
import math
import re
from collections import Counter, defaultdict
from pathlib import Path
import struct


def binary64(value):
    return Fraction.from_float(float(value))


def carry(value):
    coefficient = int(value['coefficient_hex'], 16)
    sign, exponent = value['sign'], value['exponent2']
    assert sign in (-1, 0, 1)
    if coefficient == 0:
        assert (sign, value['coefficient_hex'], exponent) == (0, '0', 0)
        return Fraction(0)
    assert sign != 0 and coefficient % 2 == 1
    assert value['coefficient_hex'] == format(coefficient, 'x')
    return sign * coefficient * (Fraction(2) ** exponent)


def bits(value):
    return struct.pack('>d', float(value)).hex()


def walk(value, path='$'):
    if isinstance(value, dict):
        if 'accepted_operands' in value and 'beginning_enthalpy_hi_j_m2_ofe_ground' in value:
            yield path, value
        for key, child in value.items():
            yield from walk(child, path + '.' + key)
    elif isinstance(value, list):
        for index, child in enumerate(value):
            yield from walk(child, f'{path}[{index}]')


# Frozen native fixture inventory, not a count inferred from a successful parse.
LAYERS = ("thermal-1", "thermal-2", "soil-1", "soil-2", "soil-dry", "soil-frozen")
REQUIRED_TAGS = ("B01_INTEGRATED_LINEAGE", "B01_INTEGRATED_INSTALLED")


def unique_object(pairs):
    value = {}
    for key, child in pairs:
        if key in value:
            raise ValueError(f"duplicate JSON key: {key}")
        value[key] = child
    return value


def reject_constant(value):
    raise ValueError(f"nonfinite JSON constant: {value}")


def read_records(raw, required=REQUIRED_TAGS):
    records = {tag: [] for tag in required}
    for number, line in enumerate(raw.decode().splitlines(), 1):
        for tag in required:
            # Libtest may prepend its test name to the first emitted record.
            marker = re.search(r"(?<![A-Z0-9_])" + re.escape(tag) + r"(?=\s|$)", line)
            if marker:
                payload = line[marker.end():].strip()
                value = json.loads(payload, object_pairs_hook=unique_object,
                                   parse_constant=reject_constant)
                if not isinstance(value, dict):
                    raise ValueError(f"{tag} line {number}: expected object")
                records[tag].append(value)
    for tag, values in records.items():
        if len(values) != 1:
            raise ValueError(f"{tag}: expected exactly one record, got {len(values)}")
    return {tag: values[0] for tag, values in records.items()}


def require(condition, message):
    if not condition:
        raise ValueError(message)


def reconstruct(raw):
    records = read_records(raw)
    lineage = records[REQUIRED_TAGS[0]]
    installed = records[REQUIRED_TAGS[1]]
    chain = lineage['ordered_layer_credit_chain']
    require(len(chain) == 7, 'required seven selected credit groups')
    expected_members = []
    for i in range(7):
        for j, layer in enumerate(LAYERS):
            expected_members.append((REQUIRED_TAGS[0],
                f'$.ordered_layer_credit_chain[{i}][{j}]', 'ofe-1', layer))
    for j, layer in enumerate(LAYERS):
        expected_members.append((REQUIRED_TAGS[0],
            f'$.physical_trial.layer_credits[{j}]', 'ofe-1', layer))
    for i in range(42):
        expected_members.append((REQUIRED_TAGS[1],
            f'$.soil_latest_accepted.credit_receipt.layer_credits[{i}]',
            'ofe-1', LAYERS[i % 6]))
    rows, members = [], []
    for tag, value in records.items():
        for path, credit in walk(value):
            members.append((tag, path, credit['ofe_id'], credit['layer_id']))
            beginning = binary64(credit['beginning_enthalpy_hi_j_m2_ofe_ground']) + carry(credit['beginning_enthalpy_carry'])
            ending = binary64(credit['ending_enthalpy_hi_j_m2_ofe_ground']) + carry(credit['ending_enthalpy_carry'])
            operands = credit['accepted_operands']
            for op in operands:
                require((op['ofe_id'], op['layer_id']) == (credit['ofe_id'], credit['layer_id']), 'operand layer membership')
                require(op['basis'] == 'ofe_ground' and op['units'] == 'J m^-2 OFE-ground', 'operand units/basis')
                require(math.isfinite(op['energy_j_m2_ofe_ground']), 'nonfinite operand')
            total = beginning + sum((binary64(op['energy_j_m2_ofe_ground']) for op in operands), Fraction(0))
            require(total == ending, f'exact receiver total: {tag}{path}')
            require(bits(float(total)) == bits(credit['ending_enthalpy_hi_j_m2_ofe_ground']), f'nearest-even high: {tag}{path}')
            rows.append(dict(tag=tag, path=path, ofe_id=credit['ofe_id'], layer_id=credit['layer_id'],
                             operand_count=len(operands), exact_sum_equal=True,
                             nearest_even_high_equal=True, exact_residual='0'))
    require(Counter(members) == Counter(expected_members), 'complete 90-row path/layer inventory mismatch')
    require(all(count == 1 for count in Counter(members).values()), 'duplicate receiver row')
    # Exact receipt chain is ordered child-major; accumulated source order is
    # independently carried and must not be replaced by a layer-major flatten.
    receipt_rows = installed['soil_latest_accepted']['credit_receipt']['layer_credits']
    require(receipt_rows == [row for group in chain for row in group], 'installed exact credit-chain order')
    require(chain[-1] == lineage['physical_trial']['layer_credits'], 'selected child versus chain tail')
    for previous, following in zip(chain, chain[1:]):
        require(len(previous) == len(following) == 6, 'credit group inventory')
        for a, b in zip(previous, following):
            require(bits(a['ending_enthalpy_hi_j_m2_ofe_ground']) == bits(b['beginning_enthalpy_hi_j_m2_ofe_ground'])
                    and a['ending_enthalpy_carry'] == b['beginning_enthalpy_carry']
                    and bits(a['ending_temperature_k']) == bits(b['beginning_temperature_k']), 'credit-chain physical continuity')
    accumulated = lineage['accumulated_operands']
    require(len(accumulated) == 92, 'required accumulated operand inventory')
    flattened = [op for group in chain for row in group for op in row['accepted_operands']]
    # Source-bound transformation: v10_soil_thermal_v2.rs
    # reordinal_and_canonicalize_v2_operands, then topology/source-kind/ordinal.
    # This reconstructs the documented ordinal change; no identity is erased.
    ordinal = defaultdict(int)
    transformed = []
    for op in flattened:
        key = (op['ofe_id'], op['layer_id'], op['source_kind'])
        transformed.append(dict(op, ordinal=ordinal[key]))
        ordinal[key] += 1
    ranks = {(ofe['ofe_id'], layer['layer_id']): (i, j)
             for i, ofe in enumerate(lineage['original_prepared_owner']['state']['ofes'])
             for j, layer in enumerate(ofe['ordered_layers'])}
    kinds = {'soil_internal': 0, 'top_boundary': 1, 'infiltration': 2}
    transformed.sort(key=lambda op: (ranks[(op['ofe_id'], op['layer_id'])],
                                    kinds[op['source_kind']], op['ordinal']))
    require(transformed == accumulated, 'source-defined accumulated reordinalization/order')
    require(Counter(op['source_kind'] for op in accumulated) ==
            Counter(soil_internal=84, top_boundary=6, infiltration=2), 'required source-kind membership')
    require(installed['soil_latest_accepted']['expected_sources']['accepted_operands'] == accumulated,
            'installed canonical accumulated source sequence')
    return dict(input_sha256=hashlib.sha256(raw).hexdigest(), required_tags=list(REQUIRED_TAGS),
                required_row_count=90, row_count=len(rows), credit_rows=rows,
                complete_required_receiver_inventory=True, ordered_credit_chain_equal=True,
                accumulated_membership_equal=True, installed_source_sequence_equal=True,
                pass_receiver_arithmetic=True,
                scope='Receiver arithmetic and complete selected receiver inventory only; independent donor correspondence and scientific conservation remain separate.')


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('stdout', type=Path)
    parser.add_argument('output', type=Path)
    args = parser.parse_args()
    try:
        result = reconstruct(args.stdout.read_bytes())
        result.update(input=str(args.stdout), passed=True)
    except (ValueError, KeyError, TypeError, OverflowError, AssertionError) as error:
        result = dict(input=str(args.stdout), passed=False, error=f'{type(error).__name__}: {error}')
    args.output.write_text(json.dumps(result, indent=2) + '\n')
    print(json.dumps({key: value for key, value in result.items() if key != 'credit_rows'}))
    raise SystemExit(0 if result['passed'] else 1)


if __name__ == '__main__':
    main()
