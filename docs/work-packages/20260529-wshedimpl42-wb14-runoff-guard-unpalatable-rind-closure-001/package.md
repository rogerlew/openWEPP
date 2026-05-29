# 20260529-wshedimpl42-wb14-runoff-guard-unpalatable-rind-closure-001

## Status
- state: hold
- date: 2026-05-29
- timezone: America/Los_Angeles
- decision: HOLD

## Objective
Execute WSHEDIMPL42 to close the active WB14 runoff guard failure blocking
`/wc1/runs/un/unpalatable-rind` hillslope execution and downstream watershed
routing, then rerun watershed generation to successful parquet output as the
closure condition.

## Why This Package Exists
The current watershed parity sanity run halts before routing because all 39
hillslope subprocesses fail with
`HKERNEL-WB14-RUNOFF-E-003` during `runoff_reconciliation`. Until this
domain failure is closed, watershed orchestration cannot produce the expected
`*.hbp` contributor set and watershed parquet outputs for parity review.

## Scope
### Included
- WB14 runoff guard root-cause assessment for `unpalatable-rind` execution
  using openWEPP runtime surfaces and typed phase evidence.
- Contract authority updates in canonical `SC-*` files when required by root
  cause.
- Contract-derived tests reproducing and fixing the discovered WB14 failure
  mode.
- Production fixes in hillslope runtime/orchestrator paths needed to remove
  the WB14 domain failure without heuristic fallback behavior.
- End-to-end rerun gates:
  1. all required hillslope contributors complete for the watershed,
  2. watershed execution completes,
  3. watershed parquet outputs are emitted.

### Explicitly Out of Scope
- Watershed process-physics redesign outside the specific WB14 blocker path.
- New optional features, refactors, or non-blocking ergonomics work.
- Any silent defaulting/clamping to mask unresolved domain violations.

## Deliverables
1. `artifacts/wshedimpl42-wb14-guard-gap-matrix.md`
2. `artifacts/wshedimpl42-contract-implementation-evidence.md`
3. `artifacts/wshedimpl42-contract-test-implementation-evidence.md`
4. `artifacts/wshedimpl42-preimplementation-contract-gate.md`
5. `artifacts/wshedimpl42-implementation-and-test-evidence.md`
6. `artifacts/wshedimpl42-kernel-profile-compliance-checklist.md`
7. `artifacts/owned-file-manifest.md`
8. `artifacts/gate-results.md`
9. `artifacts/wshedimpl42_disposition.md`
10. `artifacts/worker-handoff.md`
11. `artifacts/review_agent_a.md`
12. `artifacts/review_agent_b.md`
13. `artifacts/verification_agent_a.md`
14. `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
1. Implement required canonical contract/index amendments for WB14 blocker
   closure scope.
2. Implement contract-derived tests for the failing WB14 scenario.
3. Record pre-implementation contract-gate evidence showing tests/contracts are
   in place before production edits.
4. Apply production code changes to close the WB14 failure and rerun closure
   gates.

## Autonomous Execution Intent (Required)
This package must execute end-to-end through disposition without requesting
additional user direction unless hard-blocked.

## Truthfulness Labeling Requirement
All evidence artifacts must explicitly label `Static:` and/or `Ran:`.

## Provenance and Authority Posture
- Canonical authority lives in
  `docs/specifications/science-contracts/contracts/SC-*.md`.
- Legacy comparator/provenance anchor defaults to
  `/workdir/wepp-forest_260430_baseline` at commit
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- No heuristic/proxy process-physics substitutions are allowed for closure.
- Variable naming continuity with legacy WEPP symbols is required; alias
  mappings must be explicit where runtime names differ.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260528-wshedimpl41-ipeak5-mvpmc3-dynamic-coeff-refresh-parity-001/artifacts/worker-handoff.md`
- `/wc1/runs/un/unpalatable-rind`
- `/workdir/wepp-forest_260430_baseline/src/watbal.for`
- `/workdir/wepp-forest_260430_baseline/src/stmget.for`

## Intended Write Set
- `docs/work-packages/20260529-wshedimpl42-wb14-runoff-guard-unpalatable-rind-closure-001/**`
- `docs/work-packages/README.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-runner/src/hillslope/mod.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
- `tests/integration/wb14_infiltration_hyetograph_kernel_contract.rs`
- `tests/integration/cli03_runner_contract_derived_tests.rs`

## Phase Plan
### Phase A - Intake and scope freeze
- Confirm queue authorization and freeze to WB14 blocker closure + watershed rerun.

### Phase B - Gap assessment and authority mapping
- Reproduce failure on `unpalatable-rind`.
- Capture typed root-cause matrix with symbol-level failure evidence.

### Phase C - Contract updates (first required gate)
- Amend canonical `SC-*` authority if the root cause indicates contract drift,
  ambiguity, or missing invariants.

### Phase D - Contract-derived tests (second required gate)
- Add test vectors that fail on pre-fix behavior and encode expected post-fix
  behavior.

### Phase E - Pre-implementation contract gate (third required gate)
- Execute targeted tests and record expected pre-fix evidence before runtime edits.

### Phase F - Production closure implementation (fourth required gate)
- Implement typed, fail-closed production fixes for the WB14 blocker.
- Do not add silent defaults/clamps for unresolved domain violations.

### Phase G - Validation and watershed closure rerun
- Execute:
  - `cargo fmt --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo test --workspace`
  - `cargo deny check`
- Rerun `unpalatable-rind` hillslopes and watershed until parquet outputs are produced.

### Phase H - Dual review, dual verification, disposition
- Complete review and verification artifacts.
- Publish explicit GO/HOLD disposition with closure map and residual risks.

## Exit Criteria
- WB14 blocker root cause is documented with typed evidence.
- Contract-first sequence is satisfied and evidenced.
- Production WB14 failure path is closed for `unpalatable-rind`.
- Watershed run completes and emits expected parquet outputs.
- Required validation gates are executed and recorded truthfully.
- Dual review + dual verification artifacts are complete.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: local contract/runtime/test updates only; no credential or remote
  service surface changes.
