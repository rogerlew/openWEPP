# HPHYS0278 Output Unit Metadata Registry Alignment

Status: completed/HOLD

## Objective

Align hillslope and watershed output Parquet unit metadata with the canonical unit registry so output schemas cannot drift from runtime units.

## Rationale

Output writers attach unit metadata, but today that metadata is local writer code. Runtime symbols and output columns can diverge without a single unit authority. Registry-backed output metadata makes publication units auditable and testable.

## Included Scope

- Map output columns to canonical unit registry entries or documented publication-only units.
- Refactor writer schema unit metadata to consume registry constants where feasible.
- Add tests that reject missing, mismatched, or stale output metadata for registered columns.
- Preserve legacy WAT column names while making unit mappings explicit.

## Excluded Scope

- Changing output values or comparator tolerances.
- Changing WAT/publication column names.
- Runtime typed BoundaryValue migration owned by HPHYS0275.

## Deliverables

- Registry-backed output metadata mapping.
- Hillslope and watershed output schema unit tests.
- Coverage report for columns not yet registry-backed.

## Dependencies

- /workdir/openWEPP/AGENTS.md
- /workdir/openWEPP/docs/codex_exec_plans.md
- /workdir/openWEPP/docs/work-packages/README.md
- docs/specifications/science-contract-authoring-procedure.md
- docs/specifications/science-contracts/kernel-process-contract-profile.md
- docs/specifications/unit-governance.md
- docs/specifications/science-contracts/index.md
- docs/decisions/0011-architecture-first-top-down-science-contracts.md
- docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md
- docs/work-packages/20260603-hphys0272-hourly-radiation-unit-closure-001/artifacts/disposition.md
- docs/work-packages/20260603-hphys0272-hourly-radiation-unit-closure-001/artifacts/worker-handoff.md
- docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md
- docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md
- docs/specifications/science-contracts/contracts/SC-WATBAL-001.md
- docs/specifications/science-contracts/contracts/SC-SOIL-001.md
- docs/specifications/science-contracts/contracts/SC-PERC-001.md
- docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md
- docs/specifications/science-contracts/contracts/SC-EVAP-001.md
- docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md

## Intended Write Set

- crates/openwepp-hillslope-output/src/hillslope_wat.rs
- crates/openwepp-watershed-output/src/writers.rs
- crates/openwepp-sim-contract/**
- tests/integration/**
- docs/work-packages/20260603-hphys0278-output-unit-metadata-registry-alignment-001/**

## Phase Plan

1. Contracts and governance authority.
2. Contract-derived tests or lint fixtures.
3. Pre-implementation contract/gate evidence.
4. Production/tooling/docs edits for the declared scope.
5. Validation, review, verification, and disposition.

Detailed phase work:

- Inventory hillslope and watershed output unit metadata.
- Bind metadata fields to registry authority or explicit exceptions.
- Add mismatch tests and update writer code.
- Run output and comparator-adjacent tests.
- Record residual publication-only unit gaps.

## Dual Review and Finding Disposition Requirement

Before final package disposition, run two independent review passes and
record them in `artifacts/review_agent_a.md` and
`artifacts/review_agent_b.md`. Each review artifact must include:

- scope reviewed,
- findings with severity,
- required disposition for every finding (`accepted`, `rejected`, `deferred`,
  or `follow-up`),
- rationale/evidence for the disposition,
- file/path references for accepted fixes or follow-up package links for
  deferred work.

The package may not move to `completed`, `completed/HOLD`, or `GO` while any
review finding is undispositioned. Accepted findings must be fixed and
verified, rejected findings must explain why no change is required, and
deferred/follow-up findings must be linked from `artifacts/disposition.md` and
`artifacts/worker-handoff.md`.

Dual verification artifacts (`artifacts/verification_agent_a.md` and
`artifacts/verification_agent_b.md`) must verify both the technical gates and
that review findings were fully dispositioned.

## Contract-First Sequence

1. Amend canonical contract or governance authority.
2. Implement contract-derived tests, lint fixtures, or red gate evidence.
3. Record pre-implementation contract gate evidence.
4. Modify production code, tooling, registry files, or docs.

## Exit Criteria

- Registered output columns have metadata matching registry units.
- Tests fail on deliberate metadata/unit mismatch.
- Unregistered publication columns are explicitly listed with rationale.

- Dual review artifacts exist and every review finding is dispositioned with fixes, rejection rationale, or linked follow-up.

## Security-Impact Gate

No external systems or network actions are required. This package is local
repository engineering work limited to flat-file reads/edits and local
command execution in the worktree.

## Autonomy

This package is intended for end-to-end autonomous execution. Agents must
execute all phases through disposition, update required artifacts with
truthfulness labels, and only ask for user direction when hard-blocked by
missing local authority or unavailable validation substrate.
