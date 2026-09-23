"""Offline arithmetic on retained records only; never calls a nonlinear evaluator."""
import collections
from decimal import Decimal as D
import hashlib
import json
import math
from pathlib import Path
import re
import struct

A = Path(__file__).resolve().parent
def read(name):
    return json.loads((A / name).read_text())
def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()
def f(bits):
    return struct.unpack('>d', struct.pack('>Q', bits))[0]
def bits(value):
    return struct.unpack('>Q', struct.pack('>d', value))[0]
def vec(values):
    return list(map(f, values))
def unwrap(value):
    return value[3:-1] if isinstance(value, str) and value.startswith('Ok(') and value.endswith(')') else value
def owner_map(value):
    value = unwrap(value)
    return json.loads(value) if isinstance(value, str) else value
def canonical(value):
    return (json.dumps(value, sort_keys=True, separators=(',', ':'), ensure_ascii=True) + '\n').encode()
def digest(value):
    return hashlib.sha256(value).digest()
def integer(value, width):
    return value.to_bytes(width, 'big')
def frame(domain, fields):
    data = b'OPENWEPP\0' + integer(1, 2) + integer(len(domain), 2) + domain.encode()
    for name, value in fields:
        data += integer(len(name), 2) + name.encode() + integer(len(value), 4) + value
    return digest(data)

j = read('support1-attribution-adjacent-04-record.json')
b = read('support1-attribution-baseline-04-record.json')
run = read('support1-attribution-adjacent-04.json')
p = j['canonical_payload']
accepted = j['accepted_support']
attempt = j['solver_attempt']
observer = attempt['observer']
original = read('original-input-reference.json')['inputs']
before = {k: json.loads(bytes(v)) for k, v in accepted['beginning_owner_bytes'].items()}
ending = {k: json.loads(bytes(v)) for k, v in accepted['ending_owner_bytes'].items()}
checks = {}
def check(name, predicate):
    checks[name] = bool(predicate)
    assert predicate, name

check('same_physical_error_observer_off_on', j['result_error'] == b['result_error'])
check('same_support0_all_retained_operands_off_on', accepted == b['accepted_support'])
check('same_provider_payload_off_on', p == b['canonical_payload'])
check('same_before_snapshot_off_on', j['snapshot_before'] == b['snapshot_before'])
for label, record in [('baseline', b), ('observed', j)]:
    check(label + '_complete_snapshot_rollback', unwrap(record['snapshot_before']) == unwrap(record['snapshot_after']))
    check(label + '_seven_staged_owner_byte_rollback', owner_map(record['staged_owner_bytes_before']) == owner_map(record['staged_owner_bytes_after']) == accepted['ending_owner_bytes'])
    check(label + '_committed_owner_byte_rollback', owner_map(record['beginning_owner_bytes_before']) == owner_map(record['beginning_owner_bytes_after']))
    check(label + '_clock_rollback', record['active_parent_clock_before'] == record['active_parent_clock_after'])
    check(label + '_cursor_retains_accepted_prefix', record['accepted_cursor_before'] == record['accepted_cursor_after'] == 1)

reservoirs = ending['vegetation']['reservoirs']
check('unique_two_occupancy_keys', len(attempt['reservoirs']) == len(reservoirs) == len({r['occupancy_id'] for r in attempt['reservoirs']}) == 2)
for r in attempt['reservoirs']:
    owner = reservoirs[r['occupancy_id']]
    check(r['occupancy_id'] + '_M_H_owner_to_failed_input', r['mass_bits'] == bits(owner['mass_kg_m2']) and r['enthalpy_bits'] == bits(owner['enthalpy_j_m2']))
