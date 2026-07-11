# Verification Agent A

Status: `EXECUTED-PASS`

Evidence mode: `Static + Ran` on 2026-07-10.

## Verdict

`PASS`. Static review, every final-tree focused gate, and the definitive
frozen-tree full profile pass. The earlier Verification A finding and all later
Verification B findings are closed. Verification A finds no remaining blocker
and considers W11B eligible for final package disposition, subject to
Verification B's independent verdict and the package's other recorded gates.

## Ran evidence

| Command | Result |
|---|---|
| `cargo nextest run -p openwepp-watershed-orchestrator hourly_tests wshedw11b_enddet` | PASS, 23/23 |
| `cargo nextest run -p openwepp-watershed-orchestrator` | PASS, 105/105 before the final MC gate; the final focused selector passed after it |
| `cargo nextest run --test wshedw5_typed_watershed_runtime_contract` | PASS, 18/18 |
| `cargo nextest run -p openwepp-runner --test mt3_hbp_hourly_consumer_contract` | PASS, 2/2 |
| protected P102 five-class production CLI test | PASS, 1/1 |
| `cargo fmt --check` | PASS |
| `git diff --check` | PASS |
| `cargo nextest run --workspace --profile full` | PASS, run `f8ad9edd-774f-40da-b5ce-63a674e12890`; 1,677/1,677 passed, 4 slow, 3 skipped, 591.594 s |

The interrupted full runs are deliberately excluded: they overlapped later
accepted-finding fixes and therefore are not same-tree evidence.

## Accepted-finding closure

Static PASS:

- The active owner keeps channel inlet `qin` distinct from whole-reach lateral
  `qlat`, recurs from routed `q1`, preserves prior-day state, adds baseflow once,
  and supplies downstream channels with upstream water and class egress at the
  same interval index. Partial dependency state fails closed.
- The segmented pinned-baseline wave route uses the channel shape, celerity,
  capped spatial grid, adjacent-time lateral average, and separate KW/static
  MC/dynamic MC recurrences. The accepted Verification A finding is closed:
  KW uses `qtmax`, while both MC branches use
  `0.5 * (qtmin + qtmax)`; nonnegative inputs and baseline zero initialization
  make `qtmin == 0` in this owner. A branch anti-alias asserts 4.0 versus 2.0.
- Signed MC spatial state remains intact inside the grid. Pinned
  `wshchr.for:447-448,567-571` `1e-8 m3/s` normalization is applied only after
  selecting the outlet. The adjacent `qmaxi`/`qlavg` gate checks prior inlet,
  prior routed outlet, current inlet, and averaged lateral flow and leaves the
  pre-zeroed state untouched only when all four are zero.
- `GAP-ROUTE-014-A/B` are closed: capped widening reconstructs erosion before
  geometry, and low boundary shear after contact re-enters incision with the
  remaining-depth clock. ENDDET returns and consumes its solved bracket span.
- Gross class detachment is constructed independently over the applicable DCAP
  span; deposition is reconstructed from inlet plus lateral plus detachment
  minus egress with typed nonfinite/negative-residual guards. An exact
  zero-inlet/zero-lateral/zero-detachment class now produces exact zero egress,
  while nonzero invalid residuals still fail closed.
- Six geometry fields carry with cardinality, finite, side-depth,
  non-refill, and non-narrowing guards. Typed primary-tillage day authority is
  the only production reseed path.
- Multi-class production CLI setup reads and parses the required watershed
  soil, selects each channel-indexed surface layer, derives Rust `prtcmp`, and
  maps it to `crfrac` per pinned `convrt.for:84-88`. Missing, invalid, or
  incomplete authority fails closed; the direct core validates and normalizes
  the required class vector.
- The updated M-T3 timing assertion proves equal daily sediment publication for
  equal totals while independently requiring different interval egress timing.
  The protected event lane remains behind the active-lane branch and retains
  its event clock and normalization.
- `review-disposition.md` records every A/B/N/V finding as accepted and fixed;
  none remains rejected, deferred, open, or follow-up.

## Line-count governance

Final measured principal files: `hourly.rs` 1,734; `hourly_tests.rs` 985;
`direct.rs` 2,325; detachment owner 1,923; segment-routing owner 1,285; runner
CLI 2,350; runner behavior test 2,971; typed integration test 1,163. No touched
or new Rust file reaches 3,000 lines. The three pre-existing 2,000+ owners have
explicit WARN/no-block dispositions and bounded W11B changes.

## Eligibility

The definitive full run compiled the changed orchestrator and runner crates
before executing. Its direct log is `artifacts/logs/nextest-full.log`. With
accepted findings closed, focused/full tests green, and line governance
dispositioned, Verification A records `EXECUTED-PASS`.
