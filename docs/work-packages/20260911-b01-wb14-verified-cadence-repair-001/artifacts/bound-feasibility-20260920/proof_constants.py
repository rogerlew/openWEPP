#!/usr/bin/env python3
"""Arithmetic enclosures for the static proof, never a q(T) or flux evaluator.

Source decimal coefficients supply a positive-coefficient interval bound on
0 <= t <= 76.85 < 77. No state, constitutive sample, sweep or model is run.
"""
from decimal import Decimal as D, localcontext
import hashlib
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
TRACE = Path('/home/roger/openwepp-experiments/b01-wb14-grid40-run-evidence-20260919/treatment/trace.json')
assert hashlib.sha256(TRACE.read_bytes()).hexdigest() == '93124ec24c18dea0bfba15d1b09a180948dc56e7b62a3e2b76bb94eda194a17e'
SOURCE = Path('/home/roger/openwepp-experiments/b01-wb14-grid40-source-20260919')
manifest = json.loads((HERE.parent/'grid40-20260919/root-terminal-t.json').read_text())
source_files = {}
for rel in ('crates/openwepp-land-surface-energy/src/solver_covered_evaluation.rs',
            'crates/openwepp-land-surface-energy/src/physics.rs',
            'crates/openwepp-land-surface-energy/src/covered_liquid.rs'):
    digest = hashlib.sha256((SOURCE/rel).read_bytes()).hexdigest()
    assert digest == manifest['source_entries'][rel]
    source_files[rel] = digest
records = json.loads(TRACE.read_text())['records']
solver_input = next(r['value'] for r in records if r['kind']=='solver_input')
assert solver_input['caps'] is None
u = solver_input['inputs']
assert u['stage3_lower_boundary'].get('native_snow_exchange') is None
assert u['stage3_lower_boundary']['sensible_to_canopy_air_w_m2'] < 0
assert u['stage3_lower_boundary']['vapor_to_canopy_air_kg_m2_s'] < 0
assert u['top_rain_kg_m2_tile'] == 0
for occ in u['occupancies']:
    assert occ['sun']['leaf_area_m2_m2_tile'] == 0
    assert occ['shade']['leaf_area_m2_m2_tile'] > 0
    assert occ['sun']['absorbed_shortwave_w_m2_tile'] == occ['shade']['absorbed_shortwave_w_m2_tile'] == occ['stem_absorbed_shortwave_w_m2_tile'] == 0
    assert occ['shade']['absorbed_par_w_m2_leaf'] == 0
with localcontext() as ctx:
    ctx.prec = 50
    c = [D(x) for x in ('6.11213476','.444007856','.0143064234','.000264461437','.00000305903558','.0000000196237241','.0000000000892344772','-.000000000000373208410','.000000000000000209339997')]
    # P'(t) has one negative term. Pair it with the t^5 term:
    # t^5 * (6*c6 + 7*c7*t), whose bracket is positive throughout [0,77].
    derivative_group_lower = 6*c[6]+7*c[7]*D(77)
    assert derivative_group_lower > 0
    # Positive-coefficient enclosure, not P evaluated at a physical state.
    es_upper = 100*sum(max(a,D(0))*D(77)**i for i,a in enumerate(c))
    pressure = D(str(u['pressure_pa']))
    denominator_lower = pressure-D('.378')*es_upper
    assert denominator_lower > 0
    q_lower = D('.622')*(100*c[0])/(pressure-D('.378')*100*c[0])
    assert q_lower > D(str(u['air_specific_humidity_kg_kg']))
    sigma = D('5.670374419e-8')
    floor_emission = sigma*D('273.15')**4
    snow_emission = sigma*D(str(u['stage3_lower_boundary']['snow_temperature_k']))**4
    assert floor_emission > D(str(u['atmospheric_downward_longwave_w_m2']))
    assert floor_emission > snow_emission
    assert D(str(u['air_temperature_k'])) < D('273.15')
    constants = {'derivative_group_lower':derivative_group_lower,
                 'saturation_pressure_upper_Pa':es_upper,
                 'saturation_denominator_lower_Pa':denominator_lower,
                 'saturation_humidity_lower_kg_kg':q_lower,
                 'floor_blackbody_emission_W_m2':floor_emission,
                 'snow_blackbody_emission_W_m2':snow_emission,
                 'atmospheric_longwave_gap_at_floor_W_m2':floor_emission-D(str(u['atmospheric_downward_longwave_w_m2']))}
result = {'evidence_class':'Ran: decimal arithmetic of static coefficient enclosures and source constants; no constitutive evaluation',
          'trace_sha256':hashlib.sha256(TRACE.read_bytes()).hexdigest(),
          'source_files_sha256':source_files,
          'source_bindings':{'polynomial_and_q':'solver_covered_evaluation.rs:779-807 canopy_saturation_q',
                             'natural_wet_sign':'solver_covered_evaluation.rs:1153-1189 covered_wet_flux',
                             'component_heat_energy_and_inactive_wet':'solver_covered_evaluation.rs:1387-1571 evaluate_covered_occupancy',
                             'shared_heat_vapor':'solver_covered_evaluation.rs:2358-2416 evaluate_covered_column_observed',
                             'coordinate_domain':'solver_covered_evaluation.rs:1773-1812 covered_trial_is_valid',
                             'radiation_and_sigma':'physics.rs:21 and322-420 reciprocal_longwave_column',
                             'liquid_preparation':'covered_liquid.rs:131-175 prepare_covered_liquid'},
          'finite_work':'One coefficient interval enclosure, one derivative grouping, constant boundary inequalities; no sampling',
          'domain':'273.15<=active liquid-vapor T<=350 K; 200<=stem/canopy/ground/soil T<=350; 0<=qcan<=0.1; natural wet branches; exact immutable inputs',
          'constants_decimal':{k:str(v) for k,v in constants.items()},
          'limitation':'Supports the real-arithmetic exact-equation maximum principle; not interval-certified binary64 or tolerance-feasibility proof'}
(HERE/'proof-constants.json').write_text(json.dumps(result,indent=2)+'\n')
print(json.dumps(result,indent=2))
