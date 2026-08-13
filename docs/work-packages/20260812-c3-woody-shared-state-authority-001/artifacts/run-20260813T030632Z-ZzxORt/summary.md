# Campaign summary
run_directory: /home/workdir/openWEPP/docs/work-packages/20260812-c3-woody-shared-state-authority-001/artifacts/run-20260813T030632Z-ZzxORt
commit: 916f24181e250d1cee5b17d9985bb082b7b53a3f
frozen_hashes_definition: 8ace38d1148f95261306cd6b0bf6f22e23ac8ead4cb6897dbdb53061b78ee437
fixture: 3072226f1d80359c548d87c1fa222be0c20b01627d9117e39163c39d9eb8824d
generator: 422f0a6fb778de73568259b0d1bad19f63e5b6fcac5fd608accace45b316bcd2
commands_run: 6

## results
- PASS: cargo clippy --workspace --all-targets -- -D warnings (rc=0, duration_s=25)
- PASS: cargo nextest run --workspace --profile full (rc=0, duration_s=3371)
- PASS: cargo test --doc --workspace (rc=0, duration_s=7)
- PASS: cargo deny check (rc=0, duration_s=1)
- PASS: cargo fmt --all -- --check (rc=0, duration_s=3)
- PASS: git diff --check (rc=0, duration_s=0)
