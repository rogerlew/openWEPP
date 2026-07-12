# Validation

Ran:

- `cargo nextest run -p openwepp-runner cqr_growth_authority` — `4/4` passed.
- `cargo clippy -p openwepp-runner --tests -- -D warnings` — PASS.
- `cargo fmt --check`; `git diff --check` — PASS.
- Shared runner profile — `108/108` passed in `80.88s`.
- `bbb = NaN` and `jdharv = +Infinity` now return typed errors containing the
  exact symbol; finite/missing/fractional/range and schedule-precedence vectors
  remain green.

No growth formula, finite-value threshold, schema, authority order, or consumer
path changed.
