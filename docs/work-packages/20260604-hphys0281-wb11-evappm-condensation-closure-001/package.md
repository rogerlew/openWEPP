# HPHYS0281 WB11 EVAPPM Condensation Closure

Status: completed/HOLD

## Objective

Diagnose, correct, and validate the WB11 `HKERNEL-WB11-ET-E-003` failure by making the EVAPPM PMET seed producer contract-faithful for condensation cases where potential soil/residue evaporation becomes negative.

## Rationale

Full workspace gates from HPHYS0275 through HPHYS0280 remain blocked by two `pl14s` SIMIMPL18 fixture failures that abort on day 1 in the WB11 evapotranspiration phase. HPHYS0280 characterization localizes the failure to the EVAPPM seed producer publishing a material-negative `pmet.es_m` under a supersaturated cold-day climate fixture. Canonical `SC-EVAP-001` already requires the pinned legacy `evappm.for` seam behavior: when `es - resint < 0`, return the negative quantity to top-layer storage instead of publishing material-negative `es`. This package closes that contract gap rather than relaxing the WB11 guard.

## Included Scope

- Preserve the WB11 consumer guard that rejects material-negative `pmet.es_m`.
- Add diagnostic evidence for the failing SIMIMPL18 cold-day fixture and the computed EVAPPM demand components.
- Amend canonical `SC-EVAP-001` only where needed to make the producer-side condensation storage-return obligation executable and testable.
- Add contract-derived tests that fail before the production fix and pass after it.
- Port baseline-authoritative negative `es - resint` handling into the openWEPP EVAPPM seed publication path.
- Reconcile WB13 publication behavior so it no longer relies on a downstream clamp for EVAPPM material-negative `Es`.
- Run focused `pl14s` SIMIMPL18 tests and relevant workspace gates; record full evidence and HOLD reasons if unrelated gates remain.

## Excluded Scope

- Reworking the full EVAPPM, SWU, or growth model beyond the condensation seam required to clear `HKERNEL-WB11-ET-E-003`.
- Relaxing, suppressing, or widening the WB11 material-negative PMET guard.
- Rejecting `dewpoint > tmax` climate inputs as a substitute for baseline condensation handling.
- Semantic parity tuning for H1/H7/H39 beyond proving this defect no longer blocks the workspace.
- Unit-boundary remediation outside the touched EVAPPM/WB11/WB13 surfaces.

## Deliverables

- Contract/gate evidence showing the current failure and producer-side root cause.
- Contract-derived regression tests for EVAPPM condensation handling and WB13 no-clamp publication behavior.
- Production implementation that publishes non-negative `pmet.es_m` and returns negative soil/residue evaporation demand to top-layer storage in the EVAPPM seed path.
- Updated artifacts with dual independent review, review finding disposition, dual verification, and final HOLD/GO disposition.

## Dependencies

- /workdir/openWEPP/AGENTS.md
- /workdir/openWEPP/docs/codex_exec_plans.md
- docs/work-packages/README.md
- docs/specifications/science-contract-authoring-procedure.md
- docs/specifications/science-contracts/kernel-process-contract-profile.md
- docs/specifications/science-contracts/index.md
- docs/specifications/science-contracts/contracts/SC-EVAP-001.md
- docs/specifications/science-contracts/contracts/SC-WATBAL-001.md
- docs/decisions/0011-architecture-first-top-down-science-contracts.md
- docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md
- /workdir/wepp-forest_260430_baseline/src/evappm.for
- /workdir/wepp-forest_260430_baseline/src/swu.for
- docs/work-packages/20260604-hphys0280-hphys0275-typed-boundary-continuation-001/artifacts/wb11-et-e-003-characterization.md
- docs/work-packages/20260604-hphys0280-hphys0275-typed-boundary-continuation-001/artifacts/worker-handoff.md

## Intended Write Set

- docs/specifications/science-contracts/contracts/SC-EVAP-001.md
- crates/openwepp-runner/src/hillslope/mod.rs
- tests/integration/pl14s_tier_a_candidate_emission_and_replay_contract.rs
- tests/integration/hphys0281_wb11_evappm_condensation_contract.rs
- Cargo.toml
- docs/work-packages/README.md
- docs/work-packages/20260604-hphys0281-wb11-evappm-condensation-closure-001/**

## Phase Plan

1. Contract authority and diagnostic evidence.
2. Contract-derived tests and pre-implementation red/characterization gate.
3. Production EVAPPM seed and WB13 publication correction.
4. Focused validation, dual review, dual verification, and disposition.

## Dual Review and Finding Disposition Requirement

Before final package disposition, run two independent review passes and record them in `artifacts/review_agent_a.md` and `artifacts/review_agent_b.md`. Each finding must be dispositioned as `accepted`, `rejected`, `deferred`, or `follow-up` with rationale and evidence. Accepted findings must be fixed and verified. Rejected findings must explain why no change is required. Deferred or follow-up findings must be linked from the disposition and worker-handoff artifacts. Package closure is blocked while any review finding is undispositioned.

Dual verification artifacts (`artifacts/verification_agent_a.md` and `artifacts/verification_agent_b.md`) must verify both technical gates and review finding disposition.

## Contract-First Sequence

1. Amend canonical contract authority when required.
2. Implement contract-derived tests and diagnostic gates.
3. Record pre-implementation contract gate evidence.
4. Modify production code.

## Exit Criteria

- The SIMIMPL18 cold-day fixtures no longer abort with `HKERNEL-WB11-ET-E-003`.
- EVAPPM PMET seed publication never emits material-negative `pmet.es_m`; within-tolerance negative roundoff still canonicalizes to zero at consumers.
- Negative EVAPPM soil/residue demand is represented as top-layer storage return rather than output clamping or loss.
- WB13 publication no longer depends on clamping EVAPPM material-negative `Es`.
- Focused HPHYS0281 and `pl14s` SIMIMPL18 tests pass.
- Dual review and dual verification are complete and fully dispositioned.

## Security-Impact Gate

No external systems or network actions are required. This package is local repository engineering work limited to flat-file reads/edits and local command execution in the worktree.

## Autonomy

This package is intended for end-to-end autonomous execution. Agents must execute all phases through disposition, update required artifacts with truthfulness labels, and only ask for user direction when hard-blocked by missing local authority or unavailable validation substrate.
