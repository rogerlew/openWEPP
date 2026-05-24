# 20260524-cli02-hillslope-simulation-and-interchange-emission-001

## Status
- state: complete
- date: 2026-05-24
- timezone: UTC

## Objective
Convert CLI02 into a planning/governance package that ratifies the runner/output
contract posture and prepares a dedicated CLI03 implementation package.

## Why This Package Exists
Initial CLI02 scaffolding targeted direct implementation, but active contract
shape decisions were still changing (output schema simplification, metric-only
discoverability, legacy-sidecar discovery precedence). Implementing code before
those decisions stabilized would violate contract-first sequencing.

CLI02 therefore closes as planning authority and implementation handoff:
1. ratify canonical contract/spec posture,
2. record implementation constraints and ambiguity resolutions,
3. prepare CLI03 as the implementation package.

## Scope
### Included
- Ratify schema-versioned hillslope `.run` contract authority and revise output
  schema to flat `[outputs]` keys:
  - required `pass`/`loss`,
  - optional parquet paths `wat`, `soil`, `plot`, `ebe`, `element`.
- Ratify explicit metric-only discoverability (`unit_system = "metric"`).
- Ratify sidecar semantics and precedence:
  - snow/frost are override parameters, not routine toggles,
  - in `--legacy-sidecar-discovery`, discovered legacy sidecars are
    authoritative and `.run` sidecar override keys are ignored.
- Produce implementation handoff notes and defer code/test execution to CLI03.
- Prepare and authorize CLI03 implementation package scaffolding.

### Explicitly Out of Scope
- Production runner/CLI code edits.
- Contract-derived test implementation.
- Fixture simulation execution and output checksum run evidence.
- Package GO decision for implementation completion.

## Deliverables
1. Updated contract/spec authority evidence:
   - `artifacts/cli02-contract-implementation-evidence.md`
   - `artifacts/cli02-runner-interchange-authority-and-guard-map.md`
2. Implementation deferral and handoff artifacts:
   - `artifacts/worker-handoff.md`
   - `artifacts/cli02_disposition.md`
3. Governance closeout artifacts:
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`
4. Prepared follow-on implementation package:
   - `/home/workdir/openWEPP/docs/work-packages/20260524-cli03-hillslope-runner-interchange-implementation-001/`

## Mandatory Contract-First Sequence (Required)
Implementation remains required to follow:
1. implement canonical contract/spec amendments,
2. implement contract-derived tests,
3. record pre-implementation contract gate evidence, then
4. modify production code.

CLI02 closes before step 2 by design; CLI03 begins at step 2.

## Truthfulness Labeling Requirement
All evidence artifacts must explicitly label evidence mode using `Static:` and/
or `Ran:` sections. Claims without explicit evidence labeling are
non-compliant.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/contracts/openwepp-hillslope-runfile-contract.md`
- `/workdir/openWEPP/docs/contracts/openwepp-runner-contract.md`
- `/workdir/openWEPP/docs/contracts/README.md`
- `/workdir/openWEPP/docs/specifications/subsystems/runner/README.md`
- `/workdir/openWEPP/docs/specifications/subsystems/runner/openwepp-hillslope-cli-specification.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/work-packages/20260522-pl09-openwepp-totality-and-pl08-hold-lift-discovery-001/artifacts/pl08-hold-lift-work-package-queue.md`

## Intended Write Set
- `docs/contracts/openwepp-hillslope-runfile-contract.md`
- `docs/contracts/openwepp-runner-contract.md`
- `docs/contracts/README.md`
- `docs/specifications/subsystems/runner/openwepp-hillslope-cli-specification.md`
- `docs/specifications/subsystems/runner/README.md`
- `docs/work-packages/20260524-cli02-hillslope-simulation-and-interchange-emission-001/**`
- `docs/work-packages/20260524-cli03-hillslope-runner-interchange-implementation-001/**`
- `docs/work-packages/README.md`
- `docs/work-packages/20260522-pl09-openwepp-totality-and-pl08-hold-lift-discovery-001/artifacts/pl08-hold-lift-work-package-queue.md`

## Phase Plan
### Phase A - Contract Planning Authority
- Ratify contract/spec posture updates for CLI02 planning scope.

### Phase B - Implementation Handoff Definition
- Capture unresolved implementation work as explicit deferred artifacts.

### Phase C - CLI03 Preparation
- Scaffold CLI03 implementation package with required governance structure and
  queued evidence placeholders.

### Phase D - Planning Closeout
- Publish review/verification/disposition artifacts for planning package
  completion.

## Exit Criteria
- CLI02 contract planning authority is documented and traceable.
- Output schema and sidecar precedence ambiguities are resolved in canonical
  docs.
- CLI03 implementation package is scaffolded and queued with explicit
  contract-first execution constraints.
- CLI02 disposition clearly defers implementation/test execution to CLI03.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- Rationale: planning/governance package only; no runtime code edits.
