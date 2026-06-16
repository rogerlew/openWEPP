# CQR29 Implementation and Test Evidence

Ran: implementation changed
`crates/openwepp-hillslope-orchestrator/src/hydrology/02_guard_errors.rs` by
splitting the prior long `Display::fmt` matcher into private display-part
helpers:

- `display_parts`
- `phase_display_parts`
- `erod13_display_parts`
- `erod14_display_parts`
- `erod18_display_parts`
- `HydrologyGuardErrorDisplayParts::fmt_with_code`

Ran: characterization changed
`crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/hydrology.rs` and
`crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/mod.rs` to cover
all guard-error variants through the public API.

Ran:

```text
cargo test -p openwepp-hillslope-orchestrator cqr29
cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path docs/work-packages/20260615-cqr29-guard-errors-complexity-001/artifacts/lcov_after.info
cargo crap --workspace --lcov docs/work-packages/20260615-cqr29-guard-errors-complexity-001/artifacts/lcov_after.info --min 0 --format json --output docs/work-packages/20260615-cqr29-guard-errors-complexity-001/artifacts/crap_after.json
```

Result: final target CRAP `1.0`; max extracted-helper CRAP
`8.000751314800901`.
