# Gate Results

Status: completed

Evidence mode: ran

## Rust and Authority Gates

| Gate | Result |
| --- | --- |
| `cargo fmt --check` | passed |
| `cargo clippy --workspace --all-targets -- -D warnings` | passed |
| `cargo test --workspace` | passed |
| `cargo deny check` | passed with pre-existing duplicate/unmatched-license warnings; advisories/bans/licenses/sources ok |
| `bash tools/release/check_authority_suite_antievasion.sh` | passed |
| `cargo test --test auth11_required_suite_obligation_guards_contract -- --nocapture` | passed, `2 passed` |
| `markdown-doc lint --path ...` | passed, `25 files validated`, `0 errors`, `0 warnings` |

## Comparator Gates

| Gate | Result |
| --- | --- |
| H1/H7/H39 targeted diagnostics | passed execution; residuals unchanged |
| Full H1..H39 semantic suite | passed execution; semantic pass `0/39` |
