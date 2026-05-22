# 20260522-arch18-hbp-authority-and-convergence-closure-001

## Status
- state: hold
- date: 2026-05-22
- timezone: UTC

## Objective
Implement `CRF-006` remediation by closing HBP authority ambiguity,
converging codec behavior, and recording exact provenance/pinning evidence for
openWEPP-owned HBP parsing/bridge surfaces.

## Why This Package Exists
ARCH14 identified risk from HBP authority split and potential divergence across
multiple implementations. ARCH18 defines a single explicit authority model,
codifies split responsibilities where needed, and provides convergence tests
and provenance evidence.

## Scope
### Included
- Define and document authoritative HBP format ownership within openWEPP.
- Codify parser vs bridge responsibility boundaries for HBP.
- Reconcile or constrain divergent HBP implementations to prevent silent drift.
- Add convergence tests/fixtures that enforce equivalent behavior across
  intended HBP consumption surfaces.
- Record provenance pinning evidence aligned to ADR-0012 baseline governance.
- Produce dual review/disposition/verification artifacts.

### Explicitly Out of Scope
- General parser-to-simulation seam ownership closure (`CRF-005`/`CRF-010`)
  except HBP-specific dependencies.
- Scheduler hot-path optimization (`CRF-003`).
- Top-level `.run` and parquet contract completion (`CRF-007`).
  Note: parquet boundary work is explicitly owned by `ARCH19`.

## Deliverables
1. HBP authority artifact:
   - `artifacts/hbp-authority-split-and-governance.md`
2. Convergence evidence artifact:
   - `artifacts/hbp-convergence-test-evidence.md`
3. Provenance pin artifact:
   - `artifacts/hbp-provenance-pin-record.md`
4. ARCH19 handoff artifact:
   - `artifacts/arch19-parquet-schema-handoff.md`
5. Code/tests implementing HBP authority/convergence constraints.
6. Worker/governance artifacts:
   - `artifacts/worker-handoff.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/arch18_disposition.md`
7. Dual review/verification artifacts:
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Dependencies
- `/home/workdir/openWEPP/AGENTS.md`
- `/home/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/home/workdir/openWEPP/docs/work-packages/20260522-arch14-claude-architecture-review-disposition-001/artifacts/remediation-work-package-queue.md`
- `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/hbp-file.spec.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-HBP-001.md`
- `/home/workdir/openWEPP/crates/openwepp-input-contract/src/parsers/hbp.rs`
- `/home/workdir/openWEPP/crates/openwepp-legacy-bridge/src/hbp.rs`
- `/workdir/wepppyo3` (parquet writer/schema reference input for ARCH19 handoff)

## Intended Write Set
- `crates/openwepp-input-contract/**` (HBP surfaces)
- `crates/openwepp-legacy-bridge/**` (HBP bridge surfaces)
- `tests/integration/infile_hbp_parser_contract.rs` (and related HBP suites)
- `Cargo.lock` (if dependency graph changes)
- `docs/work-packages/20260522-arch18-hbp-authority-and-convergence-closure-001/**`
- `docs/work-packages/README.md`
- `docs/work-packages/20260522-arch14-claude-architecture-review-disposition-001/artifacts/remediation-work-package-queue.md`

## Phase Plan
### Phase 0 - Authority Inventory
- Inventory all HBP readers/writers, specs, and active call sites.

### Phase 1 - Authority Decision
- Define explicit parser/bridge responsibility and governance policy.

### Phase 2 - Convergence Implementation
- Implement and/or constrain behavior to enforce convergent outcomes.
- Add/expand convergence tests and fixture coverage.

### Phase 3 - Provenance and Closeout
- Capture provenance pin evidence.
- Run gates and complete review/disposition/verification artifacts.

## Exit Criteria
- HBP authority split is explicit and documented.
- Convergence tests demonstrate no unresolved behavior drift in in-scope
  surfaces.
- Provenance pin evidence is recorded with exact references.
- Required gates pass:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`
- Dual review and verification artifacts are complete.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- Rationale: parser/bridge authority closure and test-hardening only.
