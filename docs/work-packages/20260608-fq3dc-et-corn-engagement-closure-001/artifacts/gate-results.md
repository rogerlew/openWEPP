# Gate Results

Status: complete

Evidence mode: Ran.

## Final Gate

Ran:

```sh
cargo fmt --check && git diff --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test -p openwepp --test hphys0320_stmtim_start_time_source_line_contract && cargo test --workspace && cargo deny check
```

Result: passed.

Notes:

- `cargo test -p openwepp --test hphys0320_stmtim_start_time_source_line_contract`
  passed: `3 passed`.
- `cargo test --workspace` passed.
- `cargo deny check` passed with existing warnings for duplicate crate entries
  (`getrandom`, `hashbrown`, `twox-hash`) and unused license allowances
  (`ISC`, `Unicode-DFS-2016`); final status was
  `advisories ok, bans ok, licenses ok, sources ok`.

## Focused Validation

Ran:

- `cargo test -p openwepp-runner fq3dc_ -- --nocapture`: passed.
- `cargo test -p openwepp-hillslope-orchestrator fq3dc_wb15_accepts_finite_non_negative_corn_vdmt_above_legacy_cap -- --nocapture`: passed.
- 36-prefix Corn population release run: passed with zero missing/failed
  prefixes and nonzero `Ep`/`Interception` on every Corn prefix.
