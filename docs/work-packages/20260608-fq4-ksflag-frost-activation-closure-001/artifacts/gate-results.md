# Gate Results

Status: complete

Evidence mode: Ran.

## Focused Gates

Ran:

- `cargo fmt --check`: passed.
- `cargo test --test clim06_frost_frozen_soil_kernel_contract -- --nocapture`:
  passed (`12 passed`).
- `cargo test -p openwepp-runner simimpl04_contract_requires_wepp_ui_requested_effective_lane_closure_manifest -- --nocapture`:
  passed.
- `cargo test -p openwepp --test hphys0319_fixed_baseline_stmtim_observe_contract -- --nocapture`:
  passed (`5 passed`).
- `cargo test -p openwepp --test hphys0320_stmtim_start_time_source_line_contract -- --nocapture`:
  passed (`3 passed`).
- `wctl doc-lint --path docs/work-packages/20260608-fq4-ksflag-frost-activation-closure-001 --path docs/work-packages/README.md --path docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`:
  passed (`1 files validated, 0 errors, 0 warnings` under current wrapper
  include rules).

## Final Gate

Status: passed.

Ran:

```sh
cargo fmt --check && git diff --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace && cargo deny check
```

Result: passed, process exit code 0.

Notes:

- Initial final-gate attempt stopped on `clippy::manual-let-else` in
  `resolve_active_frost_coupling`; fixed by rewriting the optional `wintRed`
  match as `let Some(wint_red) = ... else { return Ok(false); };`.
- Final rerun passed `cargo fmt --check`, `git diff --check`,
  `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo test --workspace`, and `cargo deny check`.
- `cargo deny check` emitted existing warning-class output for duplicate
  transitive `getrandom`, `hashbrown`, and `twox-hash` lock entries, plus
  unmatched `ISC` and `Unicode-DFS-2016` license allowances in `deny.toml`;
  policy summary remained `advisories ok, bans ok, licenses ok, sources ok`.
