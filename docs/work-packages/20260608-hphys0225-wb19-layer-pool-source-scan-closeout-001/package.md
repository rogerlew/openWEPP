# 20260608-hphys0225-wb19-layer-pool-source-scan-closeout-001

## Status
- state: complete
- date: 2026-06-08
- timezone: UTC
- decision: accepted

## Objective
Resolve the REFACTOR015-era HPHYS0225 contract test regression by updating the
runtime source-scan assertion to track the actual hydrology kernel phase module
layout after `03_kernel_support_01_kernel_phases.rs` was split.

## Why this package exists
`HPHYS0225` source-scan semantics were anchored to the monolithic
`03_kernel_support_01_kernel_phases.rs`. After REFACTOR015, those expressions
moved into `kernel_phases_mod/*`, making the scan brittle and false-positive.
This package keeps the scoped contract-check intent (`HPHYS0225`-style legacy max
reconciliation prohibition) while making the scan resilient to module layout.

## Scope
### Included
- `tests/integration/hphys0225_wb19_layer_pool_withdrawal_cap_contract.rs`
- A tiny follow-on work-package artifact set for scoped closeout evidence.
- `docs/work-packages/README.md` entry for discoverability.

### Explicitly Out of Scope
- Additional HPHYS authority changes.
- Runtime behavior changes outside existing `HPHYS0225` code paths.
- Any production Rust behavior changes beyond source-scan helper logic in tests.

## Deliverables
1. Updated `hphys0225_runtime_source_forbids_legacy_max_reconciliation` test to
   scan all Rust sources under `crates/openwepp-hillslope-orchestrator/src/hydrology`.
2. Package artifact evidence for test implementation and verification.
3. `docs/work-packages/README.md` pointer to this closeout follow-on.

## Autonomous execution intent (required)
Execute through disposition without waiting for user follow-up unless hard-blocked.

## Truthfulness Labeling Requirement
All evidence artifacts must explicitly label `Static` and `Ran` sections.

## Phase plan
### Phase A - Scope and test updates
- Update the HPHYS0225 source-scan test for refactor-safe module coverage.

### Phase B - Evidence
- Record targeted execution and closure status in package artifacts.

### Phase C - Registry/docs pointer
- Add package index entry in `docs/work-packages/README.md`.

## Exit criteria
- Targeted test compiles and the updated scan enforces the same HPHYS0225 source
  prohibition invariants without hardcoded monolith-only path assumptions.
- `cargo test --test hphys0225_wb19_layer_pool_withdrawal_cap_contract` passes.
- Required artifacts are populated with `Static`/`Ran` evidence and disposition.
