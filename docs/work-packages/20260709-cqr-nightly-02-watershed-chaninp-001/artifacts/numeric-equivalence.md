# Numeric Equivalence

Status: `COMPLETE`

Scope:

- WS12 impoundment coefficient derivation in `chaninp.rs`.
- Active branch projection helpers for drop spillway, culvert-like, rockfill,
  emergency, filter, and perforated riser payloads.
- Private helper math for riser sampling/regression, 5x5 quartic fitting,
  piecewise stage-discharge interpolation, and power-law coefficient fitting.

Evidence:

- The implementation moved existing arithmetic into helpers without changing
  expression grouping or constants.
- The active-projection sum still evaluates in the original order:
  `culvert[0]`, `culvert[1]`, rockfill, emergency, filter.
- `active_impoundment_projection_covers_all_function_families` asserts expected
  coefficient values for the active projection, including drop-spillway,
  aggregate culvert-like, riser, rockfill, emergency, filter, and
  representative f04/f10/f11/f12/f14/f15 family coefficient paths.
- Riser sampling/regression and quartic solver tests use tolerance comparisons
  for floating-point results; exact array equality on `f64` was removed after
  clippy identified it.
- Guard tests assert the expected typed error variants, symbols, and rule text
  for representative non-finite and out-of-domain paths.

Ran:

- `cargo nextest run -p openwepp-watershed-orchestrator chaninp`
  - Exit code: `0`
  - Current result: `13 tests run: 13 passed, 9 skipped`
- `cargo clippy --workspace --all-targets -- -D warnings`
  - Exit code: `0`

Disposition:

- No numeric deltas are introduced intentionally.
- No new bounded canonicalization, fallback wrapper, or surrogate process math
  was added.
