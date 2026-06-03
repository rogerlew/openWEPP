# HPHYS0261 WB17 Ep Magnitude Initialization Lineage Closure

Status: HOLD

Evidence mode: ran

This work package is an autonomous ExecPlan-style package. It follows
`docs/codex_exec_plans.md` and must remain self-contained enough for an agent
to execute all phases from kickoff through disposition without user
intervention unless a hard blocker is encountered.

## Objective

Execute the HPHYS0260 continuation recommendation by diagnosing and, when
baseline-authoritative, correcting the H1/H7/H39 day-1 WB17 `Ep` magnitude
split. The package must make the `evap`/`swu` seam observable for
initialization lineage: partition `Etp`, plant state (`lai`, `cancov`, `rtd`),
effective `pltol`, WB18 `ul(i)` thresholds, per-layer `UPi`/`Ui`, and final
WB13 `Ep`/storage publication. It must then run targeted H1/H7/H39 diagnostics
and the full H1..H39 hillslope semantic suite.

## Rationale

HPHYS0259 and HPHYS0260 close WB19, WB17 uptake, WB18 percolation, and final
storage trace identities for H1/H7/H39. The remaining stable day-1 split is
now a magnitude/initialization question: candidate `Ep` is `0.235294 mm`
above baseline while `Dp`, `latqcc`, and final storage move by the same order.
Baseline authority shows `evap.for` seeds plant transpiration demand from
current `lai`, then `watbal(_hourly).for` executes daily plant growth and
calls `swu.for` with the already-seeded `ep`. HPHYS0261 must distinguish a
true equation defect from missing trace observability before changing kernel
math.

## Included Scope

- Amend canonical `SC-EVAP-001` and `SC-WATBAL-001` for WB17 `Ep`
  magnitude/initialization trace requirements.
- Add contract-derived tests that fail before trace rows expose `pltol`,
  effective `pltol`, WB18 `ul(i)`, and stress-threshold lineage.
- Add additive, opt-in HPHYS0245 trace fields needed to diagnose `Ep`
  magnitude residuals; preserve default-off behavior.
- Run H1/H7/H39 targeted diagnostics against HPHYS0260 evidence and rerun the
  full H1..H39 hillslope semantic suite after implementation.
- Complete review, verification, gate, disposition, and worker-handoff
  artifacts with `Static:` vs `Ran:` truthfulness labels.

## Excluded Scope

- No heuristic reduction of `Ep`, `Etp`, `lai`, or `rtd`.
- No empirical tuning of water-stress or canopy partition coefficients.
- No reopening WB19 cap/publication, HPHYS trace publication, WB18 `D=Pe`, or
  final WB13 storage shadowing without new baseline-authoritative evidence.
- No wepppy orchestration changes, external comparator-authority changes, or
  branch creation.

## Deliverables

- Canonical `SC-*` amendments for WB17 `Ep` magnitude/initialization evidence.
- Red/green contract-derived trace observability test.
- HPHYS0245 trace schema bump with `pltol`, effective `pltol`, `ul(i)`, and
  stress-threshold lineage fields.
- H1/H7/H39 `Ep` magnitude/initialization diagnostic report.
- Full H1..H39 semantic metric snapshot.
- Dual review and dual verification artifacts.
- Final disposition and continuation-focused worker handoff.

## Dependencies

- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-PLANT-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260603-hphys0260-wb17-wb18-final-storage-reconciliation-closure-001/artifacts/disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260603-hphys0260-wb17-wb18-final-storage-reconciliation-closure-001/artifacts/worker-handoff.md`
- `/workdir/openWEPP/docs/work-packages/20260603-hphys0260-wb17-wb18-final-storage-reconciliation-closure-001/artifacts/full-39-suite-metrics.md`
- `/workdir/openWEPP/docs/work-packages/20260603-hphys0260-wb17-wb18-final-storage-reconciliation-closure-001/artifacts/targeted-h1-h7-h39-wb17-wb18-storage-classification.md`

Physics/equation authority defaults to
`/workdir/wepp-forest_260430_baseline` at commit
`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.

## Intended Write Set

