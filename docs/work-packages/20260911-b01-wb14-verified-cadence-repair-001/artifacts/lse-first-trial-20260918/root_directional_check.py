"""Reconcile captured canonical primal directional probes without rerunning physics."""
import hashlib
import json
import math
from pathlib import Path

HERE = Path(__file__).resolve().parent
first = HERE/'instrumented-first-trial-trace.json'
second = HERE/'directional-followup-trace.json'
a=json.loads(first.read_text())['records']
b=json.loads(second.read_text())['records']
probes=[r['value'] for r in b if r['kind']=='diagnostic_directional_probe']
ordinary=[r for r in b if r['kind']!='diagnostic_directional_probe']
assert a==ordinary
base=next(r['value'] for r in b if r['kind']=='accepted_base' and r['value']['iteration']==6)
linear=next(r['value'] for r in b if r['kind']=='linear_system' and r['value']['iteration']==6)
units=([1000.]*4+[1.]*6)*2+[1.,.001,1.]+[1.]*6
slopes=[math.fsum(v*d/u for v,d,u in zip(row,linear['direction'],units)) for row in linear['jacobian']]
# The consumed solver matrix replaced zero-area sun-temperature rows by unit
# identities. Restore their recorded normalization for primal comparison.
for index in [6,16]:
    assert base['evaluation']['occupancies'][index//10]['component_areas_m2_m2_tile'][0]==0
    assert base['coordinates'][20]<273.15
    slopes[index]/=base['applied_normalizers'][index]
results=[]
for v in probes:
    e=v['frozen_evaluation']
    dr=[y-x for y,x in zip(e['normalized_residuals'],base['normalized_residuals'])]
    predictions=[v['factor']*s for s in slopes]
    defects=[x-y for x,y in zip(dr,predictions)]
    norm=max(map(abs,e['normalized_residuals']))
    results.append(dict(exponent=v['exponent'],factor=v['factor'],domain_valid=v['domain_valid'],canonical_frozen_calls=v['frozen_canonical_evaluator_calls'],canonical_unfrozen_calls=v['unfrozen_canonical_evaluator_calls'],normalized_infinity_norm=norm,strict_decrease=norm<base['infinity_norm'],ground_residual_change=dr[22],ground_linear_prediction=predictions[22],largest_abs_linearization_defect=max(map(abs,defects)),largest_defect_row=max(range(len(defects)),key=lambda i:abs(defects[i])),gas_branches=[o['gas_branches'] for o in e['occupancies']],wet_branches=[o['wet_branch'] for o in e['occupancies']],frozen_unfrozen_evaluations_equal=e==v['unfrozen_evaluation'] if v['unfrozen_evaluation'] else None,per_row_change=dr,per_row_prediction=predictions,per_row_defect=defects))
baseline=Path('/workdir/openWEPP/docs/work-packages/20260911-b01-wb14-verified-cadence-repair-001/artifacts/file-reader-20260918/reader-exact-02.stderr')
def diagnostic(path):
    return next(x.strip() for x in path.read_text().splitlines() if x.lstrip().startswith('local staged native day 0'))
assert diagnostic(baseline)==diagnostic(HERE/'instrumented-first-trial.stderr')==diagnostic(HERE/'directional-followup.stderr')
result=dict(evidence_class='Ran: independent offline captured-operand reconciliation; no new evaluator calls',trace_sha256={p.name:hashlib.sha256(p.read_bytes()).hexdigest() for p in [first,second]},canonical_records_equal=True,canonical_records=95,baseline_complete_terminal_diagnostic_byte_equal=True,diagnostic_evaluator_calls=sum(r['canonical_frozen_calls']+r['canonical_unfrozen_calls'] for r in results),diagnostic_points_installed=0,normalization_mapping='Consumed scaled J uses unit rows6/16 for zero-area sun temperature; divide those predicted row changes by captured1.0001e-6 anchor normalizer before comparison with canonical primal residual. Anchor273.15K is constant throughout these probes (Tc<273.15). Other rows use consumed normalized matrix directly.',probes=results,limitations='Three small feasible one-sided probes establish local descent and consistency, not global Jacobian qualification or an admissible converged root. Factors21..23 are diagnostic only and outside the canonical b0..20 search; no threshold or production policy changed. Terminal hashes remain reported owner-state evidence, not independent global rollback/conservation.')
(HERE/'root-directional-check.json').write_text(json.dumps(result,indent=2)+'\n')
print(json.dumps({k:v for k,v in result.items() if k!='probes'},indent=2))
