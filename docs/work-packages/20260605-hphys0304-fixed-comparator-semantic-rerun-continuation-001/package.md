# HPHYS0304 Fixed-Comparator Semantic Rerun Continuation

## Status

Executed-HOLD.

## Objective

Execute ADR-0016 Required Continuation Order step 1: re-run the H1..H39
semantic water-balance suite with openWEPP candidate outputs compared against
the fixed `wepp_260430` baseline comparator artifacts, then re-classify the
H1/H7/H39 snow/`RM` target windows under ADR-0011 confidence-tier rules. This
package also scaffolds HPHYS0305 as the required paired melt-term/state
instrumentation package for continuation order step 2.

## Rationale

ADR-0016 ratified the fixed negative-melt comparator anchor and regenerated the
H1..H39 fixed-baseline parquets, but explicitly left the semantic rerun and
snow/`RM` reclassification as required continuation. The old HPHYS0302 route
used the original `dac3c950` baseline outputs; keeping those residuals as the
active direction after ADR-0016 would mix comparator authorities. The lowest
regret next step is to rerun the semantic comparator against the fixed baseline
and preserve the production-edit `HOLD` until paired term/state evidence names
the first divergent producer.

## Included Scope

- Use HPHYS0303 fixed-baseline H1..H39 parquet artifacts from
  `/tmp/hphys0303_adr0016_1780691036/reports/hillslope/fixed_baseline_partitions`.
- Use existing HPHYS0300 openWEPP candidate WAT parquets from
  `/tmp/hphys0300_full_20260605T155527Z/hillslope_output` only if the runner
  verifies that runtime source files under `crates/` and `src/` are unchanged
  since the candidate-output commit.
- Run `tools/legacy_comparison_suite/semantic_hillslope_wat_compare.py` for
  H1..H39 with `--candidate-year-offset 2012` and the PL14S tolerance config.
- Aggregate per-hillslope semantic reports into fixed-baseline full-suite
  metrics, including `RM`, `Snow-Water`, `Total-Soil`, `SoilWaterTotal`, `Ep`,
  `Es`, `Dp`, `Q`, and `latqcc`.
- Recompute H1/H7/H39 target-window daily WAT residual sums against the fixed
  baseline and compare them to the prior original-baseline residual posture.
- Re-classify snow/`RM` target windows under ADR-0011:
  daily single-OFE WAT residuals are higher-confidence investigation signals,
  while hourly/term-level melt deltas remain investigation-only until paired
  baseline/openWEPP term-state surfaces exist.
- Scaffold HPHYS0305 as the paired melt-term/state instrumentation package.
- Complete dual review, review disposition, dual verification, gate evidence,
  and worker handoff.

## Excluded Scope

- Production openWEPP physics edits.
- WB13/WB17/WB18/WB19/WB12 compensation patches.
- Snow, forcing, or melt producer edits before paired term/state evidence.
- Treating aggregate `RM`, `Snow-Water`, raw `hrmlt`, or routed `wmelt`
  residuals as term-level producer authority.
- Regenerating fixed comparator baseline artifacts; HPHYS0303 already owns that
  artifact source.
- Pushing remote refs, creating branches, or changing external repositories.

## Dependencies

- `/home/workdir/openWEPP/AGENTS.md`
- `/home/workdir/openWEPP/docs/codex_exec_plans.md`
- `/home/workdir/openWEPP/docs/work-packages/README.md`
- `/home/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/home/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/home/workdir/openWEPP/docs/decisions/0016-promote-260430-baseline-as-canonical-comparator-and-abandon-kernel-rewrite.md`
- `/home/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/home/workdir/openWEPP/docs/work-packages/20260605-hphys0302-comparator-surface-audit-closure-001/artifacts/comparator-surface-audit-ledger.json`
- `/home/workdir/openWEPP/docs/work-packages/20260605-hphys0303-adr0016-fixed-comparator-ratification-001/artifacts/comparator-ratification-ledger.json`
- `/home/workdir/openWEPP/docs/work-packages/20260605-hphys0303-adr0016-fixed-comparator-ratification-001/artifacts/fixed-baseline-parquet-manifest.json`
- `/home/workdir/openWEPP/tools/legacy_comparison_suite/semantic_hillslope_wat_compare.py`
- `/home/workdir/openWEPP/tools/legacy_comparison_suite/configs/pl14s_wat_tolerances.json`
- `/tmp/hphys0300_full_20260605T155527Z/hillslope_output`
- `/workdir/wepp-forest_260430_baseline`

## Intended Write Set

- `docs/work-packages/README.md`
- `docs/work-packages/20260605-hphys0304-fixed-comparator-semantic-rerun-continuation-001/**`
- `docs/work-packages/20260605-hphys0305-paired-melt-term-state-instrumentation-001/**`
- `tests/integration/hphys0304_fixed_comparator_semantic_rerun_contract.rs`
- `Cargo.toml`

## Phase Plan

1. **Scaffold**: create HPHYS0304 and HPHYS0305 package structures, prompts,
   placeholders, and a source-level contract test guarding the continuation
   order.
2. **Contract and gate posture**: record that ADR-0016/ADR-0011/SC contract
   authority is already ratified for this diagnostic rerun and that no
   production code edit is authorized before paired term/state evidence.
3. **Semantic rerun**: run H1..H39 semantic comparator reports against fixed
   baseline parquets with year-offset normalization and record full-suite
   metrics.
4. **Window reclassification**: compute fixed-baseline daily WAT target-window
   residual sums for H1/H7/H39, compare them to prior original-baseline
   residuals, and classify the next evidence need under ADR-0011.
5. **Continuation scaffold**: ensure HPHYS0305 is queued with autonomous,
   contract-first instructions for paired melt-term/state instrumentation.
6. **Review and validation**: run focused validations, perform dual review,
   disposition every finding, complete dual verification, and update
   disposition/worker handoff.

## Progress

- [x] (2026-06-05T21:03:04Z) Identified ADR-0016 Required Continuation Order
  and HPHYS0303 fixed-baseline artifacts.
- [x] (2026-06-05T21:03:04Z) Scaffold HPHYS0304 and HPHYS0305 required
  structure.
- [x] (2026-06-05T21:03:04Z) Add source-level continuation-order guard test.
- [x] (2026-06-05T21:03:04Z) Run H1..H39 fixed-baseline semantic comparator
  rerun.
- [x] (2026-06-05T21:03:04Z) Reclassify H1/H7/H39 snow/`RM` target windows.
- [x] (2026-06-05T21:03:04Z) Complete dual review, disposition,
  verification, and validation gates.

## Exit Criteria

- H1..H39 fixed-baseline semantic comparator reports exist and aggregate metrics
  are recorded.
- Target-window reclassification covers all nine H1/H7/H39 snow/`RM` windows.
- Reclassification explicitly applies ADR-0011 confidence tiers and does not
  authorize production edits from aggregate or hourly-only evidence.
- HPHYS0305 is scaffolded and queued for paired melt-term/state instrumentation
  against the fixed baseline.
- HPHYS0302 production-edit `HOLD` remains explicit unless paired term/state
  evidence is produced, which is outside this package.
- Dual review and dual verification artifacts have no undispositioned findings.

## Security Impact Gate

No credentials, network actions, branch changes, or remote pushes are in scope.
Work is limited to local flat-file reads/edits, local comparator invocation,
local artifact generation, and local tests.
