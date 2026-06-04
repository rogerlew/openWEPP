# HPHYS0286 Layer Retention and WB18/WB17 Coupling Closure

Status: executed-hold

## Objective

Diagnose, correct, and validate the post-HPHYS0285 H1/H7/H39 soil-storage residual, focusing on post-ingress layer capacity/retention, WB18 percolation routing, WB17 soil-evaporation/root-uptake withdrawal, and aggregate `Total-Soil` / `SoilWaterTotal` publication after same-pass infiltration now reaches layer storage.

## Rationale

HPHYS0285 corrected local-liquid same-pass infiltration into WB18 layer storage and improved `Total-Soil`, `Dp`, `latqcc`, and `Ep` residuals, but semantic parity remained `0/39`. The remaining signal is mixed-sign: H7/H39 are still too dry during spring 2014 meltout, while H1 becomes too wet in late 2015. That split means the next lowest-regret work is not another broad ingress fix; it is post-ingress retention and withdrawal ordering: whether WB18 holds water against field-capacity/upper-limit constraints, whether percolation drains it at the baseline cadence, and whether WB17 consumes the correct post-WB18/WB19 layer state.

## Included Scope

- Amend canonical `SC-*` contracts for any newly proven WB18 retention, percolation-cap, or WB17 post-ingress withdrawal invariant.
- Add contract-derived tests before production code edits.
- Diagnose H1/H7/H39 layer traces around the known spring dry residual and late-season H1 wet residual.
- Correct one baseline-authoritative post-ingress layer retention or WB18/WB17 coupling defect if diagnosis proves one.
- Include a snow-column mass trace around the H1 spring depth/SWE seam before assigning spring residual exclusively to WB18/WB17.
- Run focused tests, adjacent hydrology tests, Rust gates, and H1..H39 runtime/semantic metrics.
- Complete dual review, review disposition, dual verification, final disposition, and worker handoff.

## Excluded Scope

- Heuristic ET, percolation, infiltration-capacity, or storage-retention multipliers.
- WB17 `Ep` tuning unless layer-state tracing proves a baseline-authoritative WB17 coupling defect.
- MOFE carry/runon storage-ingress promotion; this remains a separate package.
- Broad snowpack timing/melt rewrite beyond the snow-column mass trace needed for residual classification.
- Frost migration unless baseline and traces prove frost-owned retention/capacity behavior in the target windows.

## Deliverables

- Canonical contract amendments in `SC-PERC-001`, `SC-EVAP-001`, `SC-WATBAL-001`, and optionally `SC-SNOWFREEZE-001` only if the snow-column trace proves new snow authority is required.
- Contract-derived regression tests for the localized post-ingress WB18/WB17 invariant.
- Targeted H1/H7/H39 trace evidence covering per-layer `theta`, `st`, `ul`, `fc`, percolation flux, ET withdrawal, aggregate `watcon`, and snow-column mass where relevant.
- Production code changes only after contract and test gates are recorded.
- Full H1..H39 runtime/semantic metrics and pre/post-HPHYS0285 comparison.
- Dual review, review disposition, dual verification, final disposition, and worker handoff artifacts.

## Dependencies

- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `docs/work-packages/README.md`
- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `docs/specifications/science-contracts/index.md`
- `docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `docs/work-packages/20260604-hphys0285-spring-soil-storage-retention-closure-001/artifacts/worker-handoff.md`
- `docs/work-packages/20260604-hphys0285-spring-soil-storage-retention-closure-001/artifacts/full-39-suite-metrics.md`
- `docs/work-packages/20260604-hphys0285-spring-soil-storage-retention-closure-001/artifacts/review-disposition.md`
- `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for` at commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`
- `/workdir/wepp-forest_260430_baseline/src/perc.for` at commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`
- `/workdir/wepp-forest_260430_baseline/src/purk.for` at commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`
- `/workdir/wepp-forest_260430_baseline/src/evap.for` at commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`
- `/workdir/wepp-forest_260430_baseline/src/swu.for` at commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`
- `/workdir/wepp-forest_260430_baseline/src/watbal.for` at commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`

## Intended Write Set

- `docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
- `tests/integration/hphys0286_layer_retention_wb18_wb17_contract.rs`
- `Cargo.toml`
- `docs/work-packages/README.md`
- `docs/work-packages/20260604-hphys0286-layer-retention-wb18-wb17-coupling-closure-001/**`

## Phase Plan

1. Scaffold package and placeholders.
2. Read required contract and baseline authority for WB18/WB17 ordering.
3. Localize H1/H7/H39 residual ownership with targeted layer and snow-column evidence.
4. Add canonical contract amendments for the localized invariant.
5. Add contract-derived failing test before production code edits.
6. Record pre-implementation contract gate evidence.
7. Implement the minimal baseline-authoritative WB18/WB17 correction.
8. Run focused tests, adjacent hydrology tests, Rust gates, and H1..H39 metrics.
9. Complete dual review, disposition accepted findings, dual verification, final disposition, and handoff.

## Contract-First Sequence

1. Contracts.
2. Contract-derived tests.
3. Pre-implementation contract gate.
4. Production code edits.

Production kernel edits are prohibited before steps 1-3 are complete and recorded.

## Exit Criteria

- Canonical `SC-*` contracts explicitly authorize any corrected WB18/WB17 behavior.
- Contract-derived test fails before and passes after implementation, or the package records why no production defect was proven and remains `HOLD`.
- Full H1..H39 runtime completes and semantic metrics are recorded.
- Dual reviews and dual verification are complete with no undispositioned accepted findings.
- Evidence artifacts label claims truthfully with `Static:` vs `Ran:`.

## Security-Impact Gate

No external service, credential, network, shell-interpolated subprocess, or unsafe Rust change is planned. Implementation must not change subprocess orchestration or sidecar discovery.

## Execution Summary

Ran:
- Implemented post-ET lower-layer upper-limit redistribution in the WB17 ET phase.
- Added focused contract tests that failed before implementation and pass after implementation.
- Ran focused tests, adjacent hydrology tests, workspace Rust gates, `cargo deny check`, release build, and full H1..H39 runtime/semantic suite.

Static:
- Closure remains in `HOLD`: HPHYS0286 materially improved aggregate storage, Dp, latqcc, and Ep residuals, but full semantic parity remains `0/39`.
