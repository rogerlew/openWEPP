# Contract-test implementation evidence

Status: `SUSPENDED — V56/V57 FOCUSED PASS, CANONICAL UNRESOLVED`

Evidence mode: `Static + Ran`

V31 vectors are authored in
`v11_covered/open_snow_convergence_tests.rs` and the adaptive source-binding
test. They cover the captured `1860..1980 s` parent composed-trial endpoints
and exact 60-second child midpoint: mixed `0 C` liquid state versus dry frozen
`-76.68060832903426 C` authentic state. A separate oracle reconstructs endpoint
and midpoint `W/H`, ice/liquid/cold projection, melt/refreeze, cumulative
fields, and independent mass/energy residuals from immutable beginning and
complete support operands.

Static: exact `H=0` and `H=Lf W` boundary vectors require all-ice/zero-cold and
all-liquid/zero-unallocated-energy results respectively. Refusal vectors poison
vapor sign/disposition, component-energy closure, nonfinite energy, exact
support, lane set, and bitwise density. The captured midpoint asserts that
cumulative melt is reconstructed from the phase projection and is unequal to
componentwise arithmetic interpolation. Raw authentic retention and explicit
`publication_eligible=false` are required.

Static: the source-binding vector requires the helper to occur exactly once,
inside the nonconverged unpublished branch, before fresh-authentic
finalization. The branch must retain the raw authentic candidate in history,
assign only `iteration_stage3_states`, continue iteration, and expose no helper
reference in finalization/replay/acceptance/publication source.

Ran: the isolated unchanged-production sentinel command recorded in
`pre-implementation-contract-gate.md` exits 101 with exactly two `E0425`
errors for the missing v31 support-image type and helper. Full vector execution
is intentionally impossible until production is implemented. The rejected v30
4/4 and 34/34 runs remain historical attempt evidence only and are not reused
as v31 acceptance evidence.

## WGHL-FULL-001F covered no-update witness

Status: `FOCUSED PASS — DOWNSTREAM REAL-CONSUMER BLOCK`

Static: `SC-LANDSURFACEENERGY-001@13#INV-LANDSURFACEENERGY-139` binds the
existing full-trial witness followed by the first domain-valid `b>=1` halved
witness only when the complete current residual vector passes. The positive
predicate test covers full-witness refusal plus first-halved acceptance.
Refusals cover nonfinite/out-of-tolerance residuals, an already-passing full
witness, a later rather than first domain-valid halving, and independent
hydraulic/beta/temperature/humidity step poisons. `ci` remains diagnostic
because canonical authority defines no `ci` step threshold.

Ran: the revised expected-red gate on unchanged production failed both
interior-terminal vectors with the retained `FinalFixedCap` iteration-4
`LSEB-E-034` backtracking limit, run
`dd87636c-6728-4b2a-b601-5e36f42eddb0` (`0/2`).

Ran: predicate vectors pass `3/3`, run
`8cf71b71-1a6f-443a-abca-3144bb14ff4f`. The complete LSE crate, including
frozen covered oracle, genuine-update, natural backtracking/iteration failure,
transaction diagnostic and exact rollback vectors, passes `84/84`, run
`9a5aaf67-de5a-4c85-b149-225c52196c66`.

Static: post-review vectors independently name both admitted full-witness
refusal classes, refuse any nonpassing member of the complete residual vector
including a member NaN, refuse missing/later-domain-valid witness identity,
and refuse every governed threshold excess plus nonfinite governed steps. The
production preflight inspects only `b>=1` and stops at the first domain-valid
trial before any actual strict-decrease update search.

Static: the production preflight is now the exact private controller directly
exercised by two more vectors for invalid-until-first-complete ordering,
evaluation-incomplete refusal, failed-step refusal without later skip, and
no-trigger/no-probe behavior. Independent reviewers A and B closed their
solver-order and controller-test findings on this source. Fresh execution is
green: 5/5 focused, run `baaf9f04-769f-4de0-82bd-f98695c081db`; 87/87 complete
crate, run `dcd3e84b-d3ce-4bae-8960-df2c2a2c1767`.

Final hardening adds full-trial classification poisons for NaN, positive
infinity, and negative infinity in every governed coordinate. Any nonfinite
governed member makes the complete exact prospective classification
unavailable; only a fully finite vector with an actual governed threshold
excess can activate that typed refusal. Focused 6/6 and full LSE 103/103 pass,
runs `6efcec2e-2666-4a58-b911-80a2267bf0dd` and
`86f824a0-4486-4b9d-80ff-fe8fe0e8fbfd`.

Ran: both unchanged interior-terminal consumers advance beyond the prior LSE
backtracking failure and currently stop at the later shared Stage-3 identity
guard `qualification terminal snow-free successor chronology`, run
`ec067bbd-443d-45ce-ba76-5c4fdd2e252b`. No `LSEB-E-034` remains in either
path. Terminal real-consumer PASS awaits the concurrently owned chronology
correction and rerun; no consumer assertion was changed.

## SC-SNOWENERGY-001 v56/v57 test disposition

Status: `FOCUSED PASS — CANONICAL FAIL — SUSPENDED`

Ran: v57 expected-red source-contract evidence failed before implementation as
required (`1/2`, run `df31d505-4d9d-4526-a1f9-043e5b5cd5ec`). After
implementation, its source-contract vectors pass `2/2`, run
`511adad7-bb21-4c61-8d1a-e1ded7f79ee1`; v57 focused vectors pass `6/6` and
the retained v56 focused vectors pass `10/10`.

These focused results do not discharge the real-consumer gate. Canonical r151
reaches the specialization and fails at the 60-second `1800..1860 s` support
inside the frozen temperature-primary safeguarded physical solve. The one-day
fixture does not complete, so no step-count, width-distribution, qualified
runtime, or final ledger-closure test claim is available. The owner stopped
execution at r151; no V58 or further numerical successor test is authorized.
