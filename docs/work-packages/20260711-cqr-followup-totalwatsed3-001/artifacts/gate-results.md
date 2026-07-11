# Gate results

Status: passed
Evidence mode: Ran

Final current-worktree closure gates:

| Gate | Result | Timing / counts | Log |
| --- | --- | --- | --- |
| `cargo fmt --check` | PASS, exit `0` | `2.19s` | `/tmp/fq04-stable-cargo-fmt.log` |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS, exit `0` | `1.74s` | `/tmp/fq04-stable-cargo-clippy.log` |
| `cargo nextest run --workspace --profile full` | PASS, exit `0` | `1776/1776` passed, `4` slow, `3` skipped across `171` binaries; test time `590.139s`, elapsed `592.72s`; run ID `fb1f0fd0-96aa-49b3-b92b-587ee3d446d4` | `/tmp/fq04-stable-cargo-nextest-full.log` |
| `cargo deny check` | PASS, exit `0` | `0.85s`; advisories, bans, licenses, and sources all `ok` | `/tmp/fq04-stable-cargo-deny.log` |
| `git diff --check` | PASS, exit `0` | `0.02s` | `/tmp/fq04-stable-git-diff-check.log` |

The full-profile run started after the final `17/17` target-test state was in
place. No repository file was edited by the gate commands.
