# Owned File Manifest — ARCH05

Evidence mode: Direct listing

## Crate Implementation
- [DIRECT] `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/Cargo.toml`
  - Status: created
  - Purpose: ARCH05 crate scaffold and dependency wiring to ARCH03/ARCH04 crates.
- [DIRECT] `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/lib.rs`
  - Status: created
  - Purpose: deterministic phase graph, precondition gate enforcement, typed execution report, and crate-local tests.

## Required Docs
- [DIRECT] `/home/workdir/openWEPP/docs/architecture/hillslope-phase-scheduler-graph.md`
  - Status: created
  - Purpose: architecture-level scheduler graph and execution boundary contract.
- [DIRECT] `/home/workdir/openWEPP/docs/specifications/science-contracts/hillslope-phase-scheduler-contract.md`
  - Status: created
  - Purpose: science-contract specification for preconditions, statuses, and failure classes.

## Package Artifact Outputs
- [DIRECT] `/home/workdir/openWEPP/docs/work-packages/20260521-arch05-hillslope-phase-scheduler-graph-001/artifacts/worker-handoff.md`
- [DIRECT] `/home/workdir/openWEPP/docs/work-packages/20260521-arch05-hillslope-phase-scheduler-graph-001/artifacts/owned-file-manifest.md`
- [DIRECT] `/home/workdir/openWEPP/docs/work-packages/20260521-arch05-hillslope-phase-scheduler-graph-001/artifacts/gate-results.md`
- [DIRECT] `/home/workdir/openWEPP/docs/work-packages/20260521-arch05-hillslope-phase-scheduler-graph-001/artifacts/arch05_disposition.md`
- [DIRECT] `/home/workdir/openWEPP/docs/work-packages/20260521-arch05-hillslope-phase-scheduler-graph-001/artifacts/review_agent_a.md`
- [DIRECT] `/home/workdir/openWEPP/docs/work-packages/20260521-arch05-hillslope-phase-scheduler-graph-001/artifacts/review_agent_b.md`
- [DIRECT] `/home/workdir/openWEPP/docs/work-packages/20260521-arch05-hillslope-phase-scheduler-graph-001/artifacts/verification_agent_a.md`
- [DIRECT] `/home/workdir/openWEPP/docs/work-packages/20260521-arch05-hillslope-phase-scheduler-graph-001/artifacts/verification_agent_b.md`

## Explicit Exclusions
- [DIRECT] No shared-file edits were made to quarantined paths (`Cargo.toml`, `Cargo.lock`, `tests/integration/**`, `src/**`).
- [DIRECT] Temporary build outputs (`crates/openwepp-hillslope-orchestrator/target/**`) and crate-local generated lockfile were not retained in ARCH05 deliverables.
