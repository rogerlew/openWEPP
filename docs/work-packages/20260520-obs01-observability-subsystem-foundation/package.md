# 20260520-obs01-observability-subsystem-foundation

## Status
- `state`: active
- `date`: 2026-05-20
- `timezone`: UTC

## Objective
Define the first-class openWEPP observability subsystem (`OBS01`) so developers
can stimulate kernels and short phase pipelines without requiring full
end-to-end WEPP runs.

## Why This Package Exists
Ad-hoc legacy debug sidecars (`wepp_observe.on`, `wepp_observe_frost.on`,
`wepp_observe_wb05e_target.dat`) were useful for incident triage but are not a
stable architecture surface for openWEPP.

openWEPP needs typed debug intent, structured telemetry, deterministic replay
hooks, and ergonomics that prioritize rapid kernel-level diagnosis.

## Scope
### Included
- Observability subsystem charter for openWEPP runtime and developer workflows.
- Typed observability/debug-intent contract (replacing cwd sentinel toggles).
- Kernel stimulation harness requirements for single-kernel and short-pipeline
  execution.
- Structured trace/event schema requirements.
- Deterministic replay-window interface requirements.
- Migration/disposition plan from legacy `wepp_observe*` behavior to `OBS01`.
- Promotion of stable OBS01 outputs into canonical subsystem specification
  location under `docs/specifications/subsystems/observability/`.
- Work-package sequencing for follow-on implementation slices.

### Explicitly Out of Scope
- Broad runtime instrumentation implementation across all kernels.
- Full UI/visualization productization.
- Replacing existing output/report pipelines in this package.
- Carry-forward parser compatibility for `wepp_observe*` sidecars.

## Deliverables
1. Draft artifacts:
   - `artifacts/observability-subsystem-charter.md`
   - `artifacts/kernel-stimulation-use-cases.md`
   - `artifacts/typed-observability-intent-schema.md`
   - `artifacts/trace-event-schema.md`
   - `artifacts/replay-window-interface.md`
   - `artifacts/legacy-observe-migration-plan.md`
2. Canonical promoted subsystem specs:
   - `docs/specifications/subsystems/observability/observability-subsystem-contract.md`
   - `docs/specifications/subsystems/observability/debug-intent-schema.md`
   - `docs/specifications/subsystems/observability/trace-event-schema.md`
   - `docs/specifications/subsystems/observability/replay-window-interface.md`
   - `docs/specifications/subsystems/observability/legacy-observe-migration.md`
3. Disposition and promotion mapping:
   - `artifacts/obs01_disposition.md` (must include artifact -> canonical
     mapping table)

## Dependencies
- `docs/planning/openwepp-observability-subsystem-assessment.md`
- `docs/planning/wepp-input-file-parser-survey.md`
- `docs/specifications/wepp-input-files/input-surface-registry.md`
- `docs/specifications/subsystems/README.md`
- `docs/architecture/README.md`
- `docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- Static provenance references:
  - `/home/workdir/wepp-forest/src/wepp_observe.for`
  - `/home/workdir/wepp-forest/src/watbal_process_probe.f90`

## Phase Plan
### Phase 0 - Current-State Inventory
- Capture current debugging paths and pain points.
- Document legacy observe sidecar behavior and limits.

### Phase 1 - Architecture Contract
- Define `OBS01` subsystem boundaries, responsibilities, and ownership.
- Define typed debug-intent and trace-event schema requirements.

### Phase 2 - Single-Mechanism Pilot Plan
- Define one narrow pilot mechanism for kernel stimulation with replayable
  fixtures and structured traces.
- Define acceptance checks and failure semantics for the pilot.

### Phase 3 - Closeout
- Publish disposition with implementation sequencing and artifact-to-canonical
  promotion mapping.
- Queue follow-on implementation work packages (OBS02+).

## Exit Criteria
- `OBS01` charter and interfaces are documented and internally consistent.
- Kernel stimulation workflow is specified with explicit entrypoints and
  acceptance checks.
- Trace schema and replay interface requirements are documented.
- Legacy observe sidecar migration/disposition is explicit.
- Canonical OBS subsystem specs are present under
  `docs/specifications/subsystems/observability/`.
- Disposition contains explicit artifact -> canonical mapping.
- Follow-on implementation sequence is clear and executable.

## Security Impact and Review Gate
- `security_impact`: `none`
- `dedicated_security_review_required`: `no`
- Rationale: architecture/specification package only.
