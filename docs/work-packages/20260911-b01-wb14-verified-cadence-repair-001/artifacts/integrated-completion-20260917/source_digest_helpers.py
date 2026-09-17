"""Independent source-digest reconstruction for the unchanged NativeMixedPhase fixture.

Canonical serializer layouts are read from real_hydrology_execution.rs and
physical_outcome_ledger.rs. This compares physical producer outputs and identity;
it is not an independent constitutive solver or whole-system conservation proof.
"""
from collections import defaultdict
import hashlib
import json
import struct

from reconstruct_credits import LAYERS, require, bits


def float_bits(value):
    return struct.unpack('>d', int(value).to_bytes(8, 'big'))[0]


def digest_json(value):
    # All reconstructed tuples are checked against source digests. If Python and
    # Rust encode a number differently, fail; never normalize the digest to pass.
    return hashlib.sha256(json.dumps(value, separators=(',', ':'), ensure_ascii=False,
                                     allow_nan=False).encode()).hexdigest()


def ordered_receipt(receipt):
    fields = 'parcel_id source_parcel_id transaction_id origin_store_key recipient_store_key recipient basis_ofe_id kind disposition start_s end_s mass_kg_m2_basis_ofe_ground temperature_k enthalpy_j_m2_basis_ofe_ground'.split()
    store = 'run_id ofe_id tile_id surface_id surface_class source_type source_id'.split()
    recipient = 'recipient_kind ofe_id production_lane_index production_lane_id ordered_soil_layer_ids soil_thermal_layer_id'.split()
    require(set(receipt) == set(fields), 'infiltration receipt field inventory')
    result = {key: receipt[key] for key in fields}
    for key in ('origin_store_key', 'recipient_store_key'):
        require(set(receipt[key]) == set(store), 'store-key field inventory')
        result[key] = {name: receipt[key][name] for name in store}
    require(set(receipt['recipient']) == set(recipient), 'infiltration recipient field inventory')
    result['recipient'] = {name: receipt['recipient'][name] for name in recipient}
    return result


def physical_sources(record, source_transaction, start, end):
    """Rebuild donor energies and full digest-bearing tuples, before reordinalization."""
    tiles = sorted(record['pre_ingress_soil_thermal'], key=lambda t: (t['ofe_id'], t['tile_id']))
    require([(t['ofe_id'], t['tile_id']) for t in tiles] == [('ofe-1', 'forest'), ('ofe-1', 'open')],
            'required two selected tile producer payloads')
    ordinals = defaultdict(int)
    operands = []
    target_transaction = None
    for tile in tiles:
        identity = tile['beginning_identity']['V2']
        require((identity['support_start_ns'], identity['support_end_ns']) == (start, end), 'producer support')
        if target_transaction is None:
            target_transaction = identity['transaction_id']
        require(identity['transaction_id'] == target_transaction, 'producer soil transaction')
        require([layer['layer_id'] for layer in tile['layers']] == list(LAYERS), 'producer layer membership/order')
        for layer in tile['layers']:
            require(layer['infiltration_enthalpy_credit_bits'] == 0, 'pre-ingress excludes infiltration')
            beginning = float_bits(layer['beginning_enthalpy_bits'])
            ending = float_bits(layer['ending_enthalpy_bits'])
            energy = ending - beginning  # Physical producer candidate, not receiver storage.
            key = (tile['ofe_id'], layer['layer_id'])
            ordinal = ordinals[key]
            ordinals[key] += 1
            preimage = ['OPENWEPP_ACCEPTED_SOIL_INTERNAL_ENERGY_V2_TX_SPLIT_V1',
                        ['source_transaction_id', source_transaction],
                        ['soil_thermal_transaction_id', target_transaction],
                        start, end, 'land-surface-energy-v1', tile['owner_id'],
                        tile['beginning_state_sha256'], tile['ofe_id'], tile['tile_id'],
                        layer['layer_id'], ordinal, beginning, ending, energy]
            operands.append(dict(ofe_id=tile['ofe_id'], layer_id=layer['layer_id'],
                source_kind='soil_internal', source_owner_id='land-surface-energy-v1',
                debit_credit_identity_sha256=digest_json(preimage), ordinal=ordinal,
                units='J m^-2 OFE-ground', basis='ofe_ground', energy_j_m2_ofe_ground=energy))
    ordinals.clear()
    for receipt in record['surface_ingress_receipts']:
        if receipt['disposition'] != 'infiltration':
            continue
        require(receipt['transaction_id'] == source_transaction, 'infiltration source transaction')
        recipient = receipt['recipient']
        require(recipient['recipient_kind'] == 'soil_infiltration', 'infiltration branch')
        require(recipient['ofe_id'] == receipt['recipient_store_key']['ofe_id'] == receipt['basis_ofe_id'], 'infiltration OFE/basis')
        key = recipient['ofe_id'], recipient['soil_thermal_layer_id']
        ordinal = ordinals[key]
        ordinals[key] += 1
        preimage = ['OPENWEPP_ACCEPTED_SOIL_INFILTRATION_ENERGY_V2_TX_SPLIT_V1',
                    ['source_transaction_id', source_transaction],
                    ['soil_thermal_transaction_id', target_transaction], start, end,
                    'production-hydrology', ordered_receipt(receipt), key[1], ordinal]
        operands.append(dict(ofe_id=key[0], layer_id=key[1], source_kind='infiltration',
            source_owner_id='production-hydrology', debit_credit_identity_sha256=digest_json(preimage),
            ordinal=ordinal, units='J m^-2 OFE-ground', basis='ofe_ground',
            energy_j_m2_ofe_ground=receipt['enthalpy_j_m2_basis_ofe_ground']))
    return operands


