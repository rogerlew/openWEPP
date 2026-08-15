# Admit And Implement Persistent Snow-Free Surface-Liquid Hydrology Custody

Status: `executing / exact-byte review remediation active`

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
- [ ] Pass focused gates and dual independent review/verification. All fourteen
  accepted implementation-review findings now have focused passing
  remediation, including exact receiver identity, contextual canonical errors,
  and nonzero-residual production-soil reconstruction. A fresh exact-byte
  re-review found one bounded E011 offender-context defect; its focused passing
  correction at `75ba70681` received another exact-byte review, which found
  two remaining preflight/deletion context paths. Their focused passing
  correction at `6a107303c` received release review, which found an incomplete
  E004 frozen/thaw/snow-liquid-only domain guard and a finite-input aggregate
  authorization overflow. Their focused passing corrections are frozen at
  `0cb11eb12` and `93c46d3db`. Hydrology closure review returned PASS at
  `ab703c83a`. The Rust closure review found only exact workspace Clippy and
  full-suite evidence blockers; test-only lint correction and the scoped
  Stage-0 source guard pass a complete exact-head heavy rerun at `74d512f44`
  (2,783/2,783). Terminal Rust review then found nonterminal receiver-deletion
  attribution and large-finite closure arithmetic defects. Their correction at
  `82bfdc3a0` exposed one remaining shared error-precedence defect on re-review:
  checked arithmetic failure was collapsed into ordinary closure failure and
  two receiver sums remained unchecked. The correction at `3b9e5ed13`
  propagates arithmetic failure as contextual E003. Final re-review found that
  independent E010 closure validation preempted producer E009 attribution.
  The correction at `47f959b43` restored local ordering, but Rust closure review
  found an earlier finite E010 could hide a later-record E003 and later-record
  E009 context fell back to the first store. The multi-record correction at
  `ee240618c` improved multi-record handling, but closure re-review found the
  per-OFE aggregate comparison absent from preflight and shifted-row deletion
  attribution in producer sequences. The correction at `86ddb8aa2` shared
  projection and attribution, but closure3 review found final comparison omitted
  per-source enthalpy, routed errors used origin rather than destination
  context, and store arithmetic remained duplicated. The correction at
  `636dd36be` shares comparison/store projection, closes per-key mass and
  enthalpy plus OFE totals, but closure4 review found a critical constitutive
  regression from canonical interval `h_mix` to source-specific temperatures,
  unvalidated zero-source identity, and fabricated first-tile aggregate
  context. The correction at `e19bcdbcf` restores `h_mix`, freezes every raw
  source identity and uses typed absent context where identity is unknowable.
  Closure5 review found evidence still collapsed chronological support into one
  whole-OFE mixture and hardcoded source support/order. The correction at
  `c4114fc8c` reconstructs per-window `h_mix,b`, actual support and canonical
  source order without changing production physics. Closure6 review found join
  keys omitted window/disposition identity, expected routed support was circular
  from actual receipts, and support domains missed E003 precedence. The
  correction at `b5453e7d8` bound routes/windows, but closure7 review found
  expected infiltration/retention still copied actual receipts, recipient tile
  identity was incomplete, routed kind drifted, and raw Q was unjoined. The
  correction at `c3fdeca50` independently replays WB14 from frozen operands and
  binds complete current/recipient identity with zero expected-side receipt
  access. Closure8 hydrology review found the replayed final store and WB14
  continuation were discarded before the persistent-state join. The correction
  at `862f26bb7` joins those independent endpoint values to the strict ending
  state and restart digest. Closure8 Rust review additionally found partition
  identity was misclassified as E003, continuation bounds were incomplete and
  mixed-kind routed ordering lacked a nondegenerate poison. The correction at
  `6e203beec` restores E003/E009/E010 precedence, enforces both continuation
  bounds, shares named physical constants and adds the routed-order vector. It
  passes 562/562 orchestrator, 19/19 focused integration and 9/9 authority
  tests, strict Clippy, formatting, diff hygiene, and line-count governance.
  Closure9/10 reviews then found fabricated first-OFE ending-state context and
  incomplete centralization/frozen evidence for parcel order and source
  identity. The correction at `aacf181d7` centralizes those definitions,
  reports membership-aware or aggregate typed context, and freezes every
  mixed-route receipt, remainder and continuation bit. Closure10 Rust review
  found equal-length replacement context still pointed to the missing expected
  key and the full line-count inventory lacked three WARN dispositions. The
  correction at `2dfd0af64` reports the actual replacement row and binds the
  full position/cardinality/context/rollback matrix; governance records every
  affected large module below. Fresh exact-byte reviews remain pending.
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
