# Implementation Test Evidence

Evidence mode: Static + Ran.

Status: executed-held.

## Code Changes

- Split direct-publication helpers into
  `crates/openwepp-runner/src/hillslope/04_direct_publication.rs`.
- Added `DirectPublicationDayInput` and
  `run_publication_capture_with_day_inputs`.
- Extended `ClimateDayProjection` with effective daily temperature.
- Changed cutover retained publication from hand-authored rows to a full
  `DirectPublicationExecution` built by the direct executor.
- Removed the per-day retained-row writer from the compatibility climate loop.
- Updated focused unit and CLI cutover tests for the HBP parity marker.

## Focused Tests

Ran:

```bash
cargo test -p openwepp-runner \
  r6e_cutover_candidate_reaches_direct_input_binding_then_fails_hbp_parity \
  -- --nocapture
cargo test -p openwepp-runner \
  r6_direct_publication_cutover_cli_flag_reaches_direct_binding_then_fails_hbp_parity \
  --test r6_direct_publication_cutover_cli_contract \
  -- --nocapture
```

Result: both pass.

## Direct Cutover Execution

Direct CLI reproduction result:

- exit status `1`;
- marker `HOLD-R6E-HBP-DIRECT-PROCESS-PARITY-MISMATCH`;
- HBP byte comparison reached with direct and compatibility byte lengths both
  `1654`;
- no public output files written.

## Retained / Rejected Candidates

Retained:

- line-count helper split;
- typed direct publication day input API;
- runner-side direct input binding from parsed climate;
- retained direct publication execution from direct capture.

Rejected:

- compatibility-row/runtime/writeback authority as a direct publication source;
- treating direct process parity mismatch as an output-writer issue;
- PASS parity claims on the current CLI fixture.
