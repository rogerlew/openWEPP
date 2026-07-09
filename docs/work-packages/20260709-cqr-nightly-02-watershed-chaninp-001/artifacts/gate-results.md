# Gate Results

Status: `COMPLETE`

Comparator runner disposition:

- Required `comparator_suite_runner` was used for the first heavy run. It found
  a real clippy failure, which was fixed.
- A final current-state `comparator_suite_runner` was launched after the
  accepted review fixes, but it stalled and was closed before completion.
- The final heavy gates were therefore run locally and recorded in
  package-local logs.
- Dedicated fallback evidence:
  `artifacts/comparator-runner-fallback.md`.

| Gate | Evidence | Result |
|---|---|---|
| `cargo llvm-cov clean --workspace` | `artifacts/logs/final-local-llvm-cov-clean.log`, SHA-256 `170da24ed016fc3cbc821ea1b37b2e7208e20ca52ecfe5dc240f8dc1f5c3646e`, `__EXIT_CODE__:0`. | PASS |
| `cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path /tmp/openwepp-cqr02-final-local-after.lcov` | `artifacts/logs/final-local-llvm-cov.log`, SHA-256 `f6a7ddd4d5778af90d8336ae73e337903aedb2394d9bde908d242493cef93604`, `__EXIT_CODE__:0`; LCOV artifact `/tmp/openwepp-cqr02-final-local-after.lcov`. Internal failures for `-p openwepp --test laned_shadow_h2637` and `-p openwepp-hillslope-orchestrator --lib` were recorded under `--ignore-run-fail`, and the report was written. | PASS-WITH-NOTE |
| `cargo llvm-cov report --json --output-path /tmp/openwepp-cqr02-final-local-after-full.json` plus target metrics extraction | `artifacts/logs/final-local-llvm-cov-report-json.log`, SHA-256 `62037946ebf36652bd8fb1e50cfdd87bf3ac56bf79376b1a806ffad7d1eb407a`, `__EXIT_CODE__:0`; `artifacts/logs/final-local-coverage-metrics.log`, SHA-256 `99c28641abc5341db3adc1e52c6682460fc42a42c28d48eef56456b22207f450`, `__EXIT_CODE__:0`; target source-region coverage `2431 / 2536 = 95.8596214511041%`; production/source-helper source-region coverage `1517 / 1610 = 94.22360248447205%`; production/source-helper functions below 75% source-region floor: `0`. | PASS |
| `cargo crap --workspace --lcov /tmp/openwepp-cqr02-final-local-after.lcov --min 0 --format json --output /tmp/openwepp-cqr02-final-local-after-crap.json` | `artifacts/logs/final-local-crap.log`, SHA-256 `4271b950b449ea18de1d61baf8c3884ef2c0ebaa5fa851e66e525f42f4d1f290`, `__EXIT_CODE__:0`; CRAP artifact `/tmp/openwepp-cqr02-final-local-after-crap.json`; target rows above `30`: `0`; max target CRAP `20.816276483846725`. | PASS |
| `git diff --check` | `artifacts/logs/final-local-diff-check.log`, SHA-256 `170da24ed016fc3cbc821ea1b37b2e7208e20ca52ecfe5dc240f8dc1f5c3646e`, `__EXIT_CODE__:0`. | PASS |
| Markdown/doc lint | `markdown-doc lint --path docs/work-packages/README.md --path docs/work-packages/20260709-cqr-nightly-02-watershed-chaninp-001 --format json`; `artifacts/logs/final-local-markdown-doc-lint.log`, SHA-256 `d790abe7bdcbb741c530fb707bab96ad13acedf3d96883366d2a207d2212db2a`, `24` files scanned, `0` errors, `0` warnings, `__EXIT_CODE__:0`. | PASS |
| `cargo fmt --check` | `artifacts/logs/final-local-fmt.log`, SHA-256 `170da24ed016fc3cbc821ea1b37b2e7208e20ca52ecfe5dc240f8dc1f5c3646e`, `__EXIT_CODE__:0`. | PASS |
| `cargo nextest run -p openwepp-watershed-orchestrator chaninp` | `artifacts/logs/final-local-nextest-chaninp.log`, SHA-256 `89d8d05ead557bc37ee03eb64c152f5d0ed374ca6730ad7d676fb61a53e8dd1d`, `13 tests run: 13 passed, 9 skipped`, `__EXIT_CODE__:0`. | PASS |
| `cargo clippy --workspace --all-targets -- -D warnings` | `artifacts/logs/final-local-clippy.log`, SHA-256 `bc9cc500bcac3e1c26fe46cb4eec282bd26da84af506d9da878d80b640f8850c`, `__EXIT_CODE__:0`. | PASS |
| `cargo nextest run --workspace --profile full` | `artifacts/logs/final-local-nextest-full.log`, SHA-256 `77a931d3a71aea3ce65b8f4ee0495d4c4f084f0cd8d75d47aa1054c526e009c9`, `1503 tests run: 1503 passed (8 slow), 3 skipped`, `__EXIT_CODE__:0`. | PASS |
| `cargo deny check` | `artifacts/logs/final-local-deny.log`, SHA-256 `97b54e040d65b9314347797b92665fe43a4b6c9e52d0cae59989a0198f7cda2f`, `advisories ok, bans ok, licenses ok, sources ok`, `__EXIT_CODE__:0`. | PASS |
