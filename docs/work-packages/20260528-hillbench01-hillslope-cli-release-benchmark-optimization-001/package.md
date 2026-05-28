# 20260528-hillbench01-hillslope-cli-release-benchmark-optimization-001

## Status
- state: complete
- date: 2026-05-28
- timezone: UTC
- decision: GO

## Objective
Establish a release-build benchmark and optimization baseline for
`openwepp-cli-hill` across both single-OFE and multi-OFE lanes, compare
performance against baseline `wepp_260430_hill`, and land scoped optimizations
that preserve hillslope runtime behavior and output contracts.

## Why This Package Exists
Hillslope CLI parity and closure work has executed primarily via functional
contract gates. The project has not yet produced explicit release-build
runtime benchmarks for single-OFE and multi-OFE lanes against the pinned legacy
baseline executable. This package creates that baseline and applies targeted
hot-path improvements with evidence.

## Scope
### Included
- Release build of `openwepp-cli-hill` and baseline comparator executable
  execution (`wepp_260430_hill`) on selected single-OFE and multi-OFE lanes.
- Repeatable timing capture (multiple runs per lane) with explicit command
  provenance and summary statistics.
- Comparison report of openWEPP vs baseline runtime on selected lanes.
- Scoped optimization edits in hillslope CLI/orchestration path where timing
  evidence identifies clear cost centers.
- Re-run benchmark lanes after optimization and record before/after deltas.
- Required package artifacts, gate results, dual review, and dual verification.

### Explicitly Out of Scope
- New process-physics features or equation-authority changes unrelated to
  performance work.
- Watershed CLI optimization.
- Non-hillslope parser/scheduler architectural rewrites not required for scoped
  hot-path improvement.

## Deliverables
1. Benchmark plan + run evidence:
   - `artifacts/hillbench01-benchmark-report.md`
2. Optimization change/effect evidence:
   - `artifacts/hillbench01-optimization-report.md`
3. Required governance artifacts:
   - `artifacts/hillbench01-contract-implementation-evidence.md`
   - `artifacts/hillbench01-contract-test-implementation-evidence.md`
   - `artifacts/hillbench01-preimplementation-contract-gate.md`
   - `artifacts/hillbench01-implementation-and-test-evidence.md`
   - `artifacts/hillbench01-kernel-profile-compliance-checklist.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/hillbench01_disposition.md`
   - `artifacts/worker-handoff.md`
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
1. Amend canonical contract/index language only if optimization requires
   authority or contract text changes.
2. Implement contract-derived tests for any changed behavioral contract surface.
3. Record pre-implementation contract-gate evidence.
4. Apply production optimization edits.

## Autonomous Execution Intent (Required)
This package must execute end-to-end through disposition without requesting
additional user direction unless hard-blocked.

## Truthfulness Labeling Requirement
All evidence artifacts must explicitly label `Static:` and/or `Ran:`.

## Provenance and Authority Posture
- Canonical authority remains in `docs/specifications/science-contracts/contracts/SC-*.md`.
- Baseline comparator executable authority defaults to
  `/workdir/wepp-forest_260430_baseline/release/wepp_260430_hill` at pinned
  baseline commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- No heuristic/proxy physics substitutions are allowed in production runtime
  closure claims.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/crates/openwepp-runner/src/bin/openwepp-cli-hill.rs`
- `/workdir/openWEPP/crates/openwepp-runner/src/hillslope/mod.rs`
- `/workdir/openWEPP/tools/legacy_comparison_suite/README.md`
- `/workdir/wepp-forest_260430_baseline/release/wepp_260430_hill`

## Intended Write Set
- `docs/work-packages/20260528-hillbench01-hillslope-cli-release-benchmark-optimization-001/**`
- `docs/work-packages/README.md`
- `crates/openwepp-runner/src/bin/openwepp-cli-hill.rs` (if required)
- `crates/openwepp-runner/src/hillslope/mod.rs` (if required)
- `crates/openwepp-runner/tests/**` (if required)
- `tests/integration/**` (if required)

## Phase Plan
### Phase A - Intake and benchmark lane selection
- Confirm benchmark lanes for single-OFE and multi-OFE runs with compatible
  input identity and reproducible run directories.

### Phase B - Baseline release benchmarking
- Build release `openwepp-cli-hill`.
- Benchmark openWEPP and baseline `wepp_260430_hill` across selected lanes with
  repeat runs.
- Record timing summary and variance.

### Phase C - Contract/test gate readiness
- Determine whether planned optimizations require contract updates.
- If required, implement contract updates and contract-derived tests.
- Record pre-implementation contract gate.

### Phase D - Scoped optimization implementation
- Apply focused hillslope CLI/runtime hot-path optimizations.
- Maintain typed guards and output contract behavior.

### Phase E - Post-optimization benchmarking and validation
- Re-run benchmark lanes and produce before/after report.
- Run required validation gates and capture results:
  - `cargo fmt --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo test --workspace`
  - `cargo deny check`

### Phase F - Dual review, verification, disposition
- Complete review and verification artifacts.
- Publish GO/HOLD disposition with next-action handoff.

## Exit Criteria
- Release benchmark evidence exists for both single-OFE and multi-OFE lanes.
- openWEPP runtime comparison vs `wepp_260430_hill` is recorded with repeatable
  command provenance and summary stats.
- If optimization code changed, before/after evidence and tests show no contract
  regressions.
- Required validation gates are executed and captured truthfully.
- Dual review and dual verification artifacts are complete.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: local runtime performance/doc/test updates only; no credential or
  external service surface changes.
