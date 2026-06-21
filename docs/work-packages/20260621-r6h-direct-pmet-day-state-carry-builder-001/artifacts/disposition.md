# R6H Disposition

Status: executed-held.

Final verdict:

`HOLD-R6H-WAT-PMET-LAYER-CARRY-ULP-PARITY`

## Summary

R6H cleared the prior R6G blocker:

`HOLD-R6G-WAT-PMET-DAY-STATE-CARRY-BUILDER-ABSENT`

The implementation replaced the precomputed direct-publication day-input vector
with an interleaved day/lane builder. The retained cutover frame now builds a
PMET seed surface for each `(day, lane)` after the prior direct day has
committed lane-carried direct state. Later-day PMET construction fails closed
if committed direct-carried layers are absent.

Current-fixture HBP identity remains green. Current-fixture WAT no longer
reduces to `Es`, `Total-Soil`, and `SoilWaterTotal`; storage totals are now
bit-identical. The remaining WAT residual is exactly `Es` on day 2 at
ulp-scale PMET layer carry:

- direct `pmet.wfevp_mm=11.93838347586016`;
- compatibility `pmet.wfevp_mm=11.938383475860162`;
- direct `pmet.es_m=0.0007677601843722604`;
- compatibility `pmet.es_m=0.0007677601843722608`.

## Code Changes

- Added `DirectFrameExecutor::run_publication_capture_with_interleaved_day_inputs`.
- Added `DirectRuntimeError::PublicationDayInputBuildFailure`.
- Reworked retained direct publication cutover to use
  `DirectPublicationDayInputBuilder`.
- Added direct lane-state overlay into private day seed surfaces, with
  fail-closed later-day layer requirements.
- Tightened R6H hold-marker classification so only exact first-row identity
  plus all-later ULP-scale `Es` residuals map to the R6H marker.
- Updated runner and CLI contracts so R6G no longer fires and R6H is the
  active fail-closed marker.
- Added/scaffolded follow-up package
  `docs/work-packages/20260621-r6i-direct-pmet-layer-ulp-parity-001/package.md`.

## Gate Results

Ran and passed:

- `cargo fmt --check`
- `cargo check -p openwepp-runner -p openwepp-hillslope-orchestrator`
- `cargo test -p openwepp-hillslope-orchestrator r6h_publication -- --nocapture`
- `cargo test -p openwepp-runner r2a_default_fixture_run_constructs_no_direct_runtime_skeleton -- --nocapture`
- `cargo test -p openwepp-runner r6h_ -- --nocapture`
- `cargo test -p openwepp-runner --test r6_direct_publication_cutover_cli_contract -- --nocapture`
- `cargo test --workspace`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo deny check`

The first `cargo test --workspace` run exposed a test-isolation race from the
new overlay test incrementing direct runtime audit counters concurrently with
the R2A default-disabled counter test. R6H fixed that test by taking the
existing runner execution lock; the replacement full workspace run passed.

## Held Gates

- WAT row/schema/metadata parity is not complete because `Es` still differs by
  ulp-scale PMET layer carry.
- Canonical multi-OFE WAT id authority remains held behind WAT parity.
- Independent WAT reconstruction is held for `Es`; storage totals are
  bit-identical.

## Next Action

Execute R6I:

`docs/work-packages/20260621-r6i-direct-pmet-layer-ulp-parity-001/package.md`

First action: localize and correct the first bit divergence in PMET
surface-layer carry feeding EVAPPM `wfevp`/`etkr`/`es_m`, without using
compatibility WB13 rows, compatibility runtime surfaces, writeback payloads,
writer rows, or output rows as authority.
