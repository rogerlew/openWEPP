# Gate Results

Status: complete

Evidence mode: ran

## Contract-First Gates

- Static: `SC-EVAP-001#INV-EVAP-017` and
  `SC-WATBAL-001#INV-WATBAL-039` were authored before production code edits.
- Ran: pre-implementation contract tests failed as expected.
  - `pre_impl_runtime_projection.log`
  - `pre_impl_wb17_root_uptake.log`
- Ran: post-implementation targeted tests passed.
  - `post_impl_targeted_hphys0251.log`
  - `post_impl_wb17_integration_full.log`

## Rust Gates

- Ran: `cargo fmt --check` passed.
- Ran: `cargo clippy --workspace --all-targets -- -D warnings` passed.
- Ran: `cargo test --workspace` passed.
- Ran: `cargo deny check` passed with existing duplicate/unmatched-license
  warnings only.

## Comparator Gates

- Ran: full `H1..H39` candidate batch completed with `39/39` runtime status `0`.
- Ran: semantic comparator completed with `39/39` status `0`.
- Ran: semantic pass remains `0/39`; package disposition remains `HOLD`.

## Anti-Evasion Guards

Static: not run. This package did not edit external-authority suite posture,
cohort fixtures, or required-case bindings.
