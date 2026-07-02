# Gate Results

Status: `passed`

Evidence mode: `Ran:`

| Gate | Result | Evidence |
| --- | --- | --- |
| Package scaffold authored | `PASS` | `package.md` |
| Handoff prompt authored | `PASS` | `prompts/active/wshedw6_kickoff_agent_prompt.md` |
| Publication path inventory | `PASS` | `publication-path-inventory.md`, `consumer-path-evidence.md` |
| Publication operand lineage | `PASS` | `publication-operand-lineage.md` |
| Direct typed publication implemented | `PASS` | public CLI calls `write_typed_publication_parquet_outputs`; source guard forbids old row-seed markers |
| Full large fixture adopted | `PASS` | `tests/fixtures/watershed/onshore-xenophobia/`, full `1305` hillslopes; no subset |
| Fixture manifests validate | `PASS` | `sha256sum --quiet -c` passed for `onshore-xenophobia` and `carnivorous-adobo` |
| Scaling matrix recorded | `PASS` | full `onshore-xenophobia` `--jobs 1/48`, full `carnivorous-adobo` `--jobs 1/32`, all `14` parquet outputs identical across job counts |
| Legacy comparison recorded | `PASS` | pinned legacy full runs completed for both fixtures; `legacy-comparison-evidence.md` |
| Output contract evidence | `PASS` | `output-contract-evidence.md` |
| Protected output identity | `PASS` | `protected-output-evidence.md` |
| Conservation reconstruction | `PASS` | independent source-runfile/slope-file area reconstruction for all `32` and `1305` committed hillslopes |
| Review findings dispositioned | `PASS` | `review_agent_a.md`, `review_agent_b.md`, `review-disposition.md` |
| Verification artifacts completed | `PASS` | `verification_agent_a.md`, `verification_agent_b.md` |
| Release build | `PASS` | `/usr/bin/time -v cargo build --release -p openwepp-runner --bins`: `1:06.84`, exit `0` |
| `cargo fmt --check` | `PASS` | post-fix run passed |
| `cargo clippy --workspace --all-targets -- -D warnings` | `PASS` | post-fix run passed |
| `cargo nextest run --workspace --profile full` | `PASS` | post-fix run passed: `1205` tests passed, `1` skipped |
| `cargo deny check` | `PASS` | `advisories ok, bans ok, licenses ok, sources ok` |
| Authority suite anti-evasion shell guard | `PASS` | `bash tools/release/check_authority_suite_antievasion.sh` |
| Authority suite obligation guard | `PASS` | `cargo nextest run --test auth11_required_suite_obligation_guards_contract`: `2` passed |
| Docs lint | `PASS` | `markdown-doc lint --path ...`: `38 files validated, 0 errors, 0 warnings` |
| Source guards | `PASS` | no public CLI row-seed markers; no `/wc1` or `wepppy` in committed fixture run dirs/manifests |
| `git diff --check` | `PASS` | post-fix run passed |
| Final disposition | `PASS` | `EXECUTED-COMPLETE` |
