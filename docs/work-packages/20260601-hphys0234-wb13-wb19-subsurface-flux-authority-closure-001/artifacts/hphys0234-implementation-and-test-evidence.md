# HPHYS0234 Implementation and Test Evidence

Status: completed  
Evidence mode: mixed (`Ran` + `Static`)

## Production implementation

Static:
1. WB13 publication now resolves subsurface family from flux-authoritative
   symbols under state/flux conflicts:
   - `q` via `require_runtime_surface_scalar_prefer_flux(...)`,
   - `Qdd` via `require_runtime_surface_scalar_prefer_flux(...)`,
   - `Qd` via `require_runtime_surface_scalar_prefer_flux(...)`.
   File: `crates/openwepp-runner/src/hillslope/mod.rs`
2. Existing WB13 typed guard posture remains intact:
   - non-negativity checks for `q`, `Qdd`, `Qd`,
   - `Qd = latqcc + Tile` coupling closure hard-fail on mismatch.
   File: `crates/openwepp-runner/src/hillslope/mod.rs`
3. Added regression vector:
   - `hphys0234_wb13_subhyd_publication_prefers_flux_surface_over_stale_state_surface`.
   File: `crates/openwepp-runner/src/hillslope/mod.rs`

## Gate and test execution

Ran:
1. `cargo fmt --check`
2. `cargo clippy --workspace --all-targets -- -D warnings`
3. `cargo test --workspace`
4. `cargo deny check`

Observed:
- all required gates passed.
- `cargo deny check` exited success with existing duplicate/license-not-encountered
  warnings only.
- runner WB13 anti-shadow vector passed inside workspace test run.

## Cohort rerun and semantic comparison

Ran:
1. `H1..H39` rerun and semantic comparison batch for `unpalatable-rind`.
2. Semantic report aggregation for monitored HOLD columns.
3. Re-ran semantic comparison with pyarrow-enabled interpreter:
   `/tmp/hphys0233_20260601T211306Z/.venv/bin/python`
   after system Python lacked parquet dependency.

Evidence root:
- `/tmp/hphys0234_20260601T215019Z/parity/`

Observed:
- execution coverage: `39/39` hillslopes (`rc=0`) from
  `hillslope_batch_status_h_only.tsv`.
- semantic coverage: `39/39` reports (`rc=0`) from `semantic_status.tsv`.
- monitored-column summary:
  - `Dp`: `fail_count=39`, `mean_abs_diff_mean=0.22350421314678484`,
  - `latqcc`: `fail_count=39`, `mean_abs_diff_mean=0.7903973406116435`,
  - `Total-Soil`: `fail_count=39`, `mean_abs_diff_mean=134.12909172196171`,
  - `SoilWaterTotal`: `fail_count=39`, `mean_abs_diff_mean=134.12909172196171`,
  - `ProfileFCStore`: `fail_count=27`, `mean_abs_diff_mean=2.0526911601041165`.
