# Gate Results

Status: complete

Evidence mode: ran

Ran:

- `cargo test --test hphys0300_raw_hourly_melt_post_raw_routing_contract`
- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `cargo deny check`
- `markdown-doc lint --path docs/work-packages/20260605-hphys0300-raw-hourly-melt-post-raw-routing-lineage-closure-001 --path docs/work-packages/README.md --path docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md --path docs/specifications/science-contracts/contracts/SC-WATBAL-001.md --path docs/specifications/science-contracts/index.md --format json`
- `bash tools/release/check_authority_suite_antievasion.sh`
- `cargo test --test auth11_required_suite_obligation_guards_contract`

Results:

- Focused HPHYS0300 contract test passed: `3 passed; 0 failed`.
- `cargo fmt --check` passed after formatting the new integration test.
- `cargo clippy --workspace --all-targets -- -D warnings` passed.
- `cargo test --workspace` passed.
- `cargo deny check` passed with existing warnings for duplicate crates and
  unmatched license allowances; advisories, bans, licenses, and sources were
  otherwise accepted.
- Scoped `markdown-doc` lint passed: `29` files scanned, `0` errors,
  `0` warnings.
- Authority-suite anti-evasion shell guard passed.
- AUTH11 required-suite obligation guard passed: `2 passed; 0 failed`.

Notes:

- The full H1..H39 semantic suite was run by
  `artifacts/hphys0300_raw_post_raw_lineage.py` and recorded separately in
  `implementation-test-evidence.md` and `full-39-suite-metrics.md`.
- After dual-review fixes, the focused route-regression gate was strengthened
  to parse `raw-post-raw-lineage-ledger.json`; `cargo fmt --check` and
  `cargo test --test hphys0300_raw_hourly_melt_post_raw_routing_contract`
  passed with `3` tests.
- After dual-verification placeholder findings were recorded and dispositioned,
  the final local placeholder audit found no remaining queued/not-run artifact
  markers or unchecked package progress markers in the HPHYS0300 touched files.
- Final post-review gates passed:
  - `cargo fmt --check`
  - `cargo test --test hphys0300_raw_hourly_melt_post_raw_routing_contract`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `markdown-doc lint --path docs/work-packages/20260605-hphys0300-raw-hourly-melt-post-raw-routing-lineage-closure-001 --path docs/work-packages/README.md --path docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md --path docs/specifications/science-contracts/contracts/SC-WATBAL-001.md --path docs/specifications/science-contracts/index.md --format json`
  - `bash tools/release/check_authority_suite_antievasion.sh`
  - `cargo test --test auth11_required_suite_obligation_guards_contract`
  - `cargo test --workspace`
  - `cargo deny check`
  - `git diff --check`
- Final `cargo deny check` retained existing warnings for duplicate crates
  (`getrandom`, `hashbrown`, `twox-hash`) and unmatched license allowances
  (`ISC`, `Unicode-DFS-2016`); advisories, bans, licenses, and sources passed.
