#!/usr/bin/env python3
"""Independently reconstruct native drain exact energy from primitive receipts."""
import argparse
from fractions import Fraction
import hashlib
import json
from pathlib import Path


def dyadic(wire):
    return wire['sign'] * int(wire['coefficient_hex'], 16) * Fraction(2) ** wire['exponent2']


def key(record):
    return json.dumps(record['surface_key'], sort_keys=True)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('log', type=Path)
    parser.add_argument('output', type=Path)
    args = parser.parse_args()
    rows = [json.loads(line.split('NATIVE_DRAIN_EXACT_OPERANDS=', 1)[1])
            for line in args.log.read_text().splitlines()
            if 'NATIVE_DRAIN_EXACT_OPERANDS=' in line]
    assert len(rows) == 1, 'exactly one executed vector required'
    row = rows[0]
    ending = {key(record): record for record in row['ending']['records']}
    evidence = []
    for begin in row['beginning']['records']:
        end = ending.pop(key(begin))
        surface = begin['surface_key']
        credits = [receipt['credited_enthalpy_j_m2_recipient_tile_ground']
                   for receipt in row['receipts']
                   if receipt['disposition'] == 'retained_surface'
                   and receipt['recipient_ofe_id'] == surface['ofe_id']
                   and receipt['recipient_tile_id'] == surface['tile_id']]
        assert all(value is not None for value in credits)
        initial = Fraction(begin['enthalpy_hi_j_m2_tile']) + dyadic(begin['enthalpy_carry'])
        final = Fraction(end['enthalpy_hi_j_m2_tile']) + dyadic(end['enthalpy_carry'])
        residual = final - initial - sum(map(Fraction, credits), Fraction())
        assert residual == 0, (surface, residual)
        evidence.append({'surface_key': surface, 'primitive_count': len(credits),
                         'exact_residual_j_m2': str(residual),
                         'ending_carry_j_m2': str(dyadic(end['enthalpy_carry']))})
    assert not ending, 'foreign ending records'
    args.output.write_text(json.dumps({'evidence': 'Ran: independent Fraction reconstruction',
        'log_sha256': hashlib.sha256(args.log.read_bytes()).hexdigest(),
        'stores': evidence, 'scope': 'isolated native allocation vector, not coupled cycle'}, indent=2)+'\n')
    print(json.dumps({'status':'PASS','stores':len(evidence)}))


if __name__ == '__main__':
    main()
