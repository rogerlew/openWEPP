# HPHYS0264 WB11/WB17 PMET Seam Correction Closure

Status: completed/HOLD

## Objective

Correct the WB11/WB17 PMET-mode seam exposed by HPHYS0263 so migrated
EVAPPM `pmet.es_m` and `pmet.ep_m` are consumed consistently by the WB17 ET
phase before SWU root uptake, without reintroducing Priestley-Taylor partition
or silently bypassing baseline `swu` semantics.

## Rationale

HPHYS0263 migrated the pinned `evappm.for` PMET demand equations and proved
H1/H7/H39 select PMET mode, but post-commit review found the migrated branch
publishes `pmet.ep_m` through `wb11_et_demand` while the WB17 ET phase still
treats that surface as generic potential demand and applies the existing
Priestley-Taylor partition. Pinned legacy `evappm.for` computes `es` and `ep`
directly, then `swu.for` consumes `ep` for root-zone uptake. The openWEPP seam
must therefore be made branch-aware before any follow-on `evappm.for:391-454`
redistribution work builds on top of it.

## Included Scope

- Canonical `SC-EVAP-001` and `SC-WATBAL-001` amendments for the PMET-mode
  WB11/WB17 boundary.
- Contract-derived tests proving PMET mode consumes `pmet.es_m` and
  `pmet.ep_m` without PT partition.
- Production WB17 ET phase correction for PMET branch inputs.
- H1/H7/H39 targeted seam diagnostics and full H1..H39 hillslope metrics.
- Truthfulness-labeled review, verification, disposition, and handoff
  artifacts.

## Excluded Scope

- Full migration of `evappm.for:391-454` post-ET soil evaporation
  redistribution unless required to correct the seam.
- Storage-drain root-cause correction outside the PMET seam.
- Watershed routing closure beyond reported H1..H39 hillslope metrics.
- Heuristic/proxy ET formulas not traced to pinned baseline authority.

## Deliverables

- Updated canonical `SC-*` authority before production edits.
- Red pre-implementation contract gate.
- PMET branch-specific WB17 ET behavior that publishes `Etp = pmet.ep_m`,
  consumes `Es` from `pmet.es_m`, and leaves SWU root uptake authoritative for
  final `Ep`.
- Targeted H1/H7/H39 seam diagnostics and full H1..H39 metrics.
- Disposition that stays `HOLD` if remaining EVAPPM/storage physics remain
  unported or unresolved.

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
- `docs/specifications/science-contracts/contracts/SC-INFILE-PMETPARA-001.md`
- `docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/wepp-forest_260430_baseline/src/evappm.for`
- `/workdir/wepp-forest_260430_baseline/src/swu.for`
- `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for`
- Pinned baseline commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`
- `docs/work-packages/20260603-hphys0263-wb11-wb17-evappm-demand-migration-closure-001/artifacts/review_claude_code_disposition.md`
- `docs/work-packages/20260603-hphys0263-wb11-wb17-evappm-demand-migration-closure-001/artifacts/worker-handoff.md`

## Intended Write Set

- `docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
- `crates/openwepp-hillslope-orchestrator/src/tests.rs`
- `crates/openwepp-runner/src/hillslope/mod.rs`
- `crates/openwepp-summary-accumulator/src/lib.rs`
- `docs/work-packages/README.md`
- `docs/work-packages/20260603-hphys0264-wb11-wb17-pmet-seam-correction-closure-001/**`

## Phase Plan

1. Amend canonical contracts with the PMET seam boundary.
2. Add contract-derived tests and record the red gate.
3. Correct production WB17 PMET ET behavior.
4. Run focused tests, targeted diagnostics, and full H1..H39 metrics.
5. Complete review, verification, disposition, and continuation handoff.

## Contract-First Sequence

1. Contracts.
2. Contract-derived tests.
3. Pre-implementation contract gate.
4. Production code edits.

## Exit Criteria

- Truthfulness-labeled artifacts distinguish `Static:` from `Ran:`.
- PMET mode has branch-aware WB17 ET behavior; `Etp` equals `pmet.ep_m` before
  SWU and `Es` lineage comes from `pmet.es_m` rather than PT partition.
- SWU remains active for final root uptake and plant stress.
- H1/H7/H39 targeted diagnostics classify the seam as corrected or blocked.
- Full H1..H39 semantic metrics are recorded.
- Known remaining migration gaps are not closed by assertion.

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

- `SC-EVAP-001` and `SC-WATBAL-001` now require PMET branch WB17 execution to
  consume `pmet.es_m` and `pmet.ep_m` directly, prohibit Priestley-Taylor
  repartition of PMET `ep`, and preserve SWU as final `Ep` authority.
- WB17 ET execution now detects `wb11_et_seed_branch_evappm`, requires
  migrated PMET component surfaces, bypasses non-PMET stage/PT partition
  behavior, publishes `Etp = pmet.ep_m`, and preserves signed EVAPPM `Es`
  where pinned baseline `evappm.for:460-523` returns negative `es`.
- WB13 summary publication now allows signed `Es` only under the EVAPPM PMET
  branch marker; non-PMET negative `Es` remains a hard error.

Ran:

- Focused HPHYS0264 contract tests passed after the production correction.
- `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo test --workspace`, and `cargo deny check` completed successfully
  (`cargo deny` warnings only).
- Full H1..H39 hillslope semantic diagnostics ran at
  `/tmp/hphys0264_20260603T083941Z`; semantic pass remains `0/39`.

## Disposition

The PMET component seam is corrected for this package scope. Full HPHYS parity
remains in `HOLD` because H1..H39 diagnostics still show unresolved storage,
snow/runoff, percolation, lateral-flow, and longer-season `Ep` residual
families outside this seam slice.
