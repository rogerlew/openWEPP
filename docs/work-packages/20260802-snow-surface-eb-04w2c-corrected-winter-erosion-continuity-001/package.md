# SNOW-SURFACE-EB-04W2C Corrected-Winter Erosion Continuity Closure

Status: complete / technical, review, and terminal-verification PASS

Package ID: `SNOW-SURFACE-EB-04W2C`

## Objective

Close the cross-domain correctness failure exposed when EB-04W2B's corrected
warm-mean snowfall changes the McKenzie burn fixture's EROD16 concave
continuity instrument from `37/227` to `61/231` flux-closure refusals. Partition
the added refusals, establish numeric/process ownership, and either land an
authority-backed erosion correction or prove the existing refusal is correct.

This is a Defect-Closure ExecPlan. It may not stop after producing another
diagnostic fact while an in-envelope correction remains possible.

## Rationale

EB-04W2B corrects snow phase/activation and its real hydrology consumer, but
cannot close while the hard quick-profile EROD16 gate fails. The named failing
surface is not the exact sediment mass-balance identity. It is the independent
trapezoid-versus-RK4 discretization diagnostic on a deliberately stiff concave
validation profile. Its population gate is intended to prevent the instrument
from refusing away its own coverage. W2C must determine whether the newly
exposed tail is a solver defect, an instrument/population mismatch, or a valid
fail-closed response.

## Correction Authority Envelope

### Defect

- `EB04W2C-EROSION-FLUX-COVERAGE-001`: valid corrected-winter fixture forcing
  produces `61/231` concave-instrument flux-closure refusals, violating the
  prospectively hard `<=20%` coverage requirement and blocking quick/full
  correctness.

### In scope

- `SC-SED-001` authority for Wave-1 continuity, its discretization diagnostic,
  refusal behavior, and validation obligations.
- Wave-1 continuity numeric mechanics and diagnostics in
  `crates/openwepp-hillslope-orchestrator/src/direct_runtime/erosion_continuity.rs`.
- EROD16 fixture/instrument construction and contract-derived tests.
- Behavior-neutral diagnostic capture needed to partition corrected-versus-
  prior refusal populations.
- A contract-first numeric correction when root cause and authority support it.
- Prospective reconciliation of the instrument gate only if the evidence shows
  its current ratio is not a valid correctness predicate and canonical erosion
  authority is amended before the test.

### Protected boundaries

- Do not revert, narrow, special-case, or hide EB-04W2B's snow correction.
- Do not weaken `TOL-SED-005`, the hard telescoping sediment mass-balance
  identity, nonnegative load/capacity invariants, or typed fail-closed guards.
- Do not accept silently mis-integrated sediment, fabricate refused sediment,
  or make refusal counters disappear.
- Do not introduce surrogate/proxy erosion physics or tune snow/erosion
  coefficients against this fixture.
- Do not amend EB-04R/04S promotion outcomes or run the W2A terminal rerun until
  this package returns a clean prerequisite.

### Intended write set

