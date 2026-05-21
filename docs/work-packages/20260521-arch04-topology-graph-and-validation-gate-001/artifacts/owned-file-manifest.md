# Owned File Manifest — ARCH04

Evidence mode: Direct listing

## Workspace Integration
- [DIRECT] `/home/workdir/openWEPP/Cargo.toml`
  - Status: modified
  - Purpose: add `openwepp-topology` workspace member/dependency and ARCH04 integration test target.
- [DIRECT] `/home/workdir/openWEPP/Cargo.lock`
  - Status: modified (generated)
  - Purpose: lockfile update for new workspace dependency graph.
  - Scope note: explicit ARCH04 scope amendment.

## New Crate
- [DIRECT] `/home/workdir/openWEPP/crates/openwepp-topology/Cargo.toml`
  - Status: created
  - Purpose: crate scaffold and ARCH03 dependency wiring.
- [DIRECT] `/home/workdir/openWEPP/crates/openwepp-topology/src/lib.rs`
  - Status: created
  - Purpose: typed topology model, fixture parser, and pre-execution validation gate.

## Integration Test
- [DIRECT] `/home/workdir/openWEPP/tests/integration/topology_graph_validation_gate.rs`
  - Status: created
  - Purpose: canonical valid/invalid topology gate behavior coverage.

## Fixtures
- [DIRECT] `/home/workdir/openWEPP/tests/fixtures/topology/canonical_valid.topo`
- [DIRECT] `/home/workdir/openWEPP/tests/fixtures/topology/invalid_disconnected.topo`
- [DIRECT] `/home/workdir/openWEPP/tests/fixtures/topology/invalid_channel_count_mismatch.topo`
- [DIRECT] `/home/workdir/openWEPP/tests/fixtures/topology/invalid_reference_domain.topo`
- [DIRECT] `/home/workdir/openWEPP/tests/fixtures/topology/invalid_cycle.topo`
  - Status: created
  - Purpose: canonical pass/fail topology scenarios for ARCH04 gate.

## Docs
- [DIRECT] `/home/workdir/openWEPP/docs/architecture/topology-graph-model.md`
  - Status: created
  - Purpose: architecture-level topology model and boundary documentation.
- [DIRECT] `/home/workdir/openWEPP/docs/specifications/science-contracts/topology-validation-gate.md`
  - Status: created
  - Purpose: validation-gate invariant/message contract.

## Package Artifact Outputs
- [DIRECT] `/home/workdir/openWEPP/docs/work-packages/20260521-arch04-topology-graph-and-validation-gate-001/artifacts/worker-handoff.md`
- [DIRECT] `/home/workdir/openWEPP/docs/work-packages/20260521-arch04-topology-graph-and-validation-gate-001/artifacts/owned-file-manifest.md`
- [DIRECT] `/home/workdir/openWEPP/docs/work-packages/20260521-arch04-topology-graph-and-validation-gate-001/artifacts/gate-results.md`
- [DIRECT] `/home/workdir/openWEPP/docs/work-packages/20260521-arch04-topology-graph-and-validation-gate-001/artifacts/arch04_disposition.md`
- [DIRECT] `/home/workdir/openWEPP/docs/work-packages/20260521-arch04-topology-graph-and-validation-gate-001/artifacts/review_agent_a.md`
- [DIRECT] `/home/workdir/openWEPP/docs/work-packages/20260521-arch04-topology-graph-and-validation-gate-001/artifacts/review_agent_b.md`
- [DIRECT] `/home/workdir/openWEPP/docs/work-packages/20260521-arch04-topology-graph-and-validation-gate-001/artifacts/verification_agent_a.md`
- [DIRECT] `/home/workdir/openWEPP/docs/work-packages/20260521-arch04-topology-graph-and-validation-gate-001/artifacts/verification_agent_b.md`
