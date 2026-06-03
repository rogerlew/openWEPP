# HPHYS0274 Boundary Symbol Unit Registry Closure

Status: completed

## Objective

Implement a machine-readable boundary-symbol unit registry and validation gate so dimensional runtime symbols have authoritative units independent of naming convention.

## Rationale

Runtime seams currently rely heavily on `BoundarySymbol` strings and `BoundaryValue::scalar`. Unit suffixes help but do not prove that producers and consumers agree. A registry gives contracts, runtime checks, tests, and output metadata one source of truth.

## Included Scope

- Choose registry shape and location from HPHYS0273 governance.
- Add registry entries for high-risk hydrology, snow/freeze, ET, climate, soil, percolation, and publication symbols.
- Add tests or lint that reject dimensional symbols missing registry entries.
- Add alias mapping from canonical `SC-*` symbols to registry units.
- Record gaps as explicit HOLD rows rather than silently omitting symbols.

## Excluded Scope

- Migrating all runtime producers to typed `BoundaryValue` variants.
- Changing physics equations or output values.
- Adding high-flux radiation physics guard.

## Deliverables

- Machine-readable boundary-symbol unit registry.
- Registry loader/validator or docs-lint guard.
- Contract-derived tests for missing/ambiguous units.
- Initial audited registry coverage report.

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

- docs/specifications/units/**
- crates/openwepp-sim-contract/**
- crates/openwepp-kernel-contract/**
- tools/**
- tests/integration/**
- docs/work-packages/20260603-hphys0274-boundary-symbol-unit-registry-closure-001/**

## Phase Plan

1. Contracts and governance authority.
2. Contract-derived tests or lint fixtures.
3. Pre-implementation contract/gate evidence.
4. Production/tooling/docs edits for the declared scope.
5. Validation, review, verification, and disposition.

Detailed phase work:

- Extract current symbol/unit declarations from contracts and output schemas.
- Create registry with explicit schema and loader validation.
- Add missing-symbol and conflicting-unit tests.
- Wire registry validation into a gate command.
- Record coverage and residual registry gaps.

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

- Dimensional runtime symbols in the touched scope have registry units.
- A missing-unit registry test fails before registration and passes after registration.
- Registry validation is documented as a mandatory gate for future packages.

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
