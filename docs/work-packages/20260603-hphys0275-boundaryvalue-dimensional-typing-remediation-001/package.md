# HPHYS0275 BoundaryValue Dimensional Typing Remediation

Status: queued

## Objective

Expand and apply unit-safe `BoundaryValue`/`openwepp-unit-boundary` typing for high-risk dimensional runtime boundary surfaces, reducing reliance on raw scalar values.

## Rationale

`openwepp-unit-boundary` and typed `BoundaryValue` variants already exist, but most hydrology and climate runtime surfaces still publish dimensional quantities as raw scalars. This allows unit mixups to pass through module boundaries until residual diagnostics expose them.

## Included Scope

- Expand unit wrapper types for high-risk units missing today: meters, millimeters, `m s^-1`, `MJ m^-2 d^-1`, `MJ m^-2 h^-1`, density, temperature, percent/fraction where appropriate.
- Migrate selected producer/consumer seams from raw scalar to typed variants in priority order.
- Keep canonical legacy aliases intact while making units explicit at construction and extraction.
- Add tests for typed construction failures and unit-label propagation.

## Excluded Scope

- Full repository-wide scalar removal in one package.
- Output metadata alignment work owned by HPHYS0278.
- Registry creation owned by HPHYS0274 except as consumed by this package.

## Deliverables

- Expanded unit-boundary wrappers and typed BoundaryValue variants.
- Migrated high-risk runtime surfaces for water depth, radiation, and key rates.
- Contract-derived unit construction/domain tests.
- Residual scalar exception list for future packages.

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

- crates/openwepp-unit-boundary/src/lib.rs
- crates/openwepp-kernel-contract/src/lib.rs
- crates/openwepp-hillslope-orchestrator/**
- crates/openwepp-runner/**
- tests/integration/**
- docs/work-packages/20260603-hphys0275-boundaryvalue-dimensional-typing-remediation-001/**

## Phase Plan

1. Contracts and governance authority.
2. Contract-derived tests or lint fixtures.
3. Pre-implementation contract/gate evidence.
4. Production/tooling/docs edits for the declared scope.
5. Validation, review, verification, and disposition.

Detailed phase work:

- Classify scalar symbols by dimensional risk using HPHYS0274 registry.
- Extend unit wrapper types with typed constructors and conversions.
- Migrate first high-risk seams with red/green contract tests.
- Record scalar exceptions and follow-on scope.
- Run focused and workspace validation gates.

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

- High-risk selected dimensional symbols no longer cross runtime seams as untyped scalars.
- Typed constructors fail closed for non-finite and domain-invalid values.
- Remaining scalar dimensional surfaces are explicitly listed as follow-on work.

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
