# 20260607-refactor009-openwepp-runner-hillslope-00-runner-intake-and-lane-setup-mechanical-modularization-001

## Status
- state: complete
- date: 2026-06-07
- timezone: UTC
- completed_utc: 2026-06-07T00:00:00Z

## Objective
Mechanically modularize `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs`
into cohesive intake/lane-setup modules while preserving behavior, public API
surface, typed guard semantics, and existing contract/test outcomes.

## Why This Package Exists
`crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs` is
currently a mixed-concern file at 2533 lines, which exceeds the `.rs` warning
threshold and increases review/maintenance risk. This package creates explicit
seams for intake parsing, mode/lane setup, and manifest/provenance construction
without introducing intended runtime semantic drift.

## Scope
### Included
- Mechanical movement of code from
  `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs`
  into dedicated module files under an intake/lane setup subtree.
- Conversion of `00_runner_intake_and_lane_setup.rs` into a thin
  wiring/facade module preserving consumed symbol visibility.
- Layout-coupled test updates only when required to preserve contract
  assertion fidelity under module decomposition.
- Validation and evidence updates proving no intended behavior changes.

### Explicitly Out of Scope
- New process-physics logic or scheduler behavior changes.
- Guard loosening, fallback wrappers, or canonicalize-and-proceed handling.
- Contract authority rewrites not required for mechanical decomposition.

## Deliverables
1. Mechanical modularization implementation with preserved behavior and API
   parity:
   - `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs`
   - `crates/openwepp-runner/src/hillslope/intake_lane_setup/*.rs`
2. Test updates for layout-coupled source-residency assertions only when
   required.
3. Work-package artifacts:
   - `artifacts/refactor009-modularization-plan-report.md`
   - `artifacts/refactor009-public-api-surface-parity-report.md`
   - `artifacts/refactor009-contract-implementation-evidence.md`
   - `artifacts/refactor009-contract-test-implementation-evidence.md`
   - `artifacts/refactor009-preimplementation-contract-gate.md`
   - `artifacts/refactor009-implementation-and-test-evidence.md`
   - `artifacts/refactor009-kernel-profile-compliance-checklist.md`
   - `artifacts/refactor009-line-count-governance-checklist.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/refactor009_disposition.md`
   - `artifacts/worker-handoff.md`
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Autonomous Execution Intent (Required)
This package is execution-ready and must proceed end-to-end through
final disposition without user intervention unless hard-blocked.

## Truthfulness Labeling Requirement
All evidence artifacts must explicitly label `Static:` and/or `Ran:` sections.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/standards/mechanical-refactor-authoring-guide.md`
- `/workdir/openWEPP/docs/prompt_templates/mechanical-refactor-kickoff-template.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/crates/openwepp-runner/Cargo.toml`
- `/workdir/openWEPP/crates/openwepp-runner/src/hillslope/mod.rs`
- `/workdir/openWEPP/crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs`
- `/workdir/openWEPP/crates/openwepp-runner/src/bin/open_wepp_runner.rs`
- `/workdir/openWEPP/crates/openwepp-runner/src/bin/openwepp-cli-hill.rs`
- `/workdir/openWEPP/tests/integration/cli03_runner_contract_derived_tests.rs`
- `/workdir/openWEPP/tests/integration/hphys0318_stmtim_control_surface_instrumentation_contract.rs`

## Intended Write Set
- `docs/work-packages/20260607-refactor009-openwepp-runner-hillslope-00-runner-intake-and-lane-setup-mechanical-modularization-001/**`
- `docs/work-packages/README.md`
- `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs`
- `crates/openwepp-runner/src/hillslope/intake_lane_setup/*.rs`
- `tests/integration/cli03_runner_contract_derived_tests.rs` (if required)
- `tests/integration/hphys*.rs` only when layout-coupled assertions require
  module-aware source aggregation updates

## Phase Plan
### Phase A - Intake, Sizing, and API Freeze
- Capture pre-refactor symbol inventory and callsites.
- Capture current line-count baseline and split target plan.

### Phase B - Mechanical Extraction by Concern
- Extract cohesive intake/lane setup seams into dedicated modules.
- Preserve signatures/visibility and typed guard behavior.

### Phase C - Test Surface Stabilization
- Update only brittle layout-coupled tests to module-aware behavior checks.

### Phase D - Validation and Evidence
- Run required validation gates and record truthful outputs.
- Complete dual review and dual verification artifacts.

### Phase E - Disposition
- Publish final disposition, parity conclusion, and residual-risk ownership.

## Contract-First Sequencing Requirement
Contract-first sequence remains mandatory for kernel-adjacent package posture:
1. canonical contract amendments,
2. contract-derived tests,
3. pre-implementation contract gate,
4. production edits.

For this package, no canonical contract amendments are expected because the
change is mechanical decomposition only with no intended behavior changes.
Artifacts must explicitly record this determination before production edits.

## Exit Criteria
- Intake/lane-setup decomposition is complete with preserved API surface.
- `00_runner_intake_and_lane_setup.rs` is reduced below 2000 lines.
- Required gates pass:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test -p openwepp-runner --tests`
  4. `cargo test --workspace`
  5. `cargo deny check`
- Required artifacts are complete with truthful `Static`/`Ran` evidence.
- Review findings are fully dispositioned and line-count governance is
  documented.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: internal Rust module organization refactor with no new external
  interface.
