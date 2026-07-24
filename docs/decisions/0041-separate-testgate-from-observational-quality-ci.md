# ADR-0041: Separate TESTGATE admission from observational quality CI

**Status:** Accepted

**Date:** 2026-07-24 UTC

**Decider:** Roger Lew

**Decision authority:** Roger Lew's 2026-07-24 ratification of the
[TESTGATE and quality-observatory roadmap](../work-packages/testgate-quality-observatory-roadmap.md)

**Amends:** [ADR-0021](0021-module-coverage-closure-thresholds.md),
[ADR-0039](0039-campaign-scoped-risk-based-testing-and-assurance-gates.md), and
[ADR-0040](0040-accelerated-testgate-cutover-on-trusted-self-hosted-runner.md)

## Context

TESTGATE was intended to make scientific-development closure faster by running
the mechanically affected correctness surface on forest1. In qualification,
the functional full profile passed, but global CRAP then treated production
snowbench functions as 0% covered because their covering tests had correctly
moved to the separate `science-manual` profile. The failure was a measurement
profile defect, not a newly discovered correctness defect.

Coupling this slow, profile-sensitive maintainability observation to increment
admission produced repeated heavy runs, obscured successful correctness
evidence, and delayed science work. Coverage and CRAP describe exercised code
and change risk. They do not independently establish scientific accuracy.

## Decision

1. **Separate authority.** TESTGATE determines whether an increment is
   admissible. The quality observatory measures maintainability debt. Neither
   may impersonate the other.
2. **TESTGATE remains manual and blocking.** It runs the mechanically selected
   correctness DAG on forest1, including applicable formatting, Clippy,
   component/contract tests, A0/A1/A3 authority, typed guards, conservation,
   reconstruction, consumer, serialization, publication, anti-evasion, and
   other selected science gates. Ordinary TESTGATE does not execute coverage or
   CRAP.
3. **Record explicit deferral.** A TESTGATE plan and receipt record coverage and
   CRAP as `DEFERRED_TO_QUALITY_CI`. This is a truthful non-blocking
   disposition, not `PASS`, `SKIPPED`, `NOT_APPLICABLE`, or a waiver. Order 2
   implements and independently reconstructs the typed state.
4. **Quality CI is optional and observational.** The forest1 quality
   observatory runs only at operator direction. Its valid execution succeeds
   independently of whether actionable debt is present. Reports carry
   `closure_eligible=false`; their absence, staleness, debt verdict, or
   actionable-row count does not block increment, campaign, or release
   transition.
5. **Campaign and release do not restore the gate.** Campaign closure and
   release qualification require their selected correctness, integration,
   authority, stability, provenance, assurance, and distribution gates, but
   they do not require coverage/CRAP execution, a current quality report, or an
   empty actionable set. An operator may attach a quality report as
   observational evidence. Promoting it back to a transition gate requires a
   later accepted ADR.
6. **Preserve the quality model.** ADR-0021's percentages, region/line
   definitions, per-function floor, CRAP formula and threshold 30, symbol-level
   eligibility taxonomy, exact adjudication registry, and exception discipline
   remain the quality-observation and remediation authority.
7. **Retain package-local metric closure.** Coverage/CRAP remain binding when an
   explicitly authorized package's objective is module test enhancement,
   coverage closure, CRAP reduction, or CQR. That package must satisfy its
   declared target metrics before claiming that metric objective complete.
   These package-local claims do not make the same metrics a gate for unrelated
   feature, science, campaign, or release closure.
8. **Measure the complete intended profile set.** The quality observatory
   accumulates one source-frozen instrumented run of `full` followed by
   `science-manual`, then derives one merged coverage identity and global
   adjudicated CRAP report. Its exact compact evidence may seed
   operator-directed CQR Nightly. After Order 5 implements the handoff, only
   typed `STALE` or `INVALID` evidence plus an explicit operator CQR directive
   permits recollection.
9. **Prioritize TESTGATE.** Optional QA never competes with live forest1
   TESTGATE work. Permanently queued records from the retired Omarchy runner
   are historical metadata: ignore them, never wait for or cancel them, and
   never count them as forest1 occupancy.
10. **Preserve history.** Existing TESTGATE attempts, receipts, CRAP reports,
    and failures retain their original bytes and verdicts. New authority may
    prospectively supersede an acceptance obligation; it does not rewrite a
    historical failure as a pass. An old incompatible receipt receives a
    separate `REJECTED_INCOMPATIBLE_RECEIPT` import decision.

## Supersession

This ADR supersedes:

- ADR-0021 Decisions 1, 5, 6, and 8 only where they make coverage/CRAP a
  transition gate outside an explicit metric-focused package;
- ADR-0039 Decisions 4 and 6 and related critical/campaign/release language
  requiring affected or global coverage/CRAP; and
- ADR-0040 only where its preservation language carries those prospective
  transition obligations forward.

It does not supersede scientific correctness authority, the ADR-0021 quality
model, historical cutover evidence, or package-local CQR/test-enhancement
acceptance.

## Consequences

- TESTGATE can close an increment after correctness admission without waiting
  for slow maintainability measurement.
- Quality debt remains visible, identity-bound, and actionable without being
  mislabeled scientific failure.
- CQR becomes an operator-directed response to a valid observation rather than
  a prerequisite to ordinary science work.
- Order 2 must align executable policy, planner, executor, verifier, schemas,
  workflows, and release consumers before changed-head qualification.
