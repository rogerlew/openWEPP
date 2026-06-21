# R6F HBP Byte Diff

Status: complete.

## Candidate Byte Streams

| Stream | Source | Byte identity result |
|---|---|---|
| Direct HBP candidate | `build_hbp_output_from_direct_publication` fed by retained `DirectRunPublicationFrame` | Equal to compatibility for the inherited near-zero runoff fixture after R6F correction. |
| Compatibility HBP candidate | `build_hbp_output` fed by compatibility WB13/runtime output path for comparison only | Equal to direct for the inherited near-zero runoff fixture after R6F correction. |

## Reduction

The inherited HBP mismatch reduced to near-zero runoff event operands:

| Field | Direct before R6F | Compatibility | R6F correction |
|---|---:|---:|---|
| `peakro` | absent/zero fallback | `WB16_PEAKRO_FLOOR` | `direct_publication_peak_runoff_operands` emits `Some(WB16_PEAKRO_FLOOR)` when `q_runoff_m < WB16_RUNOFF_NEAR_ZERO_THRESHOLD`. |
| `watdur` | absent/zero fallback | `0.0` | Same direct helper emits `Some(0.0)` for near-zero runoff. |

## Validation

`cargo test -p openwepp-runner r6f_cutover_candidate_hbp_identity_exposes_wat_producer_gap -- --nocapture`

Result: passed. The test parses direct and compatibility HBP bytes with
`parse_hbp_from_bytes_with_latest_event_payload`, asserts latest-event payload
identity, and asserts full HBP byte identity before reducing the next WAT
blocker for the current CLI fixture.

## Authority

R6F did not source HBP values from WB13 rows or runtime surfaces as direct
authority. The direct helper derives near-zero runoff publication operands from
typed direct `q_runoff_m` and existing WB16 near-zero constants.

## Residual R6 Scope

This file closes the inherited R6E HBP blocker for the current near-zero runoff
fixture only. Architecture section 5.2.1 still requires a nonzero peak-runoff
and distinct event-duration fixture plus independent reconstruction before HBP
can be treated as fully closed for R6 cutover.
