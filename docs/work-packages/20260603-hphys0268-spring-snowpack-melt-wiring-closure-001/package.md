# HPHYS0268 Spring Snowpack Melt Wiring Closure

Status: completed/HOLD

## Objective

Execute the snowpack-first continuation after HPHYS0267 by re-anchoring
H1/H7/H39 diagnostics on the first material `Ep` divergence (`>1 mm`, spring
snowmelt window), proving whether openWEPP's snowpack/SWE/RM lineage is wired
to baseline-authoritative `winter -> snowd -> melt` semantics, and correcting
only proven in-scope snowpack wiring defects.

## Rationale

HPHYS0259, HPHYS0265, HPHYS0266, and HPHYS0267 ruled out the local liquid-water
seams that were instrumented: WB17/SWU identities, WB11/WB18 aggregate
recompute, WB19 realized lateral publication, WB19 threshold eligibility, and
post-lateral/pre-SWU stress thresholds. The review of HPHYS0267 identified a
strategic targeting error: the first `0.05 mm` January `Ep` crossing is
non-material, while the first `>1 mm` residual occurs in April when snowpack
melt/storage coupling is active. Snowpack must be made authoritative before
returning to `Ep`.

## Included Scope

- Canonical `SC-SNOWFREEZE-001` and `SC-WATBAL-001` amendments for spring
  material-divergence snowpack/SWE/RM lineage.
- Contract-derived trace/tests for snowpack runtime state and closure:
  `snow.runtime_swe`, `snow.runtime_depth_m`,
  `snow.runtime_density_kg_m3`, `snow.runtime_settle_day_count`, hourly rain,
  hourly snowfall, hourly melt, signed `S`, WB13 `RM`, and WB13 `Snow-Water`.
- H1/H7/H39 targeted classification using first `|Ep diff| > 1 mm` rows.
- Full H1..H39 semantic metrics after diagnostics and after any production
  patch.
- Production correction only if evidence proves a pinned-baseline snowpack
  wiring defect.
- Explicit parity posture that non-ag frost remains disabled for HPHYS
  baseline parity unless a separate correctness decision changes the target.

## Excluded Scope

- Heuristic or proxy snowmelt tuning.
- Frost process activation for non-ag HPHYS parity.
- Forest `ksatadj` implementation unless snowpack evidence proves it is the
  immediate snow/RM lineage blocker.
- Returning to WB17/SWU `Ep` production edits before snowpack/RM lineage is
  defensible.

## Deliverables

- Updated canonical `SC-*` authority before production edits.
- Trace schema extensions and focused tests for spring snowpack lineage.
- H1/H7/H39 first-material-divergence snowpack diagnosis.
- Full H1..H39 semantic summary for continuation metrics.
- Final `GO`/`HOLD` disposition and HPHYS0269 handoff.

## Dependencies

- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `docs/specifications/science-contracts/index.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
- `docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/wepp-forest_260430_baseline/src/winter.for`
- `/workdir/wepp-forest_260430_baseline/src/snowd.for`
- `/workdir/wepp-forest_260430_baseline/src/melt.for`
- `/workdir/wepp-forest_260430_baseline/src/stmtim.for`
- `/workdir/wepp-forest_260430_baseline/src/hr_tmp.for`
- `/workdir/wepp-forest_260430_baseline/src/radcur.for`
- Pinned baseline commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`
- `docs/audits/20260525_water_erosion_kernel_audit.md`
- `docs/audits/20260603_wepp_forest_nonag_frost_disable_audit.md`
- `docs/work-packages/20260603-hphys0267-post-lateral-pre-swu-threshold-lineage-closure-001/artifacts/review_claude_code.md`
- `docs/work-packages/20260603-hphys0267-post-lateral-pre-swu-threshold-lineage-closure-001/artifacts/worker-handoff.md`

## Intended Write Set

- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `crates/openwepp-runner/src/hillslope/mod.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
- `docs/work-packages/README.md`
- `docs/work-packages/20260603-hphys0268-spring-snowpack-melt-wiring-closure-001/**`
- Production kernel files only if evidence proves an in-scope
  baseline-authoritative snowpack defect.

## Phase Plan

1. Amend canonical contracts for material spring snowpack lineage.
2. Add contract-derived snowpack trace/test coverage.
3. Record the pre-implementation contract gate before production-code decision.
4. Run H1/H7/H39 targeted traces at first `>1 mm` `Ep` divergence.
5. Patch production code only if evidence identifies an in-scope
   baseline-authoritative snowpack wiring defect.
6. Run focused gates and full H1..H39 semantic metrics.
7. Complete review, verification, disposition, and continuation handoff.

## Contract-First Sequence

1. Contracts.
2. Contract-derived diagnostic/test evidence.
3. Pre-implementation contract gate.
4. Production code edits, if justified.

## Exit Criteria

- Truthfulness-labeled artifacts distinguish `Static:` from `Ran:`.
- H1/H7/H39 material-divergence rows include candidate/baseline `Ep`, `RM`,
  `Snow-Water`, `Total-Soil`, `SoilWaterTotal`, `Q`, `Dp`, `latqcc`, and trace
  snowpack lineage.
- Any production patch traces directly to pinned baseline authority and has
  focused tests.
- Full H1..H39 semantic metrics are recorded.
- Known remaining gaps stay in `HOLD`; no heuristic closure is claimed.

## Security-Impact Gate

No external systems or network actions are required. This package is local
repository engineering work limited to flat-file reads/edits and local command
execution in the worktree.

## Autonomy

This package is intended for end-to-end autonomous execution. Agents must work
through the phase plan, update artifacts through disposition, and only ask for
user direction when hard-blocked by missing local authority or unavailable
validation substrate.
