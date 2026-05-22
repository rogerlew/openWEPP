# 20260522-arch15-typed-kernel-state-and-unit-boundary-seam-001

## Status
- state: complete
- date: 2026-05-22
- timezone: UTC

## Objective
Implement the mandatory CRF-001/CRF-002 remediation track by migrating the
kernel seam from stringly scalar maps to typed state/flux surfaces and wiring
`openwepp-unit-boundary` into that seam for unit-safe boundary values.

## Why This Package Exists
ARCH14 disposition concluded that typed-state seam migration and unit-boundary
wiring are required outcomes, not optional improvements. This package executes
that implementation closure path and produces gate evidence.

## Scope
### Included
- Refactor `openwepp-kernel-contract` request/writeback surfaces to typed
  symbol/value boundaries.
- Replace `BTreeMap<String, f64>` kernel seam usage in hillslope and watershed
  orchestrators with typed boundary maps.
- Wire unit-boundary value types into kernel seam value modeling.
- Update kernel seam tests and integration tests to exercise typed writeback
  behavior.
- Run full workspace validation gates and capture evidence.
- Produce dual review, disposition, and verification artifacts.

### Explicitly Out of Scope
- Parser-to-simulation seam integration (`CRF-005`) beyond kernel seam types.
- Scheduler hot-path performance redesign (`CRF-003`) beyond compatibility-safe
  seam changes required by this package.
- Kernel purity trait-signature redesign (`CRF-004`) unless needed to keep
  compilation green.

## Deliverables
1. Typed seam implementation in:
   - `crates/openwepp-kernel-contract/**`
   - `crates/openwepp-hillslope-orchestrator/**`
   - `crates/openwepp-watershed-orchestrator/**`
   - `tests/integration/kernel_writeback_contract.rs`
2. Implementation design artifact:
   - `artifacts/typed-kernel-state-design.md`
3. Unit-boundary seam mapping artifact:
   - `artifacts/unit-boundary-seam-mapping.md`
4. Migration/write-set evidence:
   - `artifacts/migration-plan-and-write-set.md`
5. Worker/governance artifacts:
   - `artifacts/worker-handoff.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/arch15_disposition.md`
6. Dual review/verification artifacts:
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Dependencies
- `/home/workdir/openWEPP/AGENTS.md`
- `/home/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/home/workdir/openWEPP/docs/work-packages/20260522-arch14-claude-architecture-review-disposition-001/artifacts/disposition-register.md`
- `/home/workdir/openWEPP/crates/openwepp-kernel-contract/src/lib.rs`
- `/home/workdir/openWEPP/crates/openwepp-unit-boundary/src/lib.rs`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `/home/workdir/openWEPP/crates/openwepp-watershed-orchestrator/src/lib.rs`
- `/home/workdir/openWEPP/tests/integration/kernel_writeback_contract.rs`

## Intended Write Set
- `crates/openwepp-kernel-contract/**`
- `crates/openwepp-hillslope-orchestrator/**`
- `crates/openwepp-watershed-orchestrator/**`
- `tests/integration/kernel_writeback_contract.rs`
- `Cargo.toml`
- `Cargo.lock`
- `docs/work-packages/20260522-arch15-typed-kernel-state-and-unit-boundary-seam-001/**`
- `docs/work-packages/README.md`

## Phase Plan
### Phase 0 - Baseline and Surface Inventory
- Inventory current stringly seam usage and impacted tests.

### Phase 1 - Kernel Contract Refactor
- Introduce typed symbol/value seam model and unit-boundary coupling.
- Preserve deterministic writeback evaluation/apply semantics.

### Phase 2 - Orchestrator and Test Migration
- Propagate typed seam surfaces through hillslope/watershed orchestrators.
- Update integration tests to verify typed state/writeback behavior.

### Phase 3 - Validation and Governance Closeout
- Run required gates.
- Produce review/disposition/verification artifacts.

## Exit Criteria
- No `BTreeMap<String, f64>` remains in kernel seam request/writeback surfaces.
- Kernel seam value model includes unit-safe boundary types from
  `openwepp-unit-boundary`.
- Hillslope and watershed orchestrator writeback surfaces use typed boundary
  symbols and values.
- `kernel_writeback_contract` integration coverage passes on typed surfaces.
- Required gates all pass:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`
- Dual review and verification artifacts are complete.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- Rationale: typed API refactor in simulation seam, no network/credential path.
