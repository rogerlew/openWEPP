# HPHYS0273 Unit Governance Standard Closure

Status: completed

## Objective

Author canonical openWEPP unit-governance policy for science contracts, runtime boundary symbols, conversions, tests, output metadata, and work-package gates so unit defects such as the HPHYS0272 radiation seam are prevented rather than discovered by downstream residuals.

## Rationale

HPHYS0272 exposed that openWEPP has strong contract expectations for units but inconsistent runtime enforcement. Existing patterns include contract Variables and Units tables, unit-suffixed symbol names, `openwepp-unit-boundary` wrappers, `BoundaryValue` typed variants, raw `BoundaryValue::scalar` maps, and Parquet field metadata. The governance gap is that these patterns are not unified into a mandatory standard.

## Included Scope

- Create or amend canonical unit-governance documentation and contract-profile language.
- Define internal canonical units by process family and boundary class.
- Define when `BoundaryValue::scalar` is allowed and when typed units are required.
- Define a machine-readable symbol-unit registry requirement.
- Define mandatory named conversion-helper and raw-conversion-literal policy.
- Define release/work-package gates for unit registry, conversion, and output metadata compliance.
- Finalize and cross-link follow-up remediation packages HPHYS0274 through HPHYS0279.

## Excluded Scope

- Large production migrations of existing symbols.
- Adding the high hourly radiation production guard.
- Changing comparator tolerances or semantic parity disposition.

## Deliverables

- Canonical unit-governance standard document or profile amendment.
- Contract amendments that make unit governance normative.
- A prioritized remediation package queue with explicit success criteria.
- Governance lint/gate requirements ready for implementation packages.

## Dependencies

- /workdir/openWEPP/AGENTS.md
- /workdir/openWEPP/docs/codex_exec_plans.md
- /workdir/openWEPP/docs/work-packages/README.md
- docs/specifications/science-contract-authoring-procedure.md
- docs/specifications/science-contracts/kernel-process-contract-profile.md
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

- docs/specifications/unit-governance.md
- docs/specifications/science-contract-authoring-procedure.md
- docs/specifications/science-contracts/kernel-process-contract-profile.md
- docs/specifications/science-contracts/index.md
- docs/work-packages/README.md
- docs/work-packages/20260603-hphys0273-unit-governance-standard-closure-001/**

## Phase Plan

1. Contracts and governance authority.
2. Contract-derived tests or lint fixtures.
3. Pre-implementation contract/gate evidence.
4. Production/tooling/docs edits for the declared scope.
5. Validation, review, verification, and disposition.

Detailed phase work:

- Inventory current unit-carrying patterns and failure modes.
- Author normative unit governance standard and contract-profile amendments.
- Define package gates and release-lint requirements.
- Review follow-up remediation scope for coverage and ordering.
- Record disposition and handoff to HPHYS0274.

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

- A single canonical unit-governance standard exists and is linked from contract authoring/profile docs.
- The standard explicitly addresses registry, typed boundaries, scalar exceptions, conversions, output metadata, and work-package gates.
- Follow-up remediation packages cover every identified governance gap without overlap ambiguity.

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
