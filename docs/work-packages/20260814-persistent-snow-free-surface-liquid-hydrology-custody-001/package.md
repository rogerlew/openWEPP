# Admit And Implement Persistent Snow-Free Surface-Liquid Hydrology Custody

Status: `executing / re-review round 1 corrections pass / final re-review pending`

Date: `2026-08-14`

Package ID: `20260814-persistent-snow-free-surface-liquid-hydrology-custody-001`

Plan class: `Critical contract-first defect closure and production-owner extraction`

This ExecPlan is a living document maintained under `docs/codex_exec_plans.md`.

## Purpose / Big Picture

Lift the exact blocker retained at commit
`af9a989063aa8751dfadb14c442e1b360653658c` by making snow-free ponded and
forest-litter liquid a persistent, restart-representable hydrology-owned state.
The resulting owner must expose one immutable beginning snapshot, same-source
request/authorization/finalized-use debit, signed condensation credit,
post-solve ingress partition and ending-state lineage. After this package
passes, the held Child 3 resumes in place and the existing campaign continues.

## Correction Authority Envelope

Defect ID: `LSE-HYDRO-CUSTODY-001`.

Observed failure: the real-owner bridge can authorize soil-layer withdrawals
but must reject forest-litter/surface-liquid withdrawal and condensation because
`DirectRunFrame` has no persistent surface-liquid owner.

Correction authority is `SC-LANDSURFACEENERGY-001` version 3 ownership,
immutable-beginning and ingress-ordering rules, plus the contract-first
`SC-SURFACELIQUID-001` contract authored here. In scope are canonical hydrology owner
state, typed identities, strict restart encoding, snapshot extraction, D/A/F,
signed condensation credit, capacity/overflow/infiltration/runoff/routing
receipts, candidate validation, rollback and the default-off bridge.

Protected boundaries are production selection, production scheduler dispatch,
legacy PMET/ET behavior, existing output publication, snow ownership, runtime
activation, calibration and cutover. The completed V7 and admitted V8/LSE model
definition bytes remain immutable.

## Intended Write Set

- this package tree and campaign lifecycle artifacts;
- new canonical `SC-SURFACELIQUID-001.md` and its lifecycle registry row;
- contract-derived integration tests;
- dependency-neutral resource identities only when needed;
- machine-readable unit-registry entries for the new custody seams;
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/**` for the actual
  hydrology owner state and candidate operations;
- `crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/**`
  for the resumed owner bridge;
- `crates/openwepp-hillslope-orchestrator/src/vegetation_real_hydrology_shadow.rs`
  for canonical production-snapshot and lane-map accessors;
- `tools/release/authority-policy/impact-map.json` for atomic authority bindings
  on every new custody science surface;
- affected crate exports and focused integration tests; and
- Cargo test registration only when required.

No runner selector, production default, published output schema, deployment,
branch, PR or push is in scope.

## Progress

- [x] (2026-08-14) Verify exact clean checkpoint and preserve historical HOLD.
- [x] (2026-08-14) Scaffold the contract-first dependency-lift package.
- [x] (2026-08-14) Admit canonical surface-liquid hydrology authority and bind contract-derived tests.
- [x] (2026-08-14) Record a passing pre-implementation contract gate.
- [x] (2026-08-14) Implement persistent state, restart, D/A/F, credit, ingress and rollback.
- [ ] Pass focused gates and dual independent review/verification. All eleven
  accepted implementation-review findings had focused passing remediation;
  exact-byte hydrology and Rust re-review findings now have focused passing
  corrections; final exact-byte re-review is pending.
- [ ] Archive this prompt and resume held Child 3 without rewriting its HOLD.

## Surprises & Discoveries

- The LSE contract already fixes water ownership and ordering, but the hydrology
  contract and runtime lack the executable per-tile owner schema and operations.

## Decision Log

- Decision: create a narrow dependency-lift package rather than widening or
  rewriting the held Child 3 package.
  Rationale: production hydrology custody lies outside Child 3's frozen write
  set, while the user directed preservation of the historical HOLD and resumption
  of that same child after the dependency is implemented.
  Date/Author: 2026-08-14 / Codex.

## Validation And Acceptance

Acceptance requires exact store identity and serialization, digest sensitivity,
same-snapshot proportional authorization, `0 <= F <= A <= D`, debit of finalized
use only, signed condensation credit, retained/overflow/infiltration/runoff and
routed-parcel mass/enthalpy joins, no same-interval ingress availability,
byte-identical rollback, unchanged production execution and no selector reachability.
Independent reconstruction must distinguish tile from OFE basis and must not
consume producer-supplied residuals.

Run affected checks, strict Clippy, focused unit/integration and authority tests,
AUTH11, anti-evasion, science admission, formatting, diff hygiene and package
Markdown lint. Critical closure additionally requires current full-workspace
evidence or truthful campaign-owned deferral under the canonical gate strategy.

## Delegation

Subagent authorization: this package explicitly authorizes and requires
spawning/delegating to one hydrology/ownership reviewer, one Rust correctness
reviewer and two terminal verifiers. Their scope is read-only exact-byte review;
expected outputs are compact findings and verdicts incorporated into the named
package artifacts. A comparator runner may write only ignored logs and the
package gate artifact.

## Outcomes & Retrospective

In progress. No custody-lift, Child-3 completion or campaign completion claim is
made until the declared gates and independent reviews pass.
