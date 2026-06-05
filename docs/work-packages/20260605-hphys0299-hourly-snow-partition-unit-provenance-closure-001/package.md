# HPHYS0299 Hourly Snow Partition Unit/Provenance Closure

## Status

Executed-hold.

## Objective

Correct and validate the HPHYS0298 paired hourly snow/rain partition lineage by
comparing pinned-baseline `hrsnow` snow-depth against openWEPP
`snow_hourly_snowfall_depth_sum_m`, not derived snowfall water equivalent,
before authorizing production producer-physics migration or downstream WB17,
WB18, WB19, or WB13 focus changes.

## Rationale

HPHYS0298 concluded that all nine H1/H7/H39 target windows first diverge at
`hourly-forcing` and recommended migrating pinned-baseline hourly precipitation
partition behavior. During HPHYS0299 kickoff inspection, openWEPP production
code already matched the baseline `stmtim` cold-branch equation:
`hrsnow = rain / wntdur * 10.0`. The HPHYS0298 runner compared baseline
`hrsnow` snow-depth to openWEPP `snow_hourly_snowfall_water_equiv_sum_m`, a
derived water-equivalent summary, producing an apparent order-of-magnitude
defect. Correctness requires resolving that diagnostic/provenance seam before
changing production physics.

## Included Scope

- Amend canonical `SC-*` contracts so `hrsnow` snow-depth and derived snowfall
  water equivalent are distinct authority surfaces.
- Add contract-derived tests that fail when paired partition diagnostics map
  canonical `hrsnow` to `snow_hourly_snowfall_water_equiv_sum_m`.
- Create and run corrected paired partition diagnostics using
  `snow_hourly_snowfall_depth_sum_m`.
- Preserve HPHYS0298 historical artifacts while publishing HPHYS0299 corrected
  evidence and disposition.
- Run same-HEAD H1..H39 semantic-suite metrics for continuation routing.
- Complete dual independent review, disposition, and dual verification artifacts.

## Excluded Scope

- Production migration of hourly precipitation-phase physics unless corrected
  depth-vs-depth evidence proves an openWEPP production defect remains.
- WB17, WB18, WB19, or WB13 compensation edits.
- Reproducing pinned-baseline negative-melt bugs or weakening corrected
  openWEPP safety guards.
- Silent defaults, canonicalize-and-proceed paths, or downstream publication
  substitutions for missing producer evidence.

## Dependencies

- `/home/workdir/openWEPP/AGENTS.md`
- `/home/workdir/openWEPP/docs/codex_exec_plans.md`
- `/home/workdir/openWEPP/docs/work-packages/README.md`
- `/home/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/home/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/home/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/home/workdir/openWEPP/docs/work-packages/20260605-hphys0298-paired-snow-rm-lineage-partition-closure-001/package.md`
- `/home/workdir/openWEPP/docs/work-packages/20260605-hphys0298-paired-snow-rm-lineage-partition-closure-001/artifacts/worker-handoff.md`
- `/home/workdir/openWEPP/docs/work-packages/20260605-hphys0298-paired-snow-rm-lineage-partition-closure-001/artifacts/disposition.md`
- `/home/workdir/openWEPP/docs/work-packages/20260605-hphys0298-paired-snow-rm-lineage-partition-closure-001/artifacts/hphys0298_paired_lineage_partition.py`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs/06_simimpl28_hourly_forcing.rs`
- `/home/workdir/openWEPP/crates/openwepp-runner/src/hillslope/mod.rs`
- `/workdir/wepp-forest_260430_baseline/src/winter.for`
- `/workdir/wepp-forest_260430_baseline/src/stmtim.for`

## Intended Write Set

- `docs/work-packages/README.md`
- `docs/work-packages/20260605-hphys0299-hourly-snow-partition-unit-provenance-closure-001/**`
- `docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `tests/integration/hphys0299_hourly_snow_partition_unit_provenance_contract.rs`
- `Cargo.toml`

## Phase Plan

1. **Contracts**: amend canonical contracts to distinguish baseline `hrsnow`
   snow-depth from derived snowfall water equivalent and to suspend HPHYS0298
   production-migration authority until corrected evidence exists.
2. **Contract-derived tests**: add tests proving contract language and the
   corrected HPHYS0299 runner use `snow_hourly_snowfall_depth_sum_m` for
   canonical `hrsnow` parity.
3. **Pre-implementation contract gate**: run the contract-derived test before
   diagnostic implementation and record truthfully labeled evidence.
4. **Diagnostics**: implement the corrected partition runner and rerun paired
   H1/H7/H39 evidence plus full H1..H39 suite metrics.
5. **Review and verification**: perform dual review, disposition findings, run
   dual verification passes, and record final continuation routing.

## Progress

- [x] Identified HPHYS0298 diagnostic unit/provenance seam.
- [x] Scaffolded HPHYS0299 package.
- [x] Amended canonical contracts.
- [x] Added contract-derived tests.
- [x] Recorded pre-implementation contract gate.
- [x] Ran corrected partition diagnostics and full-suite metrics.
- [x] Completed dual review, disposition, and dual verification.

## Surprises and Discoveries

- Static inspection found openWEPP `simimpl28_stmtim_hourly_partition` already
  routes cold hourly precipitation as `rain_m / wntdur * 10.0`, matching
  pinned-baseline `stmtim.for`.
- HPHYS0298 mapped canonical `hrsnow` to
  `snow_hourly_snowfall_water_equiv_sum_m`; the runner already publishes
  `snow_hourly_snowfall_depth_sum_m`, which is the appropriate depth surface.
- The HPHYS0298 addendum cited `winter.for:410-412` as the migration target.
  The authoritative partition routine is `winter.for:296-300` calling
  `stmtim.for:43-95`; `winter.for:410-412` is an observe cut-point, not the
  partition equation.

## Decision Log

- Proceed with diagnostic correction before production migration. A production
  physics edit based on a depth-vs-water-equivalent comparison would violate
  correctness-over-completion and canonical unit governance.
- Corrected depth-vs-depth evidence supersedes HPHYS0298's all-window
  hourly-forcing routing: seven windows now route to `raw-hourly-melt`, one to
  `negative-melt-correction`, and only H39 first-2013 remains
  `hourly-forcing`.

## Exit Criteria

- Canonical contracts distinguish `hrsnow` depth from snowfall water-equivalent
  summaries and cite pinned-baseline `stmtim.for` provenance.
- Contract-derived tests pass and fail on the old water-equivalent mapping.
- Corrected HPHYS0299 partition ledger is published with source-line provenance,
  units, and per-window verdicts.
- Full H1..H39 same-HEAD metrics are published.
- Dual review and dual verification artifacts are completed and dispositioned.
- Final disposition states whether a production migration remains justified.

## Security Impact Gate

No external systems, credentials, network actions, or subprocess shell
interpolation changes are in scope. The work reads local baseline files and
edits local repository flat files only.

## Outcomes and Retrospective

HPHYS0299 corrected a material diagnostic unit/provenance seam before any
production physics migration. The corrected ledger supersedes HPHYS0298's
all-window hourly-forcing verdict:

- `raw-hourly-melt`: `7` windows.
- `negative-melt-correction`: `1` H7 first-2013 row, still
  `OPENWEPP-DEFECTIVE` and routed to post-raw follow-on rather than
  legacy-defective acceptance.
- `hourly-forcing`: `1` H39 first-2013 row with corrected depth-vs-depth
  forcing divergence.

The package made no production code changes. Continuation should target raw
hourly melt and post-raw routed-melt lineage, with H39 first-2013 forcing as a
secondary corrected-depth forcing case.
