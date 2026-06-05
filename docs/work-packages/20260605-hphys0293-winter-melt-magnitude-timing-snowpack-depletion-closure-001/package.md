# HPHYS0293 Winter Melt Magnitude Timing Snowpack Depletion Closure

Status: executed-hold

This ExecPlan follows `/workdir/openWEPP/docs/codex_exec_plans.md`.

## Purpose / Big Picture

After this package, H1/H7/H39 spring `Snow-Water` and `RM` residual ownership
should be localized to a defended winter/snow producer mechanism or excluded
from the snow producer so the next package can safely move to post-ingress
WB18/WB19 storage routing. HPHYS0292 proved WB14 capacity now consumes routed
snowmelt before `Q`; HPHYS0293 must determine why candidate snowpack depletes
earlier than baseline and correct any baseline-authoritative melt magnitude,
timing, or state-depletion defect.

## Progress

- [x] (2026-06-05) Scaffolded HPHYS0293 package structure and queued artifacts.
- [x] (2026-06-05) Read HPHYS0292 handoff, canonical snow/runoff/water-balance contracts, and baseline winter/snow seams.
- [x] (2026-06-05) Amended contracts for melt magnitude/timing and snowpack depletion ownership.
- [x] (2026-06-05) Added contract-derived tests before production edits.
- [x] (2026-06-05) Recorded pre-implementation contract-gate evidence.
- [x] (2026-06-05) Production patch not applied: no baseline-authoritative defect was proven after corrected negative-melt authority review.
- [x] (2026-06-05) Ran focused tests, H1/H7/H39 traces, full H1..H39 metrics, and gates.
- [x] (2026-06-05) Completed truthful review/verification disposition and worker handoff; independent dual review remains not run under current delegation policy, so package remains `HOLD`.

## Objective

Diagnose, correct if proven, and validate the winter/snow producer lineage that
controls spring `Snow-Water`, `RM`, and downstream storage residuals for
H1/H7/H39 by tracing:

- hourly melt raw terms and post-redistribution routed melt,
- snowpack depth/density/SWE before and after melt,
- snow/rain retention and release,
- density-gated melt release versus retained pack warming/settling,
- final WB13 `Snow-Water` and `RM` publication.

## Rationale

HPHYS0292 closed the WB14 capacity blocker. Target rows now show active routed
melt infiltrates before `Q`, and full H1..H39 `Q` parity is `39/39`. Remaining
spring rows still show candidate snowpack below baseline by roughly
15-27 mm on key melt days, with `RM` timing/magnitude residuals. That pattern
must be localized before downstream retention/percolation/lateral residuals can
be interpreted.

## Included Scope

- Canonical contract amendments in `SC-SNOWFREEZE-001`, `SC-RUNOFFPART-001`,
  and `SC-WATBAL-001` for melt magnitude/timing and snowpack depletion
  ownership after WB14 capacity closure.
- Contract-derived tests proving:
  - hourly melt traces expose producer terms needed to classify depletion,
  - routed melt cannot exceed producer hourly melt plus released rain,
  - final `Snow-Water` consumption remains runtime-SWE based and fail-closed,
  - HPHYS0292 WB14 capacity behavior remains intact.
- H1/H7/H39 targeted traces for the HPHYS0292 spring rows plus earlier lead-in
  days needed to classify depletion timing.
- Full H1..H39 runtime and semantic metrics.

## Excluded Scope

- Reintroducing WB13 inference, state-derived `RM`, or snow flux fallback.
- Empirical snowmelt multipliers or proxy formulas.
- WB18/WB19 storage routing changes unless snow producer evidence proves they
  are the direct package-scoped defect.
- Replicating known legacy bugs that openWEPP explicitly rejects by documented
  guard-consistency decision.

## Deliverables

- Canonical contract amendments with baseline provenance.
- Contract-derived tests and pre-implementation gate evidence.
- Minimal production correction for any proven baseline-authoritative defect.
- H1/H7/H39 melt/depletion trace artifact and full H1..H39 metrics artifact.
- Review, disposition, verification, final disposition, and worker handoff
  artifacts with truthfulness labels.

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
- `docs/work-packages/20260605-hphys0292-spring-snowmelt-infiltration-capacity-lineage-closure-001/artifacts/worker-handoff.md`
- `docs/work-packages/20260605-hphys0292-spring-snowmelt-infiltration-capacity-lineage-closure-001/artifacts/full-39-suite-metrics.md`
- `docs/work-packages/20260605-hphys0292-spring-snowmelt-infiltration-capacity-lineage-closure-001/artifacts/spring-snowmelt-infiltration-localization.md`
- `/workdir/wepp-forest_260430_baseline/src/snowd.for` at commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`
- `/workdir/wepp-forest_260430_baseline/src/winter.for` at commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`
- `/workdir/wepp-forest_260430_baseline/src/disag.for` at commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`
- `/workdir/wepp-forest_260430_baseline/src/grna.for` at commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`
- `/workdir/wepp-forest_260430_baseline/src/contin.for` at commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`

## Intended Write Set

- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
- `crates/openwepp-runner/src/hillslope/mod.rs`
- `tests/integration/hphys0293_winter_melt_timing_contract.rs`
- `Cargo.toml`
- `docs/work-packages/README.md`
- `docs/work-packages/20260605-hphys0293-winter-melt-magnitude-timing-snowpack-depletion-closure-001/**`

## Phase Plan

1. Scaffold package and placeholders.
2. Read HPHYS0292 handoff, canonical contracts, and baseline snow seams.
3. Amend contracts for melt magnitude/timing and snowpack depletion ownership.
4. Add contract-derived tests before production edits.
5. Record pre-implementation contract-gate evidence.
6. Patch production code only for proven baseline-authoritative defects.
7. Run focused tests, Rust gates, targeted traces, and full H1..H39 metrics.
8. Complete review/verification disposition and handoff.

## Contract-First Sequence

1. Contracts.
2. Contract-derived tests.
3. Pre-implementation contract gate.
4. Production code edits.

## Exit Criteria

- Canonical contracts define snowmelt magnitude/timing and snowpack depletion
  ownership needed for H1/H7/H39 continuation.
- Contract tests prove hourly melt/depletion traces expose the producer terms
  needed to classify residual ownership.
- Any production patch is baseline-authoritative and covered by a regression
  that fails without the patch.
- H1/H7/H39 traces classify whether remaining residuals are snow producer or
  post-ingress storage routing.
- Full H1..H39 runtime and semantic metrics are recorded.
- Review findings are dispositioned or package remains explicitly in `HOLD`.
- Evidence artifacts label claims truthfully with `Static:` vs `Ran:`.

## Security-Impact Gate

No external service, credential, network, shell-interpolated subprocess, or
unsafe Rust change is planned.
