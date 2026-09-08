# Contract-derived expected-red evidence

Ran: 2026-09-08 PDT in detached A-source-05 worktree
`/tmp/openwepp-residual-jacobian-FC2T1v/J` before candidate implementation.

Command:
`nix develop --offline -c cargo test -p openwepp-land-surface-energy
v33_stem_derivative_capability_is_not_present_in_baseline_a --no-run`

Result: expected FAIL, exit 101. Rustc E0425 reported
`experimental_v33_stem_derivative_capability` absent from `super::solver` at
`solver_stem_jacobian_tests.rs:5`. This proves the named baseline capability seam
did not exist; it does not prove derivative correctness. The compile-fail seam
was then removed so Phase A could build, and replaced by baseline-only selector
and oracle tests. Because the frozen oracle admitted 0/16 columns, no candidate
test or implementation was authorized.

Baseline kit verification separately PASSed all 10,147 declared source rows
with source identity `e9899b0e…`; no build was part of that verification.
