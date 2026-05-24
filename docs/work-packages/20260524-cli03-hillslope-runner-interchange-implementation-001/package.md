# 20260524-cli03-hillslope-runner-interchange-implementation-001

## Status
- state: queued
- date: 2026-05-24
- timezone: UTC

## Objective
Implement the hillslope runner/CLI execution path to conform to the ratified
openWEPP `.run` contract, including simplified output configuration,
metric-only enforcement, and explicit legacy sidecar discovery behavior.

## Why This Package Exists
CLI02 closed as planning authority and ratified the hillslope `.run` contract,
runner boundary semantics, output contract simplification, and legacy sidecar
precedence rules. CLI03 is the follow-on execution package for code/test
implementation.

CLI03 must execute under contract-first sequencing:
1. validate/ratify canonical authority sufficiency for execution,
2. implement contract-derived tests,
3. record pre-implementation contract gate evidence, then
4. modify production runner/CLI code.

## Scope
### Included
- Implement `.run` ingestion/validation behavior aligned to
  `openwepp-hillslope-runfile-v1` contract:
  - required schema id and metric-only `unit_system`,
  - required core input bindings,
  - optional sidecar controls,
  - required output keys (`pass`, `loss`) and optional parquet keys
    (`wat`, `soil`, `plot`, `ebe`, `element`).
- Implement `--legacy-sidecar-discovery` behavior where discovered sidecars are
  authoritative and `.run` sidecar override keys are ignored.
- Preserve non-legacy mode behavior where optional sidecar keys in `.run` are
  accepted as optional controls.
- Enforce semantic rule that `snow`/`frost` are overrides, not routine toggles.
- Implement required output validation:
  - required `pass` (`.hbp`) and `loss` (`.json`) hard-fail when missing,
  - optional parquet outputs emitted only when configured.
- Preserve launcher-managed manifest path behavior and checksum coverage rules.
- Implement contract-derived tests and integration evidence for CLI03
  acceptance surfaces.
- Extract/organize hillslope output implementation into dedicated crate
  `crates/openwepp-hillslope-output/` with crate-owned contracts, serializers,
  and tests.

### Explicitly Out of Scope
- Legacy WEPP binary orchestration (owned by `wepppy/wepp_runner`).
- New output-family subsystem design beyond current optional parquet keys.
- Crop output contract authoring.
- Watershed executable implementation scope expansion.

## Deliverables
1. CLI03 contract authority implementation evidence:
   - `artifacts/cli03-contract-implementation-evidence.md`
2. Runner/interchange authority and guard map:
   - `artifacts/cli03-runner-interchange-authority-and-guard-map.md`
3. CLI03 contract-derived test implementation evidence:
   - `artifacts/cli03-contract-test-implementation-evidence.md`
4. CLI03 pre-implementation contract gate evidence:
   - `artifacts/cli03-preimplementation-contract-gate.md`
5. CLI03 implementation and test evidence:
   - `artifacts/cli03-implementation-and-test-evidence.md`
6. Fixture simulation and interchange output evidence:
   - `artifacts/cli03-fixture-simulation-run-and-interchange-output-evidence.md`
7. Manifest schema and output checksum evidence:
   - `artifacts/cli03-manifest-schema-and-output-checksum-evidence.md`
8. Legacy sidecar discovery mode evidence:
   - `artifacts/cli03-legacy-sidecar-discovery-evidence.md`
9. Output crate organization evidence:
   - `artifacts/cli03-output-crate-organization-evidence.md`
10. Kernel-profile compliance checklist:
   - `artifacts/cli03-kernel-profile-compliance-checklist.md`
11. wepppy consumer boundary note:
   - `artifacts/cli03-wepppy-consumer-boundary-note.md`
12. Package governance artifacts:
   - `artifacts/worker-handoff.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/cli03_disposition.md`
13. Dual review artifacts:
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
14. Dual verification artifacts:
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
1. Confirm canonical contract/spec authority is sufficient for CLI03 execution;
   if material authority gaps exist, amend canonical docs first.
