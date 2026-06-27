# SNOWDENSITY-10.3.8 Closeout

Evidence mode: Static/Ran.

Disposition: `COMPLETE-OPT-IN-IMPROVEMENT`.

## What Changed

- Added `coe_liquid_holding_capacity_v1` as an opt-in CoE melt/liquid model.
- Added persistent retained-liquid snow-lane carry state.
- Added package-bound direct-production selector
  `OPENWEPP_SNOWDENSITY1038_MELT_MODEL`.
- Added snowbench diagnostic columns for liquid holding capacity, retained
  liquid, and released liquid.
- Amended `SC-SNOWFREEZE-001` to v95 with `INV-SNOWFREEZE-067` and
  `OBL-SNOWFREEZE-P-042`.

## Protected Boundaries

Unchanged: `legacy_coe` default/rollback, CoE melt terms, radiation, canopy,
phase partition, density constants, frost, rain heat, sub-canopy longwave,
Qwet/frzftp, fixtures, public schemas, parser/runfile/user surfaces, and
compatibility runtime.

## Evidence

- Event-window report:
  `artifacts/liquid-holding-capacity-event-window.json`
- Coupled WAT report:
  `artifacts/liquid-holding-capacity-coupled-wat.json`

Event-window result:

- Disposition: `WINTER-THAW-MELT-RESPONSE-CANDIDATE-IMPROVES`
- Under-ablation windows: `132 -> 94`
- Aggregate depth-loss deficit: `24.105059374337998 m -> 15.506372398659 m`
- Candidate conservation: passed

Coupled WAT result:

- Disposition: `WINTER-THAW-COUPLED-WAT-IMPROVES`
- Paired snow-control failures: `1147 -> 761`
- Candidate paired row count: `1415`
- No paired surface worsened

## Validation

Ran:

- `cargo build -p openwepp-runner --bin openwepp-snowbench`
- `cargo build -p openwepp-runner --bin openwepp-cli-hill`
- `.venv/bin/python -m py_compile tools/snowfreeze_observed/winter_thaw_melt_response.py tools/snowfreeze_observed/winter_thaw_melt_response_correction.py tools/snowfreeze_observed/winter_thaw_melt_response_coupled_gate.py tools/snowfreeze_observed/maritime_overaccumulation_diagnosis.py`
- `.venv/bin/python tools/snowfreeze_observed/winter_thaw_melt_response_correction.py --output-dir target/snowdensity10_3_8_liquid_holding_capacity/event-window --package-artifacts-dir docs/work-packages/20260627-snowdensity-10-3-8-liquid-holding-capacity-001/artifacts --candidate-model coe_liquid_holding_capacity_v1 --schema snowdensity10-3-8-liquid-holding-capacity-event-window-v1 --contract 'SC-SNOWFREEZE-001 INV-SNOWFREEZE-067 OBL-SNOWFREEZE-P-042' --artifact-stem liquid-holding-capacity-event-window`
- `.venv/bin/python tools/snowfreeze_observed/winter_thaw_melt_response_coupled_gate.py --output-dir target/snowdensity10_3_8_liquid_holding_capacity/coupled-wat --package-artifacts-dir docs/work-packages/20260627-snowdensity-10-3-8-liquid-holding-capacity-001/artifacts --melt-env OPENWEPP_SNOWDENSITY1038_MELT_MODEL --candidate-model coe_liquid_holding_capacity_v1 --schema snowdensity10-3-8-liquid-holding-capacity-coupled-wat-v1 --contract 'SC-SNOWFREEZE-001 INV-SNOWFREEZE-067 OBL-SNOWFREEZE-P-042' --artifact-stem liquid-holding-capacity-coupled-wat`
- `cargo test --test snowdensity10_3_7_winter_thaw_melt_response_correction --test snowdensity10_3_8_liquid_holding_capacity -- --nocapture`
- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `cargo deny check`
- `bash tools/release/check_authority_suite_antievasion.sh`
- `cargo test --test auth11_required_suite_obligation_guards_contract`
- `markdown-doc lint --path docs/planning/snow-frost-fidelity-strategy.md --path docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md --path docs/work-packages/README.md --path docs/work-packages/20260627-snowdensity-10-3-8-liquid-holding-capacity-001 --format json`
- `git diff --check`

Full-test fixups:

- Updated the 10.3.1a snowbench CSV schema assertion to include the new
  liquid-capacity diagnostic columns while preserving the original conservation
  operand checks.
- Updated the direct-runtime layout guard to the measured `DirectLaneFrame`
  bound of `1216` bytes; the `+8` byte growth is the package-authorized
  retained-liquid lane state.

## Carry-Forward

Residual blocker: `SNOW-CONTROL-NOT-CLEARED`.

The opt-in candidate materially improves snow-depth control but does not clear
the snow-control gate. It is not default activation and does not by itself
authorize frost attribution.
