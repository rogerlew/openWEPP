"""Analyze the fixed attribution cohort; never launch numerical work."""
import argparse
import json
from pathlib import Path
from statistics import median

ORDERS = [('R','D','E'),('D','E','R'),('E','R','D'),('E','D','R'),('D','R','E'),('R','E','D')]
PHASES = ('initialization', 'proposal', 'combined')
CLOCKS = ('cpu', 'wall')


def spread(values):
    return {'median': median(values), 'min': min(values), 'max': max(values), 'samples': values}


def batch_cost(row, phase, clock):
    if phase == 'combined':
        return batch_cost(row, 'initialization', clock) + batch_cost(row, 'proposal', clock)
    return row[phase][clock + '_ns'] / row['count'] / 1000


def analyze_outer(rows):
    expected = [('warmup-' + a, a, 16) for a in ('R', 'D', 'E')]
    expected += [('triplet-' + str(i + 1) + '-' + a, a, 32) for i, order in enumerate(ORDERS) for a in order]
    actual = [(r['label'], r['configuration'], r['count']) for r in rows]
    if actual != expected:
        raise ValueError('Cohort is not the prospectively fixed inventory/order')
    for row in rows:
        for phase in ('initialization', 'proposal'):
            for clock in CLOCKS:
                if row[phase][clock + '_ns'] <= 0:
                    raise ValueError('Nonpositive timing interval')
        if row['exit_code'] != 0 or len(row['result']['outcomes']) != row['count']:
            raise ValueError('Incomplete batch')
    measured = rows[3:]
    costs = {a: {p: {c: spread([batch_cost(r, p, c) for r in measured if r['configuration'] == a])
                    for c in CLOCKS} for p in PHASES} for a in ('R', 'D', 'E')}
    comparisons = {}
    for numerator, denominator in [('D', 'R'), ('E', 'D')]:
        name = numerator + '/' + denominator
        comparisons[name] = {}
        for phase in PHASES:
            comparisons[name][phase] = {}
            for clock in CLOCKS:
                ratios = []
                for i in range(6):
                    by_arm = {r['configuration']: r for r in measured[3*i:3*i+3]}
                    ratios.append(batch_cost(by_arm[numerator], phase, clock) / batch_cost(by_arm[denominator], phase, clock))
                stats = spread(ratios)
                stats['median_overhead_percent'] = 100 * (stats['median'] - 1)
                stats['ratio_of_cost_medians'] = costs[numerator][phase][clock]['median'] / costs[denominator][phase][clock]['median']
                stats['screen_applicable'] = phase != 'combined'
                stats['paired_median_screen_pass'] = stats['median'] <= 1.10 if phase != 'combined' else None
                stats['ratio_of_medians_screen_pass'] = stats['ratio_of_cost_medians'] <= 1.10 if phase != 'combined' else None
                comparisons[name][phase][clock] = stats
    triplets = [{a: {p: {c: batch_cost(r, p, c) for c in CLOCKS} for p in PHASES}
                 for r in measured[3*i:3*i+3] for a in [r['configuration']]} for i in range(6)]
    return {'units': 'microseconds per fresh proposal or initialization',
            'costs': costs, 'comparisons': comparisons, 'triplets': triplets,
            'cohort_complete_outcomes': sum(r['count'] for r in rows),
            'measurement_outcomes_per_configuration': 192, 'warmups_in_cost_statistics': False}


def amdahl(f, acceleration=None):
    if not 0 <= f <= 1:
        raise ValueError('Share outside [0,1]')
    if acceleration is None:
        return None if f == 1 else 1 / (1 - f)
    if acceleration <= 0:
        raise ValueError('Acceleration must be positive')
    return 1 / ((1 - f) + f / acceleration)


SCOPE_NAMES = ['natural_jacobian','selected_jacobian','natural_base_core','natural_fd_core',
               'selected_base_core','selected_fd_core','other_core','stage1','svd','damping',
               'refinement','refinement_svd','refinement_damping','active_kkt','acceptance']
OPPORTUNITIES = {
    'all_core_evaluations': ['natural_base_core','natural_fd_core','selected_base_core','selected_fd_core','other_core'],
    'jacobian_assembly_only': ['natural_jacobian','selected_jacobian'],
    'all_svd_factorization': ['svd','refinement_svd'],
    'all_damping_search': ['damping','refinement_damping'],
    'refinement_excluding_timed_children': ['refinement'],
    'active_kkt_and_acceptance_exclusive': ['active_kkt','acceptance'],
    'stage1_other_exclusive': ['stage1'],
}


