# Public API Parity

Evidence class: Static and Ran.

## Public Surface

The crate-level export remains through the existing direct-runtime module:

- `crates/openwepp-hillslope-orchestrator/src/lib.rs:56` still contains
  `pub use direct_runtime::{...};`.
- No direct-runtime export list item was added, removed, or renamed.
- The refactor did not convert the moved sections to child modules, so public
  item paths remain `openwepp_hillslope_orchestrator::<item>` through the
  existing crate re-export and `direct_runtime::<item>` inside the crate.

## Moved Public Items

Static symbol scan after the split found the major public runtime symbols in
the expected included files:

- `00_core_frames.rs`: `DirectExecutorMode`, `DirectPhaseKind`,
  `DirectRunIdentity`, `DirectRunFrame`, `DirectLaneFrame`, `DirectDayFrame`.
- `01_publication.rs`: `DirectPublicationFrame`,
  `DirectRunPublicationFrame`, `DirectPublicationDayRow`.
- `02_state_reports.rs`: `DirectExecutionReport`,
  `DirectPublicationExecution`.
- `03_executor.rs`: `DirectFrameExecutor`.
- `04_audit_error_helpers.rs`: `DirectRuntimeAuditSnapshot`,
  `DirectRuntimeError`.

## Compile and Test Evidence

The public API remained usable under compile and focused direct-runtime tests:

- `cargo check -p openwepp-hillslope-orchestrator`: passed.
- `cargo test -p openwepp-hillslope-orchestrator direct_runtime -- --nocapture`:
  `61 passed; 0 failed; 174 filtered out`.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.

## Semantic-Diff Disposition

No semantic change was intended or accepted. The code edits are limited to:

- Moving contiguous source ranges into textually included section files.
- Adding ordered `include!` lines to the retained root module.
- Updating a static source-scan test to read the new section files.
- Running `cargo fmt`, which removed one extra blank line in the retained root
  module.
- Removing trailing EOF blank lines from four newly added section files after
  `git diff --cached --check` exposed them.
