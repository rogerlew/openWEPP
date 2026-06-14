# Implementation Test Evidence

Status: W-A executed

Evidence mode: Ran + Static

No production implementation was performed in W-A.

Ran:

- `target/debug/openwepp-cli-watershed --run-dir /tmp/openwepp_wshed01_wa/watershed/run --run-file case.run --output-dir /tmp/openwepp_wshed01_wa/watershed/output --policy compat --legacy-sidecar-discovery`

Observed:

- Exit code `1`.
- `CLIWAT-E-010` wrapping `IMP-E-004` on `pw0.imp` line 2, `jpond=0`.
- `0` output files under `/tmp/openwepp_wshed01_wa/watershed/output`.

Not run:

- `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo test --workspace`, and `cargo deny check`. W-A made no Rust edits and
  did not claim production closure.

Static evidence:

- Parser defect: `watershed_impoundment.rs:581-588`.
- CLI failure wrapper: `openwepp-cli-watershed.rs:239-254`.
- Output writer path not reached: `openwepp-cli-watershed.rs:476-497`.
