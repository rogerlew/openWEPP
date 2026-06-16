# CQR35 Gate Results

Status: complete.

| Gate | Evidence | Result |
| --- | --- | --- |
| `cargo fmt --check` | Ran at CQR35 closeout. | PASS |
| `cargo clippy --workspace --all-targets -- -D warnings` | Ran at CQR35 closeout. | PASS |
| `cargo test --workspace` | Ran at CQR35 closeout. | PASS |
| `cargo deny check` | Ran at CQR35 closeout; output `advisories ok, bans ok, licenses ok, sources ok`. | PASS |
| `markdown-doc lint --path docs/work-packages/README.md --path docs/work-packages/20260616-cqr35-lateral-drainage-complexity-001 --format json` | Ran at CQR35 closeout; scanned 22 files, 0 errors, 0 warnings. | PASS |
| `git diff --check` | Ran before and after package artifact completion. | PASS |

Ran: before LCOV/CRAP and after LCOV/CRAP both passed.

Ran: target-file CRAP rows above `30`: before `0`, after `0`.

Warning: `cargo crap` reported 126 LCOV source-map warnings on both package
metrics runs. The target file was represented in LCOV.
