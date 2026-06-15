# Verification Agent A

Status: complete.

Ran:

- `cargo fmt --check`: exit `0`
- `cargo clippy --workspace --all-targets -- -D warnings`: exit `0`
- `cargo test --workspace`: exit `0`
- `cargo deny check`: exit `0`

Verified:

- current highest target-file CRAP:
  `HillslopeRuntimeInputError::soil_core_code` at `14.0478515625`;
- every target-file function is CRAP `<= 30`;
- target-file coverage stayed `497/515` lines and `20/20` functions;
- Gate Evidence Non-Deferral is satisfied for Rust and metric gates already
  run.

Disposition: verified.
