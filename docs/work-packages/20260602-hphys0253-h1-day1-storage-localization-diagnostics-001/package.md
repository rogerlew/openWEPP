# 20260602-hphys0253-h1-day1-storage-localization-diagnostics-001

Status: completed

This package is a living ExecPlan. Keep `Progress`, `Surprises & Discoveries`,
`Decision Log`, and `Outcomes & Retrospective` current while executing it. It
follows `/workdir/openWEPP/docs/codex_exec_plans.md`.

## Purpose / Big Picture

HPHYS0252 corrected a real WB19 frozen lateral-storage defect but produced no
selected residual movement in the current `H1..H39` semantic suite. Its
post-commit review recommends a diagnostic-only localization gate before any
additional process-surface correction. This package executes that gate for
`H1`: compare openWEPP post-seed and day-1 storage against baseline daily
water-balance surfaces, compute day-1 conservation residuals, and decide
whether the next implementation package should target initial/runtime storage
projection, WB18 deep seepage, WB19 lateral withdrawal, snow/runoff timing, or
another observable handoff.

## Objective

Execute the HPHYS0252 continuation recommendation without production physics
edits. The observable deliverable is an evidence-backed H1 t=0/day-1
localization report plus a fresh full `H1..H39` semantic metric snapshot.

## Rationale

HPHYS0252 showed that chasing another lateral-transfer sub-path is not justified
until the storage deficit is localized. The strongest current signature is
storage starvation or early day-1 depletion: `Ep` remains low, aggregate storage
is far below baseline, and selected residuals did not move after the WB19
`fzdrfc` correction. A targeted diagnostic can distinguish a seed-state scale
problem from a downstream loss-surface problem without inventing or tuning
physics.

## Included Scope

- Scaffold and execute a diagnostic-only work package.
- Reuse the environment-gated HPHYS0245 trace sidecar for current `H1` day-1
  post-seed, post-phase, post-scheduler, and post-WB13 state.
- Compare `H1` candidate post-seed/post-WB13 storage with baseline `H1` WAT
  `Total-Soil` and `SoilWaterTotal` rows.
- Compute candidate and baseline day-1 conservation balances using available
  WAT terms: input (`P` and `RM` variants), ET (`Ep + Es + Er`), `Dp`,
  `latqcc`, `Q`, and storage change.
- Run a fresh full `H1..H39` openWEPP hillslope batch and semantic comparator
  snapshot.
- Record review, verification, gate, disposition, and worker-handoff artifacts.

## Excluded Scope

- No production kernel, runner, or science-contract edits.
- No heuristic storage inflation, default clamping, or compensation terms.
- No WB18/WB19/WB17 process correction in this package.
- No watershed rerun unless explicitly authorized by a later package.
- No commit/push unless separately requested.

## Deliverables

1. Package-local evidence artifacts under this directory.
2. `artifacts/hphys0253_diagnostics.py`, a package-local reproducibility helper.
3. `artifacts/h1-day1-storage-localization.md`, the H1 t=0/day-1 diagnosis.
4. `artifacts/h1-day1-conservation-audit.md`, conservation-residual evidence.
5. `artifacts/full-39-suite-metrics.md`, fresh current semantic metrics.
6. Dual review and verification artifacts, truthfully labeled.
7. Final disposition and worker handoff naming the next implementation focus.

## Dependencies

- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260602-hphys0252-wb19-lateral-storage-availability-closure-001/artifacts/hphys0252_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260602-hphys0252-wb19-lateral-storage-availability-closure-001/artifacts/review_claude_code_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260602-hphys0252-wb19-lateral-storage-availability-closure-001/artifacts/worker-handoff.md`
- `/tmp/unpalatable_parity_20260529T192707Z/runs/`
- `/tmp/unpalatable_parity_20260529T192707Z/reports/hillslope/baseline_partitions/`
- `/workdir/wepppy/.venv/bin/python`
- `/workdir/wepp-forest_260430_baseline` at commit
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`

## Intended Write Set

- `docs/work-packages/README.md`
- `docs/work-packages/20260602-hphys0253-h1-day1-storage-localization-diagnostics-001/**`

## Contract-First Sequence

This package is diagnostic-only and does not authorize production code edits.
If execution discovers a proven production defect, stop short of code edits,
record the finding, and scaffold a follow-on contract-first implementation
package. Any follow-on implementation must execute:

