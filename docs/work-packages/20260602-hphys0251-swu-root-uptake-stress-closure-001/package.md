# 20260602-hphys0251-swu-root-uptake-stress-closure-001

Status: HOLD

This package is an ExecPlan-style living document. Keep `Progress`,
`Surprises & Discoveries`, `Decision Log`, and `Outcomes & Retrospective`
current while executing it. It follows `/workdir/openWEPP/docs/codex_exec_plans.md`.

## Purpose / Big Picture

HPHYS0250 proved that openWEPP now publishes nonzero final `Ep`, but the
remaining `Ep` magnitude is far below the pinned legacy baseline. This package
migrates the next baseline-authoritative `swu.for` details that control actual
root uptake and water-stress magnitude, then measures H1/H13/H39 and full
`H1..H39` semantic deltas. The observable result is a documented decision on
whether `Ep`, `Ws`, and aggregate storage residuals materially improved without
heuristic tuning.

## Objective

Diagnose, correct, and validate baseline-authoritative `swu.for` actual root
uptake and water-stress magnitude lineage for WB17/WB13 parity, focusing on
`pltol`, `UPi`, `Ui`, `Ep`, `Ws`, `rtd`, and layer storage.

## Rationale

HPHYS0250’s worker handoff recommends the next package: “baseline-authoritative
`swu.for` actual root uptake and water-stress magnitude closure, using H1/H13/H39
as diagnostic anchors and the HPHYS0245 trace fields for `Etp`, `UPi`, `Ui`,
`Ep`, `Ws`, `rtd`, and layer storage.” The HPHYS0250 full suite still reports
`0/39` semantic pass, `Ep` fail count `56230`, and `Ep` mean abs diff mean
`1.683414`. H1 emits nonzero candidate `Ep`, but only `92.67 mm` total versus
legacy `~3036.91 mm`, so this is an uptake/stress magnitude problem rather than
a publication-ingestion problem.

## Scope

Included:

- Amend canonical `SC-*` authority before production edits.
- Add contract-derived tests for `swu.for` effective `pltol` normalization,
  per-layer `UPi`/`Ui` lineage surfaces, final `Ep = ΣUi`, bounded `Ws`, and
  post-root-uptake aggregate storage recomputation.
- Implement only baseline-authoritative `swu.for` migration details traced to
  `/workdir/wepp-forest_260430_baseline/src/swu.for` at pinned baseline commit
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- Run targeted H1/H13/H39 diagnostics and full `H1..H39` semantic metrics.
- Dispatch dual static reviews and dual verification checks, disposition
  findings, and update package artifacts.

Excluded:

- Heuristic tuning of `Ep`, `Ws`, storage, snow, runoff, percolation, or
  lateral flow residuals.
- Full Penman-Monteith `evappm.for` migration.
- Live-plant interception carryover (`plaint`/`pintlv`) migration unless a
  traced `swu.for` invariant can be implemented with existing canonical
  runtime surfaces and contract authority.
- Snow/frost/runoff or WB18/WB19 process-family changes outside the minimal
  root-uptake write set.

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
- `docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/wepp-forest_260430_baseline/src/swu.for`
- `docs/work-packages/20260602-hphys0250-wb17-ep-lineage-closure-001/artifacts/worker-handoff.md`
- `docs/work-packages/20260602-hphys0250-wb17-ep-lineage-closure-001/artifacts/ep-lineage-diagnosis.md`
- `docs/work-packages/20260602-hphys0250-wb17-ep-lineage-closure-001/artifacts/full-39-suite-metrics.md`
- `docs/work-packages/20260602-hphys0245-wb11-wb18-layer-telemetry-and-storage-continuity-001/artifacts/hphys0245-telemetry-run-evidence.md`

## Intended Write Set

- `docs/work-packages/README.md`
- `docs/work-packages/20260602-hphys0251-swu-root-uptake-stress-closure-001/**`
- `docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
- `crates/openwepp-hillslope-orchestrator/src/constants.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/01_management.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/08_tests.rs`
- `crates/openwepp-hillslope-orchestrator/src/tests.rs`
- `tests/integration/wb17_et_physics_kernel_contract.rs`
- `crates/openwepp-runner/src/hillslope/mod.rs` only if runner trace,
  management projection, or publication evidence requires it.

## Contract-First Sequence

1. Implement required contract amendments.
2. Implement contract-derived tests.
3. Record pre-implementation contract-gate evidence.
4. Modify production code.

Kernel code edits before those three preceding steps are invalid package
execution.

## Phase Plan

### Phase A - Contract Authority

Amend `SC-EVAP-001` and `SC-WATBAL-001` with HPHYS0251 authority for
`swu.for` `pltol` normalization, layer-wise potential/actual uptake, `Ep`,
`Ws`, and aggregate storage recomputation. Update the science-contract index.

### Phase B - Contract-Derived Tests

Add tests that fail before production edits for:

- management runtime projection of crop `pltol` from the plant data line;
- legacy `swu.for` effective `pltol` normalization (`<=0 -> 0.25`,
  `<0.1 -> 0.1`, `>0.4 -> 0.4`);
- per-layer `UPi_####` and `Ui_####` publication with aggregate `UPi`/`Ui`;
- final `Ep = ΣUi`, `Ws = ΣUi / Etp` for positive demand, and `Ws = 1`
  for zero demand.

