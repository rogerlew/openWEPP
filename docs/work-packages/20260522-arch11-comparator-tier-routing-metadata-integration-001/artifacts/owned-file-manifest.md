# Owned File Manifest — ARCH11

Evidence mode: Direct listing

## Workspace Integration
- [DIRECT] `/home/workdir/openWEPP/Cargo.toml`
  - Status: modified
  - Purpose: workspace member/dependency wiring for `openwepp-comparator-metadata`; integration test target registration.
- [DIRECT] `/home/workdir/openWEPP/Cargo.lock`
  - Status: modified (generated)
  - Purpose: lockfile update aligned with workspace package graph changes.

## Comparator Metadata Crate
- [DIRECT] `/home/workdir/openWEPP/crates/openwepp-comparator-metadata/Cargo.toml`
  - Status: created
  - Purpose: ARCH11 crate scaffold.
- [DIRECT] `/home/workdir/openWEPP/crates/openwepp-comparator-metadata/src/lib.rs`
  - Status: created
  - Purpose: typed comparator surface/tier mapping, deterministic message IDs, and typed invalid-path errors.

## Summary Integration
- [DIRECT] `/home/workdir/openWEPP/crates/openwepp-summary-accumulator/Cargo.toml`
  - Status: modified
  - Purpose: dependency on comparator metadata crate.
- [DIRECT] `/home/workdir/openWEPP/crates/openwepp-summary-accumulator/src/lib.rs`
  - Status: modified
  - Purpose: attach routed comparator metadata to summary rollups and enforce typed routing validation.

## Integration Tests
- [DIRECT] `/home/workdir/openWEPP/tests/integration/comparator_tier_routing_metadata.rs`
  - Status: created
  - Purpose: deterministic tier mapping coverage and typed invalid-path behavior coverage.

## Docs
- [DIRECT] `/home/workdir/openWEPP/docs/architecture/comparator-tier-routing-metadata.md`
  - Status: created
  - Purpose: architecture placement and routing-governance semantics.
- [DIRECT] `/home/workdir/openWEPP/docs/specifications/science-contracts/comparator-tier-routing-metadata-contract.md`
  - Status: created
  - Purpose: typed contract surface and invariants for comparator-tier routing metadata.

## Package Artifact Outputs
- [DIRECT] `/home/workdir/openWEPP/docs/work-packages/20260522-arch11-comparator-tier-routing-metadata-integration-001/artifacts/worker-handoff.md`
- [DIRECT] `/home/workdir/openWEPP/docs/work-packages/20260522-arch11-comparator-tier-routing-metadata-integration-001/artifacts/owned-file-manifest.md`
- [DIRECT] `/home/workdir/openWEPP/docs/work-packages/20260522-arch11-comparator-tier-routing-metadata-integration-001/artifacts/gate-results.md`
- [DIRECT] `/home/workdir/openWEPP/docs/work-packages/20260522-arch11-comparator-tier-routing-metadata-integration-001/artifacts/arch11_disposition.md`
- [DIRECT] `/home/workdir/openWEPP/docs/work-packages/20260522-arch11-comparator-tier-routing-metadata-integration-001/artifacts/review_agent_a.md`
- [DIRECT] `/home/workdir/openWEPP/docs/work-packages/20260522-arch11-comparator-tier-routing-metadata-integration-001/artifacts/review_agent_b.md`
- [DIRECT] `/home/workdir/openWEPP/docs/work-packages/20260522-arch11-comparator-tier-routing-metadata-integration-001/artifacts/verification_agent_a.md`
- [DIRECT] `/home/workdir/openWEPP/docs/work-packages/20260522-arch11-comparator-tier-routing-metadata-integration-001/artifacts/verification_agent_b.md`
