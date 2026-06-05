# Gate Results

Status: complete
Evidence mode: Ran

| Gate | Result | Notes |
| --- | --- | --- |
| `cargo fmt --check` | pass | Passed after `cargo fmt` normalized the new test. |
| `cargo test --test hphys0294_post_ingress_storage_retention_contract -- --nocapture` | pass | `3 passed; 0 failed`. |
| HPHYS0294 diagnostics | pass | Full H1..H39 plus H1/H7/H39 traces completed under `/tmp/hphys0294_full_20260605T050323Z`. |
| `cargo clippy --workspace --all-targets -- -D warnings` | pass | No warnings. |
| `cargo test --workspace` | pass | Full workspace passed. |
| `cargo deny check` | pass with warnings | Existing duplicate-crate and unmatched-license allowance warnings; final policy checks ok. |
| `bash tools/release/check_authority_suite_antievasion.sh` | pass | Authority anti-evasion checks passed. |
| `cargo test --test auth11_required_suite_obligation_guards_contract -- --nocapture` | pass | `2 passed; 0 failed`. |
| `wctl doc-lint --path docs/work-packages/20260605-hphys0294-post-ingress-storage-percolation-lateral-retention-closure-001` | pass | `0 files validated, 0 errors, 0 warnings`. |
| `wctl doc-lint --path docs/work-packages/README.md` | pass | `1 files validated, 0 errors, 0 warnings`. |
