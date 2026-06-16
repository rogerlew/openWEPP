# CQR27 - Management Parser Complexity Closure

Status: complete-with-warnings

Package type: code-quality refactor

## Objective

Close the current CQR27 target in
`crates/openwepp-input-contract/src/parsers/management.rs`, originally tracked
as rank 21 with snapshot CRAP `291`, CC `35`, and coverage `41%`, so the live
target function and any newly extracted helpers have CRAP `<= 30`.

## Rationale

The management parser is an external input compatibility boundary. The CQR27
package must reduce parser complexity without changing accepted grammar,
strict/compatibility behavior, error IDs, diagnostics, defaults, parser output
shape, or runtime projection compatibility.

## Quality Dimension

- Dimension: cyclomatic-complexity / CRAP burn-down.
- Closure metric: current target function and any newly extracted helpers have
  CRAP `<= 30` using `cargo crap` against package LCOV.
- Supporting metrics: before/after LCOV, before/after CRAP, target identity,
  line counts, parser surface parity, behavior equivalence, dual review, dual
  verification, and full gates.

## Included Scope

- Fresh before/after LCOV and CRAP capture for the target file.
- Characterization coverage before production refactor where current tests do
  not pin selected parser branches.
- Behavior-preserving private helper extraction for the scoped target.
- Package catalog update in `docs/work-packages/README.md`.
- Package artifacts, dual reviews, dual verification, disposition, worker
  handoff, package commit/push, and tracker update after push.

## Excluded Scope / Protected Boundaries

- No parser grammar changes.
- No public API, struct, enum, error ID, diagnostic text, field name, default,
  strict/compatibility mode, or accepted/rejected fixture behavior changes.
- No runtime symbol, unit, alias, or projection compatibility changes.
- No dependency changes.
- No unrelated parser cleanup outside the scoped metric target.

## Acceptance Gate

This package is parser-boundary-affecting. The acceptance gate is strict
behavior preservation:

- preserve public and crate-visible signatures;
- preserve accepted management file grammar and section dispatch;
- preserve strict and compatibility mode decisions;
- preserve error variants, error IDs, and display text;
- preserve defaults, optional field handling, output ordering, and parsed data
  shape;
- stop and hold if closure requires changing parser authority.

## Intended Write Set

- `docs/work-packages/20260615-cqr27-management-parser-complexity-001/**`
- `docs/work-packages/README.md`
- `docs/work-packages/cqr-burndown-execplan.md`
- `crates/openwepp-input-contract/src/parsers/management.rs`
- Focused tests only if characterization coverage is required before refactor.

## Dependencies

- `AGENTS.md`
- `crates/AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/work-packages/README.md`
- `docs/work-packages/cqr-burndown-execplan.md`
- `docs/standards/AGENTS.md`
- `docs/standards/mechanical-refactor-authoring-guide.md`
- `docs/standards/code-quality-refactor-authoring-guide.md`
- `docs/decisions/0021-module-coverage-closure-thresholds.md`
- `docs/codex_exec_plans.md`

## Phase Plan

### Phase A - Intake, Baseline, and Surface Freeze

- Capture target-file line count and suppression census.
- Generate before LCOV and before `cargo-crap` JSON.
- Identify the live target function from current metrics.
- Record protected parser API, error, grammar, and output surfaces.

### Phase B - Precondition and Focused Characterization

- Run existing focused parser tests before production edits.
- Add characterization before refactor if current tests do not freeze the
  selected branches.

### Phase C - Behavior-Preserving Decomposition

- Extract cohesive private helpers for the scoped target function.
- Preserve parser branch order and all error/default behavior.
- Keep the public API and parsed output shape unchanged.

### Phase D - Validation and Evidence

- Re-run LCOV and `cargo-crap`; target and extracted helpers must be `<= 30`.
- Run the required closure gates:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`
  5. `markdown-doc lint --path docs/work-packages/README.md --path docs/work-packages/20260615-cqr27-management-parser-complexity-001 --format json`
  6. `git diff --check`

### Phase E - Review, Verification, Disposition, Commit, and Push

- Complete dual local review artifacts.
- Complete dual local verification artifacts.
- Complete disposition and worker handoff.
- Commit and push the package write set, then update the CQR ExecPlan tracker.

## Exit Criteria

- Current CQR27 target function and any newly extracted helpers have CRAP
  `<= 30`.
- Target-file coverage is not regressed relative to the package baseline.
- Parser grammar, public API, error IDs, diagnostics, defaults,
  strict/compatibility behavior, and parsed output shape are unchanged.
- Required closure gates are run and recorded.
- No touched non-exempt `.rs` file is at or above `3000` lines.
- No review finding remains undispositioned.
- Package commit and tracker update are pushed before the ExecPlan row is
  checked off.

## Review and Verification

This package requires dual independent local review and dual independent local
verification before disposition.

## Closeout Summary

Ran: CQR27 reduced the live target
`parse_yearly_annual_fallow` from CRAP `290.7314769280208`, CC `35.0`, and
coverage `40.67796610169492%` to CRAP `4.0`, CC `4.0`, and coverage
`100.0%`.

Ran: newly extracted helpers are also below the closure threshold:

| Helper | CRAP |
| --- | ---: |
| `parse_yearly_annual_fallow_header` | `5.0` |
| `parse_yearly_annual_extension` | `19.045125` |
| `parse_yearly_annual_cut_records` | `6.0` |
| `parse_yearly_annual_cut_entry` | `6.0` |

Static: the production change is private helper extraction only. Public parser
types and functions, error variants, error IDs, diagnostic strings, field
names, strict/compatibility mode behavior, accepted record order, defaults,
and parsed output shape are unchanged.

Ran: target-file LCOV improved from lines `749/1114` (`67.24%`) and functions
`40/49` (`81.63%`) to lines `816/1147` (`71.14%`) and functions `45/54`
(`83.33%`).

Warnings: `cargo crap` emitted LCOV source-map warnings for 126 workspace
test/support source files, while the target file was represented in LCOV.
Non-target management parser rows over CRAP `30` remain for later ranked CQR
work; CQR27 target and helpers are closed.
