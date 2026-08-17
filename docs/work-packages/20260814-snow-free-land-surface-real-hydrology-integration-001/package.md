# Execute Snow-Free Land-Surface and Real-Hydrology Integration

Status: `executing / Children 1-2 complete / Child 3 resumed after custody lift`

Date: `2026-08-14`

Package ID: `20260814-snow-free-land-surface-real-hydrology-integration-001`

Plan class: `Critical multi-package science and integration campaign`

This ExecPlan is a living document maintained under
`docs/codex_exec_plans.md`.

## Objective

Coordinate four independently closed child packages that admit a complete
snow-free ground/forest-floor energy model, connect V7 vegetation to the real
hillslope hydrology owner, implement the energy runtime as a default-off
shadow, and exercise both through a real hillslope scheduler consumer without
changing production selection or state.

## Protected Boundaries

The campaign begins at local commit
`0db1960129ad4f8fc4e292b20574dfe7229d5fe1`. It must not reset, pull, rebase,
push, create a branch or PR, activate a selector, alter production defaults,
or modify the completed V7 package or immutable V1--V7 model definitions.
Production hydrology and legacy ET remain unchanged; all new execution is
explicit and default-off.

## Child Order

1. `20260814-snow-free-land-surface-energy-authority-001`
2. `20260814-vegetation-real-hydrology-arbitration-shadow-001`
3. `20260814-snow-free-land-surface-energy-runtime-shadow-001`
4. `20260814-vegetation-land-surface-real-consumer-shadow-001`

Each child owns its terminal evidence. A PASS is not transferable between
children. This coordinator closes only after every child has a truthful
terminal disposition.

## Intended Write Set

The coordinator and child package trees; canonical contracts and contract
tests admitted by Child 1; dependency-neutral DTOs and affected crates named by
the later child packages; focused integration tests; Cargo manifests/lockfile
when required; and roadmap/catalog reconciliation. No production selector,
default, publication, or deployment path is in scope.

## Progress

- [x] (2026-08-14) Verified exact clean local base and older remote state.
- [x] (2026-08-14) Confirmed no existing exact-objective campaign package.
- [x] Complete Stage 0 owner, scheduler, authority-gap, and baseline freeze.
- [x] Complete Child 1 authority admission and terminal verification.
- [x] Complete Child 2 real-hydrology arbitration shadow.
- [x] Retain Child 3's exact LSE runtime and soil-layer real-owner bridge as a
  bounded passing checkpoint.
- [ ] Complete Child 3 LSE runtime shadow.
- [ ] Complete Child 4 real hillslope consumer shadow.
- [ ] Run campaign-wide reviews, heavy gates, terminal verification, archive
  prompts, and reconcile lifecycle documentation.

## Required Reviews And Delegation

Subagent authorization: this campaign explicitly authorizes and requires
subagent spawning/delegation to an independent land-surface science reviewer,
hydrology/ownership reviewer, Rust correctness reviewer, comparator suite
runner, and two terminal verifiers. Reviewers and verifiers are read-only;
expected outputs are compact findings and verdicts incorporated into the named
package artifacts. The comparator runner may write only ignored logs and
package gate artifacts.

## Exit Criteria

Close only when all four children pass their own reviews, gates and terminal
verification; the real default-off scheduler consumer proves exact-one custody
and atomic rollback; production bytes remain behaviorally unchanged; the docs
scan introduces no new broken link; and terminal limitations remain explicit.

## Decision Log

- Decision: retain four separate child closure boundaries under one campaign.
  Rationale: authority, hydrology extraction, runtime physics, and real-consumer
  integration have distinct evidence and cannot borrow acceptance.
  Date/Author: 2026-08-14 / Codex.

## Outcomes And Retrospective

The campaign is held at Child 3. The exact LSE runtime core and a default-off
bridge to the production soil-layer water owner pass their bounded focused
gates, but production hydrology exposes no persistent, restart-serialized
forest-floor/surface-liquid beginning store, candidate debit, ending state or
signed condensation-credit operation. Residue interception inputs,
depression/WAT5 diagnostics and snow liquid are not valid aliases.

The first lift action is a hydrology-owned per-OFE/tile surface or litter
liquid state with immutable snapshot, request/authorization/final-use debit,
condensation credit, capacity/infiltration/runoff joins and atomic rollback.
Child 3 then resumes in place. Child 4 has not started, campaign-wide heavy
gates and terminal verification have not run, and no campaign completion or
production claim is made.

## Resume disposition

The named dependency closed at `a7d692da4` with dual terminal PASS. The
historical campaign and Child-3 HOLD records above remain immutable; active
lifecycle now resumes Child 3 in place. Child 4, campaign heavy gates and
terminal verification remain pending.
