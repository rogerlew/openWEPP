# Gates

Evidence class: Ran

## Package Gates

| Gate | Result | Evidence |
| --- | --- | --- |
| Profile before fixing | PASS | Stage A RSS, jemalloc stats, and static size accounting recorded. |
| H2637 RSS measured after each step | PASS | False-start and final H2637 full/minimized measurements recorded. |
| Shorter run RSS measured | PASS | `cli01` post-fix RSS recorded at `19584 KiB`. |
| RSS slope flattened | HOLD | H2637 minimized output remains `184644 KiB` vs `cli01` `19584 KiB`; whole-run publication rows remain retained. |
| H2637 full-output byte identity | PASS | HBP/WAT/PASS/loss/plot all byte-identical to baseline. |
| H2637 minimized-output byte identity | PASS | HBP/loss byte-identical to baseline. |
| Compatibility proof counters | PASS | H2637 full/minimized and `cli01` manifests show `compatibility_edge_invocations=0`. |
| No runtime-selection regression | PASS | Manifests selected `direct-production-executor`. |

## Rust and Guard Gates

| Gate | Result | Notes |
| --- | --- | --- |
| `cargo check -p openwepp-runner` | PASS | Focused compile gate. |
| `cargo test -p openwepp-runner direct_publication -- --nocapture` | PASS | `2` tests passed. |
| `cargo test -p openwepp-hillslope-orchestrator direct_runtime -- --nocapture` | PASS | `104` tests passed. |
| `cargo fmt --check` | PASS | Formatting clean. |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS | Clean after replacing `map_or` with `is_none_or`. |
| `cargo deny check` | PASS | Dependency policy clean. |
| `bash tools/release/check_authority_suite_antievasion.sh` | PASS | Authority anti-evasion clean. |
| `cargo test --test auth11_required_suite_obligation_guards_contract` | PASS | `2` tests passed. |
| `cargo nextest run --workspace --profile full` | FAIL-ENV | `1848` passed, `8` failed, `1` skipped. Failures were in harness/diagnostic tests that launch `.venv/bin/python` or an environment checker; `.venv/bin/python` was absent in this worktree. |
| Scoped Markdown lint/validate | PASS | `markdown-doc lint` and `markdown-doc validate` passed for the package and work-package README. |

## Disposition Gate

The package cannot close as complete because the requested acceptance condition
was run-length-flat RSS. It closes as a held partial reduction with a named
remaining blocker.
