# Owned File Manifest — ARCH07

Evidence mode: Direct listing

## Workspace Integration
- [DIRECT] `/home/workdir/openWEPP/Cargo.toml`
  - Status: modified
  - Purpose: workspace member wiring for `openwepp-kernel-contract`, root dependency wiring, and ARCH07 integration test target registration.
- [DIRECT] `/home/workdir/openWEPP/Cargo.lock`
  - Status: modified (generated)
  - Purpose: lockfile update aligned with workspace/package graph.

## Kernel Contract Crate
- [DIRECT] `/home/workdir/openWEPP/crates/openwepp-kernel-contract/Cargo.toml`
  - Status: created
  - Purpose: crate scaffold and dependency wiring to ARCH03 status/closure contracts.
- [DIRECT] `/home/workdir/openWEPP/crates/openwepp-kernel-contract/src/lib.rs`
  - Status: created
  - Purpose: typed kernel traits, request/response/writeback payloads, and writeback accept/reject/apply protocol helpers.

## Orchestrator Integration
- [DIRECT] `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/lib.rs`
  - Status: modified
  - Purpose: ARCH07 hillslope kernel-boundary execution + writeback ownership enforcement.
- [DIRECT] `/home/workdir/openWEPP/crates/openwepp-watershed-orchestrator/src/lib.rs`
  - Status: modified
  - Purpose: ARCH07 watershed kernel-boundary execution + writeback ownership enforcement.

## Integration Tests
- [DIRECT] `/home/workdir/openWEPP/tests/integration/kernel_writeback_contract.rs`
  - Status: created
  - Purpose: integration coverage for writeback success, rejection, and typed failure propagation semantics.

## Docs
- [DIRECT] `/home/workdir/openWEPP/docs/architecture/kernel-trait-boundary-and-writeback.md`
  - Status: created
  - Purpose: architecture ownership and boundary semantics for ARCH07.
- [DIRECT] `/home/workdir/openWEPP/docs/specifications/science-contracts/kernel-writeback-contract.md`
  - Status: created
  - Purpose: typed science/contract surface for writeback decisions and invariants.

## Package Artifact Outputs
- [DIRECT] `/home/workdir/openWEPP/docs/work-packages/20260521-arch07-kernel-trait-boundary-and-writeback-contract-001/artifacts/worker-handoff.md`
- [DIRECT] `/home/workdir/openWEPP/docs/work-packages/20260521-arch07-kernel-trait-boundary-and-writeback-contract-001/artifacts/owned-file-manifest.md`
- [DIRECT] `/home/workdir/openWEPP/docs/work-packages/20260521-arch07-kernel-trait-boundary-and-writeback-contract-001/artifacts/gate-results.md`
- [DIRECT] `/home/workdir/openWEPP/docs/work-packages/20260521-arch07-kernel-trait-boundary-and-writeback-contract-001/artifacts/arch07_disposition.md`
- [DIRECT] `/home/workdir/openWEPP/docs/work-packages/20260521-arch07-kernel-trait-boundary-and-writeback-contract-001/artifacts/review_agent_a.md`
- [DIRECT] `/home/workdir/openWEPP/docs/work-packages/20260521-arch07-kernel-trait-boundary-and-writeback-contract-001/artifacts/review_agent_b.md`
- [DIRECT] `/home/workdir/openWEPP/docs/work-packages/20260521-arch07-kernel-trait-boundary-and-writeback-contract-001/artifacts/verification_agent_a.md`
- [DIRECT] `/home/workdir/openWEPP/docs/work-packages/20260521-arch07-kernel-trait-boundary-and-writeback-contract-001/artifacts/verification_agent_b.md`
