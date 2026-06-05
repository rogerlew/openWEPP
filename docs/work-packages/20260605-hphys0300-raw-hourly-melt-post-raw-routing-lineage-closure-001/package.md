# HPHYS0300 Raw Hourly Melt/Post-Raw Routing Lineage Closure

## Status

Executed-hold.

## Objective

Diagnose and, only when canonical contract evidence proves a production defect,
correct the H1/H7/H39 raw hourly `hrmlt` and post-raw
`wmelt`/routed-melt lineage after HPHYS0299 corrected canonical `hrsnow`
snowfall-depth mapping.

## Rationale

HPHYS0299 superseded HPHYS0298's all-window hourly-forcing verdict. Corrected
depth-vs-depth evidence routes seven target windows to `raw-hourly-melt`, one
H7 first-2013 row to post-raw routed-melt/negative-melt handling, and one H39
first-2013 row to a corrected-depth hourly forcing seam. The next lowest-regret
step is term/state lineage evidence at the raw melt and post-raw routing seam,
not downstream WB17/WB18/WB19/WB13 compensation.

This package starts after HPHYS0299 corrected the `hrsnow` unit/provenance seam.

## Included Scope

- Amend canonical `SC-SNOWFREEZE-001` and `SC-WATBAL-001` to require HPHYS0300
  term/state lineage evidence before any raw-melt or post-raw production edit.
- Add contract-derived tests proving HPHYS0300 keeps the ordered cut-point
  authority and fail-closed evidence requirements.
- Build an HPHYS0300 diagnostic runner that reuses corrected HPHYS0299
  depth-vs-depth partition evidence and adds row/window classification for:
  raw hourly melt magnitude, post-raw redistribution/rain-release residuals,
  and the H39 corrected-depth hourly-forcing seam.
- Run same-HEAD full H1..H39 semantic metrics and targeted H1/H7/H39 traces.
- Record whether additional baseline observe instrumentation is required for
  melt-term (`amelt`, `bmelt`, `cmelt`, `dmelt`) and snow-state input closure.
- Complete dual independent review, review disposition, and dual verification.

## Excluded Scope

- WB17, WB18, WB19, or WB13 compensation edits.
- Reproducing the pinned-baseline negative-melt sign/branch bug; corrected
  `/workdir/wepp-forest` negative-melt authority remains target behavior.
- Production raw-melt or post-raw routing edits from aggregate deltas alone.
- Silent defaults, canonicalize-and-proceed paths, or weakening typed
  fail-closed snow-state guards.
- H39 first-2013 precipitation-forcing migration unless the package proves the
  forcing seam must be fixed before raw/post-raw melt evidence can be trusted.

## Dependencies

- `/home/workdir/openWEPP/AGENTS.md`
- `/home/workdir/openWEPP/docs/codex_exec_plans.md`
- `/home/workdir/openWEPP/docs/work-packages/README.md`
- `/home/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `/home/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/home/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/home/workdir/openWEPP/docs/work-packages/20260605-hphys0299-hourly-snow-partition-unit-provenance-closure-001/package.md`
- `/home/workdir/openWEPP/docs/work-packages/20260605-hphys0299-hourly-snow-partition-unit-provenance-closure-001/artifacts/corrected-partition-ledger.json`
- `/home/workdir/openWEPP/docs/work-packages/20260605-hphys0299-hourly-snow-partition-unit-provenance-closure-001/artifacts/worker-handoff.md`
- `/home/workdir/openWEPP/docs/work-packages/20260605-hphys0299-hourly-snow-partition-unit-provenance-closure-001/artifacts/disposition.md`
- `/home/workdir/openWEPP/docs/work-packages/20260605-hphys0299-hourly-snow-partition-unit-provenance-closure-001/artifacts/hphys0299_corrected_partition.py`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`
- `/home/workdir/openWEPP/crates/openwepp-runner/src/hillslope/mod.rs`
- `/workdir/wepp-forest_260430_baseline/src/winter.for`
- `/workdir/wepp-forest_260430_baseline/src/snowd.for`
- `/workdir/wepp-forest_260430_baseline/src/melt.for`

## Intended Write Set

- `docs/work-packages/README.md`
- `docs/work-packages/20260605-hphys0300-raw-hourly-melt-post-raw-routing-lineage-closure-001/**`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `tests/integration/hphys0300_raw_hourly_melt_post_raw_routing_contract.rs`
- `Cargo.toml`
- Production snow kernel files only if term/state evidence proves a
  baseline-authoritative openWEPP defect:
  `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`

## Phase Plan

1. **Contracts**: add HPHYS0300 canonical authority for raw hourly melt and
   post-raw routing diagnostics, preserving corrected negative-melt target
   authority and downstream compensation prohibitions.
2. **Contract-derived tests**: add tests that enforce package autonomy,
   corrected HPHYS0299 dependency, ordered cut-point evidence, and fail-closed
   diagnostic requirements.
3. **Pre-implementation contract gate**: run the focused contract test before
   diagnostic implementation and record truthfully labeled evidence.
4. **Diagnostics**: implement and run the HPHYS0300 runner, reusing HPHYS0299
   corrected evidence and generating raw/post-raw lineage summary artifacts.
5. **Correction checkpoint**: edit production snow code only if diagnostics
   identify a contract-authorized file:line openWEPP defect; otherwise keep
   disposition in `HOLD` with precise follow-on instrumentation requirements.
6. **Review and verification**: complete dual review, disposition findings,
   run dual verification, and publish final continuation routing.

## Progress

- [x] Scaffold package and required artifacts.
- [x] Amend contracts.
- [x] Add contract-derived tests.
- [x] Record pre-implementation contract gate.
- [x] Run diagnostics and full-suite metrics.
- [x] Complete dual review/disposition/verification.

## Exit Criteria

- Canonical contracts define HPHYS0300 evidence requirements and keep
  downstream compensation prohibited.
- Contract-derived tests pass.
- `artifacts/raw-post-raw-lineage-summary.md` and
  `artifacts/raw-post-raw-lineage-ledger.json` classify all nine target
  windows with same-HEAD evidence.
- Full H1..H39 metrics are published for continuation.
- Production edits, if any, are justified by term/state evidence and pass
  focused plus workspace gates.
- Production edits, if any, are justified by term/state evidence from
  `artifacts/raw-post-raw-lineage-ledger.json`; aggregate deltas alone keep
  disposition in `HOLD`.
- Dual review and dual verification artifacts are completed with no
  undispositioned findings.

## Security Impact Gate

No external systems, credentials, network actions, or shell-interpolation
changes are in scope. Work is limited to local flat-file reads/edits, local
Rust test execution, and local comparator subprocesses with explicit argument
arrays.

## Outcomes and Retrospective

HPHYS0300 executed as a diagnostic/authority package and made no production
kernel edits. Corrected HPHYS0299 evidence still routes seven windows to
raw-hourly-melt, one H7 first-2013 row to post-raw routed-melt without
baseline negative raw melt, and one H39 first-2013 row to corrected-depth
hourly forcing. Aggregate raw/post-raw deltas alone do not identify the
term/state source, so production edits remain unauthorized pending paired
baseline `melt.for`/`snowd.for` term and state instrumentation.

Dual review found closeout, metadata, package-index, and route-regression
gaps. Those were dispositioned by completing final artifacts, updating
contract metadata/index dates, strengthening the focused test to parse the
published nine-row ledger, and rerunning the focused gate.
