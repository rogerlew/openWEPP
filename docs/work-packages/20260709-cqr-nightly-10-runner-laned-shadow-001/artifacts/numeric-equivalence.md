# Numeric And Output Equivalence

Evidence label: Static/Ran.

Status: `EXECUTED`

Conclusion:

- PASS for this CQR package.

Evidence:

- No production code changed; implementation is `#[cfg(test)]` only.
- `cargo test -p openwepp-runner laned_shadow --lib -- --nocapture`
  - PASS, `15` passed.
- `OPENWEPP_LANED_SHADOW_PROFILE=1 cargo test -p openwepp-runner diagnostic_profile_helpers_cover_opt_in_surfaces_without_public_outputs --lib -- --nocapture`
  - PASS, `1` passed.
- `cargo nextest run -p openwepp-runner laned_shadow`
  - PASS, `15` passed.
- `cargo clippy -p openwepp-runner --lib --tests -- -D warnings`
  - PASS.

Behavior identity basis:

- The new tests construct the same public `DirectPublicationDayRow` and
  `LanedShadowLaneDayOperands` surfaces that production consumes.
- The tests assert fail-closed guard messages, zero-source day accounting,
  positive uniform-shape classification with and without routed-melt class
  counting, missing dynamic operand fail-closed behavior, source
  reconstruction, and diagnostic profile helper behavior.
- Protected public outputs and manifest semantics are unaffected because no
  production code, serializer, manifest builder, selector, or fixture changed.

Higher-cost output oracle:

- Existing `tests/integration/laned_shadow_h2637.rs` retains the native shadow
  HBP/parquet byte-identity test. It was not needed for this test-only CQR diff.
