# HPHYS0318 Stmtim Control-Surface Instrumentation

Status: executed-hold

## Objective

Add contract-backed openWEPP trace instrumentation for the SIMIMPL28
`stmtim` hourly precipitation partition control surfaces, then classify the
2013 day 11 hour 11 positive-`hrsnow` route without production physics or
downstream water-balance compensation.

This is the HPHYS0318 `stmtim` control-surface instrumentation package.

## Rationale

HPHYS0317 preserved the combined HPHYS0315 spring-2014 `24` rows and
HPHYS0316 spring-2016 `33` rows as one `57`-row route at the 2013 day 11 hour
11 key. Fixed baseline records `hrsnow = 0.0007454545120708644 m` while
openWEPP records `snow.hourly.snowfall_m_0011 = 0.0 m`, but paired controlling
surfaces for `rain`, `stmdur`, `wntdur`, `wnttim`, `hrtemp`, `rst`,
`hrrain`, active interval membership, and branch choice are incomplete.

## Included Scope

- Add canonical climate, snow/freeze, and water-balance contract authority for
  HPHYS0318 openWEPP-side `stmtim` trace instrumentation.
- Add contract-derived tests for HPHYS0318 contract authority, required trace
  aliases, artifact status, route totals, and no-compensation posture.
- Publish openWEPP runtime symbols for `snow.hourly.stmtim.*_####` controls
  and branch outcomes from the same SIMIMPL28 partition helper that emits
  `snow.hourly.rain_m_####` and `snow.hourly.snowfall_m_####`.
- Extend the HPHYS0245 runner trace row with maps for the new
  `snow_hourly_stmtim_*` families.
- Preserve the combined `57` carried-row route and classify missing fixed-
  baseline observe surfaces as the remaining blocker.

## Excluded Scope

- No production precipitation-phase physics edit.
- No snow producer, drift, melt-term, branch-predicate, WB13, WB17, WB18,
  WB19, or WB12 compensation.
- No source-ownership claim from source-code resemblance.
- No fallback defaults, surrogate physics, or canonicalize-and-proceed behavior
  for missing paired control surfaces.
- No fixed-baseline Fortran instrumentation unless explicitly scoped by a
  follow-on package.

## Deliverables

- `SC-CLIMATE-001` HPHYS0318 `stmtim` trace invariant.
- `SC-SNOWFREEZE-001` HPHYS0318 57-row trace/hold invariant.
- `SC-WATBAL-001` HPHYS0318 water-balance consumer gate.
- Contract-derived HPHYS0318 integration test.
- OpenWEPP SIMIMPL28 `snow.hourly.stmtim.*` runtime symbols.
- HPHYS0245 trace row maps for the new `snow_hourly_stmtim_*` families.
- Unit-registry entries for the new canonical runtime aliases.
- Gate, review, verification, disposition, and worker-handoff artifacts.

## Dependencies

- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `docs/specifications/science-contracts/index.md`
- `docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/unit-governance.md`
- `docs/specifications/correctness-authority-model.md`
- `docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `docs/decisions/0017-re-pin-operational-distrust-comparator-is-flag-not-target.md`
- `docs/work-packages/20260606-hphys0317-paired-hourly-snowfall-input-surface-parity-001/artifacts/worker-handoff.md`
- `/workdir/wepp-forest_260430_baseline/src/winter.for`
- `/workdir/wepp-forest_260430_baseline/src/stmtim.for`

## Intended Write Set

- `docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/index.md`
- `Cargo.toml`
- `crates/openwepp-sim-contract/src/units.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/06_simimpl28_hourly_forcing.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/08_tests.rs`
- `crates/openwepp-runner/src/hillslope/mod.rs`
- `tests/integration/sim_contract_boundary_unit_registry.rs`
- `tests/integration/hphys0318_stmtim_control_surface_instrumentation_contract.rs`
- `docs/work-packages/README.md`
- `docs/work-packages/20260606-hphys0318-stmtim-control-surface-instrumentation-001/**`

## Phase Plan

1. Contracts: amend canonical climate, snow/freeze, and water-balance
   contracts.
2. Contract tests: add HPHYS0318 authority and trace-surface tests.
3. Pre-implementation gate: run focused contract authority tests before
   production code edits.
4. Implementation: publish OpenWEPP `stmtim` control-surface diagnostics and
   trace maps without changing partition physics.
5. Metrics: preserve the `57` carried-row route and record H1..H39 metrics as
   instrumentation-only carry-forward unless a behavioral suite is run.
6. Review: complete dual review, disposition findings, and dual verification.
7. Closeout: update disposition, handoff, artifact statuses, and README.

## Contract-First Sequence

1. Contracts.
2. Contract-derived tests.
3. Pre-implementation contract gate.
4. Production code edits limited to trace/diagnostic instrumentation.

## Exit Criteria

- New `snow.hourly.stmtim.*` aliases are contract-authorized, registry-backed,
  emitted by SIMIMPL28, and serialized by the HPHYS0245 trace row.
- Existing `snow.hourly.rain_m_####` and `snow.hourly.snowfall_m_####`
  outputs remain behaviorally unchanged.
- The combined `57` carried rows remain represented without stale
  `OPENWEPP-DEFECTIVE` labels.
- Missing fixed-baseline paired control surfaces are recorded as the remaining
  blocker and assigned to a concrete follow-on package.
- Evidence artifacts label truthfulness (`Static:` vs `Ran:`).
- Dual review findings are dispositioned and dual verification is recorded.

## Security-Impact Gate

No security-sensitive runtime behavior, credentials, subprocess invocation, or
network operation is in scope. Package execution is local flat-file reads/edits
and local validation commands only.

## Execution Notes

HPHYS0318 amended the canonical climate, snow/freeze, and water-balance
contracts before adding the HPHYS0318 contract-derived test. OpenWEPP now emits
the `snow.hourly.stmtim.*_####` diagnostic family from the same SIMIMPL28
partition helper that produces `snow.hourly.rain_m_####` and
`snow.hourly.snowfall_m_####`; HPHYS0245 trace rows serialize the corresponding
`snow_hourly_stmtim_*` maps.

Final classification: `executed-hold`. The OpenWEPP-side observability gap is
closed, but fixed-baseline paired `stmtim` observe values are still unavailable
for `rain`, `stmdur`, rounded `wntdur`, adjusted `wnttim`, `hrtemp`, `rst`,
`hrrain`, active interval membership, and branch choice at 2013 day 11 hour
11. HPHYS0319 owns that fixed-baseline observe recovery. No precipitation-
phase, snow-producer, melt-term, branch-predicate, WB13, WB17, WB18, WB19, or
WB12 edit is authorized by HPHYS0318.