def trial_receipt_digest(receipt):
    """Rebuild canonical binary source receipt without receiver state or formulas."""
    require(receipt['experimental_bulk'] is None, 'unchanged fixture has no experimental bulk receipt')
    start, end = int(receipt['support']['start_ns']), int(receipt['support']['end_ns'])
    require(start < end, 'trial receipt support')
    ofe = receipt['ofe_id'].encode()
    raw = bytearray(b'OPENWEPP_TERMINAL_SNOW_SOIL_TRIAL_RECEIPT_V1')
    raw += start.to_bytes(16, 'big') + end.to_bytes(16, 'big')
    raw += receipt['lane_id'].to_bytes(4, 'big')
    raw += len(ofe).to_bytes(8, 'little') + ofe
    raw += bytes.fromhex(receipt['canonical_source_sha256'])
    for field in ('beginning_snow_temperature_k', 'ending_snow_temperature_k',
                  'beginning_soil_temperature_k', 'ending_soil_temperature_k',
                  'snow_heat_j_m2', 'soil_heat_j_m2'):
        raw += struct.pack('>d', receipt[field])
    raw += bytes.fromhex(receipt['ending_soil_candidate_sha256'])
    require(bits(receipt['snow_heat_j_m2']) == bits(-receipt['soil_heat_j_m2']), 'source equal/opposite transfer')
    digest = hashlib.sha256(raw).hexdigest()
    require(digest == receipt['receipt_sha256'], 'source trial receipt canonical digest')
    return digest


def framed_digest(domain, fields):
    domain = domain.encode()
    raw = bytearray(b'OPENWEPP\0' + (1).to_bytes(2, 'big') + len(domain).to_bytes(2, 'big') + domain)
    for tag, value in fields:
        tag = tag.encode()
        raw += len(tag).to_bytes(2, 'big') + tag + len(value).to_bytes(4, 'big') + value
    return hashlib.sha256(raw).hexdigest()


def probe_child_digest(child):
    """CoveredProbeChildIdentityV1's complete canonical framed identity."""
    uint = lambda value, width: int(value).to_bytes(width, 'big')
    digest = lambda name: bytes.fromhex(child[name])
    parent = child['enclosing_parent_support']
    trial = child['trial_support']
    require(int(parent['start_ns']) <= int(trial['start_ns']) < int(trial['end_ns']) <= int(parent['end_ns']),
            'child support containment')
    fields = [
        ('schema', uint(1, 4)),
        ('parent_transaction', digest('parent_transaction_sha256')),
        ('enclosing_support_start', uint(parent['start_ns'], 16)),
        ('enclosing_support_end', uint(parent['end_ns'], 16)),
        ('trial_support_start', uint(trial['start_ns'], 16)),
        ('trial_support_end', uint(trial['end_ns'], 16)),
        ('physical_child_ordinal', uint(child['physical_child_ordinal'], 4)),
        ('attempt', uint(child['attempt_ordinal'], 4)),
        ('role', uint(child['role_u8'], 1)),
        ('beginning_joint', digest('beginning_joint_sha256')),
        ('beginning_owner_set', digest('beginning_owner_set_sha256')),
        ('complete_forcing', digest('complete_forcing_sha256')),
        ('topology', digest('topology_sha256')),
    ]
    result = framed_digest('covered-probe-child-identity', fields)
    require(result == child['receipt_sha256'], 'probe canonical framed receipt digest')
    return result
