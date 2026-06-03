# HPHYS0266 Layer Storage, Lateral, and Snow Coupling Closure

Status: completed/HOLD

## Objective

Execute the HPHYS0265 continuation by diagnosing whether first longer-season
SWU stress under closed PMET/WB17 identities is owned by layer storage
distribution, WB19 lateral active-zone coupling, snow/runoff timing, or a
proven baseline-authoritative production defect, then publish corrected code
only when the evidence proves the defect.

## Rationale

HPHYS0265 showed that the first H1/H7/H39 seasonal `Ep` divergences do not
originate at the PMET seam or WB17/SWU publication identities: `pmet_ep_m =
Etp`, `Ep = ΣUi`, and `Ws = Ep/Etp` close at the first residual crossing. The
same days already show material WAT context in storage, snow/runoff, `Dp`, and
`latqcc`, so the next slice must classify layer storage and WB19 lateral-zone
lineage before assigning ownership or patching production code.

## Included Scope

- Canonical `SC-WATBAL-001` and `SC-SUBHYD-001` amendments requiring
  first-divergence layer/lateral/snow context before residual ownership claims.
- Contract-derived diagnostic harness for H1/H7/H39 first longer-season `Ep`
  residuals, including WB11/WB18 layer storage closure, WB17 stress-layer
  context, WB19 lateral potential/target/realized identities, and WAT
  snow/runoff context.
- H1/H7/H39 trace execution through at least day 130.
- Full H1..H39 semantic metrics after diagnostic execution and after any
  production patch.
- Production correction only if diagnostic evidence identifies an in-scope
  baseline-authoritative defect.

## Excluded Scope

- Heuristic/proxy ET, storage, snow, runoff, percolation, or lateral-flow
  tuning.
- Reassigning WB17/SWU ownership when its internal identities close.
- Full snow/frost, runoff, or vertical redistribution migration unless the
  package evidence directly proves that exact defect.
- Watershed routing changes.

## Deliverables

- Updated canonical `SC-*` authority before production edits.
- Contract-derived diagnostic script and pre-implementation evidence.
- H1/H7/H39 first-divergence layer/lateral/snow classification.
- Full H1..H39 semantic summary for continuation metrics.
- If warranted, baseline-proven production fix with focused tests.
- Final `GO`/`HOLD` disposition and continuation handoff.

## Dependencies

- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `docs/specifications/science-contracts/index.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
- `docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for`
- `/workdir/wepp-forest_260430_baseline/src/watbal.for`
- `/workdir/wepp-forest_260430_baseline/src/swu.for`
- Pinned baseline commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`
- `docs/work-packages/20260603-hphys0265-longer-season-ep-divergence-localization-closure-001/artifacts/disposition.md`
- `docs/work-packages/20260603-hphys0265-longer-season-ep-divergence-localization-closure-001/artifacts/worker-handoff.md`
- `docs/work-packages/20260603-hphys0265-longer-season-ep-divergence-localization-closure-001/artifacts/targeted-h1-h7-h39-first-ep-divergence-classification.md`

## Intended Write Set

- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`
- `crates/openwepp-runner/src/hillslope/mod.rs`
- `crates/openwepp-hillslope-orchestrator/src/tests.rs`
- `docs/work-packages/README.md`
- `docs/work-packages/20260603-hphys0266-layer-storage-lateral-snow-coupling-closure-001/**`

## Phase Plan

1. Amend canonical contracts with layer/lateral/snow first-divergence authority.
2. Add contract-derived diagnostics and record the pre-implementation gate.
3. Run H1/H7/H39 trace classification through day 130.
4. Patch production code only if evidence proves an in-scope baseline defect.
5. Run focused gates and full H1..H39 semantic metrics.
6. Complete review, verification, disposition, and continuation handoff.

## Contract-First Sequence

1. Contracts.
2. Contract-derived diagnostic/test evidence.
3. Pre-implementation contract gate.
4. Production code edits, if justified.

## Exit Criteria

- Truthfulness-labeled artifacts distinguish `Static:` from `Ran:`.
- H1/H7/H39 first `|Ep diff| > 0.05 mm` days include WB11/WB18 aggregate
  closure, WB17 stress-layer state, WB19 lateral active/withdrawal zone state,
  WB19 `q`/`Qdd`/`Qd` identities, and same-day WAT snow/runoff context.
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

## Execution Summary

Static:

- Added canonical HPHYS0266 first-divergence authority to `SC-WATBAL-001` and
  `SC-SUBHYD-001`.
- No production kernel/runtime files were modified.

Ran:

- `/workdir/wepppy/.venv/bin/python -m py_compile docs/work-packages/20260603-hphys0266-layer-storage-lateral-snow-coupling-closure-001/artifacts/hphys0266_diagnostics.py`
- `/workdir/wepppy/.venv/bin/python docs/work-packages/20260603-hphys0266-layer-storage-lateral-snow-coupling-closure-001/artifacts/hphys0266_diagnostics.py --run-root /tmp/hphys0266_20260603T155434Z --trace-max-days 130`

Outcome:

- H1/H7/H39 WB17 PMET/SWU identities closed at first divergence.
- WB11/WB18 aggregate recompute closure held with `Recomputed-WB11 = 0`.
- WB19 potential/target/realized `q`, `Qd=q+Qdd`, and withdrawal-sum identities
  closed.
- Full H1..H39 semantic parity remains `0/39`.
- Disposition is `HOLD`: residuals remain coupled layer-distribution,
  snow/runoff, lateral-magnitude, and storage-context problems; no
  baseline-authoritative production defect was proven in this package.
