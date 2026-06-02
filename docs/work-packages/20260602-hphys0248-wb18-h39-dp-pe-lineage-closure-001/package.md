# HPHYS0248 WB18 H39 Dp/Pe Lineage Closure

Status: hold

## Objective

Diagnose, correct, and validate the WB18 H39 early-season
percolation/deep-seepage lineage (`Dp`/`Pe`) using contract-first authoring,
baseline-authoritative WEPP physics, dual independent review, and a full
`H1..H39` hillslope rerun with semantic metrics for continuation planning.

## Rationale

HPHYS0247 closed runtime winter activation and WB19 saturated-zone lateral
lineage defects, but held disposition because H39 still emits early-season
`Dp` around `22..24 mm/day` while the pinned legacy baseline emits
`0.24 mm/day`. Static lineage inspection shows legacy hourly `watbal_hourly`
calls `purk` once per hour and `purk` attenuates `sep` by `ui_LFtstp` before
mutating `st` and accumulating `deepSeep`. The openWEPP WB18 hourly lane must
therefore be adjudicated against this exact `purk`/`perc` lineage rather than
continuing daily-capacity scaling behavior.

## Included Scope

- Amend canonical `SC-*` authority for WB18 hourly `Dp`/`Pe` lineage.
- Add contract-derived tests for hourly `purk`/`perc` capacity attenuation and
  `D == Pe == Σ hourly bottom sep` publication.
- Diagnose H39 day `1..30` early-season `Dp`/`Pe` residuals before/after the
  correction.
- Modify only the WB18 percolation production path and directly coupled
  surfaces needed to satisfy the canonical contract.
- Run targeted H39 semantic parity and the full `H1..H39` hillslope semantic
  suite, then publish metrics.
- Dispatch two independent review agents, resolve actionable findings, and
  record disposition.

## Excluded Scope

- WB17 ET partition (`Ep`/`Es`) migration.
- Snowmelt/runoff timing closure (`RM`, `Q`, `Snow-Water`).
- WB19 residual lateral magnitude beyond effects caused by corrected WB18
  storage/percolation inputs.
- Heuristic tuning, empirical compensation, or surrogate percolation formulas.

## Deliverables

1. Canonical contract amendments for WB18 hourly `purk`/`perc` `Dp`/`Pe`
   lineage.
2. Contract-derived Rust tests proving hourly capacity attenuation,
   bottom-layer publication, and guard behavior.
3. Production WB18 correction, if contract/test evidence proves a defect.
4. H39 before/after diagnosis and semantic parity report.
5. Full `H1..H39` hillslope semantic metrics with monitored residual families.
6. Dual review artifacts and recorded fixes.
7. Updated package disposition, worker handoff, gate results, owned-file
   manifest, and kernel-profile compliance checklist.

## Dependencies

- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `docs/specifications/science-contracts/index.md`
- `docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `docs/work-packages/20260602-hphys0247-h39-single-ofe-hourly-waterbalance-closure-001/artifacts/worker-handoff.md`
- `docs/work-packages/20260602-hphys0247-h39-single-ofe-hourly-waterbalance-closure-001/artifacts/hphys0247_disposition.md`
- `/workdir/wepp-forest_260430_baseline/src/purk.for`
- `/workdir/wepp-forest_260430_baseline/src/perc.for`
- `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for`
- `/tmp/unpalatable_parity_20260529T192707Z/runs/`
- `/tmp/unpalatable_parity_20260529T192707Z/reports/hillslope/baseline_partitions/`

## Intended Write Set

- `docs/work-packages/README.md`
- `docs/work-packages/20260602-hphys0248-wb18-h39-dp-pe-lineage-closure-001/**`
- `docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/index.md`
- `tests/integration/wb18_percolation_physics_kernel_contract.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`

## Phase Plan

1. Scaffold the package, artifacts, and kickoff prompt.
2. Read required authority and map baseline `watbal_hourly`/`purk`/`perc`
   `Dp`/`Pe` lineage.
3. Amend canonical contracts before production edits.
4. Add contract-derived tests before production edits.
5. Record pre-implementation contract gate evidence with truthfulness labels.
6. Implement the minimal WB18 production correction.
7. Run targeted Rust tests and H39 semantic validation.
8. Run the full `H1..H39` hillslope semantic suite.
9. Dispatch dual agent review, resolve actionable findings, and update review
   artifacts.
10. Run final gates feasible for the touched code and record disposition.

## Contract-First Sequence

1. Contracts.
2. Contract-derived tests.
3. Pre-implementation contract gate.
4. Production code edits.

Kernel production code edits before steps 1-3 are invalid package execution.

## Exit Criteria

- Canonical `SC-*` contracts cite pinned baseline lineages for hourly
  `purk`/`perc` `Dp`/`Pe` behavior.
- Contract-derived tests fail before the WB18 production fix and pass after it.
- H39 run and semantic comparator produce concrete `Dp`/`Pe` before/after
  metrics.
- Full `H1..H39` semantic suite completes with per-hillslope reports and
  aggregated continuation metrics.
- Dual review findings are dispositioned and actionable fixes are resolved.
- `cargo fmt --check`, targeted WB18 tests, workspace tests/clippy/deny when
  feasible, and `git diff --check` are recorded truthfully.
- Disposition is `GO` only if no known contract/profile violations remain;
  otherwise disposition is `HOLD` with focused continuation scope.

## Security-Impact Gate

No security-sensitive behavior is expected. This package changes deterministic
local hydrology math and documentation only. Any new file I/O must stay inside
explicit run/artifact paths and must not introduce shell interpolation.

## Evidence Truthfulness

All artifacts must label evidence with `Static:` when based on source or
document inspection and `Ran:` when based on an executed command. Do not claim a
workflow ran when only a build/check/static read was performed.

## Autonomy

This package is intended for no-intervention execution. Agents must execute the
phase plan end-to-end and update artifacts through disposition without asking
the user for next steps unless hard-blocked by missing required authority,
missing baseline data, or repeated tool failures.
