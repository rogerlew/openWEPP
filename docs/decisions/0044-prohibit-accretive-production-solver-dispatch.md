# ADR-0044: Prohibit accretive production solver dispatch

**Status:** Accepted

**Date:** 2026-09-01 UTC

**Deciders:** Roger Lew, Codex

**Applies with:** ADR-0011, ADR-0025, ADR-0031, ADR-0037,
`docs/standards/numerical-solver-architecture.md`

## Context

The Stage 3 workspace-gate effort accumulated numerical revisions v33 through
v57. Each revision added a narrower eligibility rule, coordinate system,
witness search, polishing path, or custody mechanism while retaining prior
solvers for ineligible or unresolved cases. The resulting runtime could spend
minutes on less than one simulated hour, remained unresolved at canonical
r151, and became difficult to reason about because current behavior depended on
which historical branch accepted or failed first.

This is not unique to snow. Keeping an old solver as live rollback appears
conservative, but it multiplies runtime cost, branch/test surface, diagnostics,
and invariant interactions. It also converts each new numerical mismatch into
an incentive to add another specialization instead of correcting or replacing
the canonical algorithm.

Distinct physical regimes can require different equations and solvers. That is
not the same as trying multiple historical algorithms for one regime after
eligibility or convergence failures.

## Decision

1. Production numerical solvers may not accrete historical-version,
   eligibility-chain, convergence, or failure-recovery fallbacks.
2. Each authoritative physical regime has exactly one canonical production
   solver, selected before iteration from physical state rather than solver
   history.
3. A successor replaces and deletes the superseded production solver for that
   regime in the same stable increment. Release/git history, not live old code,
   provides rollback.
4. Nonconvergence may use only the canonical solver's bounded internal response,
   retry that same solver through the canonical adaptive time-support
   controller, or return a typed failure.
5. Bounded line searches, trust regions, preconditioners, bracketing, and other
   components are permitted inside one prospectively specified canonical
   algorithm. Historical solvers cannot be relabeled as components.
6. Historical persisted schemas may be decoded and migrated into current state,
   but schema versions cannot select historical numerical behavior.
7. Comparison or shadow solvers must be unreachable from production admission
   and publication and must have a named deletion gate.
8. Work packages changing a solver must provide a regime/solver inventory,
   superseded-path deletion map, physical tolerance and evaluation-budget
   authority, failure-path proof, real-consumer evidence, and representative
   performance qualification.

The normative implementation rules are maintained in
`docs/standards/numerical-solver-architecture.md`.

## Existing Stage 3 chain

The v33--v57 Stage 3 chain is known noncompliance and is not grandfathered. It
is quarantined for replacement and deletion by
`20260901-stage3-native-vegetation-laned-watershed-throughput-recovery-001`.
No V58 or equivalent successor is permitted, and other process families may
not copy the pattern while removal is pending.

## Consequences

- Solver changes may require larger atomic replacements because an old runtime
  cannot remain as a safety net.
- Packages must distinguish physical regime selection from numerical recovery
  explicitly in contracts and tests.
- Runtime and debugging become bounded by one algorithm, one evaluation budget,
  and one failure vocabulary per regime.
- Tests follow current equations, guards, regimes, and public behavior instead
  of preserving dead solver implementations.
- A failed replacement holds or rolls back as a source/release increment; it
  does not create a permanent dual-solver runtime.
