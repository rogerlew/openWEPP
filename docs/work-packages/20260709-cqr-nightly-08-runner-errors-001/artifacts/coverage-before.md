# Coverage Before

Evidence label: Static/Ran.

Status: `SCAFFOLDED`

Target module:
`crates/openwepp-runner/src/errors.rs`

Baseline LCOV provenance:

- `cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path /tmp/openwepp-cqr-nightly.lcov` -
  exit `0`.

LCOV line coverage before:

- `LF:267`
- `LH:56`
- Line coverage: `20.973782771535582%`

ADR-0021 tier:

- `glue`

Disposition:

- Baseline target line coverage is below the glue-tier `85%` threshold.
- Characterization is expected before or alongside decomposition.
