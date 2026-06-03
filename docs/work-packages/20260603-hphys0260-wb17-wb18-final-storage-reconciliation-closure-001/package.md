# HPHYS0260 WB17/WB18 Final Storage Reconciliation Closure

Status: completed/HOLD

Evidence mode: mixed

This work package is an autonomous ExecPlan-style package. It follows
`docs/codex_exec_plans.md` and must remain self-contained enough for an agent
to execute all phases from kickoff through disposition without user
intervention unless a hard blocker is encountered.

## Objective

Execute the HPHYS0259 continuation recommendation by extending trace-grade
evidence for WB17 layer transpiration uptake (`UPi_####`/`Ui_####`), WB18
percolation/storage aggregate components (`wb18_perc_theta_####`,
`wb19_thetdr_####`, `wb19_dg_####`, optional
`wb18_perc_frozen_depth_####`), and final WB13 storage publication
(`Total-Soil`/`SoilWaterTotal`). Then run H1/H7/H39 targeted diagnostics and
the full H1..H39 hillslope semantic suite to classify whether the remaining
day-1 `Ep`/`Dp`/storage residuals are internal identity defects or
baseline-magnitude/initialization defects requiring a follow-on physics
migration package.

## Rationale

HPHYS0259 proved that H1/H7/H39 day-1 WB19 realized lateral identities close
internally. The continuation focus is therefore downstream of WB19 unless new
baseline-authoritative WB19 evidence appears. The observed day-1 residuals are
stable and small at the targeted seam: `Ep` differs by `0.235294 mm`, `Dp`
differs by about `0.0048 mm`, and `Total-Soil` differs by
`-0.247876/-0.209171/-0.336200 mm` for H1/H7/H39. HPHYS0260 must make the
WB17/WB18/storage identities observable in the same trace artifacts before any
new physics correction is attempted.

## Included Scope

- Amend canonical `SC-EVAP-001`, `SC-PERC-001`, and `SC-WATBAL-001` contracts
  for trace-grade WB17/WB18/final-storage residual classification.
- Add contract-derived tests proving opt-in trace rows serialize WB17 layer
  uptake and WB18 aggregate-storage component maps.
- Add trace row fields only; preserve default-off trace behavior and do not
  change production hydrology equations unless a hard contract-derived defect
  is found.
- Add/run package-local diagnostics that execute H1/H7/H39 trace runs and the
  full H1..H39 semantic suite.
- Complete required review, verification, gate, disposition, and worker
  handoff artifacts with truthfulness labels.

## Excluded Scope

- No heuristic or empirical compensation for `Ep`, `Dp`, `Total-Soil`, or
  `SoilWaterTotal`.
- No replacement of baseline-authoritative WB17/WB18 equations unless the
  package discovers and proves a concrete contract-derived defect.
- No reopening WB19 cap/publication logic unless new trace evidence contradicts
  HPHYS0259.
- No wepppy orchestration changes, external comparator-authority changes, or
  branch creation.

## Deliverables

- Canonical `SC-*` amendments for WB17/WB18/final-storage trace evidence.
- Red/green contract-derived trace serialization tests.
- HPHYS0245 opt-in trace schema bump carrying WB17 layer uptake and WB18
  aggregate-storage component maps.
- H1/H7/H39 WB17/WB18/storage classification report.
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
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260603-hphys0259-wb19-trace-ep-dp-storage-localization-closure-001/artifacts/disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260603-hphys0259-wb19-trace-ep-dp-storage-localization-closure-001/artifacts/worker-handoff.md`
- `/workdir/openWEPP/docs/work-packages/20260603-hphys0259-wb19-trace-ep-dp-storage-localization-closure-001/artifacts/targeted-h1-h7-h39-wb19-classification.md`

Physics/equation authority defaults to
`/workdir/wepp-forest_260430_baseline` at commit
`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.

## Intended Write Set

- `docs/work-packages/README.md`
- `docs/work-packages/20260603-hphys0260-wb17-wb18-final-storage-reconciliation-closure-001/**`
- `docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
- `docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `crates/openwepp-runner/src/hillslope/mod.rs`

