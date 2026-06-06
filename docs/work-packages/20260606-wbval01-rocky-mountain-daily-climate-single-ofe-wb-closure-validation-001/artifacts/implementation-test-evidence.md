# Implementation and Test Evidence

Status: complete

Evidence mode: Ran

Ran:

1. `cargo build --release -p openwepp-runner --bin openwepp-cli-hill`
   - Result: pass.

2. `target/release/openwepp-cli-hill --run-dir /wc1/runs/in/indispensable-presenter/wepp/runs --run-file p1.run --output-dir /tmp/wbval01_rocky_mountain_20260606T000000Z/hillslopes/p1 --policy compat`
   - Result: fail closed (`CLIHILL-E-010` invalid TOML).
   - Disposition: generated TOML wrappers were required because the release
     runner currently accepts `openwepp-hillslope-runfile-v1` TOML, not legacy
     WEPP text `.run` files.

3. Generated TOML wrappers under
   `/tmp/wbval01_rocky_mountain_20260606T000000Z/generated_runfiles_nodiscovery/`.
   - Result: pass.
   - Scope: wrapper generation only; `/wc1` authoritative inputs were not
     modified.

4. Batch ran all `22` single-OFE hillslopes with `openwepp-cli-hill`.
   - Status file:
     `/tmp/wbval01_rocky_mountain_20260606T000000Z/run_status_nodiscovery.tsv`
   - Result: `12/22` emitted complete WAT parquet outputs; `10/22` failed
     closed before output publication.

5. Read WAT schemas with DuckDB and verified required residual terms were
   populated for all emitted WAT rows.
   - Result: pass for `12/12` emitted WAT files.
   - Required terms: `P`, `Irr`, `Q`, `Ep`, `Es`, `Er`, `Dp`, `latqcc`,
     `SoilWaterTotal`, `Snow-Water`.

6. Computed annual residual ledger with DuckDB/Python.
   - Output scratch:
     `/tmp/wbval01_rocky_mountain_20260606T000000Z/closure_ledger.csv`
   - Summary scratch:
     `/tmp/wbval01_rocky_mountain_20260606T000000Z/closure_summary.json`
   - Result: `12/12` emitted hillslopes are `conservation-break` for full
     years `2..6` against the `1.0 mm/year` tolerance.

Not run:

- `cargo fmt --check`, `cargo clippy`, `cargo test --workspace`, and
  `cargo deny check` were not run because WBVAL01 made no production Rust or
  contract changes. The package validation gate is release-binary execution plus
  documentation lint.
