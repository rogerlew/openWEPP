# Implementation

Static: behavior-preserving cyclomatic decomposition only. Source SHA-256 after
formatting is
`c31512f697a5867ae089b599a9131de1247069fa50da03dfaa96248f748530e0`.

- Added typed column-view structs whose constructors preserve the exact
  original lookup order.
- Extracted one-row decoding and WAT-value decoding from `read_wat_batch`.
- Preserved eager `wepp_id` validation under a filename override, Area
  validation before date reads, area-map insertion before later fallible reads,
  Total-Soil-Water before `P` through `Irr`, and push-after-complete-row order.
- Did not edit tests, formulas, numeric expressions, accumulation, schemas,
  units, typed errors, public APIs, or output semantics.

Diff size: `278` inserted and `152` deleted lines in the single target file.

Final disposition: rejected by independent cover-first/eligible-row review.
Ran: the target-only implementation diff was mechanically reversed, then
`git diff --exit-code e2ff321e -- crates/openwepp-runner/src/totalwatsed3.rs`
exited `0`. No Rust or test edit remains in the hold package.
