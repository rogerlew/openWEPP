# 20260529-relproc01-openwepp-release-procedure-draft-001

## Status
- state: complete
- date: 2026-05-29
- timezone: UTC
- decision: GO

## Objective
Draft and disposition a canonical openWEPP release procedure that translates
existing release contracts/ADRs into executable maintainer steps, including:
1. required validation gates,
2. candidate artifact assembly and naming,
3. release sidecar and lint expectations,
4. post-HILLSTAB06 stability evidence requirements.

## Rationale
Release policy authority exists (`ADR-0007`, release contracts), but there is
no single maintainer runbook that sequences these rules into one repeatable
procedure. This leaves release execution dependent on scattered context.

## Scope
### Included
- Draft release runbook in `docs/governance/`.
- Index/linkage updates in top-level docs navigation surfaces.
- Package artifacts for gap assessment, evidence, and disposition.

### Excluded
- CI pipeline implementation.
- New runner subcommands or release automation code.
- External publication/tagging workflows.

## Deliverables
1. Runbook draft:
   - `docs/governance/openwepp-release-procedure-draft.md`
2. Package artifacts:
   - `artifacts/releaseproc01-gap-assessment.md`
   - `artifacts/releaseproc01-implementation-and-test-evidence.md`
   - `artifacts/gate-results.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/releaseproc01_disposition.md`
   - `artifacts/worker-handoff.md`
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/contracts/openwepp-binary-release-contract.md`
- `/workdir/openWEPP/docs/contracts/openwepp-runner-contract.md`
- `/workdir/openWEPP/docs/decisions/0007-openwepp-runner-and-release-governance.md`
- `/workdir/openWEPP/docs/work-packages/20260529-hillstab06-wb16-peak-closure-and-p24-climate-triage-001/artifacts/worker-handoff.md`
- `/workdir/openWEPP/crates/openwepp-runner/src/bin/open_wepp_runner.rs`
- `/workdir/openWEPP/crates/openwepp-runner/src/release.rs`

## Intended Write Set
- `docs/governance/openwepp-release-procedure-draft.md`
- `docs/governance/README.md`
- `docs/README.md`
- `README.md`
- `docs/work-packages/README.md`
- `docs/work-packages/20260529-relproc01-openwepp-release-procedure-draft-001/**`

## Truthfulness Requirement
Each evidence artifact labels evidence class explicitly:
- `Static:` read/reasoned evidence
- `Ran:` executed command evidence

## Phase Plan
### Phase A - Gap and authority assessment
- Confirm release governance authority and identify documentation gaps.

### Phase B - Runbook drafting
- Author canonical draft procedure in governance docs with concrete commands.

### Phase C - Linkage and discoverability
- Wire doc index/readme surfaces to the new runbook.

### Phase D - Validation and disposition
- Run docs lint on touched surfaces.
- Complete review/verification/disposition artifacts.

## Exit Criteria
- A discoverable release runbook exists in `docs/governance/`.
- Runbook includes executable commands for validation, candidate assembly, lint,
  and stability evidence capture.
- Known release-process gaps are explicitly documented as follow-on items.
- Package artifacts are complete and dispositioned.

## Security Impact Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: documentation-only work, no runtime behavior changes.

## Autonomy
Package scope was executed end-to-end without requiring additional user
intervention.
