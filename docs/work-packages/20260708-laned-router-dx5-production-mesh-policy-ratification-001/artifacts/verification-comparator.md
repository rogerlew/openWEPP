# Independent comparator/timing verification for DX5 production mesh policy

Run scope: static re-audit of
`/home/workdir/openWEPP/docs/work-packages/20260708-laned-router-dx5-production-mesh-policy-ratification-001/artifacts/default-dx5-evidence.json`
and
`/home/workdir/openWEPP/docs/work-packages/20260708-laned-router-dx5-production-mesh-policy-ratification-001/artifacts/rev44-promotion-matrix.json`.

I did not rerun the full 12-run cohort.

## Commands (executed)

| Command | Exit | Result |
|---|---:|---|
| `jq -r '.runs[] | select(.mode=="active_default_dx5") | "\(.member_id)\t\(.status)\t\(.laned_active.mesh_policy.mode)\t\(.laned_active.mesh_policy.target_dx_m)\t\(.laned_active.mesh_policy.min_cells)\t\(.laned_active.mesh_policy.max_cells)\t\(.laned_active.max_dt_s)"' .../default-dx5-evidence.json` | 0 | PASS |
| `jq -r '.identity_comparisons[] | "\(.member_id)\t\(.comparison)\t\(.status)\t\((.mismatches|length))"' .../default-dx5-evidence.json` | 0 | PASS |
| `jq -r '[.runs[] | select(.mode|startswith("active_")) | .laned_active.max_day_cascade_residual_rel] | max' .../default-dx5-evidence.json` and companion residual queries | 0 | PASS |
| `jq -r '.status, .row_count, .ratified_by_evidence, .blockers, [ .rows[] | select(.verdict==\"PASS\") ] | length, [ .rows[] | select(.gate_class==\"report-only\") ] | length, [ .rows[] | select(.verdict==\"FAIL\") ] | length' .../rev44-promotion-matrix.json` | 0 | PASS |

## Evidence checks

### 1) Selected real-cohort active no-env default target
- For all three selected real members, `active_default_dx5` reports:
  - `mesh_policy.mode=target_dx`
  - `target_dx_m=5.0`
  - `min_cells=10`
  - `max_cells=4096`
  - `max_dt_s=300.0`
- All three active-default runs have `status=PASS`.

### 2) Active default vs explicit dx5 output identity (HBP/pass/loss/wat/trace)
- `active_default_vs_explicit_dx5` comparisons: 3/3 PASS, `mismatch_count=0` each.
- Explicit hash parity across required outputs is exact-match for all selected members:
  - `.hbp`, `.loss.json`, `.pass.parquet`, `.wat.parquet`, `laned_active_trace.jsonl`.

### 3) off_default vs off_mesh_env_control protected-output parity
- `off_default_vs_mesh_env_control` comparisons: 3/3 PASS, `mismatch_count=0` each.
- Protected outputs exact-match for all selected members:
  - `.hbp`, `.loss.json`, `.pass.parquet`, `.wat.parquet`.

### 4) Closure residual gates
Contract gate thresholds used by package evidence:
`clamp_rel <= 1e-12`, `max_supply_reconstruction_rel <= 1e-12`, `max_day_*_residual_rel <= 1e-10`.

Observed maxima across active runs:

- `max_day_cascade_residual_rel`: `4.705058001136025e-14`
- `max_day_identity_residual_rel`: `8.752483860221787e-14`
- `max_day_seam_residual_rel`: `4.832475752036399e-14`
- `max_supply_reconstruction_rel`: `8.378371644554163e-16`
- `max_clamp_rel_source`: `8.44135663405994e-19` (from explicit max `total_clamp_m3/total_source_m3`)

All are within contract gates.

### 5) Promotion-matrix consistency
- Matrix summary: `status=DX5_PRODUCTION_RATIFIED_BY_EVIDENCE`, `ratified_by_evidence=true`, `blockers=[]`, `row_count=21`.
- `verdict` counts: PASS `18`, report-only `3`, FAIL `0`.
- `candidate_vs_reference` and all `timestep_control` rows are PASS; one report-only fine-reference row has a documented shape delta (`mn_corn_h4` `dx2p5_dt300`).

## Verdict

PASS. The compact evidence artifacts independently confirm:

1. Active no-env default runs resolve to `target_dx_m=5.0` with required mesh-policy settings.
2. Active default output hashes match explicit `OPENWEPP_LANED_ACTIVE_MESH_TARGET_DX_M=5.0` for HBP/pass/wat/loss/trace.
3. `off_default` protected outputs match `off_mesh_env_control`.
4. Closure residuals are within gate tolerances.
5. Promotion matrix shows no blockers and ratification by evidence.

No rerun of the full selected-cohort comparator timing batch was performed in this verification pass.
