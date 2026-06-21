# R6I Execution Evidence

Evidence class: Ran.

## Localization

R6I reproduced the inherited R6H state and localized the current-fixture WAT
`Es` residual to the day-1 to day-2 PMET seed boundary.

- Direct day-2 seed before the fix carried
  `pmet.wfevp_mm=11.93838347586016`,
  `pmet.es_m=0.0007677601843722604`.
- Compatibility day-2 seed carried
  `pmet.wfevp_mm=11.938383475860162`,
  `pmet.es_m=0.0007677601843722608`.
- Compatibility HPHYS trace showed the one-ULP layer change occurs after
  `runoff_reconciliation`, not in WB17 ET/root uptake and not in WB13
  publication.

Root cause: direct lane commit cloned
`evapotranspiration_compute.layer_state_after_root_uptake` into lane carry,
skipping the active-frost fine-layer topology projection applied during
runoff reconciliation before compatibility carries layer state forward.

## Implementation

Files changed:

- `crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs`
- `crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs`
- `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs`
- `crates/openwepp-runner/src/hillslope/03_tests.rs`
- `crates/openwepp-runner/src/hillslope/04_direct_publication.rs`
- `crates/openwepp-runner/tests/r6_direct_publication_cutover_cli_contract.rs`

Direct runtime changes:

- Added typed `DirectFrostLayerCarryProjection`.
- Added optional projection input to `DirectPublicationDayInput` and
  `DirectDayFrame`.
- Applied projection during lane commit before persisting
  `subsurface_layers`.
- Projection performs canonical coarse-to-fine default frost liquid-theta
  redistribution and fine-to-coarse active liquid aggregation.

Runner changes:

- The interleaved direct day-input builder now builds the carry projection
  from direct seed-surface symbols:
  `frost.options.wintRed`, `frost.options.fineTop`,
  `frost.options.fineBot`, and `wb19_dg_####`.
- Disabled or absent frost options produce no projection.
- R6H hold tests were updated to assert R6I closure and the next manifest
  fail-closed boundary.

## Focused Gates

Ran:

- `cargo test -p openwepp-hillslope-orchestrator r6i_direct_frost_carry_projection_preserves_fine_layer_aggregate_ulp -- --nocapture`
  - PASS.
- `cargo test -p openwepp-runner r6i_direct_day_two_pmet_seed_matches_compatibility_runtime_seed -- --nocapture`
  - PASS.
- `cargo test -p openwepp-runner r6i_cutover_candidate_hbp_and_wat_identity_clear_pmet_layer_ulp_gap -- --nocapture`
  - PASS.
- `cargo test -p openwepp-runner r6i_cutover_candidate_clears_pmet_layer_ulp_then_fails_manifest_cutover -- --nocapture`
  - PASS.
- `cargo test -p openwepp-runner --test r6_direct_publication_cutover_cli_contract -- --nocapture`
  - PASS.

## Closure Gates

Ran:

- `cargo fmt --check`
  - PASS.
- `cargo check -p openwepp-runner -p openwepp-hillslope-orchestrator`
  - PASS.
- `cargo clippy --workspace --all-targets -- -D warnings`
  - PASS.
- `cargo test --workspace`
  - PASS.
- `cargo deny check`
  - PASS: advisories, bans, licenses, and sources all OK.

## Static No-Compatibility Proof

Static:

- The production correction path is:
  `DirectPublicationDayInputBuilder::build_with_seed_surface` ->
  `direct_publication_frost_layer_carry_projection` ->
  `DirectPublicationDayInput.frost_layer_carry_projection` ->
  `DirectFrameExecutor::apply_publication_day_input` ->
  `DirectLaneFrame::commit_day` ->
  `apply_direct_frost_carry_projection`.
- That path consumes direct seed-surface frost option/layer symbols and direct
  lane-carried layer state.
- It does not read compatibility WB13 rows, compatibility runtime snapshots,
  writeback payloads, writer rows, output rows, or HPHYS trace files as
  authority.
- Compatibility references that remain in tests are comparison gates and
  marker-classification fixtures only.

## Result

R6I closes `HOLD-R6H-WAT-PMET-LAYER-CARRY-ULP-PARITY`.

Current-fixture HBP byte identity and WAT row value identity are green. The
R6 cutover candidate still fails closed before public output writes, now at
manifest writer cutover:

`manifest direct projection is not wired to the production manifest writer`.