soil = {x['layer_id']: x for x in ending['soil_thermal']['ofes'][0]['ordered_layers']}
geometry = {x['layer_id']: x for x in original['ground']['soil_nodes']}
check('unique_six_soil_keys', len(soil) == len(attempt['soil_nodes']) == len({n['layer_id'] for n in attempt['soil_nodes']}) == 6)
for n in attempt['soil_nodes']:
    key = n['layer_id']
    check(key + '_staged_temperature_to_failed_input', n['temperature_bits'] == bits(soil[key]['temperature_k']))
    check(key + '_immutable_geometry', all(n[target] == bits(geometry[key][source]) for target, source in [('depth_bits', 'depth_m'), ('conductivity_bits', 'conductivity_w_m_k'), ('heat_capacity_bits', 'heat_capacity_j_m2_k')]))
record = p['forcing_projection']['records'][1]
fields = ['air_temperature_k', 'air_specific_humidity_kg_kg', 'atmospheric_downward_longwave_w_m2', 'top_liquid_rate_kg_m2_tile_s', 'top_liquid_temperature_k']
override = [int(record['overrides'][k], 16) for k in fields]
check('all_five_failed_override_bits', override == attempt['override_bits'])
check('support1_identity', record['ordinal'] == attempt['record_ordinal'] == 1 and [record['start_ns'], record['end_ns']] == ['60000000000', '120000000000'])
check('duration_and_rate_times_duration', f(attempt['duration_s_bits']) == 60 and bits(f(override[3]) * 60) == attempt['top_liquid_amount_bits'])
check('liquid_specific_enthalpy', bits(4218.0 * (f(override[4]) - 273.15)) == attempt['top_liquid_specific_enthalpy_bits'])
column = attempt['prepared_solver_column_inputs']
check('top_level_atmospheric_projection', [column[k] for k in ['air_temperature_bits', 'specific_humidity_bits', 'longwave_bits']] == override[:3])
check('unchanged_pressure_wind_shortwave', column['pressure_bits'] == bits(original['pressure_pa']) and column['reference_wind_bits'] == bits(original['reference_wind_m_s']) and column['shortwave_bits'] == [bits(original['shortwave']['incident_w_m2_tile'][k]) for k in ['direct_vis', 'diffuse_vis', 'direct_nir', 'diffuse_nir']])
zero_overrides = [int(p['forcing_projection']['records'][0]['overrides'][k], 16) for k in fields]
native_expected = [zero_overrides[0], zero_overrides[1], bits(original['pressure_pa']), bits(original['reference_wind_m_s'])] + column['shortwave_bits'] + [zero_overrides[2], bits(f(zero_overrides[3]) * 60)]
check('support0_reached_native_all_ten_forcing_bits', accepted['native_consumed_forcing_bits'] == native_expected)
seed = [override[0]] * 21
for offset, reservoir in zip([0, 6], attempt['reservoirs']):
    seed[offset + 3] = reservoir['mass_bits']
    seed[offset + 4] = reservoir['enthalpy_bits']
    seed[offset + 5] = bits(0.0)
seed[13] = override[1]
seed[14] = bits(original['stage3_lower_boundary']['snow_temperature_k'])
seed[15:] = [n['temperature_bits'] for n in attempt['soil_nodes']]
check('canonical_seed_all_21_bits', seed == observer['seed_coordinates_bits'])
check('complete_capture', observer['status'] == 'Complete' and len(observer['bases']) == 16 and len(observer['linear_solves']) == 16)

