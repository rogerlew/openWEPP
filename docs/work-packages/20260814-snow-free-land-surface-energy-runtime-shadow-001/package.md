# Implement Snow-Free Land-Surface-Energy Runtime Shadow

Status: `COMPLETE / terminally verified / default-off shadow`

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
- V8-only projection, receipt, candidate and protocol-ordering additions in
  `crates/openwepp-vegetation/**` required to connect the already admitted V8
  owner to the LSE runtime without rerunning or mutating V7;
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
- [x] Connect forest-litter/surface-liquid withdrawal and condensation credit
  to the terminally released persistent hydrology owner. The historical blocker
  remains recorded in `artifacts/real-hydrology-surface-liquid-hold-audit.md`;
  resumption authority and scope are recorded in `artifacts/resume-intake.md`.
  The resumed path now proves evaporative and positive-condensation branches,
  WB14 overflow routing, exact root and ground `D/A/F`, and one aligned public
  physical-to-persistent-to-BGC envelope from immutable V8 beginning state.
  The envelope is uncommitted and the production frame remains byte-identical.
- [x] Pass final exact-byte science review and execute the required benchmark
  surfaces after the custody hold was lifted and the complete endpoint existed.
- [x] Correct the reopened forest-litter projection so immutable beginning
  hydrology-owned litter liquid determines `lambda_l`, and select per-tile LSE
  VIS/NIR optics as the sole E01--E03 ground lower-boundary owner.
- [x] Close the terminal full-workspace authority gate and dual verification.
  The separate reconciliation package preserved immutable V3--V8 identity,
  admitted the reviewed reproducible V9 successor, passed the clean 2,999-test
  workspace gate, and closed before this package.

The separate contract-first package
`20260817-c3-woody-v3-v5-oracle-reconciliation-001` resolved that authority
contradiction without rewriting historical bytes, and the clean exact-head
full-workspace gate passes.

Fresh review at `dfc7cf971` rejected the first endpoint claim. The retained
physical kernels and custody owner remain active implementation evidence, while
all accepted findings in `artifacts/review-finding-disposition.md` are corrected
before performance or heavy gates become legitimate.

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
