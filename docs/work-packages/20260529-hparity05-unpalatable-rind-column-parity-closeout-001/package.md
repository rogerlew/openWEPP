# 20260529-hparity05-unpalatable-rind-column-parity-closeout-001

## Status
- state: queued
- date: 2026-05-29
- timezone: America/Los_Angeles
- decision: HOLD

## Objective
Execute HPARITY05 as the parity closeout package: rerun full `unpalatable-rind`
hillslope and watershed integration, verify all 12 previously always-fail
hillslope columns are closed, and publish hold-lift disposition evidence.

## Why This Package Exists
HPARITY02/03/04 close residual families in sequence. HPARITY05 is the single
closure package that proves end-to-end behavior after those family closures,
with one authoritative evidence bundle and explicit GO/HOLD decision output.

## Scope
### Included
- Final residual fixes needed after HPARITY02/03/04 reruns (if any).
- Full 39-hillslope integration rerun and semantic parity comparison.
- Watershed integration rerun (`pw0_openwepp.run`) with emitted parquet checks.
- Final hold-lift decision artifact with explicit closure metrics.

### Explicitly Out of Scope
- New process families not tied to the 12 always-fail columns.
- Scope expansion into unrelated watershed channel/sediment parity domains.

## Closure Measures (Required)
1. `MEASURE-HP05-001`: hillslope semantic pass count is `39/39`.
2. `MEASURE-HP05-002`: each prior always-fail column
   (`Dp`, `Ep`, `Es`, `ProfileDepth`, `ProfileFCStore`, `ProfilePorosityCap`,
   `ProfileWPStore`, `RM`, `Snow-Water`, `SoilWaterTotal`, `Total-Soil`,
   `latqcc`) has fail count `0`.
3. `MEASURE-HP05-003`: hillslope batch execution success is `39/39`, with
   binary HBP magic `WFPHBP01` present for all pass files.
4. `MEASURE-HP05-004`: watershed CLI exit code is `0` and required interchange
   parquet outputs are present.
5. `MEASURE-HP05-005`: required validation gates pass and are truthfully
   recorded (`cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
   `cargo test --workspace`, `cargo deny check`).

## Deliverables
1. `artifacts/hparity05-parity-closeout-gap-matrix.md`
2. `artifacts/hparity05-contract-implementation-evidence.md`
3. `artifacts/hparity05-contract-test-implementation-evidence.md`
4. `artifacts/hparity05-preimplementation-contract-gate.md`
5. `artifacts/hparity05-implementation-and-test-evidence.md`
6. `artifacts/hparity05-kernel-profile-compliance-checklist.md`
7. `artifacts/owned-file-manifest.md`
8. `artifacts/gate-results.md`
9. `artifacts/hparity05_disposition.md`
10. `artifacts/worker-handoff.md`
11. `artifacts/review_agent_a.md`
12. `artifacts/review_agent_b.md`
13. `artifacts/verification_agent_a.md`
14. `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
1. Confirm prior package contract/test closures and update any final canonical
   contract amendments required by residual fixes.
2. Implement/adjust contract-derived tests for closeout coverage where needed.
3. Record pre-implementation gate evidence before residual production edits.
4. Apply final scoped production edits and execute full closeout reruns.

## Autonomous Execution Intent (Required)
This package must execute end-to-end through disposition without requesting
additional user direction unless hard-blocked.

## Truthfulness Labeling Requirement
All evidence artifacts must explicitly label `Static:` and/or `Ran:`.

## Provenance and Authority Posture
- Canonical authority lives in
  `docs/specifications/science-contracts/contracts/SC-*.md`.
- Legacy comparator/provenance anchor is
  `/workdir/wepp-forest_260430_baseline` at commit
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- Do not claim GO with unresolved invariant or closure-measure failures.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260529-hparity02-profile-capacity-storage-lineage-closure-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260529-hparity03-rainmelt-energy-snow-column-closure-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260529-hparity04-percolation-lateralflow-soilwater-closure-001/package.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/wc1/runs/un/unpalatable-rind`

## Intended Write Set
- `docs/work-packages/README.md`
- `docs/work-packages/20260529-hparity05-unpalatable-rind-column-parity-closeout-001/**`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-runner/src/hillslope/mod.rs`
- `crates/openwepp-hillslope-output/src/hillslope_wat.rs`
- `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs`
- `tests/integration/**/hparity*_*.rs`
- `tools/legacy_comparison_suite/**`

## Phase Plan
### Phase A - Intake and closure target freeze
- Confirm HPARITY02/03/04 outputs and unresolved residuals.
- Freeze closeout criteria to `MEASURE-HP05-001..005`.

### Phase B - Final contract/test adjustments
- Apply any final canonical SC amendments required by residual fixes.
- Ensure closeout test surfaces cover all 12 columns.

### Phase C - Pre-implementation contract gate
- Record contract/test readiness before any final production edits.

### Phase D - Final production edits
- Apply residual scoped edits needed to satisfy closeout measures.

### Phase E - Full validation and reruns
- Execute full validation gates.
- Run full 39-hillslope + watershed integration closure sequence.
- Run semantic comparisons and summarize final per-column status.

### Phase F - Dual review, dual verification, disposition
- Complete dual review/verification artifacts.
- Publish explicit GO/HOLD decision based on measure outcomes.

## Exit Criteria
- Closure measures `MEASURE-HP05-001..005` are satisfied and evidenced.
- Final disposition artifact contains absolute run paths, comparator reports,
  and gate outputs needed for audit replay.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: local runtime/test/doc updates only; no auth/network changes.
