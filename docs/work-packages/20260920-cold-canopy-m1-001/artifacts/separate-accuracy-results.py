#!/usr/bin/env python3
"""Summarize retained operations; never execute or retry a physical case."""
import argparse
import importlib.util
import json
from pathlib import Path
import statistics

ROOT = Path(__file__).resolve().parent


def load(path):
    return json.loads(Path(path).read_text())


def module(name, path):
    spec = importlib.util.spec_from_file_location(name, path)
    result = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(result)
    return result


def identifier(cell):
    return json.dumps(cell, sort_keys=True, separators=(',', ':'))


def phase(reservoir):
    mass = reservoir['ending_mass_kg_m2']
    enthalpy = reservoir['ending_enthalpy_j_m2']
    if mass == 0:
        return 'empty'
    if enthalpy < -333700 * mass:
        return 'ice'
    if enthalpy < 0:
        return 'mixed'
    return 'liquid'


def summarize_day(records, verifier):
    if not records:
        return {'status': 'UNRESOLVED', 'reason': 'No accepted support records'}
    reconstruction = verifier.reconstruct_document(records)
    elapsed = 0.0
    integrals = {}
    events = []
    previous = None
    endpoints = []
    for record in records:
        dt = record['interval_s']
        categories = [phase(r) for r in record['reservoirs']]
        categories += [dt * r['drainage_kg_m2_s'] <= r['liquid_capacity_kg_m2'] - r['diagnosed_liquid_mass_kg_m2']
                       for r in record['reservoirs']]
        categories += [r['drainage_kg_m2_s'] > 0 for r in record['reservoirs']]
        if categories != previous:
            events.append({'interval_s': [elapsed, elapsed + dt], 'categories': categories})
        previous = categories
        elapsed += dt
        endpoints.append({'end_s': elapsed, 'coordinates': record['coordinates'],
                          'categories': categories})
        rates = {}
        for occupancy in range(2):
            dry = record['leaf_vapor_kg_m2_s'][occupancy]
            wet = record['wet_vapor_kg_m2_s'][occupancy]
            for component, rate in enumerate(dry):
                rates[f'occupancy{occupancy}_dry{component}_kg_m2'] = rate
            rates[f'occupancy{occupancy}_wet_kg_m2'] = wet
            rates[f'occupancy{occupancy}_ET_kg_m2'] = sum(dry) + wet
        transfer = record['upper_to_lower_intercepted_mass_kg_m2_s']
        rates['upper_to_lower_captured_kg_m2'] = transfer
        rates['upper_to_lower_captured_j_m2'] = transfer * record['upper_to_lower_intercepted_specific_enthalpy_j_kg']
        rates['lower_release_kg_m2'] = record['lower_terminal_release_mass_kg_m2_s']
        for name, rate in rates.items():
            pair = integrals.setdefault(name, {'signed': 0.0, 'gross': 0.0})
            pair['signed'] += dt * rate
            pair['gross'] += dt * abs(rate)
    resolved = [s for s in reconstruction['supports'] if s['status'] == 'RESOLVED']
    row_max = [max((abs(s['reconstructed_residuals'][i] / s['reconstructed_normalizers'][i])
                    for s in resolved), default=None) for i in range(21)]
    return {'status': reconstruction['status'], 'duration_s': elapsed,
            'support_count': len(records), 'endpoints': endpoints,
            'phase_and_drainage_events': events, 'integrals': integrals,
            'independent_reconstruction': reconstruction,
            'maximum_absolute_original_row_over_normalizer': row_max}


