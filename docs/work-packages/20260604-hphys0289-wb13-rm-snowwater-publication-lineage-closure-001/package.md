# HPHYS0289 WB13 RM/Snow-Water Publication Lineage Closure

Status: executed-hold

This ExecPlan is a living document. Keep `Progress`, `Surprises & Discoveries`,
`Decision Log`, and `Outcomes & Retrospective` current as work proceeds. This
package follows `/workdir/openWEPP/docs/codex_exec_plans.md`.

## Purpose / Big Picture

After this package, openWEPP should publish WB13 `RM` and `Snow-Water` from the
same winter/runoff state surfaces consumed by the hydrology kernel. A user can
observe the change with a contract-derived WB13 publication test, H1/H7/H39
trace rows over the HPHYS0288 material residual days, and full H1..H39 semantic
metrics that show whether `RM`, `Q`, or `Snow-Water` moved.

## Progress

- [x] (2026-06-04) Scaffolded HPHYS0289 package structure, kickoff prompt, and queued artifacts.
- [x] (2026-06-04) Read required contracts, HPHYS0288 handoff, and pinned baseline WB13 output files.
- [x] (2026-06-04) Amend canonical `SC-*` contracts for WB13 `RM` and `Snow-Water` publication authority.
- [x] (2026-06-04) Add contract-derived tests before production code edits.
- [x] (2026-06-04) Record pre-implementation contract-gate evidence.
- [x] (2026-06-04) Implement minimal WB13/kernel publication-lineage correction.
- [x] (2026-06-05) Run focused tests, adjacent tests, Rust gates, and full H1..H39 metrics.
- [x] (2026-06-05) Complete dual review, review disposition, dual verification, final disposition, and handoff.

## Surprises & Discoveries

- Observation: HPHYS0288 improved `Ep`, storage, and lateral metrics but did not move `Q`, `RM`, or `Snow-Water`.
  Evidence: `docs/work-packages/20260604-hphys0288-winter-rain-on-snow-melt-partition-magnitude-closure-001/artifacts/worker-handoff.md`.
- Observation: Pinned baseline daily and hourly water-balance output publish `RM` as `rain(iplane) + wmelt(iplane) + irdept(iplane) + iraplo(iplane)`, and `Snow-Water` as `snodpy(iplane) * densg(iplane)`.
  Evidence: `/workdir/wepp-forest_260430_baseline/src/watbalprint.for:84-106` and `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for:1082-1142`.


- Observation: Review found a stale HPARITY01 `RM` lineage row still documenting the rejected SWE-delta proxy.
  Evidence: Dual review findings A-001/B-002; fixed in `SC-WATBAL-001` HPARITY01 register.
- Observation: Full H1..H39 runtime completes after the routed-melt publication seam, but semantic parity remains `0/39`; `RM` fail count improves from 6633 to 5868 while mean absolute `RM` residual increases from 0.248018 to 0.258409.
  Evidence: `/tmp/hphys0289_full_release_current_20260605T000159Z/reports/hillslope_semantic_summary.md`.
- Observation: H39 2014-146 still publishes `RM=2.62 mm` from raw rain when runtime SWE and routed melt are zero, while trace shows only `0.3825 mm` released rain and no routed melt remains.
  Evidence: `/tmp/hphys0289_target_traces_current_20260605T000516Z/H39.trace.jsonl`.

## Decision Log

- Decision: Scope HPHYS0289 to WB13 publication lineage and the missing daily routed-melt publication surface, not ET/storage tuning.
  Rationale: HPHYS0288 already corrected the residual rain-on-snow forcing seam; remaining unchanged `RM`/`Snow-Water` metrics point at WB13 publication or winter/runoff state surfaces.
  Date/Author: 2026-06-04 / Codex.

## Outcomes & Retrospective

Executed and left in HOLD. HPHYS0289 corrected the baseline-obvious WB13 `RM` lineage from raw precipitation plus SWE-delta proxy to routed `wmelt` plus post-winter rain/irrigation, published `snow.routed_melt_m` from the hydrology kernel, and added behavior tests for snow-active routed melt, warm-rain/no-snow, missing routed melt, negative routed melt, and flux-over-state shadowing. Full H1..H39 runtime completes, but semantic parity remains `0/39` and targeted traces show remaining residuals require an explicit post-winter `rain(iplane)` publication surface rather than the current WB13 inference branch. Recommended continuation: HPHYS0290 post-winter rain publication seam, focused on baseline `contin.for` warm-rain/no-snow restoration and snow-active rain clearing.

## Objective

Diagnose, correct, and validate WB13 `RM`/`Snow-Water` publication lineage so
daily water-balance rows consume baseline-authoritative `rain + wmelt +
irrigation` and runtime snowpack storage surfaces rather than stale or derived
proxy quantities.

## Rationale