# Independently implement the canonical framed provider receipt domains.
imp = p['implementation']
source_root = Path(run['cwd'])
check('provider_implementation_digest', sha(source_root / imp['repository_relative_path']) == imp['sha256'])
payload_digest = digest(canonical(p))
run_id = frame('openwepp-m1-fixed-sequence-run-v1', [('schema_version', integer(1, 4)), ('provider_id', p['provider_id'].encode()), ('calendar', canonical(p['calendar'])), ('original_source_sha256', bytes.fromhex(p['sources'][0]['sha256'])), ('continuous_source_sha256', bytes.fromhex(p['sources'][1]['sha256'])), ('implementation_version', imp['provider_version'].encode()), ('implementation_path', imp['repository_relative_path'].encode()), ('implementation_sha256', bytes.fromhex(imp['sha256'])), ('payload_content_sha256', payload_digest)])
cycle = p['cycle']
calendar = frame('openwepp-m1-fixed-sequence-calendar-provider-v1', [('run_identity', run_id), ('cycle_ordinal', integer(cycle['ordinal'], 4)), ('cycle_id', cycle['id'].encode()), ('initial_phase_records', canonical(cycle['initial_phase_records']))])
joins = {x['field']: x for x in p['forcing_projection']['source_joins']}
check('co2_and_reference_height_source_joins', int(joins['co2_pa']['value'], 16) == bits(original['ca_pa']) and int(joins['reference_height_m']['value'], 16) == bits(original['ground']['open_geometry']['reference_height_m']))
gsi_bits = int(joins['current_gsi']['value'], 16)
check('authenticated_gsi_scalar', gsi_bits == attempt['gsi_bits'] == accepted['gsi_bits'] == bits(1.0))
gsi_receipts = []
for rec in p['forcing_projection']['records'][:30]:
    gsi_receipts.append(frame('openwepp-m1-fixed-sequence-gsi-support-v1', [('run_identity', run_id), ('cycle_ordinal', integer(cycle['ordinal'], 4)), ('cycle_id', cycle['id'].encode()), ('support_ordinal', integer(rec['ordinal'], 4)), ('support_start_ns', integer(int(rec['start_ns']), 16)), ('support_end_ns', integer(int(rec['end_ns']), 16)), ('gsi_bits', integer(gsi_bits, 8)), ('gsi_source_join_content_sha256', digest(canonical(joins['current_gsi'])))]))
check('support0_gsi_receipt', gsi_receipts[0] == bytes(accepted['gsi_receipt']))
check('support1_gsi_receipt_in_actual_parent_state', str(list(gsi_receipts[1])) in j['snapshot_before'])
collection = integer(30, 4) + b''.join(integer(32, 4) + x for x in gsi_receipts)
parent_forcing = frame('openwepp-m1-fixed-sequence-parent-forcing-v1', [('calendar_receipt', calendar), ('full_projection_content_sha256', digest(canonical(p['forcing_projection']))), ('parent_ordinal', integer(0, 4)), ('support_start_ns', integer(0, 16)), ('support_end_ns', integer(1800000000000, 16)), ('record_count', integer(30, 4)), ('ordered_records', canonical(p['forcing_projection']['records'][:30])), ('ordered_gsi_support_receipts', collection)])
check('parent_forcing_receipt_all30_gsi_members', parent_forcing == bytes(accepted['parent_forcing_receipt']))

linear = []
for base, lu in zip(observer['bases'], observer['linear_solves']):
    it = base['iteration']
    check(f'base_{it}_LU_source_correspondence', it == lu['iteration'] and base['normalized_jacobian_bits'] == lu['full_matrix_bits'] and base['rhs_bits'] == lu['full_rhs_bits'] and base['direction_bits'] == lu['full_direction_bits'])
    check(f'base_{it}_row_ids', base['row_identities'] == [f'm1_row_{i}' for i in range(21)])
    units = ['WattsPerSquareMeter'] * 21
    for index in [3, 5, 9, 11]:
        units[index] = 'KilogramsPerSquareMeter'
    for index in [4, 10]:
        units[index] = 'JoulesPerSquareMeter'
    units[13] = 'KilogramsPerSquareMeterSecond'
    units[14:] = ['Kelvin'] * 7
    check(f'base_{it}_row_units', base['row_units'] == units)
    check(f'base_{it}_normalized_raw_ratio', all(bits(f(raw) / f(scale)) == norm and math.isfinite(f(scale)) and f(scale) > 0 for raw, scale, norm in zip(base['raw_residual_bits'], base['normalizer_bits'], base['normalized_residual_bits'])))
    check(f'base_{it}_phase_selection_provenance', not base['corrected_assembly'] and base['natural_selections'] == base['predictor_selections'] == base['final_selections'])
    retained, eliminated = lu['retained_indices'], lu['eliminated_indices']
    check(f'base_{it}_partition', eliminated == [5, 11] and sorted(retained + eliminated) == list(range(21)))
    full, rhs, direction = list(map(vec, lu['full_matrix_bits'])), vec(lu['full_rhs_bits']), vec(lu['full_direction_bits'])
    reduced = list(map(vec, lu['reduced_matrix_bits']))
    check(f'base_{it}_reduced_submatrix', reduced == [[full[i][k] for k in retained] for i in retained])
    check(f'base_{it}_reduced_direction', vec(lu['reduced_direction_bits']) == [direction[i] for i in retained])
    rebuilt_rhs = []
    for i in retained:
        value = rhs[i]
        for k in eliminated:
            value -= full[i][k] * direction[k]
        rebuilt_rhs.append(value)
    check(f'base_{it}_reduced_rhs', list(map(bits, rebuilt_rhs)) == lu['reduced_rhs_bits'])
    residuals = [math.fsum(x * y for x, y in zip(row, direction)) - value for row, value in zip(full, rhs)]
    scale = max(abs(value) + math.fsum(abs(x * y) for x, y in zip(row, direction)) for row, value in zip(full, rhs))
    linear.append({'iteration': it, 'full_dimension': len(full), 'reduced_dimension': len(reduced), 'max_absolute_A_d_minus_rhs': max(map(abs, residuals)), 'component_scaled_backward_error': max(map(abs, residuals)) / scale})

