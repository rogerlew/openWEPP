# 20260529-relproc02-runner-sidecar-emission-cli-001

## Status
- state: complete
- date: 2026-05-29
- timezone: UTC
- decision: GO

## Objective
Implement and disposition a dedicated runner CLI surface for release sidecar
emission so release operators can generate contract-valid sidecars for explicit
binary path/role inputs without manual scripting.

## Rationale
RELPROC01 documented the release procedure and identified a blocking automation
gap: no dedicated runner subcommand exists for sidecar emission. This package
closes that gap and updates release documentation to use the new command.

## Scope
### Included
- Contract/runbook amendments for runner release-sidecar command surface.
- `open_wepp_runner release sidecar` implementation.
- Command parsing/unit tests and crate-level validation for touched behavior.
- Package artifacts, review notes, verification, and disposition.

### Excluded
- CI workflow automation for release gates.
- External release/tag publication workflow changes.
- Kernel/science-process behavior changes.

## Deliverables
1. Runner CLI implementation and tests for:
   - `open_wepp_runner release sidecar --binary <path> --role <role>`
2. Contract/runbook updates:
   - `docs/contracts/openwepp-runner-contract.md`
   - `docs/contracts/openwepp-binary-release-contract.md`
   - `docs/governance/openwepp-release-procedure-draft.md`
3. Package artifacts:
   - `artifacts/relproc02-contract-assessment.md`
   - `artifacts/relproc02-contract-test-evidence.md`
   - `artifacts/relproc02-preimplementation-contract-gate.md`
   - `artifacts/relproc02-implementation-and-test-evidence.md`
   - `artifacts/gate-results.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/relproc02_disposition.md`
   - `artifacts/worker-handoff.md`
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/contracts/openwepp-runner-contract.md`
- `/workdir/openWEPP/docs/contracts/openwepp-binary-release-contract.md`
- `/workdir/openWEPP/docs/decisions/0007-openwepp-runner-and-release-governance.md`
- `/workdir/openWEPP/docs/governance/openwepp-release-procedure-draft.md`
- `/workdir/openWEPP/docs/work-packages/20260529-relproc01-openwepp-release-procedure-draft-001/artifacts/worker-handoff.md`
- `/workdir/openWEPP/crates/openwepp-runner/src/bin/open_wepp_runner.rs`
- `/workdir/openWEPP/crates/openwepp-runner/src/errors.rs`
- `/workdir/openWEPP/crates/openwepp-runner/src/release.rs`
- `/workdir/openWEPP/crates/openwepp-runner/src/lib.rs`

## Intended Write Set
- `docs/contracts/openwepp-runner-contract.md`
- `docs/contracts/openwepp-binary-release-contract.md`
- `docs/governance/openwepp-release-procedure-draft.md`
- `docs/work-packages/README.md`
- `crates/openwepp-runner/src/bin/open_wepp_runner.rs`
- `crates/openwepp-runner/src/errors.rs`
- `docs/work-packages/20260529-relproc02-runner-sidecar-emission-cli-001/**`

## Truthfulness Requirement
Every evidence artifact must label evidence class:
- `Static:` source-inspected evidence.
- `Ran:` executed command evidence.

## Phase Plan
### Phase A - Contract/rule amendments
Amend runner and binary-release contracts to define release-sidecar command
surface, argument semantics, and failure behavior.

### Phase B - Contract-derived test updates
Add/adjust tests that assert command parsing and sidecar emission behavior
required by the amended contract surface.

### Phase C - Runner implementation
Implement `release sidecar` command parsing and sidecar emission execution,
including typed error propagation.

### Phase D - Validation and disposition
Run formatter/tests for touched crate and document evidence, review,
verification, and package disposition.

## Exit Criteria
- `open_wepp_runner` exposes `release sidecar` command with explicit binary path
  and role flags.
- Command writes/refreshes a contract-valid sidecar for the target binary.
- Runner/binary release contracts and release runbook are aligned with the new
  command surface.
- Package artifacts contain Static and Ran evidence with GO/HOLD disposition.

## Security Impact Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: local CLI argument parsing + metadata file emission only; no new
  external connectivity or privilege boundary changes.

## Autonomy
Package executed end-to-end across all phases through disposition without
requiring additional user intervention.
