# HPHYS0292 Spring Snowmelt Infiltration Capacity Lineage Closure

Status: executed-hold

This ExecPlan follows `/workdir/openWEPP/docs/codex_exec_plans.md`.

## Purpose / Big Picture

After this package, the remaining H1/H7/H39 spring collapse should be localized
to a defended producer-side mechanism: snowmelt magnitude/timing, rain-on-snow
release, WB12 infiltration capacity, or downstream storage. The package must
preserve the HPHYS0291 WB13 fail-closed flux lifecycle and must not reintroduce
state-derived `RM` fallback.

## Progress

- [x] (2026-06-05) Scaffolded HPHYS0292 package structure and queued artifacts.
- [x] (2026-06-05) Read HPHYS0291 handoff, canonical contracts, and baseline winter/grna seams.
- [x] (2026-06-05) Amended contracts for producer-side snowmelt/infiltration ownership.
- [x] (2026-06-05) Added contract-derived tests before production edits.
- [x] (2026-06-05) Recorded pre-implementation contract-gate evidence.
- [x] (2026-06-05) Patched production WB14 snowmelt capacity allocation for proven lineage defect.
- [x] (2026-06-05) Ran focused tests, gates, H1/H7/H39 traces, and full H1..H39 metrics.
- [ ] Complete dual independent review and verification; current execution is held pending explicit subagent authorization.

## Objective

Diagnose, correct if proven, and validate spring snowmelt liquid partition
upstream of WB13 for H1/H7/H39 by tracing:

- snowpack state and daily/hourly melt magnitude,
- retained and released rain-on-snow,
- `wmelt` / `snow.routed_melt_m` producer publication,
- WB12 infiltration capacity and `Q`,
- WB13 `RM`, `Snow-Water`, `Total-Soil`, and `SoilWaterTotal`.

## Rationale

HPHYS0291 closed the same-day publication lifecycle for `snow.routed_melt_m` and
`snow.post_winter_rain_m`, but full H1..H39 semantic parity remains `0/39`.
H1/H7/H39 traces show spring days where the candidate produces large `RM` and
`Q` while baseline has `Q = 0` and higher snowpack / soil storage. That pattern
is upstream of WB13 publication and must be split between snowmelt timing and
WB12 infiltration-capacity ownership before returning to ET or storage tuning.

## Included Scope

- Contract amendments in canonical `SC-*` files for producer-side
  snowmelt/rain-on-snow/infiltration ownership.
- Contract-derived tests proving:
  - no-snow dry days publish zero routed-melt surfaces,
  - warm-rain/no-snow days remain direct liquid, not snowmelt,
  - rain-on-snow retention/release remains routed through `wmelt`,
  - active snowmelt enters WB12 infiltration before `Q`.
- H1/H7/H39 diagnostic traces for spring 2014/2016 collapse days.
- Full H1..H39 runtime and semantic metrics.
- Dual reviews, review disposition, dual verification, final disposition, and
  worker handoff.

## Excluded Scope

- Reintroducing WB13 inference, fallback, or state-derived `RM` publication.
- Empirical snowmelt or runoff multipliers.
- Broad ET, lateral flow, percolation, or storage rewrites unless a
  package-scoped contract test proves direct ownership.
- Replicating known legacy bugs where openWEPP already has a documented
  guard-consistency correction.

## Deliverables

- Canonical contract amendments with baseline provenance.
- Contract-derived tests and pre-implementation gate evidence.
- Minimal production correction for any proven baseline-authoritative defect.
- H1/H7/H39 trace localization artifact and full H1..H39 metrics artifact.
- Dual reviews, review disposition, dual verification, final disposition, and
  worker handoff.

## Dependencies

- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `docs/work-packages/README.md`
- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `docs/specifications/science-contracts/index.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `docs/work-packages/20260605-hphys0291-snow-publication-lifecycle-partition-localization-closure-001/artifacts/worker-handoff.md`
- `docs/work-packages/20260605-hphys0291-snow-publication-lifecycle-partition-localization-closure-001/artifacts/h1-h7-h39-trace-evidence.md`
- `docs/work-packages/20260605-hphys0291-snow-publication-lifecycle-partition-localization-closure-001/artifacts/full-39-suite-metrics.md`
- `/workdir/wepp-forest_260430_baseline/src/snowd.for` at commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`
- `/workdir/wepp-forest_260430_baseline/src/winter.for` at commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`
- `/workdir/wepp-forest_260430_baseline/src/contin.for` at commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`
- `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for` at commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`
- `/workdir/wepp-forest_260430_baseline/src/grna.for` at commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`

## Intended Write Set

- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
- `tests/integration/hphys0292_spring_snowmelt_infiltration_capacity_contract.rs`
- `Cargo.toml`
- `docs/work-packages/README.md`
- `docs/work-packages/20260605-hphys0292-spring-snowmelt-infiltration-capacity-lineage-closure-001/**`

## Phase Plan

1. Scaffold package and placeholders.
2. Read HPHYS0291 handoff, canonical contracts, and baseline provenance.
3. Amend contracts for producer-side snowmelt/infiltration ownership.
4. Add contract-derived tests before production edits.
5. Record pre-implementation contract-gate evidence.
6. Patch production code only for proven baseline-authoritative defects.
7. Run focused tests, Rust gates, targeted traces, and full H1..H39 metrics.
8. Complete dual reviews, disposition, verification, and handoff.

## Contract-First Sequence

1. Contracts.
2. Contract-derived tests.
3. Pre-implementation contract gate.
4. Production code edits.

## Exit Criteria

- Canonical contracts define the snowmelt/rain-on-snow/WB12 partition
  ownership needed for H1/H7/H39 continuation.
- Contract tests prove active snowmelt enters WB12 infiltration before `Q`.
- H1/H7/H39 traces classify spring collapse residual ownership without WB13
  fallback.
- Full H1..H39 runtime and semantic metrics are recorded.
- Dual reviews and dual verification have no undispositioned blocking findings.
- Evidence artifacts label claims truthfully with `Static:` vs `Ran:`.

## Security-Impact Gate

No external service, credential, network, shell-interpolated subprocess, or
unsafe Rust change is planned.
