# HPHYS0291 Snow Publication Lifecycle and Partition Localization Closure

Status: executed-hold

This ExecPlan follows `/workdir/openWEPP/docs/codex_exec_plans.md`.

## Purpose / Big Picture

After this package, the post-winter rain / routed-melt producer lifecycle should
be contractually guarded from the runoff reconciliation producer through WB13
publication, and H1/H7/H39 plus full H1..H39 metrics should identify whether
remaining residuals are upstream snowpack timing/state or downstream liquid
storage/runoff partitioning.

## Progress

- [x] (2026-06-05) Scaffolded HPHYS0291 package structure and queued artifacts.
- [x] (2026-06-05) Read HPHYS0290 handoff, canonical contracts, and daily lifecycle seams.
- [x] (2026-06-05) Amended contracts for same-day snow publication lifecycle authority.
- [x] (2026-06-05) Added contract-derived lifecycle and localization tests before production edits.
- [x] (2026-06-05) Recorded pre-implementation contract-gate evidence.
- [x] (2026-06-05) Patched production code for proven lifecycle defects.
- [x] (2026-06-05) Ran focused tests, Rust gates, H1/H7/H39 traces, and full H1..H39 metrics.
- [x] (2026-06-05) Completed dual review, disposition, verification, final handoff.

## Objective

Diagnose, guard, and validate the daily producer/scheduler/WB13 lifecycle for
`snow.post_winter_rain_m` and `snow.routed_melt_m`, then localize remaining
snowpack/liquid partition residuals without changing WB13 publication math.

## Rationale

HPHYS0290 closed the WB13 post-winter rain inference seam and held on semantic
parity because residuals remained upstream of WB13 publication. Its handoff
identified two immediate needs: preserve the fail-closed same-day producer flux
contract and continue diagnosis upstream in snowpack timing/state and
runoff/storage partition lineage. This package closes the lifecycle regression
gap first so later physics packages cannot silently reintroduce
canonicalize-and-proceed defaults.

## Included Scope

- Contract amendments in canonical `SC-*` files for same-day snow publication
  lifecycle requirements.
- Contract-derived tests proving:
  - runoff reconciliation publishes required snow publication fluxes,
  - WB13 cannot be satisfied by state-only post-winter rain,
  - trace/localization surfaces expose the producer values used by WB13.
- H1/H7/H39 targeted trace rerun and full H1..H39 semantic metrics.
- Dual review, review disposition, dual verification, final disposition, and
  worker handoff.

## Excluded Scope

- Changing WB13 `RM` publication formula.
- Heuristic rain/snow/runoff multipliers.
- Broad snow/frost migration or empirical tuning.
- WB17 `Ep`, WB18 storage, or WB19 lateral corrections unless a lifecycle test
  proves a direct package-scoped defect.

## Deliverables

- Canonical lifecycle contract amendments with provenance.
- Contract-derived tests and pre-implementation gate evidence.
- Any minimal production correction required by failing lifecycle tests.
- H1/H7/H39 trace artifact and full H1..H39 metrics artifact.
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
- `docs/work-packages/20260605-hphys0290-post-winter-rain-publication-lineage-closure-001/artifacts/worker-handoff.md`
- `docs/work-packages/20260605-hphys0290-post-winter-rain-publication-lineage-closure-001/artifacts/disposition.md`
- `/workdir/wepp-forest_260430_baseline/src/contin.for` at commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`
- `/workdir/wepp-forest_260430_baseline/src/winter.for` at commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`
- `/workdir/wepp-forest_260430_baseline/src/watbalprint.for` at commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`

## Intended Write Set

- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
- `crates/openwepp-runner/src/hillslope/mod.rs`
- `tests/integration/hphys0291_snow_publication_lifecycle_contract.rs`
- `Cargo.toml`
- `docs/work-packages/README.md`
- `docs/work-packages/20260605-hphys0291-snow-publication-lifecycle-partition-localization-closure-001/**`

## Phase Plan

1. Scaffold package and placeholders.
2. Read HPHYS0290 handoff, canonical contracts, and lifecycle code.
3. Amend contracts for same-day producer flux lifecycle requirements.
4. Add contract-derived failing tests before production edits.
5. Record pre-implementation contract-gate evidence.
6. Patch production code only for proven lifecycle defects.
7. Run focused tests, Rust gates, targeted traces, and full H1..H39 metrics.
8. Complete dual reviews, disposition, verification, and handoff.

## Contract-First Sequence

1. Contracts.
2. Contract-derived tests.
3. Pre-implementation contract gate.
4. Production code edits.

## Exit Criteria

- Same-day snow publication flux lifecycle is canonical and test-covered.
- Missing producer flux cannot be masked by state defaults.
- H1/H7/H39 trace artifact records producer fluxes and residual localization.
- Full H1..H39 runtime and semantic metrics are recorded.
- Dual reviews and dual verification have no undispositioned blocking findings.
- Evidence artifacts label claims truthfully with `Static:` vs `Ran:`.

## Security-Impact Gate

No external service, credential, network, shell-interpolated subprocess, or
unsafe Rust change is planned.
