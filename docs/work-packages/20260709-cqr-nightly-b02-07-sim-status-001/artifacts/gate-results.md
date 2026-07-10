# Gate Results

| Gate | Result | Command / evidence |
|---|---|---|
| Focused taxonomy | PASS | `cargo nextest run --test sim_contract_status_taxonomy --profile quick`; 11/11. |
| Target coverage/CRAP | PASS | Delegated workspace `llvm-cov`/`crap`; 199/199 lines, 152/152 regions, 0 >30. |
| Docs lint | PASS | `markdown-doc lint --path docs/work-packages/20260709-cqr-nightly-b02-07-sim-status-001 --path docs/work-packages/README.md --format plain`; 16 files, exit 0. |
| Fmt | PASS | Delegated `cargo fmt --check`, exit 0. |
| Workspace clippy | PASS | Delegated `cargo clippy --workspace --all-targets -- -D warnings`, exit 0. |
| Full nextest | PASS | Delegated `cargo nextest run --workspace --profile full`; 1638/1638, 3 skipped. |
| Deny | PASS | Delegated `cargo deny check`, exit 0. |
