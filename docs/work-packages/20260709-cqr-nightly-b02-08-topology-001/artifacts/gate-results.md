# Gate Results

| Gate | Result | Command / evidence |
|---|---|---|
| Fmt | PASS | `cargo fmt --check`; exit 0 after applying rustfmt to the new test import/line wrap. |
| Test-first proof | PASS | Detached worktree at `87e15ffb469c27b74e47cc69e09e7ac26cff3523`; applied only final focused test diff; `CARGO_TARGET_DIR=/tmp/openwepp-cqr-b02-t08-predecomp-final-target cargo nextest run --manifest-path /tmp/openwepp-cqr-b02-t08-predecomp-final/Cargo.toml --test topology_graph_validation_gate --profile quick`; 13/13. |
| Focused topology test | PASS | `cargo nextest run --test topology_graph_validation_gate --profile quick`; 13/13 after implementation. |
| Focused test clippy | PASS | `cargo clippy --test topology_graph_validation_gate -- -D warnings`; exit 0. |
| Focused topology-crate clippy | PASS | `cargo clippy -p openwepp-topology --all-targets -- -D warnings`; exit 0. |
| Target coverage/CRAP | PASS | `CARGO_TARGET_DIR=/tmp/openwepp-cqr-b02-t08-cov4 cargo llvm-cov --workspace --test topology_graph_validation_gate --lcov --output-path /tmp/openwepp-cqr-b02-t08-final4.lcov`; 710/746 lines, 841/874 regions; `cargo crap --workspace --lcov /tmp/openwepp-cqr-b02-t08-final4.lcov --min 0 --format json --output /tmp/openwepp-cqr-b02-t08-final4-crap.json`; 0 rows >30, max 10. |
| Diff whitespace | PASS | `git diff --check`; exit 0. |
| Docs lint | PASS | `markdown-doc lint --path docs/work-packages/20260709-cqr-nightly-b02-08-topology-001 --path docs/work-packages/README.md --format plain`; rerun after final artifacts/catalog update, 23 files validated, 0 errors, 0 warnings. |
| Workspace clippy | PASS | Delegated comparator/closure runner: `cargo clippy --workspace --all-targets -- -D warnings`; `/tmp/openwepp-cqr-b02-t08-closure-final2/clippy.exit` has `EXIT=0`. |
| Full nextest | PASS | Delegated comparator/closure runner: `cargo nextest run --workspace --profile full`; `/tmp/openwepp-cqr-b02-t08-closure-final2/nextest-full.exit` has `EXIT=0`; 1645/1645 passed, 4 slow, 3 skipped, 588.874s. |
| Deny | PASS | Delegated comparator/closure runner: `cargo deny check`; `/tmp/openwepp-cqr-b02-t08-closure-final2/deny.exit` has `EXIT=0`; advisories ok, bans ok, licenses ok, sources ok. |
