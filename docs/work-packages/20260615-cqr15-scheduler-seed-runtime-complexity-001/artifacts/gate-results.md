# Gate Results

Status: complete.

Already run during implementation:

| Command | Result |
| --- | --- |
| `cargo test -p openwepp-runner publication_wb11_seed --lib` | Pass, `16 passed; 0 failed` |
| `cargo clippy -p openwepp-runner --all-targets -- -D warnings` | Pass |
| Before `cargo llvm-cov ... lcov_before.info` | Pass |
| Before `cargo crap ... crap_before.json` | Pass with recurring no-matching-LCOV warning |
| Final after `cargo llvm-cov ... lcov_after.info` | Pass |
| Final after `cargo crap ... crap_after.json` | Pass with recurring no-matching-LCOV warning |

Final required gate transcript:

| Gate | Status |
| --- | --- |
| `cargo fmt --check` | Pass |
| `cargo clippy --workspace --all-targets -- -D warnings` | Pass |
| `cargo test --workspace` | Pass |
| `cargo deny check` | Pass: `advisories ok, bans ok, licenses ok, sources ok` |
| `markdown-doc lint --path docs/work-packages/README.md --path docs/work-packages/20260615-cqr15-scheduler-seed-runtime-complexity-001 --format json` | Pass: `files_scanned: 23`, `errors: 0`, `warnings: 0` |
| `git diff --check` | Pass |

Ran: final gates passed before package disposition was moved to
complete-with-warnings.