- this package tree;
- `docs/specifications/science-contracts/contracts/SC-SED-001.md` and index only
  if authority changes;
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/erosion_continuity.rs`
  and directly owned module/test files;
- `tests/integration/erod16_wave1_continuity_fixture_conservation.rs` and a
  narrowly named W2C integration test if needed;
- `docs/planning/snow-surface-energy-balance-roadmap.md`, `docs/ROADMAP.md`, and
  `docs/work-packages/README.md`.

### Acceptance criteria

1. Reproduce the corrected `61/231` result and retain a per-storm partition of
   refusals versus clean solves.
2. Compare old-only, shared, corrected-only, and newly qualifying storms using
   forcing and solver-state operands without changing the snow correction.
3. Establish whether each refusal reflects mass-balance failure, numerical
   discretization error, or instrument-only quadrature disagreement.
4. Preserve independent exact sediment closure for every accepted solve and
   prove refused quanta remain explicit.
5. Make the EROD16 instrument pass for an authority-backed reason, not by
   weakening the `<=20%` assertion after seeing the result.
6. Pass focused EROD16/orchestrator gates, quick, frost, and Critical full
   correctness before returning W2B's prerequisite to green.
7. Complete exact-diff reconciliation, line-count governance, dual review with
   finding disposition, and dual terminal verification.

## Dependencies

- `AGENTS.md`
- `docs/codex_exec_plans.md`
- `docs/defect_closure_execplans.md`
- `docs/work-packages/AGENTS.md`
- `docs/standards/testing-and-gate-strategy.md`
- `docs/standards/kernel-work-package-preparation.md`
- `docs/standards/prompt-wording-guidance.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `docs/planning/snow-surface-energy-balance-roadmap.md`
- EB-04W2B package and terminal artifacts
- pinned legacy baseline `/workdir/wepp-forest_260430_baseline` at
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`

## Implementation Intent And Risk

- Intent: defect closure, contract/test/runtime implementation as required.
- Risk: Critical. This package may alter a production erosion continuity
  diagnostic or solver behavior and is the cross-domain prerequisite for snow
  campaign continuation.
- Authority classes: A0 sediment conservation and A1 hard invariants are
  binding; pinned legacy is A5/provenance and not a magnitude oracle.
- Assurance impact: assess the snow/frost report source and any erosion report
  dependency; do not publish or approve assurance content here.
- `cargo deny check`: required only if manifest/lock/dependency resolution
  changes.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to two independent review agents and, after finding
disposition, two independent terminal-verification agents. Expected outputs are
`artifacts/review_agent_a.md`, `artifacts/review_agent_b.md`,
`artifacts/verification_agent_a.md`, and `artifacts/verification_agent_b.md`.
Each agent has a bounded write set limited to its named artifact and may run
read-only inspection or validation commands; production, contract, roadmap,
catalog, and other package files remain read-only to those agents. The primary
agent owns finding disposition and any authorized corrective edits.

## Phase Plan

1. Scaffold and freeze authority, defect, write set, reading map, operand
   lineage, diagnostic partitions, and acceptance predicates.
2. Reproduce and retain the corrected failure; build a deterministic
   per-storm diagnostic partition and compare against the documented prior
   population without result-aware threshold edits.
3. If the root cause is in-envelope, amend/confirm `SC-SED-001`, author
   contract-derived red tests, record the pre-implementation gate, and land the
   direct production correction.
4. Reconstruct conservation and numeric diagnostics independently; run
   focused, affected-domain, quick, frost, and Critical full validation.
5. Conduct two reviews, disposition every finding, conduct two terminal
   verifications, reconcile the exact diff, and publish a truthful disposition.

## Conversion Rule

If this package establishes a reproducible root cause inside the envelope and
expected behavior is supported by canonical `SC-*` authority, pinned-baseline
provenance, or a contract-authorized physical invariant, it must proceed
through contract amendment/confirmation, contract-derived tests,
pre-implementation evidence, production correction, validation, and review. It
may not close as `HOLD` merely because further investigation is possible.

## Progress

- [x] (2026-08-02) User authorized scaffold and execution.
- [x] (2026-08-02) Package scaffold, authority envelope, protected boundaries,
  and prospective acceptance criteria authored before diagnostic/code edits.
- [x] (2026-08-02) Reproduced `61/231`, retained the prior/corrected storm
  partition, and isolated the lower-order diagnostic response from exact mass
  closure.
- [x] (2026-08-02) Amended `SC-SED-001` revision 56, recorded a red
  contract-derived test, and implemented matched-order diagnostic quadrature
  without changing physics, tolerance, refusal behavior, or the exact mass
  gate.
- [x] (2026-08-02) Focused EROD16, owning-crate, quick, frost, Critical full,
  warnings-denied clippy, formatting, and assurance validation pass.
- [x] (2026-08-02) Complete the two independent reviews, finding disposition, and two
  independent terminal verifications required for formal closure. The user
  explicitly authorized this delegated work on 2026-08-02.
- [x] (2026-08-02) Initial independent reviews returned HOLD with five findings
  each. All findings were accepted and corrected under revision 57; focused,
  EROD16, owning-crate, clippy, and formatting gates pass on the corrected tree.
- [x] (2026-08-02) Obtain fresh independent review of every accepted correction before
  terminal verification.
- [x] (2026-08-02) Both independent reviewers accept all corrections with no
  remaining findings.
- [x] (2026-08-02) Renew quick, frost, erosion, Critical full, owning-crate,
  warnings-denied clippy, formatting, doctest, and assurance gates on the
  accepted source.
- [x] (2026-08-02) Complete dual terminal verification.
- [x] (2026-08-02) Initial terminal verifiers accepted the technical
  correction and returned HOLD on complete review-history retention, exact
  diff/status/lint provenance, and kernel-profile conformance evidence.
- [x] (2026-08-02) Accepted every verification finding; added revision-58
  profile authority, calibration-not-applicable readiness, canonical finding
  inventories, and current terminal-evidence reconciliation surfaces.
- [x] (2026-08-02) First revision-58 review returned HOLD on exact readiness/
  unit schemas, Binding Exposure Index, step-local/degenerate/conversion
  detail, and final package/Markdown provenance. Every finding was accepted
  and corrected under revision 59.
- [x] (2026-08-02) Revision-59 QA review passed; Review A held the EROD13
  semantic BEI map and stale revision/schema wording. Both findings were
  accepted and corrected under revision 60.
- [x] (2026-08-02) Fresh dual review of revision 60 passed with no remaining
  findings after `R60-A-001` was accepted and corrected.
- [x] (2026-08-02) Dual terminal re-verification passed after
  `R60-VB-01` was accepted and corrected; no findings remain.

## Surprises And Discoveries

- The corrected population was not the prior 227 storms plus four new days.
  Thirty-one prior clean storms became refusals, ten prior refusals became
  clean, five new storms refused, and ten new storms were clean. The old
  diagnostic was responding to forcing-dependent solution curvature.
- All accepted solutions retained exact telescoping sediment closure. The hard
  failure was the order mismatch between cellwise trapezoids and the RK4/
  analytic Wave-1 solution, not a mass leak.
- Matched-order Simpson blocks reduce the explicit refusal tail to `4/231`
  while retaining the pre-existing `5e-3` bound.

## Decision Log

- Decision: treat the `<=20%` instrument-coverage failure as binding at intake.
  Rationale: it was a hard quick-profile assertion before W2C and cannot be
  relaxed retroactively without canonical erosion authority.
- Decision: correct the diagnostic order rather than the snow correction,
  Wave-1 physics, solver, grid, coefficient set, or population bound.
  Rationale: the retained old/current partition and unbounded solves show
  forcing-sensitive truncation disagreement while independently reconstructed
  mass closure remains valid.
- Decision: the initial lack of delegated review authority was a historical
  hold only. The user explicitly authorized reviews and terminal verification
  on 2026-08-02; the package and prompt now bind two reviewers followed by two
  verifiers.
- Decision: accept all initial-review findings and require fresh review before
  verification. Rationale: numerical-zone boundaries, behavioral anti-evasion,
  real-consumer disposition, in-place contract authority, independent per-cell
  reconstruction, validation provenance, and current authorization status are
  closure-relevant rather than optional cleanup.
- Decision: accept all terminal-verification governance findings without
  reopening production behavior. Rationale: both verifiers independently
  passed the technical correction, while the missing profile/history/diff
  evidence is mandatory package authority that can be corrected without
  changing the validated solver or test identities.

## Outcomes And Retrospective

The corrected implementation passed fresh independent review. `SC-SED-001` now distinguishes the exact
Wave-1 mass identity (`TOL-SED-007`) from its independent discretization check
(`TOL-SED-008`). The runtime applies matched-order nonoverlapping Simpson
blocks within recorded numerical sub-marches, with trapezoid retained only for
an unavoidable single interval. EROD16 passes with 227 clean/depositing storms and four
explicit refusals out of 231; the former quick-profile blocker and all focused
and broad terminal gates pass on the accepted source.

No snow process, erosion process equation, solver, grid, diagnostic tolerance,
exact mass tolerance, refusal semantics, coefficient, observation, or forcing
fixture changed. The correction therefore returns a technically clean
prerequisite to W2B. Revision 60 subsequently passed fresh
review and W2C's dual terminal-reverification obligation is completed.
