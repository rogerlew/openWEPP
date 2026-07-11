# Release Binary Provenance

Status: `PASS`

Evidence mode: `Ran`

Runner: delegated `comparator_suite_runner`

## Source Baseline

- W11D routing baseline commit at gate start:
  `21f2844a1ee4ebcc265477a716da54c494dd6e89`
  (`Close WSHED-W11D hourly routing numerical defects`).
- HEAD after unrelated concurrent documentation work:
  `592df2f11eeef1c13aa346cee794921cb6b64cef`.
- `git diff 21f2844a..592df2f1` contains only
  `docs/audits/20260710_h2637_34yr_laned_active_endpoint_audit.md` and
  `docs/dev-guide/06-history-and-performance.md`.
- `git diff --quiet` confirms no `crates/`, `tests/`, or
  `docs/specifications/science-contracts/` change either beyond the W11D
  baseline or relative to current HEAD.

The release binary therefore binds the unchanged W11D production/test/contract
tree. The unrelated documentation paths were not edited by this runner.

## Exact Build

Ran:

`cargo build --release -p openwepp-runner --bin openwepp-cli-watershed`

Result: `PASS`, exit `0`, Cargo wall `0.15 s`, measured command wall `0.26 s`.
Cargo reported the exact target current; no generic workspace release build was
used.

## Binary Identity

- Absolute path:
  `/home/workdir/openWEPP/target/release/openwepp-cli-watershed`
- Size: `9,367,904 bytes`
- Mtime: `2026-07-10 22:55:04.977364784 -0700`
- Inode: `92144061`
- SHA-256:
  `f82cc9fa539d26cdf9a6797d3e272bca22a7a19dc4b9988a3a95e7cd4c38d792`

## Exact Release Consumer

Ran:

`OPENWEPP_W11C_WATERSHED_CLI=/home/workdir/openWEPP/target/release/openwepp-cli-watershed cargo nextest run -p openwepp-runner --test mt3_hbp_hourly_consumer_contract --no-capture`

Result: `PASS`, `7/7` passed, `0` skipped, nextest `2.553 s`, measured
wall `3.13 s`, run ID `65342801-a814-43a7-be38-b5234c4ceeff`.

The environment variable names the exact absolute binary above. The suite
consumed its real Parquet outputs; it did not substitute a test-only routing
result or compatibility publication path.
