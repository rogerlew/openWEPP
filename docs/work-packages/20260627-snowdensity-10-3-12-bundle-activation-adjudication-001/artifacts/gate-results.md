# Gate Results

Evidence mode: Ran.

## Commands

- `.venv/bin/python -m py_compile tools/snowfreeze_observed/bundle_activation_adjudication.py`
- `cargo fmt`
- `cargo build -p openwepp-runner --bin openwepp-cli-hill`
- `.venv/bin/python tools/snowfreeze_observed/bundle_activation_adjudication.py`
- `cargo test --test snowdensity10_3_12_bundle_activation_adjudication -- --nocapture`
- `cargo test -p openwepp --test snowdensity03_physics_bulk_offline_contract -- --nocapture`
- `cargo clippy --test snowdensity10_3_12_bundle_activation_adjudication -- -D warnings`
- `cargo fmt --check`
- `git diff --check`
- `cargo test --workspace`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo deny check`
- `bash tools/release/check_authority_suite_antievasion.sh`
- `cargo test --test auth11_required_suite_obligation_guards_contract`
- `rg -n "qwet|frzftp" crates tools/snowfreeze_observed/bundle_activation_adjudication.py`
- `.venv/bin/python tools/snowfreeze_observed/bundle_activation_adjudication.py --skip-model-runs`

The `rg` source guard found only the package report field
`qwet_or_frzftp_changed = false` in the adjudication tool; no production
`qwet`/`frzftp` implementation references were introduced.

## Coupled WAT Summary

- Disposition: `HOLD-OPT-IN-BUNDLE`
- Activation policy: `POLICY-B`
- Activation blocker: `POLICY-B-FULL-SURFACE-NO-REGRESSION-EVIDENCE-MISSING`
- Frost-attribution blocker: `SNOW-CONTROL-RESIDUALS-REMAIN`
- Default failures: `1147`
- Holding-capacity-only failures: `761`
- Bundle failures: `498`
- Spring-densification failures: `502`
- Paired rows: `1415`
- Paired surfaces worse vs holding-only: `0`
- Trace rows selecting `coe_liquid_holding_capacity_v1`: `112502`
- Trace rows selecting `physics_bulk_density_compaction_v1`: `112502`

## Residual Profile

- Failure counts by residual sign:
  `MODELED_OVER_OBSERVED = 264`, `MODELED_UNDER_OBSERVED = 234`.
- Failure counts by month:
  November `5`, December `46`, January `138`, February `112`, March `170`,
  April `27`.
- March/April cap classes:
  `CAP_LIMITED_DEPLETION_REQUIRED = 33`,
  `COMPACTION_ONLY_FEASIBLE_WITHIN_522_CAP = 20`,
  `PATCHY_MELTOUT_OR_DEPLETION_REQUIRED = 16`,
  `UNDER_PERSISTENCE_OR_ACCUMULATION_DEFICIT = 128`.

Primary artifacts:

- `artifacts/bundle-activation-adjudication.json`
- `artifacts/bundle-activation-adjudication.md`

## Boundary Result

Default activation is not authorized because Policy-B full-model-surface
no-regression evidence was not produced. Paired observed snow-control residuals
still block frost attribution separately. No new process physics, parser/
runfile/user selector, fixture input, public output schema, density cap,
coefficient, canopy, radiation, phase, rain-heat, longwave, frost, Qwet/frzftp,
or compatibility-runtime change was made.