1. Implement required canonical `SC-*` contract amendments.
2. Implement contract-derived tests.
3. Record pre-implementation contract gate evidence.
4. Modify production code.

No kernel code edits are allowed in HPHYS0253.

## Phase Plan

### Phase A — Scaffold and Verify Inputs

Create required package structure and verify local runfile, baseline parquet,
comparator, Python, and runner prerequisites. Record blockers explicitly if any
required input root is missing.

### Phase B — H1 Day-1 Localization

Build `openwepp-cli-hill`, run current `H1` with
`OPENWEPP_HPHYS0245_TRACE_PATH` and `OPENWEPP_HPHYS0245_TRACE_MAX_DAYS=1`, then
analyze post-seed, post-phase, post-scheduler, and post-WB13 state. Compare
candidate day-1 storage and fluxes to baseline `H1`.

### Phase C — Full 39 Metric Snapshot

Run current `H1..H39` hillslopes using the existing `unpalatable-rind` runfiles.
Run `semantic_hillslope_wat_compare.py` for each hillslope against baseline
partitions and summarize selected residual families.

### Phase D — Review, Verification, and Disposition

Review the diagnostic logic and results. Record verification artifacts, gates,
final disposition, and a concrete next work-package recommendation.

## Exit Criteria

- H1 post-seed and day-1 storage evidence is recorded.
- H1 day-1 conservation residuals are computed and interpreted.
- Fresh full `H1..H39` runtime and semantic metrics exist.
- All claims in artifacts are labeled `Static:` or `Ran:`.
- No production code was changed.
- Disposition names the next implementation focus or a hard blocker.

## Security-Impact Gate

No external systems or network actions are required. Work is local repository
engineering over flat files and local diagnostic commands. No secrets,
credentials, user data, or production service state are accessed or modified.

## Progress

- [x] (2026-06-02) Scaffolded HPHYS0253 diagnostic package.
- [x] (2026-06-02) Ran current H1 day-1 trace and localization audit.
- [x] (2026-06-02) Ran full `H1..H39` metric snapshot.
- [x] (2026-06-02) Recorded review, verification, disposition, and handoff.

## Surprises & Discoveries

- Observation: H1 day-1 candidate water accounting closes exactly against the
  trace and WAT terms, so the immediate defect is not an unbalanced WB13
  publication or hidden day-1 sink.
  Evidence: `artifacts/h1-day1-conservation-audit.md` reports candidate
  inferred initial from `P` equals actual post-seed `323.346740 mm`.
- Observation: H1 starts `20.153260 mm` drier than the baseline WAT-derived
  t=0 proxy before any day-1 scheduler phase runs, and day-1 excess loss adds
  only `2.072666 mm` of the end-of-day storage gap.
  Evidence: `artifacts/h1-day1-storage-localization.md`.
- Observation: candidate post-seed layer theta sum is `293.945130 mm`, while
  candidate aggregate `wb11_soil_water` is `323.346740 mm`; the
  `29.401610 mm` aggregate-layer alias gap persists at t=0.
  Evidence: `artifacts/h1-day1-storage-localization.md`.

## Decision Log

- Decision: Make HPHYS0253 diagnostic-only with no production code edits.
  Rationale: HPHYS0252 review requires localization before another
  process-surface correction; contract-first implementation should start only
  after the failing handoff is identified.
  Date/Author: 2026-06-02 / Codex.
- Decision: Recommend WB11 initial/runtime storage projection as the next
  implementation package, not another WB18/WB19 loss-surface correction.
  Rationale: H1 post-seed deficit is the dominant day-1 storage gap component;
  Dp is near baseline and latqcc over-withdrawal is much smaller than the
  pre-scheduler storage deficit.
  Date/Author: 2026-06-02 / Codex.

## Outcomes & Retrospective

- HPHYS0253 completed as a diagnostic-only package. It produced a reproducible
  H1 localization helper, H1 day-1 trace/conservation evidence, and a fresh
  full `H1..H39` semantic snapshot. Disposition remains `HOLD` for semantic
  parity, with continuation focused on contract-first WB11 initial/runtime
  storage projection and direct layer `st`/`theta`/`watcon` authority.
