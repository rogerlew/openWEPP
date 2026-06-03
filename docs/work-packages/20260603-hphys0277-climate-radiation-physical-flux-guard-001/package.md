# HPHYS0277 Climate Radiation Physical Flux Guard

Status: queued

## Objective

Add a production typed guard for physically impossible hourly radiation flux using baseline/physics-derived potential-radiation bounds rather than fixed heuristic clipping.

## Rationale

HPHYS0272 fixed the unit conversion, but production still only rejects non-finite hourly radiation. A future unit or branch error could emit finite but impossible `MJ m^-2 h^-1` radiation. The guard must fail closed and preserve baseline lineage rather than clip values.

## Included Scope

- Amend `SC-CLIMATE-001#INV-CLIMATE-013` or add a follow-on invariant for high hourly flux fail-closed behavior.
- Derive allowable hourly radiation bounds from `radcur`/potential-radiation lineage or explicit physical extraterrestrial/clear-sky authority.
- Implement typed runtime guard for impossible finite hourly radiation.
- Add red/green tests with a deliberately unit-inverted or overlarge radiation path.
- Run H1/H7/H39 and full H1..H39 metrics after guard implementation.

## Excluded Scope

- Heuristic fixed cutoff as final production authority.
- Radiation clipping or value substitution.
- Snowmelt/WB13/WB17 compensation.

## Deliverables

- Canonical high-flux radiation invariant and guard map.
- Typed runtime error for physically impossible hourly radiation.
- Contract-derived tests proving fail-closed behavior.
- Targeted and full-suite metrics proving no valid HPHYS0272 traces trip the guard.

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

- docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md
- crates/openwepp-hillslope-orchestrator/src/runtime_inputs/06_simimpl28_hourly_forcing.rs
- crates/openwepp-hillslope-orchestrator/src/runtime_inputs/08_tests.rs
- docs/work-packages/20260603-hphys0277-climate-radiation-physical-flux-guard-001/**

## Phase Plan

1. Contracts and governance authority.
2. Contract-derived tests or lint fixtures.
3. Pre-implementation contract/gate evidence.
4. Production/tooling/docs edits for the declared scope.
5. Validation, review, verification, and disposition.

Detailed phase work:

- Author high-flux guard authority in canonical contract text.
- Add red contract-derived tests for finite impossible radiation.
- Implement typed runtime guard without clipping.
- Run targeted H1/H7/H39 and full H1..H39 metrics.
- Disposition valid-run compatibility and residuals.

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

- Finite impossible hourly radiation fails with typed runtime error.
- Valid HPHYS0272 H1/H7/H39 traces pass the guard.
- No downstream compensation or clipping is introduced.

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
