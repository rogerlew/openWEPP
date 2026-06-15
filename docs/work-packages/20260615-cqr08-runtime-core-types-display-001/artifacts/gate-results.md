# Gate Results

Ran: focused gates and characterization:

| Gate | Exit | Evidence |
| --- | ---: | --- |
| `cargo test -p openwepp-hillslope-orchestrator runtime_inputs::tests -- --nocapture` before production refactor | 0 | 66 passed |
| `cargo test -p openwepp-hillslope-orchestrator runtime_inputs::tests -- --nocapture` after characterization | 0 | 67 passed |
| `cargo test -p openwepp-hillslope-orchestrator runtime_inputs::tests::hillslope_runtime_input_error_codes_and_display_are_stable -- --nocapture` after refactor | 0 | 1 passed |
| `cargo test -p openwepp-hillslope-orchestrator runtime_inputs::tests -- --nocapture` after refactor | 0 | 67 passed |
| `cargo clippy -p openwepp-hillslope-orchestrator --all-targets -- -D warnings` | 0 | warnings denied |

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
| `markdown-doc lint --path docs/work-packages/README.md --path docs/work-packages/20260615-cqr08-runtime-core-types-display-001 --format json` | 0 | 27 files scanned; 0 errors; 0 warnings |
| `git diff --check` | 0 | no whitespace errors |

Disposition: all executed gates passed.
