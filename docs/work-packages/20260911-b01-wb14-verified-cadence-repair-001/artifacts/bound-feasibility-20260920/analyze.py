#!/usr/bin/env python3
"""Finite retained-array and custody arithmetic only; no constitutive evaluator.

Three captured bases, one predeclared direction each. All 29 residual rows
remain in the infinity merit. Source algebra restores the overwritten inactive
rows; no nonlinear calls, FD samples, optimization or search occur.
"""
import datetime as dt
import hashlib
import json
import math
from pathlib import Path

HERE = Path(__file__).resolve().parent
PACKAGE = HERE.parent.parent
OLD = HERE.parent / 'grid40-20260919'
SOURCE = Path('/home/roger/openwepp-experiments/b01-wb14-grid40-source-20260919')
PAIR = Path('/home/roger/openwepp-experiments/b01-wb14-grid40-run-evidence-20260919')


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def read_bound(path, expected):
    assert sha(path) == expected, str(path)
    return json.loads(path.read_text())


def bounds():
    # Source-derived inequalities, not canonical domain-function invocation.
    result = [(-math.inf, math.inf)] * 29
    for off in (0, 10):
        for j in (4, 5):
            result[off+j] = (0.0, 1.0)
        for j in (6, 7, 8):
            result[off+j] = (273.15, 350.0)
        result[off+9] = (200.0, 350.0)
    result[20:23] = [(200.0, 350.0), (0.0, 0.1), (200.0, 350.0)]
    result[23:] = [(200.0, 350.0)] * 6
    return result


