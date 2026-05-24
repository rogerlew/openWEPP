# 20260524-simimpl03-contract-authority-amendments-for-production-watbal-execution-001

## Status
- state: complete
- date: 2026-05-24
- timezone: UTC

## Objective
Execute SIMIMPL03 end-to-end by amending canonical science/system/input
contracts to authorize production watbal execution ownership, runtime
mode-propagation invariants, simulation-owned WB13/H.wat publication authority,
and consolidated-kernel intake guardrails.

## Why This Package Exists
SIMIMPL01 and SIMIMPL02 identified and quantified production closure gaps across
runner execution ownership, `wepp_ui` runtime branch propagation, and output
provenance authority.

SIMIMPL03 is the mandatory contract-first gate before contract-derived test
implementation (`simimpl04`) and any production code integration packages
(`simimpl05+`).

## Scope
### Included
- Amend canonical contract authority surfaces as required:
  - `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
  - `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
  - `docs/specifications/science-contracts/contracts/SC-INFILE-WEPPUI-001.md`
- Encode/clarify invariants for:
  - production runner -> scheduler/kernel execution ownership,
  - `requested`/`effective` mode propagation and typed mismatch closure,
  - simulation-owned WB13/H.wat publication provenance,
  - selective consolidated-kernel intake policy boundaries.
- Amend contract index metadata if contract cross-references/status entries
  require updates.
- Produce amendment evidence, review/disposition/verification, and handoff
  artifacts required to unblock SIMIMPL04.

### Explicitly Out of Scope
- Contract-derived integration test implementation (SIMIMPL04 scope).
- Production Rust kernel/runner/orchestrator code edits.
- Comparator replay reruns and parity promotion decisions.

## Deliverables
1. Canonical contract amendments:
   - `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
   - `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
   - `docs/specifications/science-contracts/contracts/SC-INFILE-WEPPUI-001.md`
2. Registry/index updates if required:
   - `docs/specifications/science-contracts/index.md`
3. Contract amendment matrix:
   - `artifacts/simimpl03-contract-amendment-matrix.md`
4. Contract/authority evidence:
   - `artifacts/simimpl03-contract-implementation-evidence.md`
5. Contract-test implementation evidence:
   - `artifacts/simimpl03-contract-test-implementation-evidence.md`
6. Pre-implementation contract gate:
   - `artifacts/simimpl03-preimplementation-contract-gate.md`
7. Implementation/test evidence log:
   - `artifacts/simimpl03-implementation-and-test-evidence.md`
8. Kernel-profile compliance checklist:
   - `artifacts/simimpl03-kernel-profile-compliance-checklist.md`
9. Governance artifacts:
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/simimpl03_disposition.md`
   - `artifacts/worker-handoff.md`
10. Dual review artifacts:
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
11. Dual verification artifacts:
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
SIMIMPL03 is itself the contract-amendment package that must close before
SIMIMPL04 and any production-code package.

For downstream code-authoring packages consuming SIMIMPL03, sequence remains:
1. implement required canonical contract amendments,
2. implement contract-derived tests,
3. record pre-implementation contract gate evidence, then
4. modify production code.

## Autonomous Execution Intent (Required)
This package must be execution-ready and self-contained. Assigned agents must
progress through all phases and disposition without requesting additional user
direction unless hard-blocked by missing local authority or contradictory
canonical requirements.

## Truthfulness Labeling Requirement
All evidence artifacts must explicitly label evidence mode using `Static:` and/
or `Ran:` sections.

## Physics and Authority Posture
- Baseline physics/comparator authority remains
  `/workdir/wepp-forest_260430_baseline@dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- Consolidated architecture intake source remains `/workdir/wepp-forest/fpm-src`
  and is selective/adoption-gated only.
- No physics invention is permitted; all added/changed invariants must map to
  canonical authority or explicit cited provenance.
- Variable naming continuity must preserve legacy WEPP symbols; alias mappings
  must be explicit when openWEPP runtime names differ.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WEPPUI-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl01-hillslope-totality-assessment-and-watbal-consolidation-001/artifacts/simulation-implementation-wp-queue.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl01-hillslope-totality-assessment-and-watbal-consolidation-001/artifacts/simimpl01-pipeline-gap-audit.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl01-hillslope-totality-assessment-and-watbal-consolidation-001/artifacts/simimpl01-watbal-authority-source-comparison.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl02-phase-b-full-routine-inventory-and-gap-closure-map-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl02-phase-b-full-routine-inventory-and-gap-closure-map-001/artifacts/simimpl02-full-hillslope-routine-inventory.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl02-phase-b-full-routine-inventory-and-gap-closure-map-001/artifacts/simimpl02-routine-owner-surface-gap-closure-map.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl02-phase-b-full-routine-inventory-and-gap-closure-map-001/artifacts/simimpl02-routine-contract-invariant-crosswalk.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl02-phase-b-full-routine-inventory-and-gap-closure-map-001/artifacts/simimpl02_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260524-pl14s-tier-a-openwepp-candidate-emission-and-replay-001/artifacts/pl14s-tier-a-semantic-parity-delta-report.md`
- `/workdir/wepp-forest_260430_baseline`
- `/workdir/wepp-forest`

## Intended Write Set
- `docs/work-packages/20260524-simimpl03-contract-authority-amendments-for-production-watbal-execution-001/**`
- `docs/work-packages/README.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/contracts/SC-INFILE-WEPPUI-001.md`
- `docs/specifications/science-contracts/index.md`

## Phase Plan
### Phase A - Intake and Authority Alignment
- Confirm dependency readability, queue authority, and SIMIMPL02 handoff
  assumptions.
- Enumerate required contract amendment targets and invariant families.

### Phase B - Contract Amendment Authoring
- Author required amendments in canonical `SC-*` targets.
- Record symbol continuity and alias mappings where runtime names diverge.

### Phase C - Contract Linkage and Gate Recording
- Update contract index references if needed.
- Record amendment matrix and pre-implementation contract gate evidence.

### Phase D - Review, Verification, and Disposition
- Complete dual review + disposition + dual verification artifacts.
- Finalize gate results, owned-file manifest, and worker handoff.

## Exit Criteria
- Required canonical contracts are amended with explicit invariant coverage for
  SIMPIPE/SIMMODE/SIMOUT/SIMCONS authority closure.
- Contract amendment matrix is complete and evidence-linked.
- Dual review and dual verification artifacts are complete with no unresolved
  high-severity findings.
- Pre-implementation contract gate is recorded and supports SIMIMPL04 start.
- Governance artifacts are complete with no queued placeholders.
- Required repository gates are run and recorded only if non-doc code changes
  are introduced:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- Rationale: contract/governance authoring package; no direct production
  executable-path code changes required in this package.
