# HPHYS0282 SC-EVAP Unit Compliance Closure

Status: completed/GO

## Objective

Resolve the remaining `SC-EVAP-001` unit-compliance lint findings by aligning canonical evapotranspiration contract rows and alias mappings with the executable boundary/output registry for `Ep`, `Es`, and `Er`.

## Rationale

HPHYS0281 closed the WB11 EVAPPM condensation implementation blocker and left only pre-existing HPHYS0279 SC-EVAP unit-compliance findings as package HOLD debt. The lint failures are documentation-governance defects: `Ep`, `Es`, and `Er` are registered as WAT publication depth columns in `mm`, but `SC-EVAP-001` does not declare all three WAT output symbols in `Variables and Units` and does not map all registered `hillslope_wat.*` aliases with registry-unit checks.

## Included Scope

- Amend canonical `SC-EVAP-001` `Variables and Units` and `Symbol Alias Map` rows for registered `Ep`, `Es`, and `Er` WAT output symbols.
- Preserve distinct runtime/process units for potential rates and final publication depths.
- Run the SC unit-compliance gate against `SC-EVAP-001` before and after edits.
- Run focused HPHYS0279 lint tests, scoped docs lint, and diff hygiene.
- Complete dual review, finding disposition, dual verification, and final HOLD/GO disposition.

## Excluded Scope

- Production kernel behavior changes.
- Boundary/output registry changes unless the lint proves the registry is inconsistent with existing output authority.
- Revisiting EVAPPM, SWU, WB17, or WAT publication physics.
- Broader unit-governance remediation outside `SC-EVAP-001`.

## Deliverables

- Contract-first package artifacts with pre-fix lint evidence.
- Updated `SC-EVAP-001` rows satisfying registry symbol, alias, and unit checks for `Ep`, `Es`, and `Er`.
- Validation evidence for SC-EVAP unit compliance, HPHYS0279 lint tests, docs lint, and diff hygiene.
- Review and verification artifacts with finding disposition and final package status.

## Dependencies

- /workdir/openWEPP/AGENTS.md
- /workdir/openWEPP/docs/codex_exec_plans.md
- docs/work-packages/README.md
- docs/specifications/science-contract-authoring-procedure.md
- docs/specifications/science-contracts/kernel-process-contract-profile.md
- docs/specifications/science-contracts/index.md
- docs/specifications/science-contracts/contracts/SC-EVAP-001.md
- docs/specifications/units/boundary-symbol-unit-registry.md
- crates/openwepp-sim-contract/src/units.rs
- tools/release/check_sc_unit_compliance.sh
- tests/integration/hphys0279_sc_unit_compliance_lint_contract.rs
- docs/work-packages/20260604-hphys0281-wb11-evappm-condensation-closure-001/artifacts/disposition.md
- docs/work-packages/20260604-hphys0281-wb11-evappm-condensation-closure-001/artifacts/gate-results.md

## Intended Write Set

- docs/specifications/science-contracts/contracts/SC-EVAP-001.md
- docs/work-packages/README.md
- docs/work-packages/20260604-hphys0282-sc-evap-unit-compliance-closure-001/**

## Phase Plan

1. Scaffold package and record pre-fix lint evidence.
2. Amend `SC-EVAP-001` contract rows for registered WAT `Ep`, `Es`, and `Er` outputs.
3. Run focused lint/tests/docs gates and record evidence.
4. Complete dual review, dual verification, and final disposition.

## Dual Review and Finding Disposition Requirement

Before final package disposition, run two independent review passes and record them in `artifacts/review_agent_a.md` and `artifacts/review_agent_b.md`. Each finding must be dispositioned as `accepted`, `rejected`, `deferred`, or `follow-up` with rationale and evidence. Accepted findings must be fixed and verified. Rejected findings must explain why no change is required. Deferred or follow-up findings must be linked from the disposition and worker-handoff artifacts. Package closure is blocked while any review finding is undispositioned.

Dual verification artifacts (`artifacts/verification_agent_a.md` and `artifacts/verification_agent_b.md`) must verify both technical gates and review finding disposition.

## Contract-First Sequence

1. Amend canonical contract authority when required.
2. Implement or identify contract-derived tests and diagnostic gates.
3. Record pre-implementation contract gate evidence.
4. Modify only the scoped contract/documentation rows.

## Exit Criteria

- `tools/release/check_sc_unit_compliance.sh --path docs/specifications/science-contracts/contracts/SC-EVAP-001.md` passes.
- Focused HPHYS0279 SC unit-compliance tests pass.
- Scoped docs lint passes.
- `git diff --check` passes.
- Dual review and dual verification are complete and fully dispositioned.
- Final package disposition is `completed/GO` unless a new unrelated gate failure is discovered and truthfully documented.

## Security-Impact Gate

No external systems or network actions are required. This package is local repository engineering work limited to flat-file reads/edits and local command execution in the worktree.

## Autonomy

This package is intended for end-to-end autonomous execution. Agents must execute all phases through disposition, update required artifacts with truthfulness labels, and only ask for user direction when hard-blocked by missing local authority or unavailable validation substrate.