def compare(left, right):
    """Compare only shared support endpoints, without invented interpolation."""
    if not left or not right or 'endpoints' not in left or 'endpoints' not in right:
        return {'status': 'UNRESOLVED', 'reason': 'Missing accepted trajectories'}
    reference = {e['end_s']: e for e in right['endpoints']}
    pairs = [(e, reference[e['end_s']]) for e in left['endpoints'] if e['end_s'] in reference]
    if not pairs:
        return {'status': 'UNRESOLVED', 'reason': 'No common support endpoints'}
    delta = [max(abs(a['coordinates'][i] - b['coordinates'][i]) for a, b in pairs) for i in range(21)]
    flux_delta = {name: {mode: value[mode] - right['integrals'][name][mode]
                         for mode in ('signed', 'gross')}
                  for name, value in left['integrals'].items()}
    return {'status': 'COMPUTED_DISCRETE_DIFFERENCE', 'shared_endpoints': len(pairs),
            'maximum_absolute_coordinate_difference': delta,
            'categories_equal_at_shared_endpoints': all(a['categories'] == b['categories'] for a, b in pairs),
            'integral_differences': flux_delta,
            'relative_error_policy': 'Absolute dimensional differences only; no division by near-zero reference.',
            'interpretation': 'Discrete difference; credibility and completion are separate prerequisites.'}


