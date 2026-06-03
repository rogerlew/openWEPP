# 20260603-hphys0258-wb19-hourly-cap-withdrawal-publication-closure-001

Status: completed/HOLD

This package is a living ExecPlan. Keep `Progress`, `Surprises & Discoveries`,
`Decision Log`, and `Outcomes & Retrospective` current while executing it. It
follows `/workdir/openWEPP/docs/codex_exec_plans.md`.

## Purpose / Big Picture

HPHYS0257 closed the hourly WB19 horizontal-conductivity lineage and materially
reduced H1/H7/H39 day-1 `latqcc` residuals, but full H1..H39 semantic parity
remains `0/39`. This package executes the continuation by diagnosing,
contracting, correcting, and validating the remaining hourly WB19 cap,
withdrawal, and publication lineage: `tdvv`, frozen-adjusted thresholds,
top-down withdrawal, `latqcc` accumulation, `Qd`/WB13 publication, and
aggregate storage reconciliation.

## Objective

Diagnose, correct, and validate baseline-authoritative hourly WB19
cap/withdrawal/publication semantics for H1/H7/H39 and the full 39-hillslope
suite. Closure requires contract-backed behavior, contract-derived red tests,
production changes only where proven, fresh metrics, and truthful disposition.

## Rationale

HPHYS0257 left small but stable H1/H7/H39 day-1 `latqcc` diffs: H1
`+0.023532 mm`, H7 `+0.047995 mm`, and H39 `+0.180364 mm`. Conductivity is no
longer the first suspect. The next observable WB19 surfaces are the cap that
limits hourly lateral flow, the withdrawal floor that mutates layer storage,
and the publication path that turns realized lateral flow into WAT outputs and
aggregate soil-water storage.

## Included Scope

- Amend canonical `SC-SUBHYD-001` and `SC-WATBAL-001` for any missing proven
  hourly WB19 cap/withdrawal/publication semantics.
- Add contract-derived tests before production edits.
- Diagnose pinned baseline `watbal_hourly.for` around `tdvv`, `latqcc`,
  top-down storage withdrawal, `sbrunf`/`ui_lfcrf`, `Qd`, and WAT publication.
- Correct only baseline-authoritative hourly WB19 behavior needed for
  `latqcc`/`Qd`/storage closure.
- Run targeted H1/H7/H39 diagnostics and a fresh full H1..H39 semantic metric
  snapshot.
- Record review, verification, gates, and final disposition.

## Excluded Scope

- No heuristic `latqcc` damping or storage compensation.
- No reopening HPHYS0257 horizontal-conductivity lineage unless new evidence
  contradicts its contract vectors.
- No WB17 Ep/root uptake, snow/runoff timing, or WB18 Pe/Dp changes unless a
  proven WB19 publication seam requires a narrow fixture expectation update.
- No watershed rerun unless separately authorized.
- No commit/push unless separately requested.

## Deliverables

1. Package-local evidence artifacts under this directory.
2. Canonical contract amendments for missing hourly WB19 cap/withdrawal/
   publication authority.
3. Contract-derived tests that fail before any production correction and pass
   after it.
4. Minimal production correction for hourly WB19 cap, withdrawal, or
   publication behavior, if diagnosis proves a defect.
5. Targeted H1/H7/H39 diagnostics.
6. Full H1..H39 semantic metrics and continuation disposition.

## Dependencies

- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260603-hphys0257-wb19-hourly-latqcc-lineage-closure-001/artifacts/disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260603-hphys0257-wb19-hourly-latqcc-lineage-closure-001/artifacts/worker-handoff.md`
- `/workdir/openWEPP/docs/work-packages/20260603-hphys0257-wb19-hourly-latqcc-lineage-closure-001/artifacts/targeted-h1-h7-h39-diagnostics.md`
- `/workdir/openWEPP/docs/work-packages/20260603-hphys0257-wb19-hourly-latqcc-lineage-closure-001/artifacts/full-39-suite-metrics.md`
- `/workdir/wepppy/.venv/bin/python`
- `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for`

## Intended Write Set

- `docs/work-packages/20260603-hphys0258-wb19-hourly-cap-withdrawal-publication-closure-001/**`
- `docs/work-packages/README.md`
- `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
- `crates/openwepp-runner/src/hillslope/mod.rs`
- `tests/integration/wb19_lateral_drainage_physics_kernel_contract.rs`
- Adjacent WB13 publication tests only if publication seam correction requires
  direct coverage.

## Contract-First Sequence

1. Implement required canonical contract amendments.
2. Implement contract-derived tests.
3. Record pre-implementation contract gate evidence.
4. Modify production code.

No production code edits are allowed before steps 1-3 are complete.

## Phase Plan

### Phase A — Baseline Cap and Publication Diagnosis

Read required contracts, HPHYS0257 evidence, and pinned hourly
`watbal_hourly.for` lateral blocks. Record cap calculation, realized `latqcc`,
layer withdrawal, `sbrunf`/`ui_lfcrf`, `Qd`, and WAT publication semantics in
`artifacts/wb19-hourly-cap-withdrawal-publication-diagnosis.md`.

### Phase B — Contract and Test Gate

Amend canonical contracts for proven missing hourly semantics. Add
contract-derived tests that demonstrate the defect. Run the targeted test
before production edits and record expected failing evidence in
`artifacts/pre-implementation-contract-gate.md`.

### Phase C — Production Correction

Apply the minimal hourly WB19 correction. Preserve typed fail-closed guards;
do not introduce silent defaults, output compensation, or heuristic flux
scaling. Run targeted tests and update
`artifacts/implementation-test-evidence.md`.

### Phase D — Metrics, Review, and Disposition

Run targeted H1/H7/H39 diagnostics and the full H1..H39 semantic suite. Record
metrics, review artifacts, verification artifacts, gate results, and final
`HOLD`/`GO` disposition.

## Exit Criteria

- Contracts and tests cover implemented behavior.
- Production changes trace to pinned baseline source, not heuristic math.
- H1/H7/H39 `latqcc`/storage evidence is recorded.
- Full H1..H39 semantic metrics exist or not-run rationale is explicit.
- All claims in artifacts are labeled `Static:` or `Ran:`.
- Full Rust gates are run or explicitly marked not-run with rationale.
- Disposition is truthful; unresolved semantic parity remains `HOLD`.

## Security-Impact Gate

No external systems or network actions are required. Work is local repository
engineering over flat files and local diagnostic commands. No secrets,
credentials, user data, or production service state are accessed or modified.

## Progress

- [x] (2026-06-03) Scaffolded HPHYS0258 from HPHYS0257 continuation.
- [x] (2026-06-03) Completed hourly WB19 cap/withdrawal/publication diagnosis.
- [x] (2026-06-03) Amended contracts and added red contract-derived tests.
- [x] (2026-06-03) Implemented observable WB19 cap/withdrawal diagnostics.
- [x] (2026-06-03) Ran targeted and full-suite metric snapshots.
- [x] (2026-06-03) Recorded review, verification, disposition, and handoff.

## Surprises & Discoveries

- Static: pinned baseline hourly WB19 already distinguishes potential,
  `tdvv`-capped target, realized withdrawal, `sbrunf`, and `ui_LfCrf`.
- Static: openWEPP already published realized `q`/`Qd`; the actionable gap was
  missing observable diagnostics that allow continuation packages to separate
  WB19 cap defects from downstream storage/ET residuals.
- Ran: full H1..H39 metrics are unchanged from HPHYS0257, confirming this
  package improved observability and contract coverage rather than numerical
  parity.

## Decision Log

- Decision: Scope HPHYS0258 to hourly WB19 cap, withdrawal, and publication.
  Rationale: HPHYS0257 closed conductivity lineage and left small but stable
  H1/H7/H39 `latqcc` residuals.
  Date/Author: 2026-06-03 / Codex.
- Decision: Add WB19 potential/target/`tdvv`/realized-withdrawal diagnostics
  rather than applying a numerical `latqcc` correction.
  Rationale: contract-derived vector proves the production path publishes
  realized withdrawal and `Qd`; no baseline-authoritative cap formula defect
  was found in the scoped kernel path.
  Date/Author: 2026-06-03 / Codex.

## Outcomes & Retrospective

- Outcome: HPHYS0258 is completed/HOLD. It closes an observability and
  contract-test gap for WB19 cap/publication lineage but does not close
  hillslope water-balance semantic parity.
- Retrospective: next continuation should use the new diagnostics to decide
  whether remaining `latqcc` residuals are WB19-internal; absent that evidence,
  the larger residual pressure remains Ep/Dp/storage coupling rather than WB19
  cap publication.
