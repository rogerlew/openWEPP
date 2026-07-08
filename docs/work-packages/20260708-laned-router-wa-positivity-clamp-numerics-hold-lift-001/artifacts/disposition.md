# Disposition

Status: EXECUTED-HOLD-SOLVER-CORRECTION-REQUIRED
Evidence mode: Static + Ran.

## Decision

The package closes the silent active-publication defect class by adding a
contract-backed clamp-source guard and executor ordering that fails before row
consumers or frame commits can observe pathological active-routed outputs.

It does not close WA active-routing fidelity or target-`dx` promotion. WA now
fails closed at the active clamp-source guard on both retained fixed10 and dx5
evidence rungs. A deeper positivity-preserving solver correction remains
required before WA active routing can produce acceptable outputs.

## Review Disposition

All review findings are accepted and fixed:

- FYN-H1: fixed by reordering active executor routing/closure before row
  consumers and commits.
- FYN-M1 / HKE-H1: fixed by widening package write-set authority.
- FYN-M2: fixed by adding rev-40 Test-Vector and BEI rows.
- HKE-H2: fixed by adding package closure artifacts.
- HKE-H3: closure remains gated on final gate recording.
- HKE-M1: fixed by structured `--expect-fail-guard` harness parsing.

## Outcome

Closed:

- Material positivity-clamp amplification can no longer silently publish in
  active mode when clamp mass exceeds the external active source mass.
- WA fixed10/dx5 evidence now fails at the explicit
  `laned_active_clamp_exceeds_source` guard.
- D10B solver/oracle semantics are untouched.

Held:

- WA active routing remains not acceptable for target-`dx` promotion or default
  activation evidence.
- A solver-level positivity-preserving correction or replacement active solver
  policy is required to turn the current fail-closed WA days into accepted
  physical outputs.

## First Follow-On

Scaffold a solver-correction package for the active explicit router:

- preserve D10B Case-4 oracle acceptance and conservative handoff semantics;
- instrument the failing WA day/rung with per-step depth/discharge/clamp
  extrema before choosing a method;
- evaluate positivity-preserving explicit limiting, active-only adaptive
  substepping with cost bounds, or a contract-authorized monotone fallback;
- prove WA fixed10 and candidate rungs run with clamp bounded below the rev-40
  guard before reopening target-`dx` promotion.
