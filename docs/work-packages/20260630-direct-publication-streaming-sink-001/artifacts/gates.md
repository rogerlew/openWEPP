# Gates

Evidence class: Ran

| Gate | Result | Evidence |
| --- | --- | --- |
| `cargo fmt --check` | pass | ran after implementation/refactor |
| `cargo clippy --workspace --all-targets -- -D warnings` | pass | passed after extracting streaming helper and simplifying infallible provenance builder |
| `cargo nextest run --workspace --profile full` | pass | second run: `1856` passed, `1` skipped, `2` slow, `653.393s` |
| `.venv` capable full suite | pass | created `.venv`, installed `pyarrow==22.0.0` from `tools/owcmp/requirements.lock.txt`, installed `pandas==3.0.3` for legacy HPHYS diagnostics |
| `cargo deny check` | pass | advisories, bans, licenses, sources ok |
| Authority anti-evasion | pass | `bash tools/release/check_authority_suite_antievasion.sh` |
| Required-suite obligation guard | pass | `cargo test --test auth11_required_suite_obligation_guards_contract -- --nocapture` |
| Scoped Markdown lint | pass | `markdown-doc lint --path docs/work-packages/README.md --path docs/work-packages/20260630-direct-publication-streaming-sink-001 --format json`: `10` files, `0` errors, `0` warnings |
| Scoped Markdown validate | pass | `markdown-doc validate --path docs/work-packages/README.md --path docs/work-packages/20260630-direct-publication-streaming-sink-001 --format json`: `10` files, `0` errors |
| Whitespace diff check | pass | `git diff --check` |
| Focused direct runtime tests | pass | `cargo test -p openwepp-hillslope-orchestrator direct_runtime -- --nocapture` |
| Focused output writer tests | pass | `cargo test -p openwepp-hillslope-output -- --nocapture` |
| Focused runner direct publication tests | pass | `cargo test -p openwepp-runner direct_publication -- --nocapture` |
| RSS slope | pass | H2637 required-only `52228 KiB`; W9 longer-day `47856 KiB`; cli01 `20736 KiB` |
| Identity | pass | H2637 full and cli01 data outputs byte-identical to retained-row baseline |

The first full nextest attempt ran in a venv-capable environment but failed
because the freshly created `.venv` lacked Python dependencies used by existing
contracts: `pyarrow` for `owcmp` and `pandas` for the HPHYS0298 diagnostic
chain. After installing those dependencies into the untracked repo-local
`.venv`, targeted reruns of both failures passed and the full profile passed.
