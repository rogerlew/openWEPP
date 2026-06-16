# CQR36 Gate Results

Status: complete.

| Gate | Evidence | Result |
| --- | --- | --- |
| `cargo fmt --check` | Ran at CQR36 closeout. | PASS |
| `cargo clippy --workspace --all-targets -- -D warnings` | Initial run failed on two `needless_raw_string_hashes` warnings in new tests; fixed the raw string delimiters and reran successfully. | PASS |
| `cargo test --workspace` | Ran at CQR36 closeout. | PASS |
| `cargo deny check` | Ran at CQR36 closeout; output `advisories ok, bans ok, licenses ok, sources ok`. | PASS |
| `markdown-doc lint --path docs/work-packages/README.md --path docs/work-packages/20260616-cqr36-watershed-impoundment-parser-complexity-001 --format json` | Ran at CQR36 closeout; scanned 22 files, 0 errors, 0 warnings. | PASS |
| `git diff --check` | Ran at CQR36 closeout. | PASS |

Ran: before LCOV/CRAP and after LCOV/CRAP both passed.

Ran: final unique target-file CRAP rows above `30`: `0`.

Warning: `cargo crap` reported 126 LCOV source-map warnings on before and after
metrics runs. The target file was represented in LCOV.
