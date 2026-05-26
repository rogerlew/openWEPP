# simimpl30 implementation and test evidence

Status: complete
Evidence mode: static+ran
Date: 2026-05-26

## Static
- No source-code implementation changes were required for SIMIMPL30.
- This package produced replay/disposition evidence and governance closure artifacts.

## Ran
- Replay evidence bundle: `artifacts/replay-run-20260526T125111Z/`
- Note: `replay-run-*` directories are intentionally git-ignored in this
  repository; key outcomes are summarized in tracked markdown artifacts.
  - `suite_p5_parquet.exit_code=1`
  - `suite_p5_parquet_passthrough.exit_code=1`
  - `semantic_direct_p5.exit_code=1`
  - `suite_p5_conversion_dat.exit_code=1`
- Direct semantic comparator failure root cause:
  - `RuntimeError: duplicate row key (1, 1, 1997) in parquet input /wc1/runs/ne/neither-liking/wepp/output/interchange/H.wat.parquet`
- Runner attempt failure root cause:
  - `CLIHILL-E-010 parse failure ... invalid TOML in /wc1/.../p5.run`
- Required workspace gates (bundle: `artifacts/gates-20260526T125552Z/`):
  - `cargo fmt --check` -> `0`
  - `cargo clippy --workspace --all-targets -- -D warnings` -> `0`
  - `cargo test --workspace` -> `0`
  - `cargo deny check` -> `0`
