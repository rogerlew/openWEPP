# Gate Results

Evidence label: Static/Ran.

Status: `EXECUTED-PASS`

## Required Gate Table

| Gate | Command | Result | Evidence |
|---|---|---|---|
| Diff whitespace | `git diff --check` | PASS, exit `0` | parent shell, 2026-07-09 |
| Formatting | `cargo fmt --check` | PASS, exit `0` | parent shell, 2026-07-09 |
| Focused parser tests | `cargo nextest run --test infile_management_parser_contract` | PASS, exit `0`; `45` passed | parent shell, 2026-07-09 |
| Focused YAML tests | `cargo nextest run --test infile_management_yaml_contract` | PASS, exit `0`; `2` passed | parent shell, 2026-07-09 |
| Target coverage after | `cargo llvm-cov --workspace --test infile_management_parser_contract --test infile_management_yaml_contract --json --output-path /tmp/openwepp-cqr-nightly-07-management-targeted-llvmcov.json --no-clean` | PASS, exit `0`; line `89.81854838709677%`, region `86.46770237121831%` | `/tmp/openwepp-cqr-nightly-07-management-targeted-llvmcov.json` |
| Target CRAP after | `cargo crap --workspace --lcov /tmp/openwepp-cqr-nightly-07-management-targeted.lcov --min 0 --format json --output /tmp/openwepp-cqr-nightly-07-management-targeted-crap.json` | PASS, exit `0`; `0` target rows above `30` | `/tmp/openwepp-cqr-nightly-07-management-targeted-crap.json` |
| Workspace clippy | `cargo clippy --workspace --all-targets -- -D warnings` | PASS, exit `0` | delegated log and parent rerun |
| Full nextest | `cargo nextest run --workspace --profile full` | PASS, exit `0`; post-review run `1566` passed, `3` skipped | delegated log |
| Deny | `cargo deny check` | PASS, exit `0` | delegated log |
| Package markdown lint | `markdown-doc lint --path docs/work-packages/20260709-cqr-nightly-07-input-management-parser-001 --path docs/work-packages/README.md` | PASS, exit `0`; `22` files, `0` errors, `0` warnings | parent shell |
| Package Markdown path sanity | `find docs/work-packages/20260709-cqr-nightly-07-input-management-parser-001 -name '*.md' ...` | PASS, exit `0`; package markdown non-empty | parent shell, 2026-07-09 |

## Delegated Heavy Gate Evidence

Ran by `comparator_suite_runner`:

- `cargo llvm-cov clean --workspace`
  - PASS, exit `0`
  - log:
    `/home/workdir/openWEPP/artifacts/cqr-20260709-cqr-nightly-07-input-management-parser-001/01-cargo-llvm-cov-clean.log`
- `cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path /tmp/openwepp-cqr-nightly-07-management-full.lcov`
  - blocked before LCOV output by unrelated
    `tests/integration/laned_shadow_h2637.rs` coverage-instrumented
    failures/long-runs
  - log:
    `/home/workdir/openWEPP/artifacts/cqr-20260709-cqr-nightly-07-input-management-parser-001/02-cargo-llvm-cov.log`
- `cargo crap --workspace --lcov /tmp/openwepp-cqr-nightly-07-management-full.lcov --min 0 --format json --output /tmp/openwepp-cqr-nightly-07-management-full-crap.json`
  - failed because the full LCOV file was not produced by the blocked coverage
    run
  - log:
    `/home/workdir/openWEPP/artifacts/cqr-20260709-cqr-nightly-07-input-management-parser-001/03-cargo-crap.log`
- `cargo clippy --workspace --all-targets -- -D warnings`
  - PASS, exit `0`
  - log:
    `/home/workdir/openWEPP/artifacts/cqr-20260709-cqr-nightly-07-input-management-parser-001/05-cargo-clippy.log`
- `cargo nextest run --workspace --profile full`
  - PASS, exit `0`; `1565` tests passed, `3` skipped
  - log:
    `/home/workdir/openWEPP/artifacts/cqr-20260709-cqr-nightly-07-input-management-parser-001/06-cargo-nextest-full.log`
- `cargo deny check`
  - PASS, exit `0`
  - log:
    `/home/workdir/openWEPP/artifacts/cqr-20260709-cqr-nightly-07-input-management-parser-001/07-cargo-deny.log`

Post-review heavy closure rerun by `comparator_suite_runner`:

- `cargo clippy --workspace --all-targets -- -D warnings`
  - PASS, exit `0`
  - log:
    `/home/workdir/openWEPP/artifacts/cqr-20260709-cqr-nightly-07-input-management-parser-001-postreview/01-cargo-clippy.log`
- `cargo nextest run --workspace --profile full`
  - PASS, exit `0`; `1566` tests passed, `3` skipped
  - log:
    `/home/workdir/openWEPP/artifacts/cqr-20260709-cqr-nightly-07-input-management-parser-001-postreview/02-cargo-nextest-full.log`
- `cargo deny check`
  - PASS, exit `0`
  - log:
    `/home/workdir/openWEPP/artifacts/cqr-20260709-cqr-nightly-07-input-management-parser-001-postreview/03-cargo-deny.log`

Parent post-change reruns:

- `cargo clippy --workspace --all-targets -- -D warnings`
  - PASS, exit `0`
  - run after final YAML characterization edits and after review-response source
    cleanup.
- `cargo nextest run --test infile_management_parser_contract`
  - PASS, exit `0`; `45` tests passed after review-response source cleanup.
- `cargo nextest run --test infile_management_yaml_contract`
  - PASS, exit `0`; `2` tests passed after review-response source cleanup.
- `cargo llvm-cov --workspace --test infile_management_parser_contract --test infile_management_yaml_contract --lcov --output-path /tmp/openwepp-cqr-nightly-07-management-targeted.lcov`
  - PASS, exit `0`; rerun after review-response source cleanup.
- `cargo llvm-cov --workspace --test infile_management_parser_contract --test infile_management_yaml_contract --json --output-path /tmp/openwepp-cqr-nightly-07-management-targeted-llvmcov.json --no-clean`
  - PASS, exit `0`; rerun after review-response source cleanup.
- `cargo crap --workspace --lcov /tmp/openwepp-cqr-nightly-07-management-targeted.lcov --min 0 --format json --output /tmp/openwepp-cqr-nightly-07-management-targeted-crap.json`
  - PASS, exit `0`; rerun after review-response source cleanup.

## Coverage/CRAP Substitution Disposition

Static/Ran:

- Full-workspace LCOV after implementation was attempted by the required heavy
  runner and blocked by unrelated `laned_shadow_h2637` coverage-instrumented
  behavior.
- Package Phase D explicitly allows a targeted equivalent when full-workspace
  coverage is blocked by unrelated coverage-instrumented tests.
- The targeted workspace test coverage/CRAP commands passed and produced the
  current package after metrics.

Disposition:

- Required current-scope closure gates are satisfied after the documented
  targeted coverage/CRAP substitution.
