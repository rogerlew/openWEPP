# Line-Count Governance Checklist

Status: complete
Evidence mode: Ran

| Check | Status | Evidence |
|---|---|---|
| New or touched `.rs` files scanned | PASS | Ran: `find crates/openwepp-meteorology tests/integration -type f \( -name '*.rs' -o -name 'Cargo.toml' \) -exec wc -l {} +`. New crate files: `phase.rs` 383 lines, `psychrometrics.rs` 445 lines, `error.rs` 65 lines, `lib.rs` 15 lines; new contract test 67 lines. |
| 2000+ line files dispositioned | PASS | Ran: no new/touched `.rs` file is at or above 2000 lines. |
| 3000+ non-exempt files absent or refactored | PASS | Ran: no new/touched `.rs` file is at or above 3000 lines. |