- `docs/work-packages/README.md`
- `docs/work-packages/20260603-hphys0261-wb17-ep-magnitude-initialization-lineage-closure-001/**`
- `docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `crates/openwepp-runner/src/hillslope/mod.rs`

## Contract-First Sequence

1. Contracts: amend canonical `SC-*` authority for WB17 `Ep`
   magnitude/initialization trace evidence.
2. Contract-derived tests: add tests that fail before the trace fields exist.
3. Pre-implementation gate: record failing contract-derived test evidence.
4. Production code edits: implement additive opt-in trace propagation only
   after the contract and test gate are complete.

## Phase Plan

### Phase A: Contract and Test Gate

Amend `SC-EVAP-001` and `SC-WATBAL-001` for HPHYS0261 trace requirements.
Add a contract-derived runner test proving trace rows serialize raw `pltol`,
effective `swu` `pltol`, WB18 `ul(i)`, and stress-threshold lineage. Run the
test before production trace implementation and record the failure.

### Phase B: Trace Propagation

Add additive HPHYS0245 trace fields for `pl_pltol`,
`pl_swu_effective_pltol`, `wb18_ul_layers_m`,
`wb17_swu_stress_threshold_layers_m`, and
`wb17_swu_storage_to_threshold_layers`. Preserve default-off tracing and avoid
changing hydrology equations unless a hard contract-derived defect is proven.

### Phase C: Targeted Diagnosis

Run H1/H7/H39 targeted diagnostics. Classify day-1 `Ep +0.235294 mm` against:

- baseline WAT `Ep` and candidate WAT `Ep`;
- candidate `Etp`, final `Ep`, and `ΣUi_####`;
- candidate `lai`, `rtd`, `pltol`, effective `pltol`, and stress thresholds;
- WB18 `ul(i)`/`theta(i)` branch evidence; and
- legacy static call-order evidence from `evap.for`, `swu.for`, and
  `watbal(_hourly).for`.

### Phase D: Full Suite Metrics

Run the full H1..H39 semantic suite and record selected residual metrics for
`Ep`, `Dp`, `latqcc`, `Total-Soil`, `SoilWaterTotal`, `Q`, `RM`, and
`Snow-Water`.

### Phase E: Review, Verification, Disposition

Complete dual review and dual verification artifacts, run required gates,
publish final disposition, and produce a worker handoff with a continuation
recommendation.

## Exit Criteria

- Contract amendments and contract-derived tests are present.
- Pre-implementation failing test evidence is recorded.
- HPHYS0245 trace rows expose WB17 `Ep` magnitude/initialization diagnostics.
- H1/H7/H39 diagnostic report is generated.
- Full H1..H39 semantic metrics are recorded.
- `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo test --workspace`, `cargo deny check`, authority anti-evasion guards,
  and `git diff --check` pass or any failure is truthfully recorded.
- Disposition is not `GO` unless semantic closure and contract gates are
  complete.

## Security-Impact Gate

No external systems, credentials, or network actions are required. The package
uses flat-file worktree edits plus local comparator and cargo commands.

## Progress

- [x] Scaffold package.
- [x] Amend contracts.
- [x] Add contract-derived test.
- [x] Record pre-implementation gate.
- [x] Implement trace propagation.
- [x] Run targeted and full diagnostics.
- [x] Run validation gates.
- [x] Complete review, verification, disposition, and handoff.

## Surprises & Discoveries

- H1/H7/H39 day-1 candidate `Ep` equals traced `Etp`, final `Ep`, and
  `ΣUi_####` at `0.385294 mm`, while baseline WAT `Ep` is `0.150000 mm`.
- All traced WB18 storage-to-`pltol*ul(i)` ratios are above one for H1/H7/H39,
  so the stable `Ep +0.235294 mm` residual is not an SWU stress-clipping
  branch.
- The common residual across hillslopes points to upstream `evap` demand or
  plant-state initialization/call-order magnitude, not hillslope-specific
  storage stress.

## Decision Log

- Decision: Scope HPHYS0261 to WB17 `Ep` magnitude/initialization lineage.
  Rationale: HPHYS0260 proved WB17/WB18/final-storage trace identities close
  for H1/H7/H39 and recommended the stable day-1 `Ep +0.235294 mm` split as
  the next closure focus.
  Date/Author: 2026-06-03 / Codex.

## Outcomes & Retrospective

- HPHYS0261 adds contract authority and opt-in trace evidence only; it does not
  change hydrology equations.
- Targeted classification for H1/H7/H39 is
  `ETP_FULL_DEMAND_NO_SWU_STRESS_MAGNITUDE_FOCUS`.
- Full H1..H39 semantic pass remains `0/39`; disposition remains `HOLD`.
- Continuation should target baseline-authoritative `evap.for` demand seeding
  and plant-state initialization/call-order lineage, especially the state used
  to seed `Etp` before the daily `ptgrp`/`ptgra` growth/root update.
