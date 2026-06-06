# Gate Results

Status: complete

Evidence mode: Ran

Ran:

| Gate | Command | Result |
|---|---|---|
| Pre-implementation contract gate | `cargo test --test hphys0317_hourly_snowfall_input_surface_parity_contract hphys0317_contract_authority_is_registered -- --nocapture` | Passed; exit status was `0`. |
| Rust formatting | `cargo fmt --check` | Passed; exit status was `0`. |
| Focused HPHYS0317 contract suite | `cargo test --test hphys0317_hourly_snowfall_input_surface_parity_contract -- --nocapture` | Passed; exit status was `0`. |
| HPHYS0315 regression contract | `cargo test --test hphys0315_hourly_snowfall_input_lineage_contract -- --nocapture` | Passed; exit status was `0`. |
| HPHYS0316 regression contract | `cargo test --test hphys0316_2013_terminal_carry_recursion_contract -- --nocapture` | Passed; exit status was `0`. |
| Authority-suite anti-evasion guard | `bash tools/release/check_authority_suite_antievasion.sh` | Passed; exit status was `0`. |
| AUTH11 obligation guard | `cargo test --test auth11_required_suite_obligation_guards_contract -- --nocapture` | Passed; exit status was `0`. |
| Clippy workspace | `cargo clippy --workspace --all-targets -- -D warnings` | Passed; exit status was `0`. |
| Workspace tests | `cargo test --workspace` | Passed; exit status was `0`. |
| Dependency/advisory gate | `cargo deny check` | Passed; exit status was `0`; duplicate-crate and unmatched-license warnings are unchanged advisory output. |
| Markdown/package lint | `markdown-doc lint --path docs/work-packages/20260606-hphys0317-paired-hourly-snowfall-input-surface-parity-001 --path docs/work-packages/README.md --path docs/specifications/science-contracts/index.md --path docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md --path docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md --path docs/specifications/science-contracts/contracts/SC-WATBAL-001.md` | Passed; exit status was `0`. |
| Whitespace diff check | `git diff --check` | Passed; exit status was `0`. |

No gate has authorized production code edits.
