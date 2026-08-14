# Connect V8 Vegetation To Real Hydrology In Shadow

Status: `executing / Child 1 authority released / real-owner extraction active`

Date: `2026-08-14`

Package ID: `20260814-vegetation-real-hydrology-arbitration-shadow-001`

Plan class: `Critical default-off real-owner integration`

## Objective

Use the actual production hydrology state and candidate logic for V8
occupancy/layer water arbitration while leaving production execution and bytes
unchanged.

## Scope

Trace and expose one dependency-neutral immutable same-snapshot owner interface,
clone the full production water state, submit V8 requests, return typed maximum
authorizations/reasons, re-solve V8, debit finalized use only, and prove exact
rollback and production invariance. Diagnostic proportional owners and copied
arithmetic are not acceptance evidence.

## Progress

- [x] Start only after Child 1 releases implementation authority.
- [ ] Freeze production owner/state/scheduler and extraction parity.
- [ ] Implement focused real-owner shadow and legacy-ET isolation.
- [ ] Pass reviews, gates, verification and archive the prompt.

## Delegation

Subagent authorization: this package explicitly authorizes and requires a
hydrology/ownership reviewer, Rust correctness reviewer, comparator runner and
two terminal verifiers; reviewers/verifiers are read-only and comparator writes
are limited to ignored logs and package gate artifacts.