HPHYS0288 added residual rain-on-snow release into routed melt and proved that
the hydrology kernel can consume the corrected forcing. Full-suite metrics still
showed no `RM`, `Q`, or `Snow-Water` movement. Static baseline inspection shows
WB13 output does not compute `RM` from total precipitation plus change in SWE;
it publishes post-winter `rain(iplane)` plus daily `wmelt(iplane)` plus
irrigation. Baseline `contin.for` clears `rain(iplane)` after winter processing
except the warm-rain/no-snow restoration branch, so snow-active days should
publish `RM` from routed `wmelt` rather than raw precipitation compensation.

## Included Scope

- Amend canonical `SC-WATBAL-001`, `SC-RUNOFFPART-001`, and
  `SC-SNOWFREEZE-001` for WB13 `RM`/`Snow-Water` publication provenance.
- Add contract-derived tests before production code edits.
- Add or validate a daily routed-melt (`wmelt`) publication surface from the
  hydrology kernel.
- Correct WB13 `RM` publication to consume post-winter rain/routed-melt/
  irrigation lineage with typed guards.
- Preserve HPHYS0287 fail-closed snow-state validation and HPHYS0288
  `resolve_snow_partition_terms` centralization.
- Run H1/H7/H39 targeted traces and full H1..H39 semantic metrics.
- Complete dual independent reviews, finding disposition, dual verification,
  final disposition, and worker handoff.

## Excluded Scope

- WB17 `Ep` compensation, plant-growth changes, or storage tuning.
- Heuristic runoff or snow-water multipliers.
- Replicating the rejected negative-melt bug from baseline WEPP.
- Loosening SWE/depth/density/liquid-domain guards.
- Broad frost migration or watershed/MOFE routing changes.

## Deliverables

- Canonical contract amendments with pinned baseline provenance citations.
- Contract-derived tests proving WB13 `RM` consumes routed `wmelt` rather than
  raw-precipitation/SWE-delta proxy math on snow-active days.
- Production correction for the minimal publication seam proven by the tests.
- H1/H7/H39 trace evidence over current material residual days.
- Full H1..H39 runtime and semantic metrics compared to HPHYS0288.
- Dual review artifacts, review-disposition artifact, dual verification
  artifacts, final disposition, and worker handoff.

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
- `docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `docs/work-packages/20260604-hphys0288-winter-rain-on-snow-melt-partition-magnitude-closure-001/artifacts/worker-handoff.md`
- `docs/work-packages/20260604-hphys0288-winter-rain-on-snow-melt-partition-magnitude-closure-001/artifacts/disposition.md`
- `docs/work-packages/20260604-hphys0288-winter-rain-on-snow-melt-partition-magnitude-closure-001/artifacts/full-39-suite-metrics.md`
- `/workdir/wepp-forest_260430_baseline/src/contin.for` at commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`
- `/workdir/wepp-forest_260430_baseline/src/winter.for` at commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`
- `/workdir/wepp-forest_260430_baseline/src/watbalprint.for` at commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`
- `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for` at commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`
- `/workdir/wepp-forest_260430_baseline/src/outfil.for` at commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`

## Intended Write Set

- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
- `crates/openwepp-runner/src/hillslope/mod.rs`
- `tests/integration/hphys0289_wb13_rm_snowwater_publication_contract.rs`
- `Cargo.toml`
- `docs/work-packages/README.md`
- `docs/work-packages/20260604-hphys0289-wb13-rm-snowwater-publication-lineage-closure-001/**`

## Phase Plan

1. Scaffold package and placeholders.
2. Read required contracts, HPHYS0288 evidence, and pinned baseline WB13 output files.
3. Amend canonical contracts for WB13 `RM`/`Snow-Water` publication authority.
4. Add contract-derived failing tests before production code edits.
5. Record pre-implementation contract gate evidence.
6. Implement minimal kernel/runner publication-lineage correction.
7. Run focused tests, adjacent tests, Rust gates, and H1..H39 metrics.
8. Complete dual reviews, review disposition, dual verification, final disposition, and handoff.

## Contract-First Sequence

1. Contracts.
2. Contract-derived tests.
3. Pre-implementation contract gate.
4. Production code edits.

Production kernel/runtime publication edits are prohibited before steps 1-3 are
complete and recorded.

## Exit Criteria

- Canonical `SC-*` contracts explicitly authorize WB13 `RM = rain + wmelt +
  irrigation` and `Snow-Water = runtime snowpack SWE` lineage.
- A contract-derived test fails before and passes after production correction,
  or the package remains `HOLD` with evidence that no production defect was proven.
- HPHYS0287 fail-closed runtime snow-state validation and HPHYS0288 partition
  centralization remain intact.
- Full H1..H39 runtime completes and semantic metrics are recorded.
- Dual reviews and dual verification are complete with no undispositioned findings.
- Evidence artifacts label claims truthfully with `Static:` vs `Ran:`.

## Security-Impact Gate

No external service, credential, network, shell-interpolated subprocess, or
unsafe Rust change is planned. Implementation must not change subprocess
orchestration or sidecar discovery.