bases = {x['iteration']: x for x in observer['bases']}
for t in observer['trials']:
    base = bases[t['iteration']]
    factor = f(t['factor_bits'])
    current, direction = vec(base['coordinates_bits']), vec(base['direction_bits'])
    expected = [x + factor * d for x, d in zip(current, direction)]
    for k in [5, 11]:
        expected[k] = (1.0 - factor) * current[k]
    check(f"trial_{t['iteration']}_{t['exponent']}_applied_coordinates", bits(factor) == bits(0.5 ** t['exponent']) and list(map(bits, expected)) == t['coordinates_bits'])
terminal = [t for t in observer['trials'] if t['iteration'] == 15]
check('terminal_all_21_authorized_factors', [t['exponent'] for t in terminal] == list(range(21)))
check('terminal_all_pre_evaluator_upper_shade_floor', all(t['reason'].startswith('pre_evaluator_domain:Some("temperature[1]=') and f(t['coordinates_bits'][1]) < 263.15 and t['raw_residual_bits'] is None and t['normalized_residual_bits'] is None for t in terminal))
reason_counts = collections.Counter(t['reason'].split(':', 1)[0] for t in observer['trials'])
check('accepted_and_rejected_search_counts', reason_counts == {'pre_evaluator_domain': 137, 'accepted_strict_decrease': 15})

# New independent reservoir and mineral-N arithmetic from primitive amounts.
reservoir_balance = []
for idx, body in enumerate(re.findall(r'M1ReservoirOperands \{ (.*?) \}', accepted['reservoir_operands'])):
    numeric = dict(re.findall(r'(\w+): (?:Some\()?([-+\d.e]+)', body))
    x = {key: D(value) for key, value in numeric.items()}
    mass = x['beginning_mass_kg_m2'] + x['dt_s'] * (x['incident_liquid_kg_m2_s'] - x['vapor_kg_m2_s'] - x['drainage_kg_m2_s'])
    temp = x['wet_temperature_k'] - D('273.15')
    heat = x['beginning_enthalpy_j_m2'] + x['dt_s'] * (x['non_vapor_heat_w_m2'] + x['incident_liquid_kg_m2_s'] * x['incident_enthalpy_j_kg'] - x['vapor_kg_m2_s'] * (D('2501000') + D('1849') * temp) - x['drainage_kg_m2_s'] * D('4218') * temp)
    reservoir_balance.append({'occupancy_id': attempt['reservoirs'][idx]['occupancy_id'], 'primitive_inputs': numeric, 'mass_difference_kg_m2': str(x['ending_mass_kg_m2'] - mass), 'enthalpy_difference_j_m2': str(x['ending_enthalpy_j_m2'] - heat)})