def main():
    manifest = json.loads((OLD/'root-terminal-t.json').read_text())
    entries = manifest['source_entries']
    assert len(entries) == 749
    actual = {name: sha(SOURCE/name) for name in entries}
    assert actual == entries
    digest = hashlib.sha256(json.dumps(actual, sort_keys=True).encode()).hexdigest()
    assert digest == 'fae897fe90f6bbadcaf298607c9d23fd986cd778f9dd56195f7a5510cbd5c4be'
    freeze_path = OLD/'pair-freeze.json'
    freeze = read_bound(freeze_path, 'a13048e5df6c41e034746c2d4d52b9468ab04a1aa03a6ce796491eceeaa2f901')
    assert len(freeze['external_build_inputs']) == 19
    assert all(sha(Path(p)) == h for p, h in freeze['external_build_inputs'].items())
    assert len(freeze['support_links']) == 5
    assert all((SOURCE/n).is_symlink() and str((SOURCE/n).resolve()) == p
               for n, p in freeze['support_links'].items())
    assert sha(Path(freeze['binary'])) == freeze['binary_sha256']
    original = read_bound(Path(freeze['input']), '527208c3e5ad07f1744f1bd92c9e4ac17cf7efd9469968a97d7964c5db482dfd')
    treatment = read_bound(PAIR/'treatment/trace.json', '93124ec24c18dea0bfba15d1b09a180948dc56e7b62a3e2b76bb94eda194a17e')
    read_bound(PAIR/'treatment/sidecar.json', 'bad1c601160d3a5e13713a5c99e92f0336531814d21d5f7cd70a8be87e4fadca')
    records = treatment['records']
    inp = next(r['value'] for r in records if r['kind'] == 'solver_input')
    oldinp = next(r['value'] for r in original['records'] if r['kind'] == 'solver_input')
    assert inp == oldinp
    units = [1000.0]*4+[1.0]*6
    units = units*2+[1.0, 0.001, 1.0]+[1.0]*6
    report = []
    for it in (0, 6, 14):
        base = next(r['value'] for r in records if r['kind']=='accepted_base' and r['value']['iteration']==it)
        linear = next(r['value'] for r in records if r['kind']=='linear_system' and r['value']['iteration']==it)
        x, residual = base['coordinates'], base['normalized_residuals']
        s = base['applied_normalizers']
        assert len(x) == len(residual) == len(s) == 29
        assert all(a/b == c for a,b,c in zip(base['raw_residuals'], s, residual))
        assert linear['v10_scaled_potential']
        consumed = linear['jacobian']
        # J_y = J_x D. Rows 6 and16 were overwritten in the solver, including
        # removal of the canopy-temperature dependency. Recover that known
        # affine residual analytically; other rows remain measured FD entries.
        jx = [[v/unit for v,unit in zip(row,units)] for row in consumed]
        substitutions = []
        for row in (6,16):
            assert x[20] != 273.15  # no max-anchor kink in these selected bases
            anchor_slope = 1.0 if x[20] > 273.15 else 0.0
            anchor = max(x[20], 273.15)
            assert base['evaluation']['occupancies'][row//10]['component_areas_m2_m2_tile'][0] == 0
            assert consumed[row][row] == 1.0
            assert sum(v != 0 for v in consumed[row]) == 1
            assert linear['rhs'][row] == anchor-x[row]
            jx[row] = [0.0]*29
            jx[row][row] = 1/s[row]
            jx[row][20] = -anchor_slope/s[row]
            substitutions.append({'row':row, 'normalizer_K':s[row], 'true_diagonal_per_K':1/s[row],
                                  'true_canopy_entry_per_K':-anchor_slope/s[row], 'anchor_K':anchor, 'consumed_diagonal':1.0,
                                  'consumed_canopy_entry':0.0, 'consumed_rhs':linear['rhs'][row],
                                  'negative_original_residual':-residual[row]})
        assert all(linear['rhs'][i] == -residual[i] for i in range(29) if i not in (6,16))
        # Frozen before results: ||D^-1 d||_inf <=1, d=-e_ground K.
        # This is a diagnostic merit direction, not a root-finder proposal.
        d = [0.0]*29
        d[22] = -1.0
        trial = [a+b for a,b in zip(x,d)]
        feasible = all(lo <= v <= hi for v,(lo,hi) in zip(trial,bounds()))
        predicted = [r+math.fsum(a*b for a,b in zip(row,d)) for r,row in zip(residual,jx)]
        exact_identity_prediction = residual.copy()
        exact_identity_prediction[22] = (x[22]-1-inp['inputs']['stage3_lower_boundary']['snow_temperature_k'])/s[22]
        assert all(jx[i][22] == 0.0 for i in range(29) if i != 22)
        assert feasible
        assert max(map(abs,predicted)) < max(map(abs,residual))
        assert max(map(abs,exact_identity_prediction)) < max(map(abs,residual))
        lin_error = [math.fsum(a*(v/unit) for a,v,unit in zip(row,linear['direction'],units))-rhs
                     for row,rhs in zip(consumed,linear['rhs'])]
        report.append({'iteration':it, 'coordinates':x, 'coordinate_scales':units,
                       'raw_residuals':base['raw_residuals'], 'normalizers':s,
                       'original_normalized_residuals':residual,
                       'inactive_row_restoration':substitutions,
                       'direction_physical':d, 'scaled_neighborhood_radius':1.0,
                       'trial_coordinates_offline_only':trial, 'all_explicit_coordinate_inequalities_pass':feasible,
                       'linearized_full_residuals':predicted,
                       'source_identity_full_residual_prediction':exact_identity_prediction,
                       'base_infinity_merit':max(map(abs,residual)),
                       'linearized_infinity_merit':max(map(abs,predicted)),
                       'source_identity_infinity_merit':max(map(abs,exact_identity_prediction)),
                       'all_other_predicted_rows_unchanged':all(predicted[i]==residual[i] for i in range(29) if i!=22),
                       'measured_ground_column_diagonal_per_K':jx[22][22],
                       'matrix_vs_identity_prediction_max_difference':max(abs(a-b) for a,b in zip(predicted,exact_identity_prediction)),
                       'max_linear_solve_equation_error':max(map(abs,lin_error)),
                       'shade_bound_margin_K':x[7]-273.15,
                       'captured_newton_shade_direction_K':linear['direction'][7],
                       'unchanged_physical_energy_rows':[7,8,9,17,18,19,20,21]})
    output = {'evidence_class':'Ran: hashes and finite retained-array arithmetic; Static: source-dependent identity restoration; no evaluator',
              'utc':dt.datetime.now(dt.timezone.utc).isoformat(),
              'source_sha256':digest, 'source_entry_count':len(entries),
              'all_source_files_nonwritable':all((SOURCE/n).stat().st_mode & 0o222 == 0 for n in entries),
              'external_inputs_verified':19,'support_links_verified':5,
              'freeze_sha256':sha(freeze_path),'original_trace_sha256':sha(Path(freeze['input'])),
              'treatment_trace_sha256':sha(PAIR/'treatment/trace.json'),
              'sidecar_sha256':sha(PAIR/'treatment/sidecar.json'),
              'binary_sha256':sha(Path(freeze['binary'])),
              'finite_work':'Three bases (0,6,14), one fixed direction each; 29 rows each; no optimizer/search/FD/model calls',
              'diagnostic_merit':'Infinity norm of ALL original normalized residuals; original coordinate scales for radius only',
              'scope_limit':'Coordinate inequalities and linearized/source-identity predictions only, not canonical admission or evaluated nonlinear descent. Does not improve physical rows or demonstrate a root.',
              'bases':report}
    (HERE/'retained-array-analysis.json').write_text(json.dumps(output,indent=2,allow_nan=False)+'\n')
    print(json.dumps({'source_entries':749,'bases':[{k:b[k] for k in ('iteration','base_infinity_merit','linearized_infinity_merit','source_identity_infinity_merit','matrix_vs_identity_prediction_max_difference')} for b in report]},indent=2))


if __name__ == '__main__':
    main()
