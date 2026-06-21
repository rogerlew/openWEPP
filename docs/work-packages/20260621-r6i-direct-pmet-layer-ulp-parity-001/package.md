# R6I - Direct PMET Layer ULP Parity

Status: complete.

Package type: Defect-Closure ExecPlan / R6 direct WAT publication cutover.

Defect ID: `R6I-DIRECT-PMET-LAYER-ULP-PARITY`.

## Purpose

Close the R6H hold marker:

`HOLD-R6H-WAT-PMET-LAYER-CARRY-ULP-PARITY`

R6H replaced the precomputed PMET day-input vector with an interleaved direct
day/lane builder and cleared `HOLD-R6G-WAT-PMET-DAY-STATE-CARRY-BUILDER-ABSENT`.
The current-fixture WAT residual is now exactly `Es` on day 2, with storage
fields bit-identical. Implementation diagnostics reduced the remaining
difference to ulp-scale PMET surface-layer carry: direct
`pmet.wfevp_mm=11.93838347586016` and `pmet.es_m=0.0007677601843722604`,
compatibility `pmet.wfevp_mm=11.938383475860162` and
`pmet.es_m=0.0007677601843722608`.

## Required Outcome

Terminal states:

- `COMPLETE-R6I-DIRECT-PMET-LAYER-ULP-PARITY`: current-fixture WAT row/schema
  parity is byte/Arrow-identical through `Es`, `Total-Soil`, and
  `SoilWaterTotal`; R6G and R6H hold markers do not fire; current-fixture HBP
  identity remains green; direct PMET layer carry is proven without using WB13
  rows, compatibility runtime surfaces, writeback payloads, writer rows, or
  output rows as authority.
- `HOLD-R6I-<SPECIFIC-BOUNDARY>`: allowed only after field-level reduction,
  attempted or ruled-out exactness corrections in the direct ET/layer carry
  write set, dual review, verification, and a new exact follow-on package.

## In Scope

- Identify the first direct-vs-compatibility bit divergence in PMET
  surface-layer carry feeding EVAPPM `wfevp`/`etkr`.
- Correct direct ET layer-state carry arithmetic, ordering, or projection
  where canonical `SC-*` authority, pinned-baseline provenance, or established
  direct runtime invariants support the correction.
- Preserve the R6H interleaved day-input builder.
- Add focused tests that catch sub-ulp PMET `Es` drift without relaxing byte
  identity.
- Keep cutover fail-closed until full WAT parity passes.

## Out of Scope

- Filling `Es` or any PMET operand from compatibility WB13/runtime/writer
  surfaces.
- Rounding, tolerance-based publication, or bit nudging without authority.
- PASS/loss/manifest cutover unless WAT parity completes first.
- Default activation of direct publication.

## Validation Gates

- `cargo fmt --check`
- `cargo check -p openwepp-runner -p openwepp-hillslope-orchestrator`
- Focused R6H/R6I runner and direct-runtime tests proving WAT `Es` parity.
- CLI cutover fail-closed test, or public-write success only if all R6 gates
  pass.
- Static no-compatibility proof for the corrected path.
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `cargo deny check`
- Dual review and verification with disposition.

## First Implementation Step

Instrument or test the direct ET carry boundary to compare, without using
compatibility as authority, these direct operands across the day-1 to day-2
transition:

- PMET seed-surface `wb18_perc_theta_####` values;
- `pmet.wfevp_mm`, `pmet.etkr`, and `pmet.es_m`;
- direct ET `layer_state_after_soil_evap`;
- direct ET `layer_state_after_root_uptake`;
- direct lane `subsurface_layers` after commit.

Then fix the first direct arithmetic/order-of-operations discrepancy supported
by contract or pinned-baseline provenance.

## Progress

- [x] Scaffolded from R6H hold evidence.
- [x] Localize first PMET layer-carry bit divergence.
- [x] Correct direct layer carry exactness without compatibility aliases.
- [x] Prove WAT `Es` parity and no R6G/R6H marker.
- [x] Complete review, verification, and gate evidence.

## Final Disposition

`COMPLETE-R6I-DIRECT-PMET-LAYER-ULP-PARITY`.

R6I localized the first bit divergence to direct lane commit carry: direct
state carried the WB17 post-root-uptake layer vector, while compatibility
carried the post-runoff-reconciliation active-frost fine-layer topology
projection. The direct runtime now carries an explicit typed
`DirectFrostLayerCarryProjection` and applies the same coarse-to-fine-to-coarse
active liquid aggregation before storing lane subsurface layers for the next
day. The runner builds the projection from direct seed-surface frost options
and layer geometry; it does not source WB13 rows, compatibility runtime
surfaces, writer rows, or output rows as direct authority.

Current-fixture HBP identity and WAT identity are green. The R6G and R6H WAT
hold markers are absent. `DirectPublicationFrameCutover` still fails closed,
but the fail-closed blocker has moved past WAT/HBP/PMET to manifest writer
cutover: `manifest direct projection is not wired to the production manifest
writer`.

Evidence:

- `artifacts/execution-evidence.md`
- `artifacts/review-disposition.md`
- `artifacts/verification.md`
- `artifacts/line-count-governance.md`
- `artifacts/worker-handoff.md`
