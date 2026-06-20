# Verification Agent B

Status: complete.

Evidence class: Static/Ran.

## Verification

- Static: direct-runtime forbidden-token scan covered `direct_runtime.rs`,
  `direct_runtime/storage.rs`, `direct_runtime/runoff.rs`, and
  `direct_runtime/subsurface.rs`; no matches were found.
- Static: scheduler diff was empty.
- Ran: H2637 default-disabled reps were `643.70 s`, `646.33 s`, and
  `639.62 s`, giving median `643.70 s`.
- Ran: PASS row identity matched the PERFDEEP07 default-disabled baseline with
  zero row differences.
- Ran: line-count closure recheck found no touched production `.rs` file in
  the 2000-line WARN band.

Gate Evidence Non-Deferral Rule: PASS. No accepted gate remains pending or
blocked.
