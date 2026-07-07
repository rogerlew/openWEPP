# LANED Router D16 Hybrid No-Harm Selector Solve-Cost Hold Lift

Status: EXECUTED-COMPLETE-NOHARM-SELECTOR

## Objective

Lift the selected-cohort hybrid timing no-harm hold by implementing the first
staged selector increment authorized by `SC-OFEROUTE-002`: when
`OPENWEPP_LANED_ACTIVE_IMPLICIT=1` requests hybrid stepping, route hybrid only
for deterministic exact-bare-skin lane-days and route plain for non-bare
lane-days. Preserve the H2637 bare-skin speedup, remove selected-cohort
non-bare timing regressions, and publish request/selected/fallback counters.

This is selector-first staging. It does not claim broad forest/fleet hybrid
viability, non-bare solve-cost closure, tolerance ratification, or default
promotion.

## Rationale

The D16 hybrid viability adjudication found a real H2637 win but a selected
cohort no-harm failure: non-bare forest/row-crop cases paid generic
fixed-point solve cost without enough explicit-step savings. The previous
review allowed selector-first staging only if the package is honest that
routing plain for non-bare cases closes a no-harm stage, not the broader
non-bare hybrid value proposition.

The existing exact bare-skin evaluator already supplies a deterministic
zero-map-evaluation win class. This package makes that class the only selected
hybrid production path under the opt-in request and records why all other
lane-days fell back to plain.

## Required Reading

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `crates/AGENTS.md`
- `tests/AGENTS.md`
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-002.md`
- `docs/work-packages/20260707-laned-router-d16-hybrid-viability-adjudication-001/package.md`
- `docs/work-packages/20260707-laned-router-d16-hybrid-viability-adjudication-001/artifacts/worker-handoff.md`
- `docs/work-packages/20260707-laned-router-d16-hybrid-viability-adjudication-001/artifacts/review-claude.md`
- `docs/work-packages/20260707-laned-router-gap-ofehyb-002-solve-cost-ratification-001/artifacts/timing-and-fidelity.md`
- `docs/work-packages/20260707-laned-router-d16-selected-cohort-active-suite-001/package.md`
- `docs/work-packages/20260707-laned-router-d16-selected-cohort-active-suite-001/artifacts/active-suite-summary.json`

## Scope

Included:

- Amend `SC-OFEROUTE-002` before code to authorize deterministic
  exact-bare-skin no-harm selector staging.
- Implement active-path hybrid request/selection/fallback counters.
- Select hybrid only when the lane-day cell operand class is exact bare-skin;
  route plain otherwise.
- Keep subsystem-off/default behavior unchanged.
- Add focused tests for bare selection and non-bare fallback.
- Rerun H2637 and selected-cohort active plain-vs-hybrid timing evidence.
- Adjudicate whether selected-cohort timing no-harm is lifted.

Excluded:

- Default hybrid promotion.
- Production default activation without opt-in environment variables.
- Non-bare implicit solve-cost math changes.
- Post-hoc promotion tolerance tuning.
- First-divergent-day/OFE fidelity attribution for H2637. Fidelity deltas in
  this package are diagnostic only and cannot ratify promotion tolerances.
- Tier-2 mesh policy changes.

## Staging Declaration

This package intentionally chooses selector-first staging. The non-bare
solve-cost problem remains a separate hold unless selected-cohort evidence
unexpectedly proves material non-bare hybrid value without timing regression.

Success for this package means:

- H2637 or another exact-bare-skin lane-day still selects hybrid and preserves
  a material speedup.
- Non-bare selected-cohort cases route plain under the hybrid request, with
  counters proving fallback.
- Aggregate selected-cohort hybrid-request timing is not materially slower
  than active plain.

This does not unblock default promotion by itself because
`SC-OFEROUTE-002#INV-OFEHYB-008` still requires fidelity/tolerance ratification.

## Kill / Scope-Narrowing Criteria

Any one of these remains sufficient to abandon broad default-promotion pursuit
in a later package:

- Shrinking implicit eligibility into ratified pass-sediment/hydrograph-shape
  tolerances removes essentially all speedup.
- A non-bare solve-cost attempt cannot make generic non-bare implicit steps
  cheaper than the explicit steps they replace while preserving deterministic
  Z-rating branch discipline.
- Tier-2 5-cells/OFE mesh ratifies and supersedes the hybrid value pool.
- This package plus at most one successor fails to flip the cohort no-harm
  gate.

Scope narrowing precedes abandonment: if non-bare value remains out of reach
but bare/low-cover value remains real, the durable target is a narrowed
bare-skin-only opt-in hybrid surface for disturbed/burned post-fire classes,
not broad default promotion.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes
spawning/delegation to science-authority review, code review, comparator /
timing, and verification subagents. Expected outputs are package-local
`artifacts/review-*.md`, `artifacts/verification-*.md`, and compact timing /
comparator summaries. Write access is bounded to package artifacts unless a
subagent is explicitly assigned implementation fixes.

## Required Artifacts

- `artifacts/required-reading-map.md`
- `artifacts/contract-amendment.md`
- `artifacts/implementation.md`
- `artifacts/selector-policy.md`
- `artifacts/timing-comparator-evidence.md`
- `artifacts/gate-results.md`
- `artifacts/review_agent_a.md`
- `artifacts/review_agent_b.md`
- `artifacts/verification_agent_a.md`
- `artifacts/verification_agent_b.md`
- `artifacts/disposition.md`
- `artifacts/final-disposition.md`
- `artifacts/worker-handoff.md`

## Gates

- `git diff --check`
- Markdown/doc lint for touched docs.
- `SC-OFEROUTE-002` unit/profile/BEI compliance check.
- Focused Lane-D / `ofe_routing` tests covering selector counters and
  bare/non-bare selection.
- H2637 active plain-vs-hybrid timing run with opt-in routed path enabled.
- Selected-cohort active plain-vs-hybrid timing run with opt-in routed path
  enabled.
- Protected default/subsystem-off byte-identity evidence or static isolation
  audit if no default-output surface is touched.
- Active-mode closure evidence for selected hybrid and fallback lane-days.
- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo nextest run --workspace --profile full`
- `cargo deny check`

## Closure

`EXECUTED-COMPLETE-NOHARM-SELECTOR`: selected-cohort timing no-harm is lifted
for opt-in hybrid request at the current mesh. Default promotion and non-bare
solve-cost viability remain held; see `artifacts/final-disposition.md` and
`artifacts/worker-handoff.md`.
