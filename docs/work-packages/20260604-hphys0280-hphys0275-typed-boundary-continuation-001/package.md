# HPHYS0280 HPHYS0275 Typed Boundary Continuation

Status: completed/HOLD

## Objective

Execute HPHYS0275 continuation work by extending typed `BoundaryValue` coverage to direction-specific climate symbols, watershed-prefixed climate aliases, and first snow runtime/trace state surfaces without changing physics or output values.

## Rationale

HPHYS0275 closed the first typed-boundary wave but explicitly left wind direction, watershed-prefixed climate aliases, and snow runtime/trace families as follow-up gaps. Those surfaces remain unit-governance risks because they can still cross runtime seams as raw scalar values even when the registry has canonical units.

## Included Scope

- Add a direction-specific degree wrapper with typed `BoundaryValue` support.
- Migrate hillslope climate `wind` direction publication from scalar to typed degrees.
- Migrate watershed-prefixed climate aliases generated from typed daily climate surfaces to typed values where unit authority is already canonical.
- Migrate selected snow runtime state and retained snow trace families to typed values: SWE/depth/rain/snow/melt water depth in meters, density in `kg m^-3`, and unit-interval or count-like scalar exceptions as documented.
- Add contract-derived tests proving new surfaces are non-scalar and invalid direction domains fail closed.
- Update canonical unit-safe boundary and registry docs plus package evidence.

## Excluded Scope

- Output Parquet metadata alignment owned by HPHYS0278.
- Repository-wide scalar removal.
- Physics changes, snowmelt equation changes, water-balance residual compensation, or comparator-threshold changes.
- Integer/count enum typing for `snow.runtime_settle_day_count`; document as scalar count follow-up unless a count wrapper is explicitly specified.

## Deliverables

- Direction-degree wrapper and `BoundaryValue` variant.
- Contract and registry amendments for HPHYS0280 typed continuation scope.
- Runtime producer migrations for declared climate/watershed/snow surfaces.
- Contract-derived red/green tests and validation evidence.
- Residual scalar follow-up list for surfaces outside this wave.

## Dependencies

- /workdir/openWEPP/AGENTS.md
- /workdir/openWEPP/docs/codex_exec_plans.md
- /workdir/openWEPP/docs/work-packages/README.md
- docs/specifications/science-contract-authoring-procedure.md
- docs/specifications/science-contracts/kernel-process-contract-profile.md
- docs/specifications/unit-governance.md
- docs/specifications/science-contracts/index.md
- docs/specifications/science-contracts/unit-safe-boundary-types-contract.md
- docs/specifications/units/boundary-symbol-unit-registry.md
- docs/architecture/unit-safe-boundary-types.md
- docs/decisions/0011-architecture-first-top-down-science-contracts.md
- docs/work-packages/20260603-hphys0275-boundaryvalue-dimensional-typing-remediation-001/artifacts/disposition.md
- docs/work-packages/20260603-hphys0275-boundaryvalue-dimensional-typing-remediation-001/artifacts/worker-handoff.md
- docs/work-packages/20260603-hphys0275-boundaryvalue-dimensional-typing-remediation-001/artifacts/unit-governance-gap-analysis.md

## Intended Write Set

- crates/openwepp-unit-boundary/src/lib.rs
- crates/openwepp-kernel-contract/src/lib.rs
- crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs
- crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs
- crates/openwepp-hillslope-orchestrator/src/runtime_inputs/**
- crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs
- tests/integration/clim05_snow_runtime_kernel_contract.rs
- tests/integration/hphys0275_boundary_value_dimensional_typing_contract.rs
- docs/architecture/unit-safe-boundary-types.md
- docs/specifications/science-contracts/unit-safe-boundary-types-contract.md
- docs/specifications/units/boundary-symbol-unit-registry.md
- docs/work-packages/README.md
- docs/work-packages/20260604-hphys0280-hphys0275-typed-boundary-continuation-001/**

## Phase Plan

1. Contracts and governance authority.
2. Contract-derived tests and red gate evidence.
3. Production runtime typing edits.
4. Validation, dual review, dual verification, and disposition.

## Dual Review and Finding Disposition Requirement

Before final package disposition, run two independent review passes and record them in `artifacts/review_agent_a.md` and `artifacts/review_agent_b.md`. Each finding must be dispositioned as `accepted`, `rejected`, `deferred`, or `follow-up` with rationale and evidence. The package may not move to `completed`, `completed/HOLD`, or `GO` while any review finding is undispositioned.

Dual verification artifacts (`artifacts/verification_agent_a.md` and `artifacts/verification_agent_b.md`) must verify both technical gates and review finding disposition.

## Contract-First Sequence

1. Amend canonical contract or governance authority.
2. Implement contract-derived tests, lint fixtures, or red gate evidence.
3. Record pre-implementation contract gate evidence.
4. Modify production code, tooling, registry files, or docs.

## Exit Criteria

- Direction degrees reject non-finite/out-of-domain values and publish `wind` with a non-scalar unit label.
- Declared watershed-prefixed climate aliases preserve typed unit labels rather than scalarizing typed daily climate values.
- Declared snow runtime/trace water-depth and density surfaces publish typed unit labels.
- Focused HPHYS0275/HPHYS0280 tests pass and no HPHYS0275 first-wave regression is introduced.
- Remaining scalar surfaces are explicitly documented as follow-up.
- Dual review and dual verification are complete and fully dispositioned.

## Security-Impact Gate

No external systems or network actions are required. This package is local repository engineering work limited to flat-file reads/edits and local command execution in the worktree.

## Autonomy

This package is intended for end-to-end autonomous execution. Agents must execute all phases through disposition, update required artifacts with truthfulness labels, and only ask for user direction when hard-blocked by missing local authority or unavailable validation substrate.
