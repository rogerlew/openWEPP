# HPHYS0276 Unit Conversion Helper and Raw Literal Guard

Status: completed/HOLD

## Objective

Centralize unit conversion constants/helpers and add anti-evasion guards so production code cannot scatter raw conversion literals for dimensional unit changes.

## Rationale

The HPHYS0272 defect involved a valid conversion constant used in the wrong direction. openWEPP also contains direct conversions such as `* 1000.0`, `* 0.001`, `* 86400.0`, and domain-specific constants. These must be named, directional, provenance-backed conversions with tests.

## Included Scope

- Inventory raw conversion literals and classify legitimate dimensional conversions.
- Create named directional conversion helpers with provenance comments or contract anchors.
- Replace selected high-risk raw literals with helpers.
- Add a source-level guard script that flags unauthorized raw conversion literals in production paths.
- Document allowed exceptions and review workflow.

## Excluded Scope

- Changing formulas or physics behavior.
- Migrating every conversion literal in one pass when risk is low or ambiguous.
- Registry creation, except consuming HPHYS0274 unit classes.

## Deliverables

- Unit conversion helper module or crate.
- Raw conversion literal guard script.
- Focused replacements for high-risk hydrology/climate conversions.
- Tests proving helper direction and guard behavior.

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
- crates/openwepp-hillslope-orchestrator/**
- crates/openwepp-runner/**
- tools/release/**
- tests/integration/**
- docs/work-packages/20260603-hphys0276-unit-conversion-helper-and-raw-literal-guard-001/**

## Phase Plan

1. Contracts and governance authority.
2. Contract-derived tests or lint fixtures.
3. Pre-implementation contract/gate evidence.
4. Production/tooling/docs edits for the declared scope.
5. Validation, review, verification, and disposition.

Detailed phase work:

- Inventory dimensional conversion literals and prioritize by risk.
- Author conversion helpers and contract-derived tests.
- Replace selected production literals.
- Add guard script with allowlist semantics.
- Run focused and anti-evasion validation.

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

- High-risk conversions use named directional helpers.
- Guard script catches unauthorized raw conversion literals without blocking documented exceptions.
- HPHYS0272 radiation conversion is represented through named helper authority.

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
