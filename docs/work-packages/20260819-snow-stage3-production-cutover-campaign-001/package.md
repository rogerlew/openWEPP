# Coordinate Stage 3 Production Cutover

Status: `queued / coordinator and Child 1 scaffolded`

Date: `2026-08-19`

Package ID: `20260819-snow-stage3-production-cutover-campaign-001`

Plan class: `Critical multi-package science, integration, qualification, and release campaign`

This ExecPlan is a living document maintained under
`docs/codex_exec_plans.md`.

## Purpose / Big Picture

Move ground snow from the current CoE production owner to the already admitted
Stage 3 owner without a partial or dual-owner deployment. The campaign first
joins the completed terminal snow event to the completed snow-free real-owner
stack, then closes unresolved turbulent-carrier authority, qualifies the whole
actual-scheduler candidate, obtains human assurance approval, and finally
changes ownership atomically. Canopy-intercepted snow remains a separate later
campaign.

## Current Authority Boundary

CoE remains the sole authoritative production snow-mass and melt generator.
Stage 3 is canonically admitted as the future sole melt owner, but its physical
efficacy, forest turbulent carrier, production selection, default, publication,
and cutover remain held. This coordinator cannot transfer acceptance between
children or use a later child to pass a current child's required gate.

## Child Order And Release Gates

1. `20260819-snow-stage3-terminal-meltout-lse-handoff-implementation-001`
   closes the mechanical terminal snow-to-land transition on isolated,
   default-off state while production remains unchanged.
2. `SNOW-STAGE3-TURBULENT-CARRIER-AUTHORITY-CLOSURE` decides and admits the
   applicable open/forest aerodynamic carrier without fitted attenuation.
3. `SNOW-STAGE3-REAL-CONSUMER-CUTOVER-QUALIFICATION` exercises the actual
   scheduler and complete owner stack over representative full snow seasons.
4. `SNOW-COE-STAGE3-ATOMIC-CUTOVER` changes the production owner/default and
   retires CoE generation in one stable increment.

Children execute in order. Child 2 may be prepared while Child 1 executes, but
Child 3 cannot start until both pass. The final-candidate `ASSURE-06` report
requires independent human scientific review and approval after Child 3 and
before Child 4. Child 4 is the first increment allowed to change production
ownership.

## Included Scope

The coordinator owns child sequencing, dependency and claim matrices,
campaign-wide exact-one custody, assurance timing, release/rollback policy,
campaign checkpoints, terminal exact-diff reconciliation, and roadmap/catalog
lifecycle. Each child owns its own contract, implementation, evidence, review,
verification, and truthful disposition.

## Excluded Scope

This coordinator does not itself authorize physics, implementation, empirical
calibration, selector/default changes, publication, deployment, or cutover. It
does not include canopy-intercepted snow, fitted wind multipliers, or CoE parity
as a correctness criterion.

## Intended Write Set

- `docs/work-packages/20260819-snow-stage3-production-cutover-campaign-001/**`
- the four child package trees as each is prospectively scaffolded;
- `docs/ROADMAP.md` and `docs/work-packages/README.md`;
- campaign-wide evidence summaries only after their underlying child evidence
  exists.

No production source is directly owned by the coordinator.

## Campaign Deliverables

- A child-release matrix that names every dependency and non-transferable gate.
- A complete owner/consumer chronology from snow interval start through routed
  post-meltout water and outputs.
- A turbulent-carrier authority decision with explicit forest/open domain.
- Actual-scheduler seasonal qualification with diagnostic CoE comparisons.
- Final-candidate `ASSURE-06` human review and approval evidence.
- One atomic owner/default cutover with a separately proven rollback release.
- Campaign-wide dual reviews, dual verification, exact-head release evidence,
  and final lifecycle reconciliation.

## Validation And Exit Criteria

The campaign closes only when every child has terminal PASS on its own exact
scope; no child borrows evidence from a later boundary; final-candidate
assurance has independent human approval; the exact production consumer reads
Stage 3 and cannot also receive CoE-generated liquid; restart, outputs, custody,
rollback, performance, full-workspace correctness, release, and dual-terminal
requirements pass on one exact clean cutover identity; and CoE generation is
retired in that same cutover increment. Any unmet required gate yields a
truthful campaign HOLD rather than partial activation.

## Security And Data Impact

No network, credential, external-message, protected-data mutation, or
deployment action is authorized by this scaffold. Each child must reassess its
exact diff. Assurance review/approval and release actions require their own
authorized workflows.

## Review And Subagent Authorization

Subagent authorization: this campaign explicitly authorizes spawning/delegating
to child-required science, ownership, Rust correctness, Rust QA, assurance,
release, comparator, and terminal-verifier roles. Expected outputs are compact
findings, exact command/count summaries, and artifact/log paths. Reviewers and
verifiers are read-only; comparator runners may write only ignored logs and
bounded package artifacts. Each child kickoff must repeat the exact roles and
limits it requires.

## Progress

- [x] (2026-08-19) Reconciled the completed root-zone and snow-free prerequisite
  lifecycle from their terminal package evidence.
- [x] (2026-08-19) Scaffolded the coordinator and mechanical handoff Child 1.
- [ ] Execute and terminally disposition Child 1.
- [ ] Scaffold, execute, and terminally disposition Child 2.
- [ ] Execute Child 3 actual-scheduler qualification.
- [ ] Obtain final-candidate `ASSURE-06` human review and approval.
- [ ] Execute Child 4 atomic cutover and campaign closure.

## Surprises & Discoveries

None at scaffold time.

## Decision Log

- Decision: preserve the historical terminal-handoff package as executed HOLD
  and create a fresh successor. Rationale: its two named prerequisites now
  exist, but historical evidence and lifecycle must remain immutable.
  Date/Author: 2026-08-19 / Codex.
- Decision: separate mechanical handoff, turbulent authority, qualification,
  and cutover. Rationale: they have distinct authority, write sets, evidence,
  and release consequences; mechanical success cannot imply physical efficacy
  or production readiness. Date/Author: 2026-08-19 / Codex.

## Outcomes & Retrospective

Queued. No runtime, selector, default, output, assurance, or ownership change
has occurred.
