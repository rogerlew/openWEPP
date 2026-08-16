# Implement Snow-Free Land-Surface-Energy Runtime Shadow

Status: `executing / surface-liquid custody dependency lifted / covered forest runtime active`

Date: `2026-08-14`

Package ID: `20260814-snow-free-land-surface-energy-runtime-shadow-001`

Plan class: `Critical contract implementation and real-owner integration`

## Objective

Implement the exact admitted land-surface model in a dependency-light crate,
using tile-local strict state and one real-hydrology authorization while
remaining default-off and without a real hillslope consumer claim.

## Intended Write Set

- `Cargo.toml` and `Cargo.lock` for the new workspace member;
- `crates/openwepp-land-surface-energy/**`;
- dependency-neutral boundary additions in `openwepp-kernel-contract` only when
  the frozen schemas cannot be represented locally without duplication;
- default-off adapter code in `openwepp-hillslope-orchestrator`;
- Child-3 contract/integration tests; and
- this package and campaign lifecycle surfaces.

Production runner selection, defaults, direct-runtime dispatch, production
outputs and production scheduler call sites are excluded.

## Frozen Architecture

The new crate owns the complete joint V8/LSE column solve and candidate DTOs,
depends only on dependency-neutral lower crates, and exposes no commit method.
The orchestrator adapts the actual Child-2 hydrology snapshot, performs exactly
one authorization and stages the complete shadow candidate. Existing V7
post-hoc energy proposals and meteorology helpers are not constitutive inputs.

## Progress

- [x] Start after Children 1 and 2 close.
- [x] Freeze crate graph, DTO boundaries and performance budget.
- [x] Implement strict state/configuration, exact open/covered potential and
  fixed-cap solves, typed diagnostics, owner operands and rollback hashes.
- [x] Connect mixed root/bare-ground soil-layer requests to one actual
  production hydrology authorization with clone-only candidate debit.
- [ ] Connect forest-litter/surface-liquid withdrawal and condensation credit
  to the terminally released persistent hydrology owner. The historical blocker
  remains recorded in `artifacts/real-hydrology-surface-liquid-hold-audit.md`;
  resumption authority and scope are recorded in `artifacts/resume-intake.md`.
  The first resumed physical increment now passes the frozen evaporative
  two-rank forest/litter path through one real authorization; a constitutive
  positive-condensation covered vector and complete V8 vegetation/BGC envelope
  are still required before checking this item.
- [ ] Pass terminal science/Rust reviews, benchmark and dual verification only
  after the custody hold is lifted and the complete forest endpoint exists.

## Delegation

Subagent authorization: this package explicitly authorizes and requires a
land-surface science reviewer, Rust reviewer, comparator runner and two
terminal verifiers with read-only review/verification and package-log-only
comparator writes.

## Exit Criteria

Close only after strict configuration/state/restart, exact oracle vectors,
potential and fixed-cap solves, source-keyed D/A/F, real-owner authorization,
independent water/energy/ground-heat/advection closure, typed failures,
byte-identical rollback, performance budget, dual reviews and dual terminal
verification all pass. No real scheduler consumer, activation or cutover claim
is permitted in this child.
