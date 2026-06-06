# Gate Results

Status: complete

Evidence mode: Ran

Ran:

| Gate | Command | Result |
|---|---|---|
| Pre-implementation contract gate | `cargo test --test hphys0316_2013_terminal_carry_recursion_contract hphys0316_contract_authority_is_registered -- --nocapture` | passed; exit status was `0` |
| Rust formatting | `cargo fmt --check` | passed; exit status was `0` |
| Focused HPHYS0316 contract suite | `cargo test --test hphys0316_2013_terminal_carry_recursion_contract -- --nocapture` | passed; exit status was `0` |
| HPHYS0315 regression contract | `cargo test --test hphys0315_hourly_snowfall_input_lineage_contract -- --nocapture` | passed; exit status was `0` |
| Authority-suite anti-evasion guard | `bash tools/release/check_authority_suite_antievasion.sh` | passed; exit status was `0` |
| AUTH11 obligation guard | `cargo test --test auth11_required_suite_obligation_guards_contract -- --nocapture` | passed; exit status was `0` |
| Clippy workspace | `cargo clippy --workspace --all-targets -- -D warnings` | passed; exit status was `0` |
| Workspace tests | `cargo test --workspace` | passed; exit status was `0` |
| Dependency/advisory gate | `cargo deny check` | passed; exit status was `0` with existing duplicate-crate/license allow warnings |
| Markdown/package lint | `markdown-doc lint --path docs/work-packages/20260606-hphys0316-2013-terminal-carry-recursion-closure-001 --path docs/work-packages/README.md --path docs/specifications/science-contracts/index.md --path docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md --path docs/specifications/science-contracts/contracts/SC-WATBAL-001.md` | passed; exit status was `0` |
| Whitespace diff check | `git diff --check` | passed; exit status was `0` |

Validation correction notes:

- Initial focused validation found a Rust formatting delta and package artifact
  phrase-token assertions that needed contiguous evidence text; corrected with
  `cargo fmt` and artifact wording updates before rerunning the focused
  HPHYS0316 contract suite.
- Initial HPHYS0315 regression validation failed because the regression test
  pinned prior contract versions `46` and `139`; corrected the regression test
  to assert `contract_version:` presence plus the invariant identifiers after
  HPHYS0316 advanced the contract versions.

No gate authorized production code edits.
