# HPHYS0265 Longer-Season Ep Divergence Localization Closure

Status: completed/HOLD

## Objective

Complete the HPHYS0264 next-focus work by localizing the first large
longer-season `Ep` divergence after the WB11/WB17 PMET seam correction, then
either correct any baseline-authoritative WB17/SWU defect identified by the
evidence or publish a truthful `HOLD` handoff with narrowed residual ownership.

## Rationale

HPHYS0264 closed the day-1 PMET component seam: WB17 now consumes
`pmet.es_m`/`pmet.ep_m` directly and day-1 H1/H7/H39 `Ep` residuals are small.
The full 39-hillslope suite still fails, with large seasonal `Ep`, storage,
snow/runoff, `Dp`, and `latqcc` residual families. The next useful slice is not
another day-1 seed fix; it is a multi-day trace around the first large `Ep`
divergence, so residual ownership can be separated from aggregate storage and
runoff/snow coupling.

## Included Scope

- Canonical `SC-EVAP-001` and `SC-WATBAL-001` amendments requiring first-large
  longer-season `Ep` divergence evidence before assigning residual ownership.
- Contract-derived diagnostic harness for H1/H7/H39 first-divergence
  localization.
- H1/H7/H39 multi-day trace classification spanning `pmet_ep_m`, `Etp`, final
  `Ep`, `ΣUi`, `Ws`, plant/root state, snow/runoff, storage, `Dp`, and
  `latqcc`.
- Baseline/candidate WAT day-window comparison around the first `Ep` residual
  crossing.
- Production correction only if evidence identifies a baseline-authoritative
  defect within WB17/SWU/runtime projection scope.
- Full H1..H39 semantic metrics after any code changes, or after diagnostic
  execution if no code change is justified.

## Excluded Scope

- Heuristic/proxy ET tuning.
- Full snow, runoff, percolation, lateral-flow, or aggregate storage migration
  unless first-divergence evidence directly proves the touched defect.
- Closing full H1..H39 parity by assertion.
- Watershed routing changes.

## Deliverables

- Updated canonical `SC-*` authority before production edits.
- Red or pre-correction diagnostic evidence showing the old posture cannot
  localize first-large longer-season `Ep` ownership.
- Diagnostic script and artifacts for first-divergence classification.
- If warranted, contract-derived tests and production fixes for the identified
  WB17/SWU defect.
- Full H1..H39 metrics and a final `GO`/`HOLD` disposition.

## Dependencies

- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `docs/specifications/science-contracts/index.md`
- `docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-PLANT-001.md`
- `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/wepp-forest_260430_baseline/src/evappm.for`
- `/workdir/wepp-forest_260430_baseline/src/swu.for`
- `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for`
- Pinned baseline commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`
- `docs/work-packages/20260603-hphys0264-wb11-wb17-pmet-seam-correction-closure-001/artifacts/disposition.md`
- `docs/work-packages/20260603-hphys0264-wb11-wb17-pmet-seam-correction-closure-001/artifacts/worker-handoff.md`
- `docs/work-packages/20260603-hphys0264-wb11-wb17-pmet-seam-correction-closure-001/artifacts/review_claude_code_disposition.md`

## Intended Write Set

- `docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
- `crates/openwepp-hillslope-orchestrator/src/tests.rs`
- `crates/openwepp-runner/src/hillslope/mod.rs`
- `docs/work-packages/README.md`
- `docs/work-packages/20260603-hphys0265-longer-season-ep-divergence-localization-closure-001/**`

## Phase Plan

1. Amend canonical contracts with first-divergence localization authority.
2. Add the first-divergence diagnostic harness and record pre-correction
   evidence.
3. Run targeted H1/H7/H39 multi-day traces and classify the first large `Ep`
   divergence.
4. Patch production code only if evidence identifies an in-scope
   baseline-authoritative defect.
5. Run focused gates and full H1..H39 metrics.
6. Complete review, verification, disposition, and continuation handoff.

## Contract-First Sequence

1. Contracts.
2. Contract-derived diagnostic/test evidence.
3. Pre-implementation contract gate.
4. Production code edits, if justified.

## Exit Criteria

- Truthfulness-labeled artifacts distinguish `Static:` from `Ran:`.
- H1/H7/H39 first `|Ep diff| > 0.05 mm` days are identified and classified.
- Classification includes `pmet_ep_m`, ET `Etp`, final `Ep`, `ΣUi`, `Ws`,
  plant/root state, storage, snow/runoff, `Dp`, and `latqcc`.
- Any production fix traces directly to pinned baseline authority and has
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

- Read HPHYS0264 disposition/handoff context and executed the declared next
  focus as a contract-first diagnostic package.
- Amended canonical contracts with first-large longer-season `Ep` divergence
  ownership gates:
  - `SC-EVAP-001` version 20 adds `INV-EVAP-023` and `GAP-EVAP-011`.
  - `SC-WATBAL-001` version 92 adds `INV-WATBAL-051`.
- Added package-local diagnostic runner
  `artifacts/hphys0265_diagnostics.py`.
- No production Rust code was edited. The first-divergence evidence did not
  identify a baseline-authoritative WB17/SWU publication defect.

Ran:

- `/workdir/wepppy/.venv/bin/python -m py_compile docs/work-packages/20260603-hphys0265-longer-season-ep-divergence-localization-closure-001/artifacts/hphys0265_diagnostics.py`
- `/workdir/wepppy/.venv/bin/python docs/work-packages/20260603-hphys0265-longer-season-ep-divergence-localization-closure-001/artifacts/hphys0265_diagnostics.py --run-root /tmp/hphys0265_20260603T151958Z --trace-max-days 130`
- `/workdir/wepppy/.venv/bin/python docs/work-packages/20260603-hphys0265-longer-season-ep-divergence-localization-closure-001/artifacts/hphys0265_diagnostics.py --run-root /tmp/hphys0265_20260603T151958Z --trace-max-days 130 --skip-full-suite`

Observed:

- H1 first `|Ep diff| > 0.05 mm`: 2013 day 15; `Ep` diff `-0.052129 mm`.
- H7 first `|Ep diff| > 0.05 mm`: 2013 day 11; `Ep` diff `-0.057740 mm`.
- H39 first `|Ep diff| > 0.05 mm`: 2013 day 22; `Ep` diff `-0.050136 mm`.
- All three first-divergence days classify as
  `WB17_IDENTITY_CLOSED_SWU_STRESS_LIMITED_WITH_STORAGE_CONTEXT`.
- Full H1..H39 semantic pass remains `0/39`.

## Disposition

HOLD. The package localized the longer-season `Ep` residual to SWU stress under
already-material storage/snow/lateral context, while `pmet_ep_m = Etp`,
`Ep = ΣUi`, and `Ws = Ep/Etp` close at the first divergence in H1/H7/H39.
Production WB17/SWU edits are not justified without a narrower
baseline-authoritative defect. Continue with root-zone layer storage,
snow/runoff timing, and lateral-flow/storage distribution lineage.
