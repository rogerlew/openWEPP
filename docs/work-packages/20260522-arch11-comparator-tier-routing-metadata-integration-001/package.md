# 20260522-arch11-comparator-tier-routing-metadata-integration-001

## Status
- state: active
- date: 2026-05-22
- timezone: UTC

## Objective
Implement explicit comparator confidence-tier routing metadata propagation across
openWEPP reporting surfaces for the greenfield scientific hydrology simulation.

This package operationalizes ADR-0011 confidence tiers as typed metadata routed
with architecture outputs so legacy deltas are triaged consistently by surface
class and execution mode.

## Why This Package Exists
ARCH02 requirement `OA-SKA-014` requires comparator confidence-tier metadata to
be carried with subsystem outputs. ARCH10 delivered typed summary rollups; ARCH11
adds typed confidence-tier routing metadata so comparator/replay/reporting
pipelines receive explicit interpretation context.

## Scope
### Included
- Implement comparator-tier metadata contract surface with deterministic message
  IDs and typed failure classes.
- Define explicit tier mapping policy for comparator surfaces:
  - higher-confidence surfaces (single OFE + daily water-balance)
  - investigation-tier surfaces (hourly and watershed)
- Integrate metadata propagation through summary/reporting outputs.
- Add integration tests proving deterministic tier routing and typed failure
  behavior when metadata is missing/invalid.
- Publish architecture/spec docs for tier routing governance.

### Explicitly Out of Scope
- New process kernel physics implementations.
- Replacing science-contract tolerances or acceptance criteria.
- Wave 4 ratification closeout bundle (`ARCH12`).

## Worktree Execution Model
- Recommended worktree path: `/home/workdir/openWEPP/.worktrees/arch11-comparator-tier-routing`
- Recommended branch name: `arch11/comparator-tier-routing-metadata`
- Ownership rule: stay within ARCH11 write-set unless scope amendment is
  explicitly recorded in package artifacts.

## Deliverables
1. Comparator tier routing implementation and workspace integration.
2. Typed tier metadata model and deterministic message-id taxonomy.
3. Integration tests for tier mapping, routing, and typed invalid-path behavior.
4. Architecture/spec docs for comparator-tier routing policy.
5. Worker handoff notes:
   - `artifacts/worker-handoff.md`
6. Owned file manifest:
   - `artifacts/owned-file-manifest.md`
7. Gate evidence summary:
   - `artifacts/gate-results.md`
8. Closeout disposition:
   - `artifacts/arch11_disposition.md`
9. Review and verification artifacts:
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Dependencies
- `/home/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/home/workdir/openWEPP/docs/decisions/0003-parity-semantic-not-bit.md`
- `/home/workdir/openWEPP/docs/architecture/simulation-subsystem-kernel-architecture.md`
- `/home/workdir/openWEPP/docs/work-packages/20260521-arch03-sim-contract-crate-and-status-taxonomy-001/`
- `/home/workdir/openWEPP/docs/work-packages/20260521-arch10-summary-accumulator-kernelization-001/`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/status-taxonomy.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/summary-accumulator-contract.md`

## Intended Write Set
- `crates/openwepp-comparator-metadata/**`
- `crates/openwepp-summary-accumulator/src/lib.rs`
- `Cargo.toml`
- `Cargo.lock`
- `tests/integration/comparator_tier_routing_metadata.rs`
- `docs/architecture/comparator-tier-routing-metadata.md`
- `docs/specifications/science-contracts/comparator-tier-routing-metadata-contract.md`
- package-local artifacts under this work-package directory

## Phase Plan
### Phase 0 - Policy Freeze
- Freeze confidence-tier policy and mapping table from ADR-0011.
- Freeze comparator metadata schema and message-id classes.

### Phase 1 - Comparator Metadata Surface
- Scaffold `openwepp-comparator-metadata` crate.
- Define typed metadata structs/enums and validation guards.

### Phase 2 - Reporting Integration
- Integrate metadata attachment into summary rollup/reporting outputs.
- Enforce typed failure behavior for invalid or missing routing metadata.

### Phase 3 - Tests and Documentation
- Add integration tests for deterministic tier routing and invalid-path
  classification.
- Publish architecture/spec docs with mapping and governance semantics.

### Phase 4 - Quality Gates and Closeout
- Run:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`
- Record review/disposition/verification artifacts.

## Exit Criteria
- Comparator confidence-tier metadata is explicitly typed and routed with
  reporting outputs.
- Tier policy mapping is deterministic and test-covered.
- Invalid metadata paths produce explicit typed failures (no silent fallback).
- All required gates pass and are recorded.
- No unresolved high-severity review findings.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- Rationale: internal contract/reporting metadata and docs changes only.
