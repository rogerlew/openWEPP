# Rust Code Review

Status: `complete`

Verdict: `PASS`

Evidence class: `Static` for exact commit
`33831787b7029b28b0716c8458f08a11899db446`; `Ran` checks below were executed
at that exact `HEAD` unless explicitly identified as package-reported.

Review range:
`a65cc3973ddd04b07cad108fcb33d83a8c161abb..33831787b7029b28b0716c8458f08a11899db446`.
The reviewed hold-lift increment is
`ff7c918466dff144f13cb3aba6d3d39f736c1497..33831787b7029b28b0716c8458f08a11899db446`.

## Findings

No Critical, High, Medium, or Low Rust/science-contract correctness findings
remain at the reviewed identity.

## Resolved Prior High Finding

### Duration custody now has one dimensionally valid contract/runtime authority

`docs/specifications/science-contracts/contracts/SC-SED-001.md:374` now defines
`TOL-SED-009` as the absolute-seconds rule
`abs(watdur - Q / peakro_depth) <= 1.001e-9 s`. It explicitly excludes
scale-relative interpretation, sediment-continuity reuse, and absorption of
missing or mismatched hydrology operands. Revision 63 and the lifecycle index
describe the same active absolute-seconds authority.

The live erosion consumer defines the separately named
`DIRECT_EROD13_DURATION_CUSTODY_TOLERANCE_S` at
`crates/openwepp-hillslope-orchestrator/src/direct_runtime/erosion.rs:78-79`.
`validate_erod13_runoff_duration_closure` uses that constant directly at
`erosion.rs:1105-1117`; the previous addition of sediment-continuity and
water-depth constants is removed. Runtime failure uses `residual > tolerance`,
which exactly preserves the contract's `residual <= tolerance` admission
boundary. Guard precedence is unchanged: non-finite and non-positive depth,
peak, and duration operands fail before the custody comparison.

The behavioral regression at
`crates/openwepp-hillslope-orchestrator/src/direct_runtime/tests/erosion_hb01.rs:192-213`
exercises sub-threshold acceptance and supra-threshold typed failure at 0.25,
10, and 80,000 seconds. This distinguishes the absolute rule from rev62's
scale-relative expression. The contract regression at
`tests/integration/peak_hourly_authority_contract.rs:82-125` binds the named
source constant, its live comparison, rev63's exact seconds equation, and the
absence of both prior malformed tolerance forms. The former contract/runtime
drift is closed without a clamp, default, or error-taxonomy change.

## Confirmed Unchanged Corrections

Revision 63 preserves revision 61's correct internal `peakro_depth` in `m/s`,
public `peakro` in `m3/s`, shared maximum-hour authority,
rectangular-equivalent duration, source custody, public area conversion, and
no-fallback posture. The terminal source change is confined to centralizing
the duration tolerance and using it in the existing guard; it does not alter
WB14/WB16 arithmetic, serialization, H2637 routing isolation, calendar
identity, census behavior, or any other typed-error path. No substantial
duplicate science logic is introduced.

## Ran Evidence

- Reviewer-run duration boundary test: PASS, 1/1; nextest run
  `588da33e-a258-4924-bb2b-b21bd29f7150`.
- Reviewer-run `peak_hourly_authority_contract`: PASS, 4/4; nextest run
  `0e221cff-f96c-4066-8369-3e5411bdbc46`.
- Package-reported exact-head focused runs: duration boundary PASS, 1/1,
  nextest run `3856c183`; contract guard PASS, 4/4, nextest run `381239de`.
- `cargo fmt --all -- --check`: PASS.
- `git diff --check` passes for both `a65cc3973..33831787b` and the hold-lift
  increment `ff7c91846..33831787b`.

## Residual Risk And Missing Tests

- The multi-scale behavioral test perturbs duration above the reconstructed
  value. Static review confirms the live guard uses `abs`, but adding symmetric
  below-value perturbations would strengthen future regression detection.
- The warmed H2637 suite remains routing-focused and does not establish
  frost-active Lane D/WB16 coupling. The real pure-melt proof also remains
  split across R4K and downstream WB16 tests rather than one end-to-end vector.
- Terminal package disposition separately depends on its required exact-head
  workspace, doctest, cohort, verification, and disposition evidence.

## Approval

`PASS` for Rust/science-contract correctness at exact commit
`33831787b7029b28b0716c8458f08a11899db446`. The seconds-dimensional equation,
named runtime tolerance, live guard, behavioral boundary test, and
source-binding contract regression now agree. No code-review blocker remains.
