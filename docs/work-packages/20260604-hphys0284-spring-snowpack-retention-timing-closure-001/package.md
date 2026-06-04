# HPHYS0284 Spring Snowpack Retention Timing Closure

Status: complete

## Objective

Diagnose, correct, and validate the remaining H1/H7/H39 spring snowpack timing/retention residual after HPHYS0283, with closure evidence focused on 2014 Julian 120-147 `Snow-Water`, `RM`, routed melt, runoff/storage magnitude, and full H1..H39 semantic metrics.

## Rationale

HPHYS0283 closed the active-snowmelt runoff/infiltration partition bypass, moving the worst spring `Total-Soil` collapse rows from roughly `30..45 mm` to roughly `300..344 mm`. Semantic parity remains open: full H1..H39 still reports `0/39` semantic pass, `Total-Soil` mean abs diff `83.841688`, and unchanged `Snow-Water` mean abs diff `4.909469`. H1 and H39 still retain material candidate snowpack around Julian 145 where the baseline is snow-free. The next lowest-regret package is therefore a baseline-authoritative snowpack timing/retention package, not an `Ep` or WB13 publication compensation package.

## Included Scope

- Amend canonical snow/water-balance contracts for any newly proven spring snowpack retention/timing invariant.
- Add contract-derived tests before production edits.
- Compare openWEPP snowpack execution against pinned baseline `winter.for`, `snowd.for`, and `melt.for` around H1/H7/H39 2014 Julian 120-147.
- Preserve corrected `wepp-forest` negative-melt authority; do not reproduce the pinned baseline negative-melt bug.
- Correct one baseline-authoritative runtime snowpack defect if diagnosis proves one.
- Run targeted H1/H7/H39 traces and full H1..H39 semantic suite after the implementation.
- Complete dual review, dual verification, disposition, and worker handoff.

## Excluded Scope

- WB17 `Ep` tuning or publication compensation.
- WB13 `Snow-Water` shadowing that is not backed by runtime SWE state.
- Heuristic snowmelt clipping, empirical residual fitting, or proxy process math.
- Frost migration for non-agricultural HPHYS parity; snow remains active while frost stays disabled for the current non-ag parity lane.
- Full direct-rain WB18 `fin/xfin` ingress beyond the active-snowmelt seam closed by HPHYS0283.

## Deliverables

- Canonical contract amendments in `SC-SNOWFREEZE-001` and `SC-WATBAL-001` if implementation authority is changed.
- Contract-derived regression test for the snowpack defect localized by this package.
- Production code change only after contract and test gates are recorded.
- Targeted H1/H7/H39 spring trace evidence.
- Full H1..H39 runtime and semantic metrics.
- Dual review, review disposition, dual verification, final disposition, and worker handoff artifacts.

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
- `docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
- `docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `docs/work-packages/20260604-hphys0283-spring-snowmelt-runoff-infiltration-partition-001/artifacts/worker-handoff.md`
- `docs/work-packages/20260604-hphys0283-spring-snowmelt-runoff-infiltration-partition-001/artifacts/full-39-suite-metrics.md`
- `/workdir/wepp-forest_260430_baseline/src/winter.for` at commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`
- `/workdir/wepp-forest_260430_baseline/src/snowd.for` at commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`
- `/workdir/wepp-forest_260430_baseline/src/melt.for` at commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`
- `/workdir/wepp-forest/src/winter.for` corrected negative-melt authority at commit `03fee4558456535138592630b5dedc4d81ce8d06`

## Intended Write Set

- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`
- `tests/integration/hphys0284_negative_melt_snowpack_state_contract.rs`
- `Cargo.toml`
- `docs/work-packages/README.md`
- `docs/work-packages/20260604-hphys0284-spring-snowpack-retention-timing-closure-001/**`

## Phase Plan

1. Scaffold package and placeholders.
2. Read required authority and local snow runtime code.
3. Add canonical contract amendments for the localized snowpack invariant.
4. Add a contract-derived failing test before production code edits.
5. Record pre-implementation contract gate evidence.
6. Implement the minimal baseline-authoritative snowpack correction.
7. Run targeted tests, focused snow runtime tests, Rust gates, targeted H1/H7/H39 traces, and full H1..H39 semantic suite.
8. Complete dual review artifacts, disposition accepted findings, and run dual verification.
9. Close package with disposition and worker handoff.

## Contract-First Sequence

1. Contracts.
2. Contract-derived tests.
3. Pre-implementation contract gate.
4. Production code edits.

Production kernel edits are prohibited before steps 1-3 are complete and recorded.

## Exit Criteria

- Canonical `SC-*` contracts explicitly authorize any corrected snowpack behavior.
- Contract-derived test fails before and passes after implementation, or the package records why no production defect was proven and remains `HOLD`.
- Targeted H1/H7/H39 traces cover Julian 120-147 with runtime SWE/depth/density, hourly rain/snow/melt, signed `S`, `RM`, `Snow-Water`, `Q`, and `Total-Soil` context.
- Full H1..H39 runtime completes and semantic metrics are recorded.
- Dual reviews and dual verification are complete with no undispositioned accepted findings.
- Evidence artifacts label claims truthfully with `Static:` vs `Ran:`.

## Security-Impact Gate

No external service, credential, network, shell-interpolated subprocess, or unsafe Rust change is intended. If implementation changes subprocess orchestration, sidecar discovery, or `unsafe`, stop and record a security-impact finding before proceeding.
