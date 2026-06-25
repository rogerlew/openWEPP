# Verification

Status: complete

Evidence mode: Ran.

| Gate | Result | Evidence |
| --- | --- | --- |
| Site3 pre-fix reproduction | PASS | `compare --site site3_scan_mandan_nd` exited `1`; stderr reported lane 1 day 487 `storage_reconciliation.frost_storage_projection_theta_m must be nonnegative`; report verdict `HARNESS-SURFACE-MISMATCH`. |
| Site4 pre-fix reproduction | PASS | `compare --site site4_ggd498_morris_mn` exited `1`; stderr reported lane 1 day 10727 `storage_reconciliation.frost_storage_projection_theta_m must be nonnegative`; report verdict `HARNESS-SURFACE-MISMATCH`. |
| Focused Rust regression tests | PASS | `cargo test -p openwepp-hillslope-orchestrator r4b_explicit_frost_storage -- --nocapture`: `2 passed; 0 failed`. |
| Site3 post-fix compare report | PASS | `compare --site site3_scan_mandan_nd` exited `0`; report verdict `UNRESOLVED`, matched rows `10643`, isotherm upper-bound rows `10583`, reason `null`. |
| Site4 post-fix compare report | PASS | `compare --site site4_ggd498_morris_mn` exited `0`; report verdict `UNRESOLVED`, matched rows `83`, frost-depth residual rows `83`, max absolute residual `0.990389751515789 m`, reason `null`. |
| `cargo build -p openwepp-runner --bin openwepp-cli-hill` | PASS | Rebuilt the harness CLI binary before post-fix site comparisons. |
| `cargo test --test snowfreeze_observed_frost_depth_contract` | PASS | `3 passed; 0 failed`. |
| `cargo fmt --check` | PASS | Completed with exit `0`. |
| `git diff --check` | PASS | Completed with exit `0`. |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS | Completed with exit `0` after mechanical adjacent lint cleanup and review-driven test hardening. |
| `cargo test --workspace` | PASS | Completed with exit `0` after review-driven test hardening; workspace unit, integration, and doc tests passed. |
| `cargo deny check` | PASS | `advisories ok, bans ok, licenses ok, sources ok`. |
| Review finding disposition complete | PASS | Runtime reviewer reported no findings. QA reviewer reported one medium and two low findings; all were accepted and fixed. |

## Site Report Paths

- Pre-fix site3: `target/snowfreeze_observed_compare_site3_direct_prefail/`
- Pre-fix site4: `target/snowfreeze_observed_compare_site4_direct_prefail/`
- Post-fix site3: `target/snowfreeze_observed_compare_site3_direct_postfix/`
- Post-fix site4: `target/snowfreeze_observed_compare_site4_direct_postfix/`
