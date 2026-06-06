# Implementation Test Evidence

Status: complete

Evidence mode: Ran

Ran:

1. Climate precondition audit:
   `.venv/bin/python` parsed the current text CLI and parquet climate artifacts,
   recomputed `sunmap.r3`, and verified zero `rad > sunmap.r3` rows.
2. Release build:
   `cargo build --release -p openwepp-runner --bin openwepp-cli-hill`
   passed.
3. Wrapper generation:
   `/tmp/wbval04_rocky_mountain_20260606T000000Z/generated_runfiles/p1.toml`
   through `p22.toml` were generated with authoritative `/wc1` inputs.
4. Validation batch:
   all `22` single-OFE hillslopes were run with
   `target/release/openwepp-cli-hill --policy compat`.
5. Closure ledger:
   `.venv/bin/python` read all emitted WAT parquet files and computed the
   complete annual identity for years `2..6`.

Results:

- Climate audit: pass.
- Release build: pass.
- Single-OFE validation runs: `18` passed with WAT output, `4` failed closed
  with `HKERNEL-WB11-PERC-E-003`.
- Closure classification: all `18` WAT emitters are conservation-break above
  `1.0 mm/year`.

Skipped gates:

- `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo test --workspace`, and `cargo deny check` were not run because WBVAL04
  made no Rust, contract, or test edits. The release build and validation batch
  were the relevant execution gates for this package.
