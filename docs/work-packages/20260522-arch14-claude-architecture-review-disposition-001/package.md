# 20260522-arch14-claude-architecture-review-disposition-001

## Status
- state: active
- date: 2026-05-22
- timezone: UTC

## Objective
Disposition the external architecture review (Claude) into explicit,
auditable decisions with dual-review and verification gates, and define a
sequenced remediation queue.

This package must explicitly record project direction that openWEPP is moving
to typed kernel state surfaces and correctly wiring `openwepp-unit-boundary`
into that seam.

## Why This Package Exists
A static external review identified architecture and integration risks across
kernel boundary typing, unit safety wiring, orchestration performance surfaces,
kernel purity enforcement, parser/orchestrator seam ownership, HBP authority,
and top-level contract sequencing.

Before implementation work continues, these findings must be normalized,
severity-scored, dispositioned, and mapped to concrete follow-on packages with
acceptance criteria.

## Scope
### Included
- Normalize the review into stable finding IDs and severity classes.
- Build a disposition register with explicit decisions per finding:
  `accept` / `amend` / `reject` / `defer`.
- Record explicit architecture direction that `CRF-001` and `CRF-002` are
  mandatory remediation tracks: remove stringly `BTreeMap<String, f64>` kernel
  seam usage and wire unit-safe boundary types at the kernel interface.
- Define required evidence and closure criteria per accepted/amended finding.
- Identify which findings require ADR-level changes vs contract/code package
  changes.
- Publish a remediation work-package queue and dependency order.
- Run required dual review and verification gates on the disposition packet.

### Explicitly Out of Scope
- Implementing code fixes for accepted findings.
- Running cargo gates for kernel/runtime behavior validation.
- Closing findings by code change inside this package.

## Deliverables
1. Review findings register:
   - `artifacts/claude-review-findings-register.md`
2. Disposition decision register:
   - `artifacts/disposition-register.md`
3. Remediation work-package queue:
   - `artifacts/remediation-work-package-queue.md`
4. Kickoff acceptance criteria:
   - `artifacts/architecture-review-disposition-acceptance-criteria.md`
5. Worker/governance artifacts:
   - `artifacts/worker-handoff.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/arch14_disposition.md`
6. Dual review and verification artifacts:
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Dependencies
- `/home/workdir/openWEPP/README.md`
- `/home/workdir/openWEPP/AGENTS.md`
- `/home/workdir/openWEPP/docs/architecture/README.md`
- `/home/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/home/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/home/workdir/openWEPP/docs/specifications/README.md`
- `/home/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/home/workdir/openWEPP/docs/specifications/wepp-input-file-parser-contract-authoring-procedure.md`
- `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/hbp-file.spec.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-HBP-001.md`
- `/home/workdir/openWEPP/crates/openwepp-kernel-contract/src/lib.rs`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `/home/workdir/openWEPP/crates/openwepp-unit-boundary/src/lib.rs`
- `/home/workdir/openWEPP/crates/openwepp-input-contract/src/lib.rs`
- `/home/workdir/openWEPP/crates/openwepp-legacy-bridge/src/hbp.rs`

## Intended Write Set
- `docs/work-packages/20260522-arch14-claude-architecture-review-disposition-001/**`
- `docs/work-packages/README.md`

## Phase Plan
### Phase 0 - Intake and Normalization
- Convert the external review narrative into stable findings (`CRF-001..010`).
- Assign severity and impacted surfaces for each finding.

### Phase 1 - Disposition Authoring
- Record proposed decision per finding.
- Define required evidence and closure criteria.
- Mark ADR-required decisions explicitly.

### Phase 2 - Remediation Queue Design
- Define follow-on package candidates (`ARCH15+`/`INIMPL+`) with dependency order.
- Separate hard blockers from sequenceable improvements.

### Phase 3 - Dual Review Gate
- Independent reviewer A and reviewer B evaluate finding normalization,
  severity, and disposition logic.

### Phase 4 - Verification Gate and Closeout
- Verification agents confirm disposition consistency and no missing high
  severity closure path.
- Publish package disposition (`GO`, `GO-WITH-AMENDMENTS`, `HOLD`).

## Exit Criteria
- All review findings are represented with stable IDs and severity labels.
- Every finding has an explicit disposition state and closure path.
- Blocker findings have explicit follow-on package ownership.
- `CRF-001` and `CRF-002` disposition text explicitly states typed-state seam
  migration and unit-boundary wiring as required implementation outcomes.
- Dual review and verification artifacts are complete.
- No unresolved high-severity finding is left without a disposition path.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- Rationale: architecture/governance disposition package only.