def reference_check(reference_operation, stability_operation, analyses):
    if (not reference_operation or not stability_operation
            or reference_operation['status'] != 'complete'
            or stability_operation['status'] != 'complete'):
        return {'status': 'UNRESOLVED', 'reason': 'Reference or stability day did not complete'}
    left = analyses[reference_operation['output']]
    right = analyses[stability_operation['output']]
    difference = compare(left, right)
    if difference['status'] != 'COMPUTED_DISCRETE_DIFFERENCE':
        return {'status': 'UNRESOLVED', 'reason': 'Reference trajectories have no comparable endpoints',
                'difference': difference}
    if left['status'] != 'RESOLVED' or right['status'] != 'RESOLVED':
        return {'status': 'UNRESOLVED', 'reason': 'Independent original-row/routing reconstruction unresolved',
                'difference': difference}
    thresholds = [1e-7] * 21
    for index, threshold in [(3, 1e-10), (4, 1e-7), (5, 1e-10),
                             (9, 1e-10), (10, 1e-7), (11, 1e-10), (13, 1e-11)]:
        thresholds[index] = threshold
    checks = {
        'complete_same_60s_horizon': left['duration_s'] == right['duration_s'] == 86400
        and left['support_count'] == right['support_count'] == 1440,
        'frozen_state_agreement': all(d <= t for d, t in zip(difference['maximum_absolute_coordinate_difference'], thresholds)),
        'phase_capacity_drainage_categories': difference['categories_equal_at_shared_endpoints'],
        'all_original_rows_admitted': all(value <= 1.0 for day in (left, right)
                                          for value in day['maximum_absolute_original_row_over_normalizer']),
    }
    ledger_max = {'kg_m2': 0.0, 'j_m2': 0.0}
    def compare_ledger(a, b):
        for name, value in a.items():
            if name == 'occupancy':
                continue
            unit = 'kg_m2' if name.endswith('kg_m2') else 'j_m2'
            ledger_max[unit] = max(ledger_max[unit], abs(value - b[name]))
    for a, b in zip(left['independent_reconstruction']['supports'], right['independent_reconstruction']['supports']):
        for occupancy in range(2):
            compare_ledger(a['reservoir_ledger'][occupancy], b['reservoir_ledger'][occupancy])
        compare_ledger(a['transfer_ledger'], b['transfer_ledger'])
    for a, b in zip(left['independent_reconstruction']['cumulative_ledgers'], right['independent_reconstruction']['cumulative_ledgers']):
        for occupancy in range(2):
            compare_ledger(a['reservoirs'][occupancy], b['reservoirs'][occupancy])
        compare_ledger(a['transfer'], b['transfer'])
    checks['per_support_and_cumulative_ledger_agreement'] = ledger_max['kg_m2'] <= 1e-10 and ledger_max['j_m2'] <= 1e-7
    return {'status': 'CREDIBLE_DISCRETE_REFERENCE' if all(checks.values()) else 'UNRESOLVED',
            'checks': checks, 'maximum_ledger_difference': ledger_max, 'difference': difference,
            'scope': 'Frozen 60-second discrete comparison only; no continuous-time or observational truth'}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    if args.output.exists():
        raise RuntimeError('Use a new report path; preserve prior evidence')
    state_path = ROOT / 'separate-accuracy-runner-state.json'
    state = load(state_path) if state_path.exists() else {'operations': {}, 'physical_seconds': 0}
    measure = load(ROOT / 'separate-accuracy-measurement-draft.json')
    verifier = module('independent_reconstruction', ROOT / 'separate-accuracy-independent-reconstruction.py')
    operations = list(state['operations'].values())
    analyses = {}
    for operation in operations:
        if operation['kind'] not in ('reference', 'stability', 'diagnostic') or not operation.get('output'):
            continue
        output_path = operation['output']
        if output_path not in analyses:
            payload = load(output_path).get('payload', {})
            days = payload.get('days', [])
            analyses[output_path] = summarize_day(days[0]['records'], verifier) if days else {'status': 'UNRESOLVED', 'reason': 'No DUMP day'}
    cells = []
    for cell in measure['cell_sequence']:
        matching = [o for o in operations if o['cell'] == cell]
        diagnostic = next((o for o in matching if o['kind'] == 'diagnostic'), None)
        timings = [o for o in matching if o['kind'] == 'timing' and o['status'] == 'complete']
        entry = {'cell': cell, 'diagnostic_status': diagnostic['status'] if diagnostic else 'NOT_RUN',
                 'completed_timing_batches': len(timings), 'accuracy_status': 'UNRESOLVED',
                 'operations': matching}
        if diagnostic and diagnostic.get('output'):
            entry['day_analysis_key'] = diagnostic['output']
        if timings:
            samples = {clock: [o[field] * 1e6 / o['fresh_complete_days'] for o in timings]
                       for clock, field in [('cpu', 'child_process_cpu_seconds'), ('wall', 'sample_wall_seconds')]}
            entry['completed_column_day_us'] = {
                clock: {'samples': values, 'median': statistics.median(values),
                        'minimum': min(values), 'maximum': max(values)}
                for clock, values in samples.items()}
            entry['timing_coverage'] = 'complete' if len(timings) == 6 else 'partial; no full six-batch claim'
        cells.append(entry)
    by_cell = {identifier(c['cell']): c for c in cells}
    references = {}
    for case in dict.fromkeys(c['case'] for c in measure['cell_sequence']):
        ref = next((o for o in operations if o['kind'] == 'reference' and o['cell']['case'] == case), None)
        stable = next((o for o in operations if o['kind'] == 'stability' and o['cell']['case'] == case), None)
        references[case] = reference_check(ref, stable, analyses)
    for entry in cells:
        cell = entry['cell']
        analysis = analyses.get(entry.get('day_analysis_key'))
        ref = next((o for o in operations if o['kind'] == 'reference' and o['cell']['case'] == cell['case']), None)
        entry['strict_reference_difference'] = compare(analysis, analyses.get(ref.get('output')) if ref else None)
        if (entry['diagnostic_status'] == 'complete' and analysis and analysis['status'] == 'RESOLVED'
                and references[cell['case']]['status'] == 'CREDIBLE_DISCRETE_REFERENCE'):
            entry['accuracy_status'] = 'DISCRETE_REFERENCE_DIFFERENCE_AVAILABLE'
        if cell['investigation'] == 'B':
            baseline = by_cell[identifier(dict(cell, investigation='A', cadence_seconds=60))]
            entry['matched_profile_60s_difference'] = compare(analysis, analyses.get(baseline.get('day_analysis_key')))
    report = {'scope': 'Detached column-day diagnostics; no whole-OFE or century runtime claim',
              'physical_seconds': state['physical_seconds'], 'cells': cells, 'day_analyses': analyses,
              'reference_credibility': references,
              'frontier': 'Not classified by this mechanical report; requires credible references and complete paired timings',
              'unmeasured': ['native owners and receivers', 'whole-OFE day', 'whole-run scaling/memory/warm regression']}
    args.output.write_text(json.dumps(report, indent=2, sort_keys=True, allow_nan=False) + '\n')


if __name__ == '__main__':
    main()
