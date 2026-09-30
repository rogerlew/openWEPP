"""Offline arithmetic and source custody only; never imports or executes a solver."""
import hashlib
import importlib.util
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[3]
R = Path('/home/roger/openwepp-experiments/cold-canopy-m1-projection-cache-20260930/frozen14/treatment')
D = Path('/home/roger/openwepp-experiments/cold-canopy-m1-cost-attribution-20260930/source')


def read(name):
    return json.loads((HERE / name).read_text())


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    spec = importlib.util.spec_from_file_location('retained_recovery', HERE / 'cost-attribution-recover.py')
    recovery = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(recovery)
    identities = []
    for root, expected in [(R, read('projection-cache-recovery.json')['cuts']['frozen14/treatment']['entries']),
                           (D, read('cost-attribution-recovery.json')['entries'])]:
        actual = recovery.entries(root)
        assert actual == expected, root
        identities.append({'path': str(root), 'entries': len(actual),
                           'sha256': hashlib.sha256(json.dumps(actual, sort_keys=True).encode()).hexdigest()})
    for rel, expected in read('architecture-decision-initial-state.json')['unrelated_dirty_sha256'].items():
        assert digest(ROOT / rel) == expected, rel
    prior = read('cost-attribution-summary.json')
    setup = prior['costs']['R']['initialization']['cpu']['median']
    proposal = prior['costs']['R']['proposal']['cpu']['median']
    budgets = {'stable': 750, 'mixed': 1500, 'transition': 2500}
    rows = []
    for supports in (1440, 49, 48, 24, 1):
        for solves_per_support in (1, 2):
            n = supports * solves_per_support
            rows.append({'supports_per_day': supports, 'solves_per_support': solves_per_support,
                         'solves_per_day': n,
                         'allowance_cpu_us_per_solve_other_zero': {k: b/n for k, b in budgets.items()},
                         'setup_only_cpu_us_if_R_transfers': n*setup,
                         'scenario_cpu_us_per_day_if_R_transfers': {str(p): n*(setup+p*proposal) for p in (1, 3, 8)},
                         'one_proposal_over_budget_ratio_if_R_transfers': {k: n*(setup+proposal)/b for k,b in budgets.items()}})
    output = {
        'evidence_class': 'Ran: offline arithmetic/source hash checks only. Static conditional work models; no physical or performance execution.',
        'units': 'microseconds unless otherwise named',
        'source_identities': identities,
        'unrelated_dirty_files_unchanged': len(read('architecture-decision-initial-state.json')['unrelated_dirty_sha256']),
        'measured_prior_phase_cpu_medians': {'initialization': setup, 'proposal': proposal,
            'sum_of_phase_medians_not_combined_median': setup+proposal,
            'combined_median': prior['costs']['R']['combined']['cpu']['median']},
        'scenarios': rows,
        'limits': ['1440 is the fixed diagnostic provider completed-day requirement, not an observed completed day or universal climate cadence.',
                   '49/48 require a different adopted support policy; 24/1 additionally cross current parent boundaries.',
                   'Two solves/support is illustrative, not inferred from two occupancies or map caps.',
                   '1/3/8 proposals are sensitivities, never observed solve averages.',
                   'G4 R initialization/proposal costs have no demonstrated transfer to any complete solve.',
                   'C_other=0 is an optimistic allowance only; failed attempts and all owners remain payable.'],
        'recovered_whole_run_target': {'ofes': 10, 'years': 100, 'days': 36525,
            'cpu_seconds': 182.625, 'wall_seconds': 210,
            'source': 'docs/work-packages/20260901-stage3-native-vegetation-laned-watershed-throughput-recovery-001/artifacts/performance-budget.md:62-73,156-165',
            'cpu_average_us_per_ofe_day': 182.625*1e6/(10*36525),
            'five_ofe_target': 'No separate explicit five-OFE wall ceiling recovered; do not scale 210 s into an adopted target.'},
        'counterfactual_100x_whole_R_phase_cost_at_1440': {
            'cpu_us_per_day': 1440*(setup+proposal)/100,
            'budget_ratios': {k:1440*(setup+proposal)/100/b for k,b in budgets.items()}},
        'new_method_evaluation_wall_subtotals_only': {
            'observed_component_transfer_envelope_us_per_core': [0.9, 2.8],
            'optimistic_two_evaluations_us': [2*0.9, 2*2.8],
            'conditional_3_to_8_updates_2_to_4_evaluations_each_us': [3*2*0.9, 8*4*2.8],
            'not_total_cost': 'Add derivative/factor/constraints/hydraulic/materialization/setup/rejection cost. Wall envelope is not a CPU prediction or universal physical lower bound.'},
        'controls': {'supports_60s': 86400//60, 'parents_1800s': 86400//1800,
                     'records_per_parent': 1800//60, 'two_occupancy_energy_dimension': 2*6+2+1+6,
                     'joint_with_downstream_hydraulics': 21+2*4, 'fixed_identity_reduction':21-7,
                     'plus_affine_q_reduction':21-7-1, 'plus_two_exact_dark_sun_anchors':21-7-1-2},
    }
    assert output['controls']['supports_60s'] == 1440
    assert abs(rows[0]['allowance_cpu_us_per_solve_other_zero']['stable']-25/48)<1e-14
    assert output['recovered_whole_run_target']['cpu_average_us_per_ofe_day'] == 500
    (HERE/'architecture-decision-arithmetic.json').write_text(json.dumps(output, indent=2)+'\n')
    print(json.dumps({'source_identities':identities,'scenarios':len(rows),'unrelated_unchanged':output['unrelated_dirty_files_unchanged']}))


if __name__ == '__main__':
    main()
