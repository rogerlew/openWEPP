# CQR37: TESTGATE Relocated-Audit Verifier Coverage

Package: `20260724-cqr37-testgate-relocated-audit-001`
Status: `ACTIVE`
Owning qualification package:
`docs/work-packages/20260723-testgate-incompatible-recovery-receipt-001/package.md`
Target module: `crates/openwepp-gate-planner/src/pre_heavy.rs`
Target symbol: `validate_relocated_audit`
Quality dimension: `CRAP/cyclomatic-complexity`

## Objective

Reduce the retained actionable CRAP row for the public relocated-audit
verifier from `72` to at most `30` by exercising the real public entry point
under LLVM coverage. Decompose only if direct characterization does not close
the metric. Preserve every validation stage, error priority, fail-closed
decision, package-admission check, and sealed artifact-root binding.

## Authority And Rationale

Forest1 TESTGATE run `30053439151` passed the uninstrumented full-workspace
Nextest gate and then failed global adjudicated CRAP because the newly public
function measured `0%` coverage at cyclomatic complexity `8`. The retained
global report contains exactly one actionable production row, so this request
creates one module package and does not constitute a multi-package CQR batch.
No aggregate CQR admission scaffold is applicable. The owning TESTGATE recovery
package already authorizes the terminal changed-head qualification diff.

## Required Reading

- `AGENTS.md`
- `crates/AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/standards/AGENTS.md`
- `docs/standards/testing-and-gate-strategy.md`
- `docs/standards/mechanical-refactor-authoring-guide.md`
- `docs/standards/code-quality-refactor-authoring-guide.md`
- `docs/standards/module-test-enhancement-authoring-guide.md`
- `docs/decisions/0021-module-coverage-closure-thresholds.md`
- `crates/openwepp-gate-planner/src/pre_heavy.rs`
- `crates/openwepp-gate-planner/src/pre_heavy_coverage_tests.rs`

## Scope

In scope:

- direct characterization of `validate_relocated_audit`;
- coverage-only test changes in `pre_heavy_coverage_tests.rs`;
- behavior-preserving private extraction in `pre_heavy.rs` only if coverage
  alone leaves the target above CRAP `30`;
- package evidence and catalog updates.

Out of scope:

- changing validation order, status/reason grammar, schemas, trust policy,
  artifact identity, or accepted inputs;
- changing global CRAP thresholds or adjudications;
- unrelated cleanup or additional TESTGATE architecture.

## Intended Write Set

- `crates/openwepp-gate-planner/src/pre_heavy.rs`
- `crates/openwepp-gate-planner/src/pre_heavy_coverage_tests.rs`
- `docs/work-packages/20260724-cqr37-testgate-relocated-audit-001/**`
- `docs/work-packages/20260723-testgate-incompatible-recovery-receipt-001/package.md`
- `docs/work-packages/README.md`

## Implementation Intent

Assurance impact: `ORDINARY`.

The first implementation increment is test-only: call the public relocated
verifier with the existing fully sealed READY-audit fixture. Production
decomposition is conditional and prohibited unless the affected adjudicated
CRAP report remains above `30`. The terminal qualification remains a campaign
boundary owned by the active TESTGATE recovery package.

## Gate Order

1. Cheap deterministic checks: `git diff --check`, Markdown lint, focused
   verifier test, `cargo fmt --check`, and affected warnings-denied Clippy.
2. Current affected adjudicated CRAP for `openwepp-gate-planner`.
3. Only after affected CRAP passes, reconcile the exact terminal diff through
   the TESTGATE planner and pre-heavy audit.
4. Dispatch one changed-head TESTGATE run and monitor it to terminal outcome.

No full-workspace or global-quality rerun is permitted before steps 1 and 2
pass. The terminal TESTGATE run owns full/global campaign evidence once.

## Exit Criteria

- `validate_relocated_audit` is called directly by an instrumented test.
- Its affected adjudicated CRAP is at most `30`, with no actionable workspace
  row in the affected report.
- Focused test, formatting, affected Clippy, diff hygiene, and documentation
  gates pass.
- No production behavior changes, or any conditional decomposition is proven
  behavior-preserving with unchanged characterization tests.
- Exact-head TESTGATE qualification is dispatched only after local gates pass.
- Package evidence records commands, results, source identity, line-count
  governance, review, verification, and final disposition truthfully.

## Security Impact Gate

- `security_impact: moderate`
- `dedicated_security_review_required: no`
- Rationale: the verifier is fail-closed trust-boundary code, but the intended
  correction adds direct coverage without changing its behavior.
