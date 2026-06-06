# Gate Results

Status: complete

Evidence mode: Ran

Ran:

| Gate | Command | Result |
|---|---|---|
| Pre-implementation contract gate | `cargo test --test hphys0315_hourly_snowfall_input_lineage_contract hphys0315_contract_authority_is_registered -- --nocapture` | passed; exit status was `0` |
| Rust formatting | `cargo fmt --check` | passed; exit status was `0` |
| Focused HPHYS0315 contract suite | `cargo test --test hphys0315_hourly_snowfall_input_lineage_contract -- --nocapture` | passed; exit status was `0` |
| HPHYS0314 regression contract | `cargo test --test hphys0314_adr0017_snow_rm_reclassification_contract -- --nocapture` | passed; exit status was `0` |
| Authority-suite anti-evasion guard | `bash tools/release/check_authority_suite_antievasion.sh` | passed; exit status was `0` |
| AUTH11 obligation guard | `cargo test --test auth11_required_suite_obligation_guards_contract -- --nocapture` | passed; exit status was `0` |
| Clippy workspace | `cargo clippy --workspace --all-targets -- -D warnings` | passed; exit status was `0` |
| Workspace tests | `cargo test --workspace` | passed; exit status was `0` |
| Dependency/advisory gate | `cargo deny check` | passed; exit status was `0` with existing duplicate-crate/license allow warnings |
| Markdown/package lint | `markdown-doc lint --path docs/work-packages/20260606-hphys0315-hourly-snowfall-input-lineage-closure-001 --path docs/work-packages/README.md --path docs/specifications/science-contracts/index.md --path docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md --path docs/specifications/science-contracts/contracts/SC-WATBAL-001.md` | passed; exit status was `0` |
| Whitespace diff check | `git diff --check` | passed; exit status was `0` |

No gate authorized production code edits.

Gate note:

An initial focused validation pass found Rust formatting differences and two
artifact phrase-wrap assertions. Those were corrected with `cargo fmt` and
artifact wording updates before the final passing gate run recorded above.