2. Implement contract-derived tests.
3. Record pre-implementation contract-gate evidence.
4. Only then modify production runner/CLI code.

Any sequencing violation keeps package disposition in `HOLD`.

## Truthfulness Labeling Requirement
All evidence artifacts must explicitly label evidence mode using `Static:` and/
or `Ran:` sections. Claims without explicit evidence labeling are
non-compliant.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/contracts/openwepp-hillslope-runfile-contract.md`
- `/workdir/openWEPP/docs/contracts/openwepp-runner-contract.md`
- `/workdir/openWEPP/docs/contracts/README.md`
- `/workdir/openWEPP/docs/specifications/subsystems/runner/README.md`
- `/workdir/openWEPP/docs/specifications/subsystems/runner/openwepp-hillslope-cli-specification.md`
- `/workdir/openWEPP/docs/work-packages/20260524-cli02-hillslope-simulation-and-interchange-emission-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260524-cli02-hillslope-simulation-and-interchange-emission-001/artifacts/worker-handoff.md`
- `/workdir/openWEPP/docs/work-packages/20260522-pl09-openwepp-totality-and-pl08-hold-lift-discovery-001/artifacts/pl08-hold-lift-work-package-queue.md`
- `/workdir/wepp-forest_260430_baseline` @ `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`
- `/workdir/openWEPP/crates/openwepp-runner/`
- `/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/`
- `/workdir/openWEPP/crates/openwepp-hillslope-output/` (new in CLI03)
- `/workdir/openWEPP/tests/integration/`

## Intended Write Set
- `crates/openwepp-runner/**`
- `crates/openwepp-hillslope-orchestrator/**`
- `crates/openwepp-hillslope-output/**`
- `tests/integration/**`
- `tests/fixtures/**`
- `docs/contracts/openwepp-hillslope-runfile-contract.md` (if authority gaps are
  discovered and ratified)
- `docs/contracts/openwepp-runner-contract.md` (if authority gaps are
  discovered and ratified)
- `docs/specifications/subsystems/runner/openwepp-hillslope-cli-specification.md`
  (if authority gaps are discovered and ratified)
- `docs/work-packages/20260524-cli03-hillslope-runner-interchange-implementation-001/**`
- `docs/work-packages/README.md`

## Phase Plan
### Phase 0 - Intake
- Confirm CLI03 queue objective, CLI02 handoff constraints, and dependency
  posture.

### Phase 1 - Contract Sufficiency Check
- Verify canonical authority remains sufficient for implementation; capture any
  required contract deltas before code work.

### Phase 2 - Contract Tests + Pre-Implementation Gate
- Implement contract-derived tests and record pre-implementation contract gate
  evidence.

### Phase 3 - Runner/CLI Implementation
- Implement production code changes for `.run` validation, sidecar mode
  behavior, required/optional output handling, and manifest/checksum behavior.

### Phase 4 - Verification
- Run targeted integration tests plus required repository gates.

### Phase 5 - Disposition
- Publish evidence set, dual review/verification, and final CLI03 disposition.

## Exit Criteria
- CLI03 objective is evidence-backed.
- `.run` contract behaviors are implemented for required/optional inputs and
  outputs.
- `unit_system = "metric"` is explicitly enforced.
- `--legacy-sidecar-discovery` behavior is implemented with authoritative
  discovered sidecars and `.run` sidecar override suppression.
- `snow`/`frost` override semantics are preserved (not routine toggles).
- Required outputs (`pass`, `loss`) are enforced with hard-fail behavior.
- Optional parquet outputs are emitted only when configured.
- Dedicated crate `crates/openwepp-hillslope-output/` owns output contracts,
  serializer logic, and output-surface tests.
- Contract-derived tests are implemented and executed.
- Pre-implementation contract gate is recorded before production code edits.
- If code is changed, run and record:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`

## Security Impact and Review Gate
- security_impact: medium
- dedicated_security_review_required: yes
- Rationale: executable runner/CLI path validation and output-surface
  enforcement behavior changes.
