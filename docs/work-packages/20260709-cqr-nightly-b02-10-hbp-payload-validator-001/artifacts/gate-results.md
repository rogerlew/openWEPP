# Gate Results

Ran: all required gates passed.

| Gate | Command | Result |
|---|---|---|
| Diff whitespace | `git diff --check` | PASS, exit 0 |
| Package markdown lint | `markdown-doc lint --path docs/work-packages/20260709-cqr-nightly-b02-10-hbp-payload-validator-001 --format plain` | PASS, 21 files, 0 errors, 0 warnings |
| Formatting | `cargo fmt --check` | PASS, exit 0 |
| Focused HBP tests | `cargo nextest run --test infile_hbp_parser_contract --profile quick` | PASS, 26/26 |
| Focused clippy | `cargo clippy -p openwepp --test infile_hbp_parser_contract -- -D warnings` | PASS, exit 0 |
| Workspace clippy | `cargo clippy --workspace --all-targets -- -D warnings` | PASS, exit 0 |
| Deny | `cargo deny check` | PASS, advisories/bans/licenses/sources ok |
| Full nextest | `cargo nextest run --workspace --profile full` | PASS, 1653/1653, 3 skipped, 4 slow |

Full nextest detached evidence:

- log: `/tmp/openwepp-cqr-b02-t10-full-nextest-setsid.log`
- log SHA-256:
  `b5f3f74f2b6748d51b336400004323139e76336636323acb063acd536bc3f4f4`
- exit file: `/tmp/openwepp-cqr-b02-t10-full-nextest-setsid.exit`
- exit content: `EXIT=0`
- pid file: `/tmp/openwepp-cqr-b02-t10-full-nextest-setsid.pid`
- summary: `1653` tests run, `1653` passed, `3` skipped, `4` slow,
  `584.384s`

Full workspace coverage/CRAP metric commands:

| Metric | Command | Result |
|---|---|---|
| Fullcov LCOV | `cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path /tmp/openwepp-cqr-b02-t10-fullcov.lcov` | PASS for LCOV emission, exit file `EXIT=0`; caveat recorded in `coverage-after.md` |
| CRAP after | `cargo crap --workspace --lcov /tmp/openwepp-cqr-b02-t10-fullcov.lcov --min 0 --format json --output /tmp/openwepp-cqr-b02-t10-fullcov-crap.json` | PASS, target rows above 30: none |
