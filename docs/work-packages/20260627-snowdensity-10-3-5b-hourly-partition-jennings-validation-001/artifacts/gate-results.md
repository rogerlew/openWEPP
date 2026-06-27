# Gate Results

Status: complete
Evidence mode: Ran

| Gate | Status | Evidence |
|---|---|---|
| `cargo fmt --check` | PASS | Ran after final provenance fix. |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS | Ran after final provenance fix. |
| `cargo test --workspace` | PASS | Full workspace rerun passed after fixing the initial `hphys0299` provenance guard failure. |
| `cargo deny check` | PASS | `advisories ok, bans ok, licenses ok, sources ok`. |
| `wctl doc-lint --path docs/work-packages` | PASS | `971` files validated, `0` errors, `0` warnings. |
| Jennings validation | PASS | Full local file2 run scored `11,711,058` rows across `6,883` stations. |
| Authority anti-evasion | PASS | `bash tools/release/check_authority_suite_antievasion.sh`; `cargo test --test auth11_required_suite_obligation_guards_contract`. |
| Review disposition | PASS | Local dual-review artifacts completed; one implementation finding fixed. |
| Verification | PASS | Local dual-verification artifacts completed. |
