"""Preserve bounded static mappings and closure limitations; no evaluator calls."""
import datetime as dt
import hashlib
import json
from pathlib import Path
P=Path(__file__).resolve().parent
save=lambda name,value:(P/name).write_text(json.dumps(value,indent=2)+'\n')
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
trace=json.loads((P/'final-first-failure-trace.json').read_text())['records']
initial=trace[0]['value']
bases=[r['value'] for r in trace if r['kind']=='accepted_base']
coords=[('psi_sun','mm'),('psi_shade','mm'),('psi_stem','mm'),('psi_root','mm'),('beta_sun','dimensionless'),('beta_shade','dimensionless'),('T_sun','K'),('T_shade','K'),('T_wet','K'),('T_stem','K')]
rows=[('inactive_sun_psi_minus_stem_anchor','mm'),('shade_gas_minus_q1','kg m^-2 tile s^-1'),('nonpositive_beta_sun_minus_one_anchor','dimensionless'),('nonpositive_beta_shade_minus_one_anchor','dimensionless'),('q1_sum_minus_q2','kg m^-2 tile s^-1'),('q2_minus_root_source_sum','kg m^-2 tile s^-1'),('inactive_sun_temperature_anchor','K'),('shade_leaf_energy','W m^-2 tile'),('wet_surface_energy','W m^-2 tile'),('dry_stem_energy','W m^-2 tile')]
mapping=[]
for i in range(29):
    if i<20:
        name,unit=coords[i%10];row,runit=rows[i%10];owner=['stratum-z-upper::forest','stratum-a-lower::forest'][i//10]
    elif i<23:
        name,unit=[('T_canopy','K'),('q_canopy','kg/kg'),('T_ground','K')][i-20]
        row,runit=[('shared_canopy_air_heat','W m^-2 tile'),('shared_canopy_air_vapor','kg m^-2 tile s^-1'),('represented_snow_T_ground_minus_T_snow_anchor','K')][i-20];owner='shared'
    else:
        owner=['thermal-1','thermal-2','soil-1','soil-2','soil-dry','soil-frozen'][i-23];name,unit='T_soil','K';row,runit='represented_snow_T_soil_minus_beginning_anchor','K'
    mapping.append(dict(index=i,owner=owner,coordinate=name,coordinate_unit=unit,actual_final_branch_residual=row,actual_residual_unit=runit,initial_coordinate=initial['initial_trial'][i],final_coordinate=bases[-1]['coordinates'][i],final_raw_residual=bases[-1]['raw_residuals'][i],applied_normalizer=bases[-1]['applied_normalizers'][i],normalized_residual=bases[-1]['normalized_residuals'][i]))
save('coordinate-residual-map.json',dict(evidence_class='Static source row/coordinate interpretation plus captured operands; no physical execution',trace_sha256=sha(P/'final-first-failure-trace.json'),actual_branch_only=True,scale_rule='Captured applied_normalizer divides raw residual; displayed diagnostic scale is not the divisor.',source_locations=['solver_covered_evaluation.rs:1177-1178,1367-1368,1737-1779,2097-2100','solver_covered_solve.rs:covered_failure_residuals'],rows=mapping))
files=[]
for path in sorted((P/'pre-private-source').glob('*')):
    if path.is_file(): files.append(dict(path=str(path.relative_to(P)),sha256=sha(path),bytes=path.stat().st_size))
save('source-custody-erratum.json',dict(evidence_class='Ran: file identity checks only',supersedes_field='source-custody-manifest.json historical_measurement_cut_before_private_api_policy_correction.preserved_files',reason='Writer receipt looked for nested original paths in the flattened three-file preservation directory and incorrectly reported absent. Actual preserved files follow. Original receipt retained.',preserved_files=files,historical_scope='These files/patch are the second directional-run measurement cut. The first no-probe run lacks a separately retained complete source cut; its raw trace and identical95canonical records remain evidence. Final source/binary/run are fully contemporaneously bound.'))
save('review-closure-status.json',dict(evidence_class='Static: retained review/tool outcomes',utc=dt.datetime.now(dt.timezone.utc).isoformat(),correctness_reviewer='/root/reader_correctness',correctness_primary='correctness-primary-evidence.txt',correctness_scope='Independent scientific/source/operand review and Decimal80 solve; no scientific blocker; explicitly preliminary pending final private patch/recorded-run custody.',qa_reviewer='/root/reader_qa',qa='qa-review.txt',qa_verdict='PASS bounded diagnostic evidence only including final source/trace/custody',terminal_correctness_signoff='UNMET',recovery_attempts=dict(followup_task=4,send_message=1,replacement_spawn=1,result='agent thread limit reached'),recovery_note='Original writer and QA completed; interrupting completed writer did not release the unavailable thread. No source author substituted as independent reviewer. Original correctness evidence remains reusable for unchanged scientific scope.',disposition='Scientific first-refusal explanation established; terminal independent-correctness closure HOLD. No numerical fix or reader acceptance.'))
save('failure-floor-reconciliation.json',dict(carried_floor=334,writer_lower_bound=9,writer_evidence='writer-failure-ledger.json',root_identified_events=[{'event':'final-first-failure expected physical refusal','count':1,'evidence':'final-first-failure.json'},{'event':'read-only rg guessed nonexistent source path','count':1,'evidence':'tool transcript only'},{'event':'read-only ls confirmed absent .cargo directory with nonzero status','count':1,'evidence':'tool transcript only'},{'event':'required correctness reviewer resumption/replacement rejected by thread limit','count':6,'evidence':'tool transcript and review-closure-status.json'}],new_floor_at_least=352,meaning='Mixed process/tool/expected-refusal lower bound, NOT a correction-cycle count.',uncertainty='Historical uncertainty retained; early missing-include and ENOSPC build multiplicity uncounted, as are unidentified prior-turn navigation/tool failures. All known writer outcomes separately retained.'))
print('Wrote coordinate mapping, custody erratum, review limit, failure floor.')
