# 20260529-relproc03-release-gate-ci-automation-001

## Status
- state: complete
- date: 2026-05-29
- timezone: UTC
- decision: GO

## Objective
Implement and disposition release-gate CI automation for the RELPROC runbook
sequence, covering workspace validation, release lint, and stability cohort
gates with explicit scripts and workflow wiring.

## Rationale
RELPROC02 closed sidecar emission automation and handed off CI automation as
the next required action. The release runbook remains draft-governed without an
in-repo CI execution surface for those gates.

## Scope
### Included
- CI workflow scaffold for release-gate execution.
- Reusable gate scripts for workspace checks, release candidate assembly,
  sidecar emission, and release lint validation.
- Stability cohort gate automation command surface with explicit pass/fail
  assertion on HILLSTAB01 JSON output.
- Runbook updates to point to automation entrypoints.
- Package artifacts and disposition evidence.

### Excluded
- Changes to kernel/science process physics.
- Changes to external infra beyond repository files (no GitHub settings edits).
- Promotion of runbook status to `completed` (handled after first full release
  candidate execution using the finalized procedure).

## Deliverables
1. Workflow and automation surfaces:
   - `.github/workflows/release-gates.yml`
   - `tools/release/run_release_candidate_gates.sh`
   - `tools/release/run_hillstab_gate.sh`
   - `tools/release/assert_hillstab_success.py`
2. Runbook/governance updates:
   - `docs/governance/openwepp-release-procedure-draft.md`
   - `docs/work-packages/README.md`
3. Package artifacts:
   - `artifacts/relproc03-automation-assessment.md`
   - `artifacts/relproc03-automation-implementation-evidence.md`
   - `artifacts/gate-results.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/relproc03_disposition.md`
   - `artifacts/worker-handoff.md`
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/governance/openwepp-release-procedure-draft.md`
- `/workdir/openWEPP/docs/contracts/openwepp-runner-contract.md`
- `/workdir/openWEPP/docs/contracts/openwepp-binary-release-contract.md`
- `/workdir/openWEPP/docs/work-packages/20260529-relproc02-runner-sidecar-emission-cli-001/artifacts/worker-handoff.md`
- `/workdir/openWEPP/docs/work-packages/20260528-hillstab01-hillslope-cli-broad-stability-cohorts-001/artifacts/hillstab01_stability_cohort.py`

## Intended Write Set
- `.github/workflows/release-gates.yml`
- `tools/release/run_release_candidate_gates.sh`
- `tools/release/run_hillstab_gate.sh`
- `tools/release/assert_hillstab_success.py`
- `tools/release/README.md`
- `docs/governance/openwepp-release-procedure-draft.md`
- `docs/work-packages/README.md`
- `docs/work-packages/20260529-relproc03-release-gate-ci-automation-001/**`

## Truthfulness Requirement
Evidence artifacts must label claim basis and execution mode:
- `Static:` source-inspected evidence
- `Ran:` command-executed evidence

## Phase Plan
### Phase A - Automation design and runbook alignment
- Define script/workflow surfaces and update runbook references.

### Phase B - Implementation
- Implement release-gate and stability-gate scripts plus CI workflow.

### Phase C - Validation
- Execute automation scripts locally (with bounded stability sample run) and
  run docs/script lint gates for touched surfaces.

### Phase D - Disposition
- Finalize artifacts, reviews, verification, and GO/HOLD decision.

## Exit Criteria
- A repository-local CI workflow exists for release-gate automation.
- Release-gate script executes `fmt`, `clippy`, `test`, `deny`, release
  candidate staging, sidecar emission, and `release lint`.
- Stability gate script invokes HILLSTAB01 harness and fails when suite
  residuals are present.
- Runbook references the automation surfaces and removes CI-gap wording.
- Package artifacts are complete and dispositioned.

## Security Impact Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: CI/workflow and local scripting only; no runtime privilege model
  change.

## Autonomy
Package executed end-to-end through disposition without additional user
intervention.