debits = collections.defaultdict(D)
for owner, layer, source, amount in re.findall(r'owner_id: "([^"]+)".*?layer_id: "([^"]+)".*?source_id: "([^"]+)".*?final_use: ([-+\d.e]+)', accepted['resource_debits']):
    debits[(owner, layer, source)] += D(amount)
nitrogen = []
for layer, pools in before['bgc']['layers'].items():
    for pool, source in [('ammonium_n', 'nh4'), ('nitrate_n', 'no3')]:
        beginning, end = D(str(pools[pool])), D(str(ending['bgc']['layers'][layer][pool]))
        use = debits[('bgc', layer, source)]
        nitrogen.append({'layer': layer, 'species': pool, 'beginning': str(beginning), 'ending': str(end), 'final_use': str(use), 'loss_minus_use': str(beginning - end - use)})

out = {'evidence_class': 'Ran: offline primitive arithmetic and retained-array checks only; no nonlinear calls', 'source_tree_sha256': run['source_tree_sha256'], 'binary_sha256': run['binary_sha256'], 'raw_stderr_sha256': sha(A / 'support1-attribution-adjacent-04.stderr'), 'script_sha256': sha(Path(__file__)), 'checks': checks, 'check_count': len(checks), 'provider': {'implementation_sha256': imp['sha256'], 'payload_sha256': payload_digest.hex(), 'run_identity': run_id.hex(), 'calendar_receipt': calendar.hex(), 'support0_gsi_receipt': gsi_receipts[0].hex(), 'support1_prepared_gsi_receipt': gsi_receipts[1].hex(), 'parent_forcing_receipt': parent_forcing.hex()}, 'keyed_failed_reservoirs': attempt['reservoirs'], 'keyed_failed_soil': attempt['soil_nodes'], 'native': {'support0_consumed_forcing_bits': accepted['native_consumed_forcing_bits'], 'support0_positive_surface_liquid_receiver': accepted['native_receiver_is_positive'], 'support0_surface_liquid_receipt': accepted['native_receiver_receipt'], 'support1_status': 'solver preparation reached; native receiver unconsumed'}, 'search': {'bases': len(bases), 'trials': len(observer['trials']), 'reason_counts': dict(reason_counts), 'reported_backtracks_before_terminal_search': 116, 'terminal_rejected_trials': 21, 'terminal_iteration': 15, 'current_upper_shade_temperature_k': f(bases[15]['coordinates_bits'][1]), 'upper_shade_direction_k': f(bases[15]['direction_bits'][1]), 'smallest_factor': f(terminal[-1]['factor_bits']), 'smallest_factor_upper_shade_temperature_k': f(terminal[-1]['coordinates_bits'][1]), 'lower_bound_k': 263.15, 'terminal_max_normalized_residual': max(map(abs, vec(bases[15]['normalized_residual_bits']))), 'terminal_selection': bases[15]['final_selections'], 'terminal_trials': [{'exponent': t['exponent'], 'factor': f(t['factor_bits']), 'upper_shade_temperature_k': f(t['coordinates_bits'][1]), 'reason': t['reason']} for t in terminal]}, 'linear_reconstruction': linear, 'reservoir_balance': reservoir_balance, 'mineral_n_balance': nitrogen, 'limits': ['Dynamic core scales were not independently recomputed; retained raw/normalizer ratios are checked exactly.', 'A linear residual reconstruction is not an additional nonlinear evaluation or convergence proof.', 'No whole-parent, full material/BGC, whole-system water, cycles, restart or cost acceptance.', 'Source-conforming bounded refusal does not prove root nonexistence.']}
(A / 'support1-attribution-independent-check-04.json').write_text(json.dumps(out, indent=2) + '\n')
print(json.dumps({'checks_passed': len(checks), 'terminal': out['search'], 'max_linear_backward_error': max(x['component_scaled_backward_error'] for x in linear)}, indent=2))