### Phase C - Production Migration

Implement the minimal baseline-authoritative root-uptake changes needed to
satisfy Phase B without introducing heuristic math. Preserve typed guard
posture and avoid silent invalid-domain defaults.

### Phase D - Diagnostics and Full Suite

Run targeted H1/H13/H39 diagnostics with HPHYS0245 trace fields and rerun the
full `H1..H39` semantic suite. Compare against HPHYS0250 metrics.

### Phase E - Review, Verification, Disposition

Dispatch dual reviews and dual verification checks, disposition all actionable
findings, update gate evidence, and record `GO` only if closure criteria are
met; otherwise keep `HOLD` and write the next continuation target.

## Exit Criteria

- Contracts, tests, and implementation preserve baseline `swu.for` provenance.
- Targeted tests pass and show the contract-derived failures are closed.
- H1/H13/H39 diagnostics exist for `Etp`, `UPi`, `Ui`, `Ep`, `Ws`, `rtd`, and
  layer storage.
- Full `H1..H39` semantic metrics exist.
- `Ep` and aggregate storage residual movement is assessed against HPHYS0250.
- No heuristic or proxy process-physics substitutions are introduced.
- Disposition is `HOLD` unless `Ep` magnitude and aggregate storage residuals
  materially reduce without regressing other high-priority water-balance
  surfaces.

## Security Impact Gate

- security_impact: low
- dedicated_security_review_required: no
- rationale: local kernel/runtime science logic and docs only; no external
  connectivity, credentials, or shell interpolation behavior changes.

## Progress

- [x] (2026-06-02) Package scaffolded from HPHYS0250 continuation recommendation.
- [x] (2026-06-02) Contract amendments authored.
- [x] (2026-06-02) Contract-derived tests added.
- [x] (2026-06-02) Pre-implementation contract gate recorded.
- [x] (2026-06-02) Production migration implemented.
- [x] (2026-06-02) Targeted validation and full suite run.
- [x] (2026-06-02) Dual reviews and verification completed.
- [x] (2026-06-02) Disposition recorded.

## Surprises & Discoveries

- Observation: HPHYS0250 already makes H1 `Ep` nonzero but still far below
  baseline magnitude.
  Evidence: HPHYS0250 `ep-lineage-diagnosis.md` reports H1 candidate `Ep` sum
  `92.6715228119 mm` versus baseline `~3036.91 mm`.
- Observation: HPHYS0251 increases H1/H13/H39 `Ep` slightly by honoring crop
  `pltol=0.1`, but full-suite `Ep` and aggregate-storage residuals worsen.
  Evidence: `/tmp/hphys0251_20260602T184933Z/reports/hphys0251_delta_from_hphys0250.md`.
- Observation: H1/H13/H39 final traces show `UPi≈Etp`, but mean `Ws` remains
  near `0.05` and candidate aggregate storage is far below baseline.
  Evidence: `/tmp/hphys0251_trace_20260602T190044Z/targeted_final_trace_summary.md`.

## Decision Log

- Decision: Scope HPHYS0251 to `swu.for` uptake/stress lineage and diagnostics,
  not snow/runoff/percolation.
  Rationale: HPHYS0250 identifies `Ep` under-uptake/stress magnitude as the
  first residual family after final `Ep` publication was corrected.
  Date/Author: 2026-06-02 / Codex.
- Decision: Keep HPHYS0251 in `HOLD` and move continuation focus upstream to
  storage availability into WB17 root uptake.
  Rationale: scoped SWU lineage is now contract-tested and observable, but
  target/full-suite metrics show the root limiter is insufficient candidate
  layer/aggregate storage, not missing `UPi` or crop tolerance lineage.
  Date/Author: 2026-06-02 / Codex.

## Outcomes & Retrospective

- Outcome: contract-first SWU lineage migration completed with passing code
  gates.
- Outcome: full `H1..H39` semantic pass remains `0/39`.
- Outcome: disposition is `HOLD`; next package should diagnose
  `wb18_perc_theta_####`/`wb11_soil_water`/`watcon` storage availability before
  post-WB19 root uptake.

## Truthfulness Labeling Requirement

All evidence artifacts must label `Static:` and/or `Ran:` claims explicitly.
