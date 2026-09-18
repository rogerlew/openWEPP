"""Independent high-precision residual check of captured, consumed linear systems."""
from decimal import Decimal, localcontext
from pathlib import Path
import hashlib
import json

HERE = Path(__file__).resolve().parent
trace_path = HERE / 'instrumented-first-trial-trace.json'
records = json.loads(trace_path.read_text())['records']
bases = {r['value']['iteration']: r['value'] for r in records if r['kind'] == 'accepted_base'}
results = []
with localcontext() as ctx:
    ctx.prec = 80
    D = Decimal.from_float
    for record in records:
        if record['kind'] != 'linear_system':
            continue
        v = record['value']
        n = len(v['direction'])
        units = [1000.,1000.,1000.,1000.,1.,1.,1.,1.,1.,1.] * 2 + [1.,.001,1.] + [1.] * (n-23)
        delta = [D(x) / D(u) if v['v10_scaled_potential'] else D(x) for x,u in zip(v['direction'],units)]
        a = [[D(x) for x in row] for row in v['jacobian']]
        rhs = [D(x) for x in v['rhs']]
        residual = [sum((x*y for x,y in zip(row,delta)), Decimal(0))-b for row,b in zip(a,rhs)]
        rn = max(map(abs,residual))
        an = max(sum(map(abs,row)) for row in a)
        dn = max(map(abs,delta))
        bn = max(map(abs,rhs))
        results.append(dict(iteration=v['iteration'],dimension=n,scaled=v['v10_scaled_potential'],residual_infinity=str(rn),normwise_backward_error=str(rn/(an*dn+bn)),relative_rhs_residual=str(rn/bn),max_residual_row=max(range(n),key=lambda i:abs(residual[i])),ground_direction=v['direction'][22],ground_diagonal=v['jacobian'][22][22]))
    base = bases[6]
    linear = next(r['value'] for r in records if r['kind']=='linear_system' and r['value']['iteration']==6)
    margin = D(base['coordinates'][7])-D(273.15)
    bound = margin/-D(linear['direction'][7])
    attempts = [r['value'] for r in records if r['kind']=='line_search_attempt' and r['value']['iteration']==6]
    last = attempts[-1]
    expected = [x+last['factor']*dx for x,dx in zip(base['coordinates'],linear['direction'])]
    assert expected == last['coordinates']
    assert len(attempts)==21 and all(not x['domain_valid'] for x in attempts)
    summary = dict(evidence_class='Ran: offline independent Decimal(80-digit) linear residual and bound calculation; no physical/evaluator execution',trace_sha256=hashlib.sha256(trace_path.read_bytes()).hexdigest(),coordinate_convention='Consumed J is J_x D; physical direction divided by [1000mm x4,1 x6] per occupancy, shared [1K,.001kg/kg,1K], soil 1K before matrix multiplication. No solver helper called.',linear_checks=results,decisive_bound=dict(coordinate_index=7,name='stratum-z-upper::forest T_shade',unit='K',base=base['coordinates'][7],lower_bound=273.15,base_margin_exact_decimal=str(margin),physical_direction=linear['direction'][7],maximum_feasible_factor=str(bound),minimum_canonical_factor=2**-20,smallest_trial_temperature=last['coordinates'][7],violation_below_bound=str(D(273.15)-D(last['coordinates'][7])),first_smaller_diagnostic_factor=2**-21,first_smaller_projected_temperature=base['coordinates'][7]+2**-21*linear['direction'][7]),all_final_attempts_domain_invalid=True,final_attempt_count=21,canonical_trial_evaluations_final_search=0,accepted_exponents=[r['value']['exponent'] for r in records if r['kind']=='line_search_attempt' and r['value'].get('accepted')],limitation='Small backward error only verifies the consumed linear system. It does not qualify Jacobian derivatives or establish existence/nonexistence of a physical root. Projected smaller step is arithmetic only, not an accepted advance or evaluator result.')
(HERE/'root-linear-check.json').write_text(json.dumps(summary,indent=2)+'\n')
print(json.dumps(summary,indent=2))
