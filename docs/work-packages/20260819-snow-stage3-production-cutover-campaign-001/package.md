# Coordinate Stage 3 Production Cutover

Status: `active / Child 1 HOLD; Child 2A coupled-time authority next`

Date: `2026-08-19`

Package ID: `20260819-snow-stage3-production-cutover-campaign-001`

Plan class: `Critical multi-package science, integration, qualification, and release campaign`

This ExecPlan is a living document maintained under
`docs/codex_exec_plans.md`.

## Purpose / Big Picture

Move ground snow from the current CoE production owner to the already admitted
Stage 3 owner without a partial or dual-owner deployment. The campaign first
joins the completed terminal snow event to the completed snow-free real-owner
stack, then establishes shared time authority, introduces segmented
vegetation, closes unresolved turbulent-carrier authority, qualifies the whole
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
   is executed HOLD before production edits. It admitted the snow/LSE/liquid
   transaction contracts, but exposed missing V10 partial-support and
   snow-covered pre-event vegetation authority. It resumes after Child 2.
2. Child 2 is a continuously executed three-increment coordinator:
   - **2A** [`COUPLED-TIME-AUTHORITY-IMPLEMENTATION`](../20260820-coupled-time-authority-implementation-001/package.md)
     admits and implements exact-one coupled clock custody, exact integer
     support, physical segments, common accepted slabs, attempt rollback,
     restart cursor/controller state, atomic owner acceptance, and buffered
     parent publication.
   - **2B** `C3-WOODY-V11-SEGMENTED-SUPPORT` preserves immutable V10 and
     supersedes transaction-time integration only, with exact full-support
     compatibility, sequential segments, one parent finalization and commit.
   - **2C** `SNOW-STAGE3-TURBULENT-CARRIER-AUTHORITY-CLOSURE` admits the shared
     snow/canopy/vegetation carrier and snow-covered pre-event chronology.
3. `SNOW-STAGE3-REAL-CONSUMER-CUTOVER-QUALIFICATION` exercises the actual
   scheduler and complete owner stack over representative full snow seasons.
4. `SNOW-COE-STAGE3-ATOMIC-CUTOVER` changes the production owner/default and
   retires CoE generation in one stable increment.

Children execute in order. Child 2A--2C execute continuously, but each retains
its own authority and evidence boundary. Child 1 resumes only after 2A--2C
pass, and Child 3 cannot start until resumed Child 1 passes. The final-candidate
`ASSURE-06` report
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
- the child package trees as each is prospectively scaffolded;
- `docs/ROADMAP.md` and `docs/work-packages/README.md`;
- campaign-wide evidence summaries only after their underlying child evidence
  exists.

No production source is directly owned by the coordinator.

## Campaign Deliverables

- A child-release matrix that names every dependency and non-transferable gate.
- A complete owner/consumer chronology from snow interval start through routed
  post-meltout water and outputs.
- A turbulent-carrier authority decision with explicit forest/open domain.
- One reusable coupled-time authority consumed by the V11 and later Richards
  campaigns; no vegetation-only `transaction_support_s` exception.
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
- [x] (2026-08-19) Executed Child 1 through its contract checkpoint and
  dispositioned HOLD before production edits on a dual-reviewed vegetation
  support/carrier authority boundary.
- [x] (2026-08-20) Split Child 2 into coupled-time (2A), V11 segmented support
  (2B), and snow-covered carrier (2C), and scaffolded 2A.
- [ ] Execute and terminally disposition 2A--2C continuously; then resume Child
  1 from checkpoint `83cf6eb8e`.
- [ ] Execute Child 3 actual-scheduler qualification.
- [ ] Obtain final-candidate `ASSURE-06` human review and approval.
- [ ] Execute Child 4 atomic cutover and campaign closure.

## Surprises & Discoveries

- Child 1 proved that `VegetationConfiguration.dt_s` is exact configuration,
  receipt, and state identity. A remaining-support receiver cannot safely
  mutate or scale it. Authority must also decide vegetation evolution on the
  snow-covered half before `t*`, which couples the mechanical handoff to Child
  2's turbulent-carrier decision.
- The same missing authority governs snow terminal events, adaptive Richards
  slabs, Lane D CFL limits, soil--plant iterations, threshold events, restart,
  and output chronology. Duration is therefore a cross-domain state/transaction
  authority, not a vegetation override.

## Decision Log

- Decision: preserve the historical terminal-handoff package as executed HOLD
  and create a fresh successor. Rationale: its two named prerequisites now
  exist, but historical evidence and lifecycle must remain immutable.
  Date/Author: 2026-08-19 / Codex.
- Decision: separate mechanical handoff, turbulent authority, qualification,
  and cutover. Rationale: they have distinct authority, write sets, evidence,
  and release consequences; mechanical success cannot imply physical efficacy
  or production readiness. Date/Author: 2026-08-19 / Codex.
- Decision: decompose former Child 2 into 2A coupled time, 2B V11 segmented
  vegetation, and 2C snow-covered carrier, executed continuously. Rationale:
  time identity/atomicity is reusable infrastructure; vegetation integration
  and snow-carrier equations remain distinct science authorities. V10 remains
  immutable and Richards must later import 2A rather than define another
  clock. Date/Author: 2026-08-20 / Codex.

## Outcomes & Retrospective

Active HOLD at Child 1. Its bounded contract checkpoint exists, but no terminal
receiver runtime, selector, default, output, assurance, or ownership change has
occurred.
