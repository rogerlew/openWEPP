# Gate Results

Status: `COMPLETE`

| Gate | Evidence | Result |
|---|---|---|
| `cargo check -p openwepp-kernel-contract` | Current-run package log: `artifacts/logs/final-current-cargo-check.log`, SHA-256 `e82a807af4371cbfd111dec393b84576cfe2822b8866dbd6456b566adc996704`, `__EXIT_CODE__:0`. | PASS |
| `cargo nextest run --test arch22_typed_state_surface_contract` | Current-run package log: `artifacts/logs/final-current-nextest-arch22.log`, SHA-256 `8a922ab3e8c3199ef667381654f8abec2ce63407c04968a69e0b9edcf95098f8`, `17 tests run: 17 passed, 0 skipped`, `__EXIT_CODE__:0`. | PASS |
| `cargo fmt --check` | Current-run package log: `artifacts/logs/final-current-fmt.log`, SHA-256 `170da24ed016fc3cbc821ea1b37b2e7208e20ca52ecfe5dc240f8dc1f5c3646e`, `__EXIT_CODE__:0`. | PASS |
| After CRAP/LCOV | `comparator_suite_runner` produced `/tmp/openwepp-cqr-nightly-01-final2.lcov`, `/tmp/openwepp-cqr-nightly-01-final2-full.json`, and `/tmp/openwepp-cqr-nightly-01-final2-crap.json` before its final report overflowed. Package-local replay logs: `artifacts/logs/final-current-llvm-cov-report-json.log` SHA-256 `ce16b32dad87f1bd7dc9db3352a50b75314f0d5a75f482b1e26753e061af82ed`, `artifacts/logs/final-current-cargo-crap-replay.log` SHA-256 `4271b950b449ea18de1d61baf8c3884ef2c0ebaa5fa851e66e525f42f4d1f290`, and `artifacts/logs/final-current-coverage-metrics.log` SHA-256 `d1d33852232ba2825fd0ba40eaad821eae219d2480eb15df42515be412a8c0ec`; all record `__EXIT_CODE__:0`. Target line coverage `278 / 284 = 97.88732394366197%`; unique source-region coverage `332 / 338 = 98.22485207100591%`; CRAP rows above `30`: `0`. | PASS |
| `git diff --check` | Current-run package log: `artifacts/logs/final-current-diff-check.log`, SHA-256 `170da24ed016fc3cbc821ea1b37b2e7208e20ca52ecfe5dc240f8dc1f5c3646e`, `__EXIT_CODE__:0`. | PASS |
| markdown/doc lint for touched docs | Current-run package log: `artifacts/logs/final-current-markdown-doc-lint.log`, SHA-256 `903cad87215d7a51bb7198dc669d08eb197ceecc7c0b18f8f2991ac983da3c34`, `23` files scanned, `0` errors, `0` warnings, `__EXIT_CODE__:0`. | PASS |
| `cargo clippy --workspace --all-targets -- -D warnings` | Current-run package log: `artifacts/logs/final-current-clippy.log`, SHA-256 `b817d18e25c76c43200c217e94a43429797c51abd1851cf80daee24e856c8d1c`, `__EXIT_CODE__:0`. | PASS |
| `cargo nextest run --workspace --profile full` | Current-run package log: `artifacts/logs/final-current-nextest-full.log`, SHA-256 `5d6341284cd716f933aacb1907ff1266bfcf29803cba1b1d4d7bc4613e46d843`, `1490 tests run: 1490 passed (4 slow), 3 skipped`, `__EXIT_CODE__:0`. | PASS |
| `cargo deny check` | Current-run package log: `artifacts/logs/final-current-deny.log`, SHA-256 `97b54e040d65b9314347797b92665fe43a4b6c9e52d0cae59989a0198f7cda2f`, `advisories ok, bans ok, licenses ok, sources ok`, `__EXIT_CODE__:0`. | PASS |
