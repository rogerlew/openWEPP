# Contract-Test Implementation Evidence

Status: `PASS`.

Evidence mode: `Static + Ran`.

Immutable source: `5e353b8c8bc56c9d36301743119dbe1c76a0e9a0`.

The registered Stage 3 contract target now binds v9/v131 within normative
invariant, guard, obligation, addendum, alias, unit, boundary, and exposure
sections. Independent test-local arithmetic vectors do not import producer or
future analyzer helpers. They cover no-cap sublimation, active-ice truncation,
deposition, wrong producer magnitude/direction, simultaneous transfer, Q
missing transfer, S/F N/A versus numeric zero, operator-order cold/melt and
unallocated energy, sublimation-reserved melt capacity, endpoint closure, and
an endpoint-preserving vapor/melt alias that independent operands reject.

Ran at the immutable source:

- focused Nextest across all three affected binaries: `27 passed`, `0 skipped`;
- `cargo fmt --all --check`: `PASS`;
- warnings-denied Clippy across all three affected binaries: `PASS`;
- strict Binding Exposure, exact Markdown lint, protocol JSON validation, and
  `git diff --check`: `PASS`.

Independent consumer re-review changed from `HOLD` to `GO` after the literal
anti-tautology vectors were added.
