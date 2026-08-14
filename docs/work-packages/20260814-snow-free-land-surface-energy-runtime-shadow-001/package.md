# Implement Snow-Free Land-Surface-Energy Runtime Shadow

Status: `queued / depends on Children 1 and 2`

Date: `2026-08-14`

Package ID: `20260814-snow-free-land-surface-energy-runtime-shadow-001`

Plan class: `Critical contract implementation and real-owner integration`

## Objective

Implement the exact admitted land-surface model in a dependency-light crate,
using tile-local strict state and one real-hydrology authorization while
remaining default-off and without a real hillslope consumer claim.

## Progress

- [ ] Start after Children 1 and 2 close.
- [ ] Freeze crate graph, DTO boundaries and performance budget.
- [ ] Implement strict state/configuration, potential/final solves, ledgers,
  diagnostics, restart and rollback.
- [ ] Pass science/Rust reviews, gates, benchmark and dual verification.

## Delegation

Subagent authorization: this package explicitly authorizes and requires a
land-surface science reviewer, Rust reviewer, comparator runner and two
terminal verifiers with read-only review/verification and package-log-only
comparator writes.