## Contract-First Sequence

1. Contracts: amend canonical `SC-*` authority for WB17/WB18/final-storage
   trace-grade residual classification.
2. Contract-derived tests: add tests that fail before trace fields exist.
3. Pre-implementation gate: record failing contract-derived test evidence.
4. Production code edits: implement additive opt-in trace propagation only
   after the contract and test gate are complete.

## Phase Plan

### Phase A: Contract and Test Gate

Amend `SC-EVAP-001`, `SC-PERC-001`, and `SC-WATBAL-001` for HPHYS0260 trace
classification authority. Add a contract-derived runner test proving WB17
`UPi_####`/`Ui_####` and WB18 aggregate storage component maps appear in trace
rows. Run the test before production trace implementation and record the
failure.

### Phase B: Trace Propagation

Add WB17 layer uptake maps and WB18 residual/depth/frozen-depth storage maps to
the opt-in HPHYS0245 trace row. Preserve default-off trace behavior and avoid
changing hydrology physics.

### Phase C: Targeted Classification

Run H1/H7/H39 with trace enabled. Generate a classification report showing:

- `Ep == ΣUi_####` at `post_phase:plant_root_uptake`;
- `Ui_#### <= UPi_####` per layer;
- `Ws == Ep/Etp` when `Etp > 0`;
- WB18/final aggregate storage recomputes as
  `Σ(wb18_perc_theta_i + wb19_thetdr_i*(wb19_dg_i - frozen_i))`; and
- day-1 `Ep`/`Dp`/`Total-Soil` residuals are assigned to identity divergence
  or to baseline-magnitude/initialization follow-up.

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
- HPHYS0245 trace rows carry WB17/WB18/final-storage diagnostics in JSONL
  output when trace is enabled.
- H1/H7/H39 classification report is generated.
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

- Observation: H1/H7/H39 WB17, WB18, and final storage trace identities all
  close internally.
  Evidence: `/tmp/hphys0260_20260603T035231Z/reports/hphys0260_wb17_wb18_storage_classification.md`
  classifies all three hillslopes as `*_IDENTITIES_CLOSED_MAGNITUDE_FOCUS`.
- Observation: Full H1..H39 semantic metrics remain unchanged from HPHYS0259.
  Evidence: `/tmp/hphys0260_20260603T035231Z/reports/hillslope_semantic_summary.md`
  reports semantic pass `0/39` with the same selected residual metrics.

## Decision Log

- Decision: Scope HPHYS0260 to trace-grade residual classification, not a
  physics correction.
  Rationale: HPHYS0259 closed WB19 internal identities and recommended WB17
  `Ep`, WB18 `Dp`, and final storage reconciliation as the next evidence gate.
  Date/Author: 2026-06-03 / Codex.
- Decision: Add trace schema v4 fields for WB17 layer uptake and WB18
  aggregate-storage components without changing hydrology equations.
  Rationale: the package objective is residual ownership classification; no
  baseline-authoritative physics defect was demonstrated before implementation.
  Date/Author: 2026-06-03 / Codex.
- Decision: Keep package disposition in `HOLD`.
  Rationale: trace identities close, but full H1..H39 water-balance semantic
  parity remains `0/39`.
  Date/Author: 2026-06-03 / Codex.

## Outcomes & Retrospective

- Outcome: HPHYS0260 completed the requested continuation work package and
  extends opt-in HPHYS trace evidence to WB17 layer uptake, WB18
  residual/depth/frozen aggregate components, recomputed `watcon`, and final
  WB13 storage publication.
- Outcome: H1/H7/H39 day-1 `Ep`, `Dp`, `Total-Soil`, and
  `SoilWaterTotal` residuals are no longer classified as trace-publication or
  final-storage identity defects.
- Retrospective: the next continuation package should target
  baseline-authoritative magnitude/initialization lineage: the stable day-1
  `Ep + Dp + storage` residual split closes internally but remains semantically
  different from the baseline comparator.
