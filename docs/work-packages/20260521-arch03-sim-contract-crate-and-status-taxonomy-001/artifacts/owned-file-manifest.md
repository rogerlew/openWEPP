# Owned File Manifest — ARCH03

Evidence mode: Direct listing

## Workspace Integration
- [DIRECT] `/home/workdir/openWEPP/Cargo.toml`
  - Status: modified
  - Purpose: workspace member wiring, dependency wiring, and three ARCH03 integration test target registrations.
- [DIRECT] `/home/workdir/openWEPP/Cargo.lock`
  - Status: modified (generated)
  - Purpose: lockfile update for new workspace crate dependency graph.
  - Scope note: explicit ARCH03 scope amendment (generated artifact).

## New Crate
- [DIRECT] `/home/workdir/openWEPP/crates/openwepp-sim-contract/Cargo.toml`
  - Status: created
  - Purpose: crate scaffold and workspace inheritance.
- [DIRECT] `/home/workdir/openWEPP/crates/openwepp-sim-contract/src/lib.rs`
  - Status: created
  - Purpose: public module surface export.
- [DIRECT] `/home/workdir/openWEPP/crates/openwepp-sim-contract/src/status.rs`
  - Status: created
  - Purpose: typed status taxonomy implementation.
- [DIRECT] `/home/workdir/openWEPP/crates/openwepp-sim-contract/src/closure.rs`
  - Status: created
  - Purpose: closure/invariant primitive implementation.
- [DIRECT] `/home/workdir/openWEPP/crates/openwepp-sim-contract/src/symbols.rs`
  - Status: created
  - Purpose: canonical symbol alias registry implementation.

## Integration Tests
- [DIRECT] `/home/workdir/openWEPP/tests/integration/sim_contract_status_taxonomy.rs`
  - Status: created
  - Purpose: classification/severity and constructor behavior coverage.
- [DIRECT] `/home/workdir/openWEPP/tests/integration/sim_contract_closure_checks.rs`
  - Status: created
  - Purpose: closure primitive typed violation coverage.
- [DIRECT] `/home/workdir/openWEPP/tests/integration/sim_contract_symbol_alias_registry.rs`
  - Status: created
  - Purpose: canonical alias map and lookup/error behavior coverage.

## Docs
- [DIRECT] `/home/workdir/openWEPP/docs/specifications/science-contracts/status-taxonomy.md`
  - Status: created
  - Purpose: status taxonomy contract documentation.
- [DIRECT] `/home/workdir/openWEPP/docs/specifications/science-contracts/closure-check-primitives.md`
  - Status: created
  - Purpose: closure primitive contract documentation.
- [DIRECT] `/home/workdir/openWEPP/docs/specifications/science-contracts/symbol-alias-registry.md`
  - Status: created
  - Purpose: canonical symbol alias policy and table.

## Package Artifact Outputs
- [DIRECT] `/home/workdir/openWEPP/docs/work-packages/20260521-arch03-sim-contract-crate-and-status-taxonomy-001/artifacts/worker-handoff.md`
- [DIRECT] `/home/workdir/openWEPP/docs/work-packages/20260521-arch03-sim-contract-crate-and-status-taxonomy-001/artifacts/owned-file-manifest.md`
- [DIRECT] `/home/workdir/openWEPP/docs/work-packages/20260521-arch03-sim-contract-crate-and-status-taxonomy-001/artifacts/gate-results.md`
- [DIRECT] `/home/workdir/openWEPP/docs/work-packages/20260521-arch03-sim-contract-crate-and-status-taxonomy-001/artifacts/arch03_disposition.md`
- [DIRECT] `/home/workdir/openWEPP/docs/work-packages/20260521-arch03-sim-contract-crate-and-status-taxonomy-001/artifacts/review_agent_a.md`
- [DIRECT] `/home/workdir/openWEPP/docs/work-packages/20260521-arch03-sim-contract-crate-and-status-taxonomy-001/artifacts/review_agent_b.md`
- [DIRECT] `/home/workdir/openWEPP/docs/work-packages/20260521-arch03-sim-contract-crate-and-status-taxonomy-001/artifacts/verification_agent_a.md`
- [DIRECT] `/home/workdir/openWEPP/docs/work-packages/20260521-arch03-sim-contract-crate-and-status-taxonomy-001/artifacts/verification_agent_b.md`