def reconcile(row):
    payload = row['result']['cost_attribution']
    if payload.get('schema') != 'm1-cost-attribution-v2':
        raise ValueError('Unexpected attribution schema')
    if payload.get('scope') != 'complete-hierarchy' or payload.get('enabled') is not True:
        raise ValueError('Missing complete enabled hierarchy')
    if [p['name'] for p in payload['phases']] != ['initialization','proposal']:
        raise ValueError('Unexpected phase inventory')
    result = {}
    for phase in payload['phases']:
        name = phase['name']; metrics = phase['metrics']; edges = phase['direct_child_wall_ns']
        if [m['name'] for m in metrics] != SCOPE_NAMES:
            raise ValueError('Unexpected scope inventory')
        if len(edges) != len(metrics) or any(len(e) != len(metrics) for e in edges):
            raise ValueError('Bad direct-child matrix shape')
        for index, metric in enumerate(metrics):
            values = [metric[k] for k in ('inclusive_wall_ns','exclusive_wall_ns','calls','jacobi_sweeps')]
            if any(v < 0 for v in values) or any(v < 0 for v in edges[index]):
                raise ValueError('Negative observer record')
            if metric['inclusive_wall_ns'] - sum(edges[index]) != metric['exclusive_wall_ns']:
                raise ValueError('Inclusive/exclusive/child accounting mismatch')
        exclusive = sum(m['exclusive_wall_ns'] for m in metrics)
        enclosing = row[name]['wall_ns']
        remainder = enclosing - exclusive
        if remainder < 0:
            raise ValueError('Negative enclosing remainder: recording defect')
        for i, metric in enumerate(metrics):
            if sum(edge[i] for edge in edges) > metric['inclusive_wall_ns']:
                raise ValueError('A child interval was charged more than once')
        result[name] = {'enclosing_wall_ns': enclosing, 'exclusive_wall_ns': exclusive,
                        'unassigned_wall_ns': remainder,
                        'scopes': {m['name']: m for m in metrics}, 'direct_child_wall_ns': edges}
    return result


def analyze_components(rows):
    enabled = [r for r in rows if r['configuration'] == 'E']
    reconciled = [(r, reconcile(r)) for r in enabled]
    measured = [(r,v) for r,v in reconciled if not r['label'].startswith('warmup')]
    phases = {}
    for phase in PHASES:
        names = SCOPE_NAMES + ['unassigned']
        components = {}
        for name in names:
            costs = []; shares = []; inclusives = []; counts = []; sweeps = []
            for row, rec in measured:
                selected = ['initialization','proposal'] if phase == 'combined' else [phase]
                denominator = sum(rec[p]['enclosing_wall_ns'] for p in selected)
                if name == 'unassigned':
                    amount = sum(rec[p]['unassigned_wall_ns'] for p in selected)
                    inclusive = amount; count = None; sweep = None
                else:
                    mm = [rec[p]['scopes'][name] for p in selected]
                    amount = sum(m['exclusive_wall_ns'] for m in mm)
                    inclusive = sum(m['inclusive_wall_ns'] for m in mm)
                    count = sum(m['calls'] for m in mm) / row['count']
                    sweep = sum(m['jacobi_sweeps'] for m in mm) / row['count']
                costs.append(amount / row['count'] / 1000)
                shares.append(amount / denominator)
                inclusives.append(inclusive / row['count'] / 1000)
                counts.append(count); sweeps.append(sweep)
            components[name] = {'exclusive_us': spread(costs), 'exclusive_wall_share': spread(shares),
                                'inclusive_us': spread(inclusives), 'calls_per_proposal': counts,
                                'calls_spread': spread(counts) if name != 'unassigned' else None,
                                'jacobi_sweeps_per_proposal': sweeps,
                                'sweeps_spread': spread(sweeps) if name != 'unassigned' else None}
        phases[phase] = dict(sorted(components.items(), key=lambda item: -item[1]['exclusive_us']['median']))
    opportunities = {}
    for name, scopes in OPPORTUNITIES.items():
        opportunities[name] = {}
        for phase in ('proposal','combined'):
            fractions = [sum(phases[phase][scope]['exclusive_wall_share']['samples'][i] for scope in scopes) for i in range(6)]
            opportunities[name][phase] = {'scope_names': scopes, 'enabled_wall_share': spread(fractions),
                'ideal_speedup_if_free': spread([amdahl(f) for f in fractions]),
                'conditional_speedup_if_2x': spread([amdahl(f,2) for f in fractions]),
                'conditional_speedup_if_10x': spread([amdahl(f,10) for f in fractions])}
    return {'basis': 'E measured monotonic wall intervals; never CPU fractions or rescaled R shares',
            'reconciliation': {row['label']: rec for row,rec in reconciled},
            'ranked_components': phases, 'conditional_amdahl': opportunities,
            'limitation': 'Conditional instrumented-workload estimates; quality screens determine whether precise baseline attribution is supportable; overlapping opportunities are not multiplied.'}


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('cohort', type=Path)
    parser.add_argument('output', type=Path)
    args = parser.parse_args()
    rows = json.loads(args.cohort.read_text())
    result = analyze_outer(rows)
    result['components'] = analyze_components(rows)
    args.output.write_text(json.dumps(result, indent=2) + '\n')


if __name__ == '__main__':
    main()
