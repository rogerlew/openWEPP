# 20260521-arch03-sim-contract-crate-and-status-taxonomy-001

## Status
- state: active
- date: 2026-05-21
- timezone: UTC

## Objective
Implement the foundational simulation contract substrate for openWEPP:
- unified typed status taxonomy
- closure/invariant check primitives
- canonical WEPP symbol alias registry

This package is the hard dependency gate for downstream architecture
implementation work.

## Why This Package Exists
ARCH02 identified that downstream scheduler, topology, kernel boundary, and
reporting work is blocked until status/failure semantics and symbol continuity
are first-class and typed.

Without this package, downstream crates risk diverging status models,
inconsistent invariant handling, and symbol drift from canonical WEPP/wepp-forest
contract language.

## Scope
### Included
- Create a dedicated simulation contract crate for cross-subsystem use.
- Implement typed status taxonomy with deterministic severity/classification.
- Implement closure/invariant check primitives and typed failure surfaces.
- Implement canonical symbol alias registry preserving WEPP/wepp-forest symbol
  continuity with explicit openWEPP boundary-name mapping.
- Add focused unit/integration tests for taxonomy, closure checks, and alias
  lookup behavior.
- Wire crate into workspace and run repository validation gates.

### Explicitly Out of Scope
- Topology graph implementation (`ARCH04`).
- Hillslope/watershed scheduler graph implementation (`ARCH05`, `ARCH06`).
- Kernel trait boundary integration (`ARCH07`).
- Legacy adapter isolation (`ARCH08`).

## Worktree Execution Model
- Recommended worktree path: `/home/workdir/openWEPP/.worktrees/arch03-sim-contract`
- Recommended branch name: `arch03/sim-contract-status-taxonomy`
- Ownership rule: stay within ARCH03 write-set unless an explicit scope
  amendment is recorded in package artifacts.

## Deliverables
1. Simulation contract crate scaffold and integration.
2. Typed status taxonomy implementation and docs.
3. Closure/invariant primitive implementation and docs.
4. Canonical symbol alias registry implementation and docs.
5. Test coverage for taxonomy/closure/alias behaviors.
6. Worker handoff notes:
   - `artifacts/worker-handoff.md`
7. Owned file manifest:
   - `artifacts/owned-file-manifest.md`
8. Gate evidence summary:
   - `artifacts/gate-results.md`
9. Closeout disposition:
   - `artifacts/arch03_disposition.md`
10. Review and verification artifacts:
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Dependencies
- `/home/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/home/workdir/openWEPP/docs/architecture/simulation-subsystem-kernel-architecture.md`
- `/home/workdir/openWEPP/docs/work-packages/20260521-arch02-simulation-subsystem-kernel-architecture-discovery/`
- `/home/workdir/openWEPP/docs/work-packages/20260521-arch02-simulation-subsystem-kernel-architecture-discovery/artifacts/openwepp-simulation-architecture-requirements.md`
- `/home/workdir/openWEPP/docs/work-packages/20260521-arch02-simulation-subsystem-kernel-architecture-discovery/artifacts/openwepp-subsystem-and-kernel-ownership-proposal.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/`

## Intended Write Set
- `crates/openwepp-sim-contract/**`
- `Cargo.toml`
- `tests/integration/sim_contract_status_taxonomy.rs`
- `tests/integration/sim_contract_closure_checks.rs`
- `tests/integration/sim_contract_symbol_alias_registry.rs`
- `docs/specifications/science-contracts/status-taxonomy.md`
- `docs/specifications/science-contracts/closure-check-primitives.md`
- `docs/specifications/science-contracts/symbol-alias-registry.md`
- package-local artifacts under this work-package directory

## Phase Plan
### Phase 0 - Contract Surface Freeze
- Derive concrete status classes and closure primitive requirements from ARCH02.
- Freeze symbol alias table schema and naming continuity rules.

### Phase 1 - Crate and Type Skeleton
- Add new crate and workspace wiring.
- Define public API for status taxonomy, closure checks, and alias registry.

### Phase 2 - Core Implementation
- Implement typed status model and constructors.
- Implement closure/invariant check primitives with typed violation results.
- Implement canonical symbol alias registry and lookup/validation paths.

### Phase 3 - Tests and Documentation
- Add integration tests for status, closure, and alias behaviors.
- Add/refresh docs for taxonomy, closure semantics, and alias policy.

### Phase 4 - Quality Gates and Closeout
- Run:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`
- Record review/disposition/verification artifacts.

## Exit Criteria
- New simulation contract crate exists and is wired into workspace.
- Status taxonomy, closure primitives, and alias registry are typed and tested.
- Symbol continuity rules are explicit and enforceable.
- All required gates pass and are recorded.
- No unresolved high-severity review findings.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- Rationale: internal crate and validation logic; no external service or network
  surface changes.
