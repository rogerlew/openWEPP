# 20260529-hillstab07-wb16-peak-flow-input-provenance-parity-001

## Status
- state: hold
- date: 2026-05-29
- timezone: America/Los_Angeles
- decision: HOLD

## Objective
Address WB16 peak-flow input provenance parity gaps documented in
`docs/audits/20260529_peak_flow_implementation_audit.md` by executing
contract-first closure for:
1. canonical authority of `m` and `ealpha` producer semantics, and
2. non-silent runtime provenance reporting when compatibility seeding is used.

## Why This Package Exists
The audit identified that WB16 currently consumes `m`/`ealpha` with incomplete
contract producer authority and a silent compatibility seed path for `ealpha`
(`1.0`) that can mask baseline divergence. This package executes immediate
closure work that is implementable without introducing non-authoritative
process-physics substitutions.

## Scope
### Included
- Canonical `SC-*` amendments for WB16 `m`/`ealpha` provenance authority and
  compatibility-seed governance posture.
- Contract-derived tests that verify explicit WB16 compatibility-seed
  provenance publication (non-silent behavior).
- Runner implementation to publish explicit WB16 compatibility-seed
  provenance and warning surfaces.
- Work-package evidence artifacts and GO/HOLD disposition.

### Explicitly Out of Scope
- Full baseline-authoritative `ealpha` producer-chain migration
  (`frcfac -> rdat -> alphay -> eplane`) when required source-state families
  are not yet fully available in production runtime surfaces.
- Heuristic/proxy process-physics substitutions for missing producer surfaces.

## Deliverables
1. `artifacts/hillstab07-wb16-peak-flow-gap-matrix.md`
2. `artifacts/hillstab07-contract-implementation-evidence.md`
3. `artifacts/hillstab07-contract-test-implementation-evidence.md`
4. `artifacts/hillstab07-preimplementation-contract-gate.md`
5. `artifacts/hillstab07-implementation-and-test-evidence.md`
6. `artifacts/hillstab07-kernel-profile-compliance-checklist.md`
7. `artifacts/owned-file-manifest.md`
8. `artifacts/gate-results.md`
9. `artifacts/hillstab07_disposition.md`
10. `artifacts/worker-handoff.md`
11. `artifacts/review_agent_a.md`
12. `artifacts/review_agent_b.md`
13. `artifacts/verification_agent_a.md`
14. `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
1. Implement required canonical contract/index amendments for WB16 input
   provenance.
2. Implement contract-derived tests for WB16 provenance publication behavior.
3. Record pre-implementation contract-gate evidence before production edits.
4. Apply production code edits.

## Autonomous Execution Intent (Required)
This package must execute end-to-end through disposition without requesting
additional user direction unless hard-blocked.

## Truthfulness Labeling Requirement
All evidence artifacts must explicitly label `Static:` and/or `Ran:`.

## Provenance and Authority Posture
- Canonical authority lives in
  `docs/specifications/science-contracts/contracts/SC-*.md`.
- Legacy comparator/provenance anchor defaults to
  `/workdir/wepp-forest_260430_baseline` at commit
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- No heuristic/proxy process-physics substitutions are allowed in production.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/audits/20260529_peak_flow_implementation_audit.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-HYDRAULICS-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/wepp-forest_260430_baseline/src/appmth.for`
- `/workdir/wepp-forest_260430_baseline/src/rdat.for`
- `/workdir/wepp-forest_260430_baseline/src/frcfac.for`
- `/workdir/wepp-forest_260430_baseline/src/irs.for`
- `/workdir/wepp-forest_260430_baseline/src/eplane.for`

## Intended Write Set
- `docs/work-packages/README.md`
- `docs/work-packages/20260529-hillstab07-wb16-peak-flow-input-provenance-parity-001/**`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-runner/src/hillslope/mod.rs`
- `tests/integration/cli03_runner_contract_derived_tests.rs`

## Phase Plan
### Phase A - Intake and scope freeze
- Confirm queue authorization from audit and freeze to WB16 input-provenance
  parity scope.

### Phase B - Gap assessment and authority mapping
- Convert audit findings into executable gap rows and closure targets.

### Phase C - Contract updates (first required gate)
- Amend canonical `SC-*` authority to encode `m` canonical-value posture and
  `ealpha` producer-chain requirements plus compatibility-seed governance.

### Phase D - Contract-derived tests (second required gate)
- Add tests that assert explicit WB16 compatibility-seed provenance/warning
  behavior.

### Phase E - Pre-implementation contract gate (third required gate)
- Record contract + test existence evidence before production edits.

### Phase F - Production implementation (fourth required gate)
- Implement explicit WB16 compatibility-seed provenance publication in runner.

### Phase G - Validation gates
- Execute:
  - `cargo fmt --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo test --workspace`
  - `cargo deny check`
- If full-workspace gates are not completed, record exact reason and executed
  subset truthfully.

### Phase H - Dual review, dual verification, disposition
- Complete review/verification artifacts.
- Publish explicit GO/HOLD with remaining parity closure requirements.

## Exit Criteria
- Canonical contracts encode WB16 `m`/`ealpha` provenance authority and
  compatibility-seed governance posture.
- Runner no longer leaves WB16 compatibility `ealpha` seeding silent.
- Contract-derived tests for WB16 provenance publication pass.
- Required artifacts are complete with truthful evidence labeling.
- Disposition explicitly states whether full WB16 input-provenance parity is
  closed or remains `HOLD`.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: local contract/runtime/test updates only; no credential or remote
  service surfaces.
