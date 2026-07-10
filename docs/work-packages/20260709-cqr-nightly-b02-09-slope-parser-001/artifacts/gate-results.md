# Gate Results

| Gate | Result | Command / evidence |
|---|---|---|
| Test-first proof | PASS | Detached worktree at `010f4ddf`; applied current focused test diff only from `/tmp/openwepp-cqr-b02-t09-testfirst.patch`; `CARGO_TARGET_DIR=/tmp/openwepp-cqr-b02-t09-testfirst-target cargo nextest run --manifest-path /tmp/openwepp-cqr-b02-t09-testfirst/Cargo.toml --test infile_slope_parser_contract --profile quick`; 27/27. |
| Focused slope parser test | PASS | `cargo nextest run --test infile_slope_parser_contract --profile quick`; 27/27. |
| Focused crate clippy | PASS | `cargo clippy -p openwepp-input-contract --all-targets -- -D warnings`; exit 0. |
| Fmt | PASS | `cargo fmt --check`; exit 0 after rustfmt. |
| Target coverage/CRAP | PASS | `cargo llvm-cov --workspace --test infile_slope_parser_contract --lcov --output-path /tmp/openwepp-cqr-b02-t09-focused2.lcov`; 628/677 lines, 668/728 regions; `cargo crap --workspace --lcov /tmp/openwepp-cqr-b02-t09-focused2.lcov --min 0 --format json --output /tmp/openwepp-cqr-b02-t09-focused2-crap.json`; 0 rows >30, max 17.1852. |
| Diff whitespace | PASS | `git diff --check`; exit 0 before final artifact updates. |
| Docs lint | PASS | `markdown-doc lint --path docs/work-packages/20260709-cqr-nightly-b02-09-slope-parser-001 --path docs/work-packages/README.md --format plain`; 23 files, 0 errors, 0 warnings after final artifact updates. |
| Workspace clippy | PASS | `cargo clippy --workspace --all-targets -- -D warnings`; exit 0. |
| Deny | PASS | `cargo deny check`; exit 0. |
| Full nextest | PASS | Exact command `cargo nextest run --workspace --profile full` completed via detached `setsid` wrapper to avoid foreground-session SIGTERM; log `/tmp/openwepp-cqr-b02-t09-full-nextest-setsid.log`; exit file `/tmp/openwepp-cqr-b02-t09-full-nextest-setsid.exit` = `EXIT=0`; summary 1652/1652 passed, 3 skipped, 4 slow, 619.187s; log SHA-256 `44ff4e544ec64121ff4cf94f10b0558a2a8c35f1be3b0d403ffeb6538896e8b4`. Earlier foreground attempts failed due runtime SIGTERM, but isolated `snowdensity05e` passed and detached full run passed. |
