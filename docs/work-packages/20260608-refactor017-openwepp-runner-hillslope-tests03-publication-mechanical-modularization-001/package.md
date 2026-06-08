# 20260608-refactor017-openwepp-runner-hillslope-tests03-publication-mechanical-modularization-001

## Status
- state: complete
- date: 2026-06-08
- timezone: UTC
- closeout: 2026-06-08T21:19:14Z

## Objective
Mechanically modularize
`crates/openwepp-runner/src/hillslope/tests03/publication.rs` into cohesive
test modules while preserving test intent, runtime/API guard expectations, and
existing contract/test outcomes.

## Why This Package Exists
`crates/openwepp-runner/src/hillslope/tests03/publication.rs` is currently a
single mixed-concern test file at 2079 lines (49 `#[test]` functions),
exceeding the `.rs` 2000+ warning threshold for required refactor. The file was
created by REFACTOR008 as part of the `tests03/` subtree split of
`03_tests.rs` and has since grown past the threshold. This package reduces
review and maintenance risk by splitting tests by publication concern without
changing production behavior or weakening assertions.

## Scope
### Included
- Mechanical movement of tests from
  `crates/openwepp-runner/src/hillslope/tests03/publication.rs` into dedicated
  test module files under a `tests03/publication/` subtree.
- Conversion of `tests03/publication.rs` into a thin module-wiring entrypoint
  that `include!`s the split test files (preserving the existing
  `mod publication { include!("tests03/publication.rs"); }` mount in
  `03_tests.rs`).
- Module-aware updates to layout-coupled assertions only when required to
  preserve test fidelity under the new module residency.
- Validation and evidence updates proving no intended behavior changes.

### Explicitly Out of Scope
- New process-physics logic or contract-authority behavior changes.
- Threshold/guard loosening, assertion weakening, or canonicalize-and-proceed
  handling.
- Production (`src/`) behavior changes outside the test seam.
- Public API changes unless explicitly declared and approved.

## Deliverables
1. Mechanical modularization implementation with preserved test intent:
   - `crates/openwepp-runner/src/hillslope/tests03/publication.rs`
   - `crates/openwepp-runner/src/hillslope/tests03/publication/*.rs`
2. Test updates only when required by module-residency assumptions.
3. Work-package artifacts:
   - `artifacts/required-reading-map.md`
   - `artifacts/refactor017-modularization-plan-report.md`
   - `artifacts/refactor017-public-api-surface-parity-report.md`
   - `artifacts/refactor017-contract-implementation-evidence.md`
   - `artifacts/refactor017-contract-test-implementation-evidence.md`
   - `artifacts/refactor017-preimplementation-contract-gate.md`
   - `artifacts/refactor017-implementation-and-test-evidence.md`
   - `artifacts/refactor017-kernel-profile-compliance-checklist.md`
   - `artifacts/refactor017-line-count-governance-checklist.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/refactor017_disposition.md`
   - `artifacts/worker-handoff.md`
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Autonomous Execution Intent (Required)
This package is execution-ready and must proceed end-to-end through final
disposition without user intervention unless hard-blocked.

## Truthfulness Labeling Requirement
All evidence artifacts must explicitly label `Static:` and/or `Ran:` sections.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/work-packages/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/standards/mechanical-refactor-authoring-guide.md`
- `/workdir/openWEPP/docs/prompt_templates/mechanical-refactor-kickoff-template.md`
- `/workdir/openWEPP/docs/prompt_templates/required-reading-map-template.md`
- `/workdir/openWEPP/docs/standards/kernel-work-package-preparation.md`
- `/workdir/openWEPP/crates/openwepp-runner/src/hillslope/03_tests.rs`
- `/workdir/openWEPP/crates/openwepp-runner/src/hillslope/tests03/publication.rs`

## Intended Write Set
- `docs/work-packages/20260608-refactor017-openwepp-runner-hillslope-tests03-publication-mechanical-modularization-001/**`
- `docs/work-packages/README.md`
- `crates/openwepp-runner/src/hillslope/tests03/publication.rs`
- `crates/openwepp-runner/src/hillslope/tests03/publication/*.rs`

## Phase Plan
### Phase A - Intake, Sizing, and Seam Freeze
- Capture pre-refactor test inventory (`#[test]` roster) and line-count
  baseline.
- Freeze split boundaries by publication concern (e.g. WB13 guards, routed-melt
  / rainfall-melt publication, scheduler/PL activation, WB11 seed
  initialization, WB19/WB12 withdrawal & reconciliation, WB16 producer alpha,
  breakpoint seed).

### Phase B - Mechanical Extraction
- Move cohesive test groups into dedicated module files under
  `tests03/publication/`.
- Preserve `use super::*;` resolution, test names, and assertion bodies.
- Reduce `tests03/publication.rs` to module wiring that `include!`s the splits.

### Phase C - Validation and Evidence
- Run required validation gates and record truthful outputs.
- Complete dual review and dual verification artifacts.

### Phase D - Disposition
- Publish final disposition, parity result, and residual-risk ownership.

## Contract-First Sequencing Requirement
Contract-first sequence remains mandatory for kernel-adjacent package posture:
1. canonical contract amendments,
2. contract-derived tests,
3. pre-implementation contract gate,
4. production edits.

For this package, no canonical contract amendments are expected because this is
mechanical test decomposition only with no intended behavior changes. Artifacts
must explicitly record this determination before edits.

## Exit Criteria
- `tests03/publication.rs` decomposition is complete with preserved test intent
  and assertion strength.
- `tests03/publication.rs` is reduced below 2000 lines and each new module file
  stays within `.rs` line-count governance.
- All 49 tests remain present and pass under their new module residency.
- Required gates pass:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test -p openwepp-runner --tests`
  4. `cargo test --workspace`
  5. `cargo deny check`
- Gate commands above are mandatory execution requirements; omission is only
  permitted when a hard blocker is recorded with command-level evidence.
- Required artifacts are complete with truthful `Static`/`Ran` evidence.
- Review findings are fully dispositioned and line-count governance is
  documented.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: internal Rust test-module organization refactor with no new
  external interface and no production behavior change.
