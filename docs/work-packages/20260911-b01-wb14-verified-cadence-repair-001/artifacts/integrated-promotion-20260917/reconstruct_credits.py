"""Independently reconstruct retained binary64 + dyadic receiver credits.

Reads raw producer JSON only. This does not certify donor correspondence,
promotion guards, accepted-parent installation, or full scientific closure.
"""
import argparse
from fractions import Fraction
import hashlib
import json
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


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('stdout', type=Path)
    parser.add_argument('output', type=Path)
    args = parser.parse_args()
    raw = args.stdout.read_bytes()
    rows = []
    tags = []
    for line in raw.decode().splitlines():
        marker = line.find('B01_INTEGRATED_')
        if marker < 0 or ' {' not in line[marker:]:
            continue
        tag, payload = line[marker:].split(' ', 1)
        try:
            value = json.loads(payload)
        except json.JSONDecodeError:
            continue  # Debug audit is preserved, not treated as JSON evidence.
        tags.append(tag)
        for path, credit in walk(value):
            beginning = binary64(credit['beginning_enthalpy_hi_j_m2_ofe_ground']) + carry(credit['beginning_enthalpy_carry'])
            ending = binary64(credit['ending_enthalpy_hi_j_m2_ofe_ground']) + carry(credit['ending_enthalpy_carry'])
            operands = credit['accepted_operands']
            total = beginning + sum((binary64(op['energy_j_m2_ofe_ground']) for op in operands), Fraction(0))
            exact_equal = total == ending
            once_rounded_equal = bits(float(total)) == bits(credit['ending_enthalpy_hi_j_m2_ofe_ground'])
            rows.append({'tag':tag, 'path':path, 'ofe_id':credit['ofe_id'], 'layer_id':credit['layer_id'],
                         'operand_count':len(operands), 'exact_sum_equal':exact_equal,
                         'nearest_even_high_equal':once_rounded_equal,
                         'exact_residual':str(ending-total)})
    result = {'input':str(args.stdout), 'input_sha256':hashlib.sha256(raw).hexdigest(),
              'json_tags':tags, 'credit_rows':rows, 'row_count':len(rows),
              'pass':bool(rows) and all(r['exact_sum_equal'] and r['nearest_even_high_equal'] for r in rows),
              'scope':'Independent rational arithmetic on reported receiver credits only; donor/source lineage and whole-system closure are separate obligations.'}
    args.output.write_text(json.dumps(result, indent=2) + '\n')
    print(json.dumps({'row_count':result['row_count'], 'pass':result['pass']}))
    raise SystemExit(0 if result['pass'] else 1)


if __name__ == '__main__':
    main()
