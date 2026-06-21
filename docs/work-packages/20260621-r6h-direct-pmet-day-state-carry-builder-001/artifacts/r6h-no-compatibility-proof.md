# R6H No-Compatibility Proof

Status: complete-with-hold.

Record static scans and code references proving direct WAT producers and
consumers do not source compatibility WB13 rows, compatibility runtime
surfaces, writeback payloads, writer rows, or output rows as direct authority.

## Required Proof Shape

- Direct day-input builder entry point and direct runtime commit flow.
- Every dynamic PMET operand and its direct source.
- Every private seed-surface symbol and why it is pre-scheduler direct input
  authority rather than post-scheduler compatibility state.
- Direct WAT row builder and downstream consumer path.
- Static scan results with expected-hit disposition.

## Rejected Proof

It is not enough to show that direct rows equal compatibility rows. Equality is
parity evidence only; it is not producer authority.

## Static Authority Map

- Direct input execution: `DirectFrameExecutor::run_publication_capture_with_interleaved_day_inputs`
  calls the runner-supplied builder inside the direct day/lane commit loop.
- Runner builder: `DirectPublicationDayInputBuilder::build` starts from parsed
  static input surfaces and day climate, overlays committed direct lane layer
  state, and publishes typed `DirectPublicationDayInput` values.
- Direct WAT consumer: `build_hillslope_wat_rows_from_direct_publication`
  consumes `DirectRunPublicationFrame` rows.
- Compatibility comparators: `build_hillslope_wat_rows(&execution.wb13_rows)`
  remains a gate comparator only after direct artifacts are built.

## Expected Hits

The touched runner files still contain compatibility symbols because the
fail-closed cutover gate compares direct artifacts to compatibility outputs.
Those comparator reads are not direct-publication authority.

## Negative Evidence

Focused tests:

- `r6h_cutover_candidate_hbp_identity_reduces_wat_to_pmet_layer_ulp_gap`
  proves direct WAT rows are produced first and then compared.
- `r6h_cutover_candidate_clears_day_state_carry_then_fails_pmet_layer_ulp_parity`
  proves the old R6G compatibility-stale marker no longer fires.
- `r6_direct_publication_cutover_cli_flag_reaches_hbp_identity_then_fails_pmet_layer_ulp_parity`
  proves no partial public outputs are written while the direct gate fails.
