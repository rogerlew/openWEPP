# HPHYS0259 WB19 Trace Ep/Dp/Storage Localization Closure

Status: completed/HOLD

Evidence mode: mixed

## Objective

Execute the HPHYS0258 continuation recommendation by making the HPHYS0245
run-trace evidence carry WB19 potential/target/`tdvv`/realized-withdrawal
diagnostics for real H1/H7/H39 runs, classify whether remaining `latqcc`
residuals are WB19-internal or downstream publication/storage, and re-adjudicate
the next Ep/Dp/storage focus with a fresh full H1..H39 semantic snapshot.

## Rationale

HPHYS0258 added canonical WB19 diagnostics inside the hydrology kernel but left
real-run trace evidence unable to consume those diagnostics directly. The next
decision point is evidence classification, not heuristic lateral damping:

- if run traces show WB19 potential, target, `tdvv`, realized withdrawal, `q`,
  and `Qd` reconcile internally, do not reopen WB19 cap/publication math;
- if they do not reconcile, keep the next package focused on WB19 internal
  lineage; and
- absent WB19-internal divergence, focus subsequent package work on WB17 `Ep`,
  WB18 `Dp`, and final aggregate storage reconciliation.

## Included Scope

- Amend canonical contracts for trace-grade WB19 diagnostic propagation and
  H1/H7/H39 classification evidence.
- Add contract-derived tests proving the opt-in trace row serializes WB19
  realized-flow diagnostics from runtime surfaces.
- Propagate HPHYS0258 WB19 diagnostic surfaces into the existing opt-in
  HPHYS0245 trace row without changing physics or default runtime behavior.
- Add/run package-local diagnostics that execute H1/H7/H39 trace runs and the
  full H1..H39 semantic suite.
- Publish package artifacts with truthfulness labels and final HOLD/GO
  disposition.

## Excluded Scope

- No heuristic WB19 `latqcc` damping, storage compensation, or empirical
  coefficient fitting.
- No replacement of baseline-authoritative WB17/WB18/WB19 equations unless a
  contract-derived defect is demonstrated during this package.
- No changes to wepppy orchestration or external comparator authority.
- No branch creation.

## Deliverables

- Canonical `SC-*` amendments for WB19 trace evidence propagation.
- Red/green contract-derived trace serialization tests.
- HPHYS0245 opt-in trace row fields for WB19 diagnostic surfaces.
- H1/H7/H39 WB19 classification report.
- Full H1..H39 semantic metric snapshot.
- Dual review and dual verification artifacts.
- Disposition and worker handoff with the next continuation recommendation.

## Dependencies

- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260603-hphys0258-wb19-hourly-cap-withdrawal-publication-closure-001/artifacts/disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260603-hphys0258-wb19-hourly-cap-withdrawal-publication-closure-001/artifacts/worker-handoff.md`
- `/workdir/openWEPP/docs/work-packages/20260603-hphys0258-wb19-hourly-cap-withdrawal-publication-closure-001/artifacts/wb19-hourly-cap-withdrawal-publication-diagnosis.md`

Physics/equation authority defaults to
`/workdir/wepp-forest_260430_baseline` at commit
`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.

## Intended Write Set

- `docs/work-packages/README.md`
- `docs/work-packages/20260603-hphys0259-wb19-trace-ep-dp-storage-localization-closure-001/**`
- `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `crates/openwepp-runner/src/hillslope/mod.rs`

## Contract-First Sequence

1. Contracts: amend canonical `SC-*` authority for trace-grade WB19 diagnostic
   propagation and classification evidence.
2. Contract-derived tests: add tests that fail before trace propagation exists.
3. Pre-implementation gate: record the failing contract-derived test evidence.
4. Production code edits: implement opt-in trace propagation only after the
   contract and test gate are complete.

## Phase Plan

### Phase A: Contract and Test Gate

Amend `SC-SUBHYD-001` and `SC-WATBAL-001` for HPHYS0259 trace propagation.
Add a contract-derived runner test proving WB19 diagnostics appear in trace
rows. Run the test before production implementation and record the failure.

### Phase B: Trace Propagation

Add WB19 potential/target/`tdvv`/capacity/unrealized/per-layer withdrawal and
`q`/`Qd` fields to the opt-in HPHYS0245 trace row. Preserve default-off trace
behavior and avoid changing kernel physics.

### Phase C: Targeted Classification

Run H1/H7/H39 with trace enabled. Generate a classification report showing
whether the lateral-transfer row reconciles:

- `q == sum(wb19_lateral_withdrawal_####)`;
- `Qd == q + Qdd` when `Qdd` is present;
- `wb19_q_lateral_unrealized == max(target - q, 0)`; and
- `latqcc` residual classification is WB19-internal or downstream.

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
- HPHYS0245 trace rows carry WB19 diagnostics in JSONL output when trace is
  enabled.
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

## Decision Log

- Decision: Scope HPHYS0259 to trace propagation and residual localization
  before reopening WB19 numerical logic.
  Rationale: HPHYS0258 found no baseline-authoritative WB19 cap/publication
  correction and explicitly recommended using new diagnostics before applying
  further WB19 changes.
  Date/Author: 2026-06-03 / Codex.
- Decision: Add the HPHYS0258 WB19 diagnostics to the opt-in trace schema
  rather than changing WB19 flux equations.
  Rationale: H1/H7/H39 trace identities close for `q`,
  `Σwb19_lateral_withdrawal_####`, unrealized residual, and `Qd = q + Qdd`.
  Date/Author: 2026-06-03 / Codex.
- Decision: Keep final package disposition in `HOLD`.
  Rationale: full H1..H39 semantic pass remains `0/39`; package closes
  residual ownership classification for WB19 cap/publication but not
  water-balance parity.
  Date/Author: 2026-06-03 / Codex.

## Outcomes & Retrospective

- Outcome: HPHYS0259 is completed/HOLD. It makes real-run WB19 diagnostics
  visible in trace evidence and classifies H1/H7/H39 day-1 lateral identities
  as closed.
- Outcome: the full H1..H39 semantic suite remains `0/39`, with residuals
  unchanged from HPHYS0258.
- Retrospective: the next continuation package should focus on WB17 `Ep`,
  WB18 `Dp`, and final aggregate storage reconciliation. Do not reopen WB19
  cap/publication logic without new baseline-authoritative divergence evidence.
