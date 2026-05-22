# 20260522-clim01-wepp-forest-climate-model-discovery-and-specification-001

## Status
- state: active
- date: 2026-05-22
- timezone: UTC

## Objective
Determine exactly what the legacy `wepp-forest_260430_baseline` climate model does, then author
an openWEPP-owned detailed specification and consumer requirement map that
covers continuous-daily and breakfile behavior, and defines how climate
modeling slots into parser and architecture constraints.

## Why This Package Exists
openWEPP is moving architecture-first with top-down contracts, but climate
model behavior must be pinned before implementation to avoid repeating legacy
churn. This package establishes authoritative behavior/spec evidence for
continuous-daily and breakfile flows, with explicit exclusions for single-storm
modeling and single-storm climates.
Legacy climate behavior authority for this package is fixed to
`/workdir/wepp-forest_260430_baseline`.

## Scope
### Included
- Static and evidence-backed investigation of `wepp-forest_260430_baseline` climate model
  behavior for:
  - continuous-daily climate execution
  - breakfile-driven climate execution
- Detailed process specification authored under openWEPP ownership.
- Consumer requirements mapping:
  - required state/flux surfaces consumed by downstream kernels/subsystems
  - required outputs emitted to orchestrator/comparator/reporting surfaces
- Integration constraints mapping:
  - how climate model requirements align with climate parser contracts
  - how climate model slots into current openWEPP architecture/orchestrators
- Coverage matrix and exclusions statement.
- Dual review and dual verification artifacts.

### Explicitly Out of Scope
- Single-storm modeling implementation.
- Single-storm climate format/spec/consumer support.
- Implementing production climate kernel code in this package.

## Deliverables
1. Legacy behavior map:
   - `artifacts/wepp-forest-climate-model-behavior-map.md`
2. Detailed openWEPP climate model specification:
   - `artifacts/openwepp-climate-model-detailed-specification.md`
3. Consumer requirement specification:
   - `artifacts/climate-consumer-requirements.md`
4. Parser/architecture integration mapping:
   - `artifacts/climate-parser-architecture-integration-map.md`
5. Coverage and exclusion matrix:
   - `artifacts/climate-coverage-and-exclusions-matrix.md`
6. Follow-on implementation queue:
   - `artifacts/climate-implementation-wp-queue.md`
7. Worker/governance artifacts:
   - `artifacts/worker-handoff.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/clim01_disposition.md`
8. Dual review/verification artifacts:
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Dependencies
- `/home/workdir/openWEPP/AGENTS.md`
- `/home/workdir/openWEPP/docs/work-packages/README.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-CLIMATE-001.md`
- `/home/workdir/openWEPP/references/50201000/`
- `/home/workdir/openWEPP/references/vendorable/usersum2024.pdf`
- `/workdir/wepp-forest_260430_baseline/`
- `/workdir/wepppy/`
- `/workdir/wepppyo3/`

## Intended Write Set
- `docs/work-packages/20260522-clim01-wepp-forest-climate-model-discovery-and-specification-001/**`
- `docs/work-packages/README.md`

## Phase Plan
### Phase 0 - Source Intake
- Inventory all climate-related sources in `wepp-forest_260430_baseline`, `wepppy`, and
  `wepppyo3` relevant to continuous-daily and breakfile behavior.

### Phase 1 - Legacy Climate Behavior Reconstruction
- Reconstruct exact climate-process behavior from static code analysis,
  references, and targeted execution evidence where needed.

### Phase 2 - Specification and Consumer Contracts
- Author openWEPP-owned detailed climate model spec.
- Define consumer requirements and parser-to-architecture integration mapping.

### Phase 3 - Coverage Closure and Disposition
- Produce coverage/exclusion matrix.
- Produce follow-on implementation WP queue.
- Complete dual review + dual verification and final disposition.

## Exit Criteria
- Continuous-daily and breakfile climate behavior is explicitly specified with
  traceable evidence.
- Single-storm modeling/climate is explicitly excluded.
- Consumer requirement map is actionable for kernel/orchestrator integration.
- Parser/architecture integration constraints are explicit.
- Dual review and verification artifacts are complete.
- If any Rust code is modified while executing package scope, run and record:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- Rationale: discovery/specification package; no production runtime code in
  package scope.
