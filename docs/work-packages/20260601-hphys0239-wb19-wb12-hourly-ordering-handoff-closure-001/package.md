# 20260601-hphys0239-wb19-wb12-hourly-ordering-handoff-closure-001

## Status
- state: completed
- date: 2026-06-01
- timezone: America/Los_Angeles
- decision: HOLD

## Objective
Carry out immediate next actions from HPHYS0238 by closing the first
Dispatch-Group-B slice: enforce authoritative WB19 -> WB12 -> WB13 hourly-lane
handoff ordering and remove stale state-surface overshadowing for hydrology
publication flux families.

## Why This Package Exists
HPHYS0238 completed WB19 hourly iterative lateral/drainage execution and
published handoff actions requiring:
1. WB19/WB12 ordering authority hardening,
2. cross-phase contract-derived handoff checks, and
3. stale-surface prevention for WB13 publication lineage.

## Scope
### Included
- Contract amendments in:
  - `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
  - `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- Contract-derived tests in:
  - `tests/integration/wb11_hydrology_kernel_contract.rs`
  - `crates/openwepp-runner/src/hillslope/mod.rs` (WB13 publication lineage tests)
- Production implementation in:
  - `crates/openwepp-runner/src/hillslope/mod.rs`
- Workspace validation gates:
  - `cargo fmt --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo test --workspace`

### Explicitly Out of Scope
- Full scheduler topology rewrite for per-hour substep phase execution.
- MOFE hourly carry-array migration (`Dispatch Group C`).
- WB14/WB12 full cadence migration (`Dispatch Group D`).

## Closure Measures (Required)
1. `MEASURE-HP239-001`: canonical contracts explicitly encode WB19/WB12
   handoff-ordering authority and stale-surface prohibition for affected WB13
   hydrology flux families.
2. `MEASURE-HP239-002`: contract-derived tests enforce canonical WB11 ordering
   for `Percolation -> ET -> Lateral -> Drainage -> RunoffReconciliation ->
   StorageReconciliation`.
3. `MEASURE-HP239-003`: WB13 publication path uses flux-authoritative values
   for `Q`, `Ep`, `Es`, `Er` when both state and flux symbols coexist.
4. `MEASURE-HP239-004`: required workspace gates pass and are recorded with
   truthful evidence labels.

## Deliverables
1. `artifacts/hphys0239-contract-implementation-evidence.md`
2. `artifacts/hphys0239-contract-test-implementation-evidence.md`
3. `artifacts/hphys0239-preimplementation-contract-gate.md`
4. `artifacts/hphys0239-implementation-and-test-evidence.md`
5. `artifacts/hphys0239-kernel-profile-compliance-checklist.md`
6. `artifacts/owned-file-manifest.md`
7. `artifacts/gate-results.md`
8. `artifacts/hphys0239_disposition.md`
9. `artifacts/worker-handoff.md`
10. `artifacts/review_agent_a.md`
11. `artifacts/review_agent_b.md`
12. `artifacts/verification_agent_a.md`
13. `artifacts/verification_agent_b.md`

## Mandatory Sequence (Required)
1. Amend canonical contracts for ordering/handoff authority.
2. Add/adjust contract-derived tests.
3. Record pre-implementation contract gate.
4. Modify production code.
5. Run required workspace gates.
6. Publish disposition and worker handoff.

## Autonomous Execution Intent (Required)
Execute phases end-to-end through disposition without requesting additional
user direction unless hard-blocked.

## Truthfulness Labeling Requirement
Each artifact must label evidence class (`Static:` vs `Ran:`).

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260601-hphys0237-hourly-bulk-routine-inventory-and-dispatch-001/artifacts/hphys0237-hourly-routine-inventory.md`
- `/workdir/openWEPP/docs/work-packages/20260601-hphys0238-wb19-hourly-iterative-lateral-drainage-closure-001/artifacts/worker-handoff.md`
- `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for`
- `/workdir/wepp-forest_260430_baseline/src/drain.for`

## Intended Write Set
- `docs/work-packages/README.md`
- `docs/work-packages/20260601-hphys0239-wb19-wb12-hourly-ordering-handoff-closure-001/**`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `tests/integration/wb11_hydrology_kernel_contract.rs`
- `crates/openwepp-runner/src/hillslope/mod.rs`

## Phase Plan
### Phase A - Contract authority amendment
- Amend `SC-WATBAL-001` and `SC-SUBHYD-001` for WB19->WB12->WB13 handoff
  ordering and flux-authoritative publication semantics.

### Phase B - Contract-derived tests + pre-implementation gate
- Add ordering and stale-surface handoff vectors.
- Record pre-implementation contract gate evidence.

### Phase C - Production implementation + gates
- Implement flux-authoritative WB13 publication updates.
- Run required workspace gates.

### Phase D - Disposition + handoff
- Publish closure evidence and next hold-lift queue actions.

## Exit Criteria
- `MEASURE-HP239-001..004` satisfied and evidenced.
- Disposition explicitly records remaining HOLD queue for Dispatch Group B/C/D.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: local kernel/runner/tests/docs edits only; no credentials/network writes.
