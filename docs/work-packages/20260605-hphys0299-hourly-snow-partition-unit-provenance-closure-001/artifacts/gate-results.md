# Gate Results

Status: complete

Evidence mode: ran

Ran:

```text
cargo fmt --check
```

Result: initial failure on new test formatting only.

Ran:

```text
cargo fmt
cargo fmt --check
```

Result: pass.

Post-review rerun:

```text
cargo fmt --check
cargo test --test hphys0299_hourly_snow_partition_unit_provenance_contract
```

Result: pass, `4 passed`.

Final closeout rerun:

```text
cargo fmt --check
cargo test --test hphys0299_hourly_snow_partition_unit_provenance_contract
markdown-doc lint --path docs/work-packages/20260605-hphys0299-hourly-snow-partition-unit-provenance-closure-001 --path docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md --path docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md --path docs/specifications/science-contracts/contracts/SC-WATBAL-001.md --path docs/work-packages/README.md --format json
rg -n "Status: queued|Evidence mode: not-run|Pending execution|pending\\." docs/work-packages/20260605-hphys0299-hourly-snow-partition-unit-provenance-closure-001 || true
```

Result: pass, focused test `5 passed`, markdown-doc `files_scanned=28`,
`errors=0`, `warnings=0`, and no queued/not-run/pending placeholders remained
in the package.

Post-review rerun:

```text
markdown-doc lint --path docs/work-packages/20260605-hphys0299-hourly-snow-partition-unit-provenance-closure-001 --path docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md --path docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md --path docs/specifications/science-contracts/contracts/SC-WATBAL-001.md --path docs/work-packages/README.md --format json
```

Result: pass, `files_scanned=28`, `errors=0`, `warnings=0`.

Post-review rerun:

```text
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

Result: pass.

Post-review rerun:

```text
cargo deny check
bash tools/release/check_authority_suite_antievasion.sh
cargo test --test auth11_required_suite_obligation_guards_contract
```

Result: pass. `cargo deny check` retained the existing duplicate dependency
and unmatched license allowance warnings while reporting advisories, bans,
licenses, and sources `ok`.

Ran:

```text
cargo test --test hphys0299_hourly_snow_partition_unit_provenance_contract
```

Result: pass, `4 passed`.

Ran:

```text
cargo clippy --workspace --all-targets -- -D warnings
```

Result: pass.

Ran:

```text
cargo test --workspace
```

Result: pass.

Ran:

```text
cargo deny check
```

Result: pass. Existing warnings reported duplicate dependency entries
(`getrandom`, `hashbrown`, `twox-hash`) and unmatched license allowances
(`ISC`, `Unicode-DFS-2016`); advisories, bans, licenses, and sources all
reported `ok`.

Ran:

```text
markdown-doc lint --path docs/work-packages/20260605-hphys0299-hourly-snow-partition-unit-provenance-closure-001 --path docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md --path docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md --path docs/specifications/science-contracts/contracts/SC-WATBAL-001.md --path docs/work-packages/README.md --format json
```

Result: pass, `files_scanned=28`, `errors=0`, `warnings=0`.

Ran:

```text
bash tools/release/check_authority_suite_antievasion.sh
cargo test --test auth11_required_suite_obligation_guards_contract
```

Result: pass.
