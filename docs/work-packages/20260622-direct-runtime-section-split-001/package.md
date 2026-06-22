# Direct Runtime Mechanical Section Split

Status: complete.

Package type: Mechanical refactor ExecPlan.

Objective: split the large direct-runtime root module
`crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs` into ordered
textually included section files without changing runtime behavior, public API
shape, formulas, thresholds, guards, direct-runtime counters, or publication
row semantics.

Subagent authorization: this package explicitly authorizes spawning/delegating
to read-only reviewer and verifier subagents for mechanical-move review,
public API parity review, line-count governance review, and gate-evidence
audit. Expected outputs are compact Markdown findings summarized into
`artifacts/review-disposition.md`; subagents may not edit files.

## Rationale

`direct_runtime.rs` is `2922` lines, below the 3000-line hard block but still a
large WARN-band file. R6J and the preceding runner split identified the
direct-runtime publication-row construction and executor/audit tail as the next
mechanical debt before more direct publication growth. The safest refactor is a
textual section split with `include!` because it preserves the existing
`direct_runtime` module namespace and private item access.

## Scope

In scope:

- Move the direct-runtime core frame, publication, state/report, executor, and
  audit/error/helper sections into ordered included files under
  `crates/openwepp-hillslope-orchestrator/src/direct_runtime/`.
- Keep `direct_runtime.rs` as the constants, submodule export, and section
  wiring surface.
- Update direct-runtime source-scan tests to include the new included section
  files.
- Preserve all item names, signatures, visibility, public re-exports, and
  behavior.
- Update package artifacts and work-package catalog.

Out of scope:

- Process physics, formula, unit, threshold, guard, publication operand, or
  direct-runtime counter changes.
- Public API changes.
- Converting the included sections to child modules.
- Opportunistic cleanup beyond compile/format fallout from the move.

## Write Set

- `crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/00_core_frames.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/01_publication.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/02_state_reports.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/03_executor.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/04_audit_error_helpers.rs`
- `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs`
- `docs/work-packages/20260622-direct-runtime-section-split-001/**`
- `docs/work-packages/README.md`

## Section Map

- `00_core_frames.rs`: `DirectExecutorMode`, `DirectPhaseKind`,
  `DirectRunIdentity`, `DirectRunFrame`, `DirectLaneFrame`, `DirectDayFrame`,
  and `DirectPhaseView`.
- `01_publication.rs`: direct publication frame/calendar/day inputs, frost
  carry projection, run publication frame, publication day row, operand DTOs,
  and publication peak-runoff helper.
- `02_state_reports.rs`: phase plan, direct state/downstream/shadow DTOs, span
  reports, execution report, and publication execution DTO.
- `03_executor.rs`: execution counters, span macro, and `DirectFrameExecutor`.
- `04_audit_error_helpers.rs`: audit snapshot/functions/counters,
  `DirectRuntimeError`, display/source implementations, and shared validation
  helpers.

## Plan

1. Capture pre-refactor symbol and line-count evidence.
2. Mechanically move the declared sections into included files.
3. Add ordered `include!` lines to `direct_runtime.rs`.
4. Update source-scan tests for the included section files.
5. Run focused compile/tests and fix only mechanical fallout.
6. Run full closure gates:
   - `cargo fmt --check`
   - `cargo clippy --workspace --all-targets -- -D warnings`
   - `cargo test --workspace`
   - `cargo deny check`
7. Run `git diff --check` and scoped Markdown lint.
8. Complete artifacts: modularization plan, API parity, line-count governance,
   review disposition, verification, and handoff.

Package-required gates override any ambient test/validation-skip guidance.

## Acceptance Criteria

- `direct_runtime.rs` is below 2000 lines.
- Every new included section file is below 2000 lines.
- Public exported direct-runtime surface remains unchanged through
  `crates/openwepp-hillslope-orchestrator/src/lib.rs`.
- No intended semantic diff exists beyond item relocation, include wiring, and
  static-source test scan-list maintenance.
- Required closure gates pass and are recorded.
- No `.rs` file touched by this package is at or above 3000 lines.

## Progress

- [x] Scaffold package.
- [x] Capture pre-refactor evidence.
- [x] Move direct-runtime sections.
- [x] Validate focused checks.
- [x] Validate full closure gates.
- [x] Complete review, verification, line-count, and handoff artifacts.

## Outcomes

Final disposition: `COMPLETE-DIRECT-RUNTIME-SECTION-SPLIT`.

The direct-runtime root module was reduced from `2922` lines to `210` lines by
moving the existing section bodies into ordered `include!` files:

- `00_core_frames.rs`: `1001` lines.
- `01_publication.rs`: `454` lines.
- `02_state_reports.rs`: `433` lines.
- `03_executor.rs`: `391` lines.
- `04_audit_error_helpers.rs`: `434` lines.

The public `pub use direct_runtime::{...}` surface in
`crates/openwepp-hillslope-orchestrator/src/lib.rs` remains unchanged. The
direct-runtime compatibility-token source scan now reads the root module, all
five new included section files, and the existing direct process modules.

Closure gates passed on the current working tree:

- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `cargo deny check`
- `git diff --check`
- Scoped Markdown lint for this package and `docs/work-packages/README.md`
