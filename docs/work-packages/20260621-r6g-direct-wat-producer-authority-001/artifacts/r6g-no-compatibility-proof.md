# R6G No-Compatibility Proof

Status: executed-held.

Record static scans and code references proving direct WAT producers and
consumers do not source compatibility WB13 rows, compatibility runtime
surfaces, writeback payloads, or output rows as direct authority.

## Static Proof

- `build_retained_direct_publication_frame` receives parsed static runtime
  surface, climate request, climate span, execution lane, and direct lane
  frames. It does not receive compatibility WB13 output rows.
- `direct_publication_day_inputs` constructs a private `direct_seed_surface`
  from parsed static runtime inputs and daily climate values. It calls
  `seed_wb11_runtime_surface_inputs` on that private seed surface before
  translating operands into typed direct process inputs. The private seed is a
  pre-scheduler producer adapter, not a post-scheduler compatibility runtime
  surface.
- `build_hillslope_wat_rows_from_direct_publication` consumes
  `DirectRunPublicationFrame` rows only. Identity and simulation year are
  reconstructed from direct publication context rather than copied from WB13
  rows.
- Cutover gates compare direct artifacts to compatibility artifacts after both
  are built. That comparison is parity evidence only; it is not direct
  producer authority.
- The remaining R6G hold refuses to fill day-2 PMET `Es` or storage from WB13
  rows or post-scheduler runtime symbols. The required follow-on is a dynamic
  direct day-input builder that reads direct-carried layer state after each
  direct day commits.

## Static Scan Evidence

Ran:

- `rg -n "wb13_rows|build_hillslope_wat_rows_from_wb13|compatibility|writeback|writer row|writer_row|runtime_surface" crates/openwepp-runner/src/hillslope/04_direct_publication.rs crates/openwepp-runner/src/hillslope/02_output_and_climate_helpers.rs crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs crates/openwepp-hillslope-orchestrator/src/direct_runtime/projection.rs`

Disposition:

- Expected hits in `04_direct_publication.rs` are parity comparators and reduced
  mismatch helpers, not direct WAT producers. The direct producer entry point is
  `build_retained_direct_publication_frame` lines 1-43, and its day input
  source is `direct_publication_day_inputs` lines 526-591.
- Expected hits in `02_output_and_climate_helpers.rs` include compatibility
  output builders and runtime-surface helpers used elsewhere. The direct WAT
  consumer path is `build_hillslope_wat_rows_from_direct_publication` lines
  780-792 and `build_hillslope_wat_row_from_direct_publication` lines 795-819,
  which consume `DirectRunPublicationFrame` rows.
- Expected hits in `00_runner_intake_and_lane_setup.rs` include compatibility
  artifact construction for parity comparison and fail-closed gate messages.
  The R6G hold gate at lines 2294-2305 explicitly rejects using compatibility
  WB13 rows or runtime surfaces to fill the remaining PMET/storage fields.
- Expected hits in `direct_runtime.rs` are audit counters and direct runtime
  compatibility-edge counters inherited from prior R phases, not WAT producer
  authority. No R6G direct WAT producer writes from those counters.

## Proof Limits

This proof is sufficient for the R6G held reduction: direct WAT rows are not
filled from WB13 rows, compatibility output rows, writeback payloads, or
post-scheduler runtime surfaces. It is not a final R6 cutover proof. Complete
cutover still needs an allowlisted direct symbol ledger for every private seed
surface input, dynamic PMET operand, and lane-dimensional publication input.
