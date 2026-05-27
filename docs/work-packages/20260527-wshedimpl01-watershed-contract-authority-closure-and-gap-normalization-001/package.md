# 20260527-wshedimpl01-watershed-contract-authority-closure-and-gap-normalization-001

## Status
- state: package-complete
- date: 2026-05-27
- timezone: UTC
- decision: GO

## Objective
Execute WSHED02 by closing canonical watershed contract authority and
normalizing explicit unresolved implementation gaps for routing, impoundment,
orchestration, and watershed-output readiness before any production-code
migration packages proceed.

## Why This Package Exists
WSHEDPLAN01 published a dependency-ordered queue (`WSHED02..WSHED09`) and
identified WSHED02 as the immediate prerequisite. Current watershed contracts
contain partial authority language and scattered gap statements; this package
consolidates that authority into canonical `SC-*` files and records explicit
residual gaps for downstream WSHED03+ implementation packages.

## Scope
### Included
- Canonical contract-authority amendments for watershed routing/orchestration
  scope in:
  - `SC-ROUTE-001`
  - `SC-IMPOUND-001`
  - `SC-SED-001`
  - `SC-SYSTEM-001`
- Contract-index synchronization in
  `docs/specifications/science-contracts/index.md`.
- Gap normalization artifact that explicitly records unresolved closures,
  dependency routing, and validation expectations carried to WSHED03+.
- Baseline provenance correction reinforcement for channel detachment lineage
  (`detach.for` authority reference).
- Required governance, review, verification, and disposition artifacts.

### Explicitly Out of Scope
- Production Rust code edits.
- Contract-derived test implementation (WSHED03 scope).
- Pre-implementation code gate execution for production code changes.
- Comparator reruns and parity closure claims.

## Deliverables
1. Contract gap normalization report:
   - `artifacts/wshedimpl01-contract-gap-normalization-report.md`
2. Contract implementation evidence:
   - `artifacts/wshedimpl01-contract-implementation-evidence.md`
3. Contract-test implementation evidence:
   - `artifacts/wshedimpl01-contract-test-implementation-evidence.md`
4. Pre-implementation contract gate:
   - `artifacts/wshedimpl01-preimplementation-contract-gate.md`
5. Implementation/test evidence:
   - `artifacts/wshedimpl01-implementation-and-test-evidence.md`
6. Kernel-profile checklist:
   - `artifacts/wshedimpl01-kernel-profile-compliance-checklist.md`
7. Governance artifacts:
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/wshedimpl01_disposition.md`
   - `artifacts/worker-handoff.md`
8. Dual review artifacts:
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
9. Dual verification artifacts:
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
Kernel-affecting sequencing remains mandatory:
1. implement required canonical contract amendments,
2. implement contract-derived tests,
3. record pre-implementation contract gate evidence, then
4. modify production code.

WSHEDIMPL01 is a contract-authority package and executes step 1 only. It must
leave explicit handoff direction for WSHED03 to execute step 2 and step 3.

## Autonomous Execution Intent (Required)
This package is execution-ready and must run end to end without user
intervention. The executing agent must complete all package phases and update
required artifacts through disposition unless hard-blocked by contradictory
canonical requirements.

## Truthfulness Labeling Requirement
All evidence artifacts must include explicit `Static:` and/or `Ran:` sections.
Do not claim gate or test execution unless the commands were actually run.

## Provenance and Authority Posture
- Canonical authority is in
  `docs/specifications/science-contracts/contracts/SC-*.md`.
- Work-package artifacts are evidence and do not replace canonical authority.
- Legacy migration provenance defaults to
  `/workdir/wepp-forest_260430_baseline@dac3c950d8b16cc73774bf5ce2e7e11f80baac70`
  unless explicitly justified.
- Channel detachment provenance must reference
  `/workdir/wepp-forest_260430_baseline/src/detach.for` where applicable.
- No heuristic/proxy process-physics substitutions are allowed in downstream
  production migration packages.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-IMPOUND-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260527-wshedplan01-watershed-channel-routing-orchestration-parquet-assessment-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260527-wshedplan01-watershed-channel-routing-orchestration-parquet-assessment-001/artifacts/watershed-channel-routing-orchestration-parquet-wp-queue.md`
- `/workdir/openWEPP/docs/work-packages/20260527-wshedplan01-watershed-channel-routing-orchestration-parquet-assessment-001/artifacts/worker-handoff.md`
- `/workdir/openWEPP/docs/work-packages/20260527-wshedplan01-watershed-channel-routing-orchestration-parquet-assessment-001/artifacts/wshedplan01-gap-assessment.md`
- `/workdir/wepp-forest_260430_baseline/src/wshdrv.for`
- `/workdir/wepp-forest_260430_baseline/src/wshcqi.for`
- `/workdir/wepp-forest_260430_baseline/src/wshpek.for`
- `/workdir/wepp-forest_260430_baseline/src/wshchr.for`
- `/workdir/wepp-forest_260430_baseline/src/wshimp.for`
- `/workdir/wepp-forest_260430_baseline/src/chnero.for`
- `/workdir/wepp-forest_260430_baseline/src/chnrt.for`
- `/workdir/wepp-forest_260430_baseline/src/detach.for`

## Intended Write Set
- `docs/work-packages/20260527-wshedimpl01-watershed-contract-authority-closure-and-gap-normalization-001/**`
- `docs/work-packages/README.md`
- `docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `docs/specifications/science-contracts/contracts/SC-IMPOUND-001.md`
- `docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/index.md`

## Phase Plan
### Phase A - Intake and authority freeze
- Confirm queue authorization from WSHEDPLAN01 and freeze baseline authority
  references for the scoped contract amendments.

### Phase B - Canonical contract authority amendments
- Amend `SC-ROUTE-001`, `SC-IMPOUND-001`, `SC-SED-001`, and `SC-SYSTEM-001`
  for explicit baseline-authoritative watershed scope and unresolved-gap rows.

### Phase C - Gap normalization and registry synchronization
- Publish normalized unresolved-gap report and update science-contract index
  metadata/notes to reflect amended watershed authority posture.

### Phase D - Governance evidence and dual review
- Complete required evidence artifacts plus dual review and dual verification.

### Phase E - Disposition and handoff
- Publish explicit disposition and WSHED03 handoff, including contract-derived
  test expectations and pre-implementation gate prerequisites.

## Exit Criteria
- WS11/WS12 watershed authority language is normalized across canonical
  contracts with baseline-authoritative provenance.
- Residual watershed-channel-sediment/impoundment/output gaps are explicit and
  mapped to follow-on queue ownership.
- `detach.for` lineage correction is reflected where channel sediment
  detachment provenance is referenced.
- `docs/specifications/science-contracts/index.md` is synchronized with amended
  contract versions/notes.
- Required governance artifacts are complete with truthful evidence labels.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: documentation-contract authority package only; no production code
  changes.

## Execution Outcome Summary
- Canonical watershed authority contracts were amended and versioned for
  explicit unresolved migration gap rows covering WS11 routing closure, WS12
  continuity/seam closure, channel-sediment migration closure, and watershed
  parquet publication blocker closure sequencing.
- Science contract registry metadata was synchronized (`last_reviewed`,
  `notes`) for `SC-ROUTE-001`, `SC-IMPOUND-001`, `SC-SED-001`,
  `SC-SYSTEM-001`.
- WSHED03 handoff prerequisites are explicit:
  contract-derived vectors and pre-implementation gate evidence remain required
  before any production migration package claims closure.
