# HPHYS0279 SC Contract Unit Compliance Lint

Status: completed/HOLD

## Objective

Add contract/documentation linting that enforces Variables and Units coverage, alias-map unit checks, and registry cross-links for kernel-affecting `SC-*` contracts.

## Rationale

The contract authoring procedure requires Variables and Units tables and alias maps, but compliance is currently human-reviewed. A lint gate should make missing units, missing alias unit checks, and unregistered runtime symbols visible before production edits.

## Included Scope

- Implement or extend docs lint tooling for `SC-*` unit sections and alias unit checks.
- Cross-check contract symbols against the HPHYS0274 unit registry where available.
- Report missing, ambiguous, or conflicting unit declarations as typed lint findings.
- Add tests/fixtures for compliant and non-compliant contract snippets.
- Run lint over current `SC-*` files and record residual gap inventory.

## Excluded Scope

- Correcting every existing contract gap in one pass unless trivial.
- Production runtime changes.
- Output metadata alignment owned by HPHYS0278.

## Deliverables

- SC contract unit compliance lint command.
- Failing/passing lint fixtures or tests.
- Current contract unit gap inventory with HOLD disposition for unresolved gaps.
- Release/work-package gate integration recommendation.

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

- tools/**
- docs/specifications/science-contract-authoring-procedure.md
- docs/specifications/science-contracts/kernel-process-contract-profile.md
- docs/specifications/science-contracts/contracts/SC-*.md
- tests/**
- docs/work-packages/20260603-hphys0279-sc-contract-unit-compliance-lint-001/**

## Phase Plan

1. Contracts and governance authority.
2. Contract-derived tests or lint fixtures.
3. Pre-implementation contract/gate evidence.
4. Production/tooling/docs edits for the declared scope.
5. Validation, review, verification, and disposition.

Detailed phase work:

- Define lint rules from HPHYS0273 governance.
- Implement parser/checker and fixture tests.
- Cross-check contracts against unit registry.
- Run lint over current contracts and record gaps.
- Disposition gate readiness and follow-up fixes.

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

- Lint fails when Variables and Units or alias unit checks are missing.
- Lint integrates with documented work-package/release gate workflow.
- Current SC unit gaps are inventoried rather than hidden.

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
