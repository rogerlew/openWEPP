# 20260608-refactor020-openwepp-hillslope-orchestrator-runtime-inputs-08-tests-mechanical-modularization-001

## Status
- state: complete
- date: 2026-06-08
- timezone: UTC

## Objective
Mechanically modularize
`crates/openwepp-hillslope-orchestrator/src/runtime_inputs/08_tests.rs`
into cohesive submodules under `runtime_inputs/08_tests/` while preserving test
intent, guard semantics, and all observable behavior.

## Why This Package Exists
`crates/openwepp-hillslope-orchestrator/src/runtime_inputs/08_tests.rs` is a
2559-line mixed test module, above the `.rs` structural threshold that requires
mechanical decomposition. The file combines unrelated test families and runtime
input seam fixtures, which increases merge conflict risk and obscures test
ownership. A focused refactor lowers review risk and makes future test
maintenance safer.

## Scope
### Included
- Move test blocks out of `runtime_inputs/08_tests.rs` into cohesive files under
a dedicated `runtime_inputs/08_tests/` module subtree.
- Replace `08_tests.rs` with a thin facading/wiring module that preserves module
  visibility and compile-time interfaces required by `runtime_inputs/mod.rs`.
- Preserve all production symbols and test assertion intent; only mechanical
  reorganization is permitted.
- Capture pre/post line-count posture and API/parity evidence.

### Explicitly Out of Scope
- Any new physics/math or runtime behavior changes.
- Contract-authority rewrites or guard semantic changes.
- Edits outside the declared write-set without explicit scope expansion.
- Canonicalize-and-proceed handling for invalid domain state.

## Deliverables
1. Mechanical modularization implementation with preserved behavior intent:
   - `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/08_tests.rs`
   - `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/08_tests/*.rs`
2. Work-package evidence:
   - `artifacts/required-reading-map.md`
   - `artifacts/refactor020-modularization-plan-report.md`
   - `artifacts/refactor020-public-api-surface-parity-report.md`
   - `artifacts/refactor020-contract-implementation-evidence.md`
   - `artifacts/refactor020-contract-test-implementation-evidence.md`
   - `artifacts/refactor020-preimplementation-contract-gate.md`
   - `artifacts/refactor020-implementation-and-test-evidence.md`
   - `artifacts/refactor020-kernel-profile-compliance-checklist.md`
   - `artifacts/refactor020-line-count-governance-checklist.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/refactor020_disposition.md`
   - `artifacts/worker-handoff.md`
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Autonomous Execution Intent (Required)
This package is execution-ready and proceeds end-to-end through disposition
without additional user direction unless hard-blocked.

## Truthfulness Labeling Requirement
All evidence artifacts must explicitly label `Static:` and/or `Ran:` sections.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/work-packages/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/standards/mechanical-refactor-authoring-guide.md`
- `/workdir/openWEPP/docs/standards/kernel-work-package-preparation.md`
- `/workdir/openWEPP/docs/prompt_templates/mechanical-refactor-kickoff-template.md`
- `/workdir/openWEPP/docs/prompt_templates/required-reading-map-template.md`
- `/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/Cargo.toml`
- `/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs/mod.rs`
- `/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs/08_tests.rs`

## Intended Write Set
- `docs/work-packages/20260608-refactor020-openwepp-hillslope-orchestrator-runtime-inputs-08-tests-mechanical-modularization-001/**`
- `docs/work-packages/README.md`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/08_tests.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/08_tests/*.rs`

## Phase Plan
### Phase A - Intake, Sizing, and Surface Freeze
- Capture pre-refactor line count and symbol/class inventory for `08_tests.rs`.
- Confirm wiring expectations in `runtime_inputs/mod.rs`.

### Phase B - Mechanical Extraction
- Split the file into cohesive test modules under `08_tests/`.
- Keep test helper visibility and imports mechanically stable.
- Keep module wiring minimal in `08_tests.rs`.

### Phase C - Validation and Evidence
- Run required closure commands and record outcomes.
- Confirm exported test/public helper surface parity.

### Phase D - Disposition
- Publish complete evidence and handoff package to disposition-ready state.

## Contract-First Sequencing Requirement
This is mechanical scaffolding only. If any scope expansion introduces contract
authority edits, the package must follow the canonical sequence:
1. canonical contract amendments
2. contract-derived tests
3. pre-implementation contract gate
4. production edits.

Current scope remained mechanical and contract-neutral.

## Exit Criteria
- `08_tests.rs` is decomposed into `runtime_inputs/08_tests/*.rs` with API and
test intent preserved.
- Required closure gates pass:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test -p openwepp-hillslope-orchestrator --tests`
  4. `cargo test --workspace`
  5. `cargo deny check`
- Required artifacts are complete with truthful `Static`/`Ran` evidence.
- Review findings are dispositioned and line-count governance is documented.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: internal test-module organization refactor in runtime-inputs domain, with
  no interface or trust-boundary change.

## Completion
- 2026-06-08: package executed to disposition with mechanical-only modularization and
  all required gates passing.
