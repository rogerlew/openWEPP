# 20260522-arch19-run-and-parquet-boundary-contracts-001

## Status
- state: active
- date: 2026-05-22
- timezone: UTC

## Objective
Implement `CRF-007` by defining canonical top-level `.run` and parquet
boundary contracts, with explicit schema authority, parser/runtime closure
mapping, and acceptance criteria for downstream implementation packages.

## Why This Package Exists
ARCH14 identified strategy inversion risk: leaf parser progress outpaced
authoring of top-level input/output contracts. ARCH19 restores architecture-
first closure by codifying the top-level boundaries that govern runtime
integration and reporting surfaces.

## Scope
### Included
- Author canonical `.run` boundary contract for openWEPP runtime orchestration
  inputs.
- Author canonical parquet output boundary contract and schema governance
  policy.
- Inventory and map `/workdir/wepppyo3` parquet writer/schema references into
  explicit openWEPP contract authority statements.
- Produce a cross-file closure map tying `.run` and parquet boundaries to
  parser/runtime surfaces and ownership packages.
- Define acceptance criteria and test evidence requirements for follow-on
  implementation work.
- Produce dual review/disposition/verification artifacts.

### Explicitly Out of Scope
- Implementing full `.run` parser runtime integration code (follow-on package).
- Implementing full parquet writer runtime pipeline code (follow-on package).
- HBP authority/convergence closure beyond dependency intake from ARCH18.

## Deliverables
1. `.run` boundary authority artifact:
   - `artifacts/run-boundary-contract-authority.md`
2. parquet boundary authority artifact:
   - `artifacts/parquet-boundary-contract-authority.md`
3. `/workdir/wepppyo3` schema intake artifact:
   - `artifacts/wepppyo3-parquet-schema-reference-inventory.md`
4. cross-file closure map artifact:
   - `artifacts/run-parquet-cross-file-closure-map.md`
5. follow-on acceptance criteria artifact:
   - `artifacts/arch19-follow-on-acceptance-criteria.md`
6. Worker/governance artifacts:
   - `artifacts/worker-handoff.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/arch19_disposition.md`
7. Dual review/verification artifacts:
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Dependencies
- `/home/workdir/openWEPP/AGENTS.md`
- `/home/workdir/openWEPP/docs/work-packages/20260522-arch14-claude-architecture-review-disposition-001/artifacts/remediation-work-package-queue.md`
- `/home/workdir/openWEPP/docs/work-packages/20260522-arch17-parser-to-simulation-seam-ownership-and-integration-closure-001/`
- `/home/workdir/openWEPP/docs/work-packages/20260522-arch18-hbp-authority-and-convergence-closure-001/artifacts/arch19-parquet-schema-handoff.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/README.md`
- `/workdir/wepppyo3` parquet writer/schema surfaces

## Intended Write Set
- `docs/specifications/science-contracts/contracts/**` (ARCH19-owned contract docs)
- `docs/work-packages/20260522-arch19-run-and-parquet-boundary-contracts-001/**`
- `docs/work-packages/README.md`
- `docs/work-packages/20260522-arch14-claude-architecture-review-disposition-001/artifacts/remediation-work-package-queue.md`

## Phase Plan
### Phase 0 - Intake and Inventory
- Inventory current `.run` and parquet-related sources and references.
- Capture `/workdir/wepppyo3` parquet schema/writer surfaces.

### Phase 1 - Boundary Contract Authoring
- Author `.run` contract boundary and governance constraints.
- Author parquet boundary contract and schema constraints.

### Phase 2 - Closure and Acceptance Mapping
- Build cross-file closure map to parser/runtime surfaces.
- Define follow-on implementation acceptance criteria.

### Phase 3 - Review and Closeout
- Run required package gates (docs package; code gates only if code touched).
- Complete dual review/disposition/verification artifacts.

## Exit Criteria
- `.run` and parquet boundary contracts are explicitly authored and traceable.
- `/workdir/wepppyo3` parquet schema references are inventoried and linked to
  openWEPP authority statements.
- Cross-file closure map exists and names follow-on ownership packages.
- Dual review and verification artifacts are complete.
- If code surfaces are modified, standard rust gates must pass:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- Rationale: contract authoring and boundary governance package.
