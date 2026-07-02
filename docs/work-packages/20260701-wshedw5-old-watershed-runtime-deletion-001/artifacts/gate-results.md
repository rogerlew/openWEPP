# Gate Results

Status: `executed`

Evidence mode: `static + ran`

| Gate | Result | Evidence |
| --- | --- | --- |
| Package scaffold authored | `PASS` | `package.md` |
| Handoff prompt authored | `PASS` | `prompts/active/wshedw5_kickoff_agent_prompt.md` |
| Deletion inventory | `PASS` | `deletion-inventory.md`; post-edit source scan has no deleted old-runtime symbols |
| Old-runtime deletion manifest | `PASS` | `old-runtime-deletion-manifest.md` |
| Source guards | `PASS` | `cargo nextest run -p openwepp-runner --test watershed_cli_behavior_contract wshedw5_public_cli_uses_typed_network_and_publication_frames` |
| Protected coverage restoration | `PASS` | `cargo nextest run --test wshedw5_typed_watershed_runtime_contract` (`8` passed) |
| Protected outputs | `PASS` | Typed publication test plus full runner/profile coverage |
| `cargo fmt --check` | `PASS` | Ran after final edits |
| `cargo clippy --workspace --all-targets -- -D warnings` | `PASS` | Ran after final production/test edits |
| `cargo nextest run --workspace --profile full` | `PASS` | `1196` tests run: `1196` passed, `1` skipped |
| `cargo deny check` | `PASS` | `advisories ok, bans ok, licenses ok, sources ok` |
| Docs lint | `PASS` | `markdown-doc lint --path ...`: `31 files validated, 0 errors, 0 warnings` |
| `git diff --check` | `PASS` | Ran after final edits |
| Final disposition | `PASS` | `EXECUTED-COMPLETE` |
