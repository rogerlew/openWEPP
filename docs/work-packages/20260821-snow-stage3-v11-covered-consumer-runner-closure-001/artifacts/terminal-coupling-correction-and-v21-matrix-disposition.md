# Terminal coupling correction and candidate-v21 matrix disposition

Status: `CORRECTION PASS / MATRIX DIVERGES / FINAL REVIEW PENDING`

Starting identity: `564f73949a1fc18b42d3c5bffeac9b85e8743f51`.
Last qualified physical implementation remains
`43cc9bbea2fbf5fe6ab6596cee4162de75cef999`.

## Correction

Ran: generic temperature-only exhaustion retains 32 iteration records, one
`IterationLoopExhausted` selection, the existing typed terminal coupling
nonconvergence result, and zero selected physical trials. Production
authorization now depends only on `selected_live_converged`; the diagnostic
post-loop three-component value remains observable but cannot authorize a
trial.

Ran: the real `BelowCarrierDomain` capture proves every real coupling group has
exactly two successful calls. Iteration zero has no incoming hint/comparison;
iteration one has four exact-zero differences and selects
`FourComponentConvergenceBreak`. The complete real transition boundary,
beginning/ending joints, child identity, precipitation sets, lower boundaries,
source receipts, LSE states, soil candidate/credit, and WB14 evidence projection
are identical when only hint and coupling ordinal vary. There is no real
`IterationLoopExhausted` group.

## Matrix

Ran:
`candidate_v21_effectivity_matrix.py` produced the adjacent JSON and Markdown
records: 28 affine-reference rows and 7 conservative two-node rows over the
exact required support set. All constitutive supports are at least 0.6 s;
selector behavior changes exactly at 1.2 s without blending; two-node total
storage conservation is exact.

Outcome: `DIVERGES`, with 20 failed componentwise enclosures. In the
`0.6 <= h < 1.2 s` smooth affine family, linear interpolation followed by
two-point Gauss integration reconstructs the same affine integral appearing in
the satisfied CN residual. Thus `d_H=0`, `e_H=0`, and `gamma*e_H=0`, while the
exact exponential endpoint establishes a nonzero CN error. No finite positive
`gamma` repairs an identically zero estimate. The real 1.875-second production
receipt remains retained, but candidate-CN effectivity there is not evaluable
without implementing the prohibited candidate operator.

## Validation

- Ran: focused generic exhaustion — PASS.
- Ran: focused lower capture — PASS.
- Ran: real BelowCarrierDomain capture, equivalence, ingress proofs and poison
  matrix — PASS.
- Ran: `cargo check --lib` and `cargo test --lib --no-run` — PASS with existing
  warnings.
- Ran: V20/V21 structural guards — 5/5 PASS, nextest run
  `76742f24-8cd0-4838-91dd-a0ede3b8fce8`.
- Ran: affected heavy suite — 845 passed / exact historical 11 failed / 1
  skipped, nextest run `33bb5f09-9bf5-4e8d-b785-b51323027f63`.

No equation, tolerance, 600 ms floor, acceptance predicate, controller,
event/root behavior, public API, production output, owner publication, restart,
receiver, runner, selector, default, CoE, Child 3 or cutover behavior changed.

## Frozen manifest

| SHA-256 | path |
|---|---|
| `d1bfde0928d93af8f92e437fec8c4f2b564cdbd55a063f63605e3ea9ab89564e` | `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation.rs` |
| `480e137dbe5d9f60c893949c3b9b3aba517df9ef00d079fdfae64e85f51752a2` | `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver/evaluation.rs` |
| `ba7c1e829d39c56d1275fb202585f17070c5a8dc74eba6ff8133f4a9a6002718` | `crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow_wb14_tests.rs` |
| `fd1eb67626aaf8dffab3118b7a2b2e57038b783ed594b07ad5e51b890946296c` | `candidate_v21_effectivity_matrix.py` |
| `36dabdea656a7c2fc2778940b09c2f8cec628994bc005e77e66a26dd49346d8e` | `candidate-v21-effectivity-conservation-matrix.json` |
| `8f7093a98dbb366b943a704270170b1c65c00db5e332f621f8050e32ebf39a4c` | `candidate-v21-effectivity-conservation-matrix.md` |
