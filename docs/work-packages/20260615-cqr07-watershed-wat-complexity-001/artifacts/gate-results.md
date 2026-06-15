# Gate Results

Ran: focused gates and characterization:

| Gate | Exit | Evidence |
| --- | ---: | --- |
| `cargo test -p openwepp-runner watershed_wat::tests -- --nocapture` before edits | 0 | 2 passed |
| `cargo test -p openwepp-runner watershed_wat::tests -- --nocapture` after characterization | 0 | 4 passed |
| `cargo fmt && cargo test -p openwepp-runner watershed_wat::tests -- --nocapture && cargo clippy -p openwepp-runner --all-targets -- -D warnings` after refactor | 0 | focused tests and clippy passed |

Ran: metric gates:

| Gate | Exit | Evidence |
| --- | ---: | --- |
| before LCOV/CRAP command | 0 | `lcov_before.info`, `crap_before.json` |
| after LCOV/CRAP command | 0 | `lcov_after.info`, `crap_after.json` |

Ran: closure gates:

| Gate | Exit | Evidence |
| --- | ---: | --- |
| `cargo fmt --check` | 0 | no formatter diff |
| `cargo clippy --workspace --all-targets -- -D warnings` | 0 | warnings denied |
| `cargo test --workspace` | 0 | workspace tests and doctests passed |
| `cargo deny check` | 0 | advisories, bans, licenses, sources ok |
| `markdown-doc lint --path docs/work-packages/README.md --path docs/work-packages/20260615-cqr07-watershed-wat-complexity-001 --format json` | 0 | 27 files scanned; 0 errors; 0 warnings |
| `git diff --check` | 0 | no whitespace errors |

Disposition: all executed gates passed.
