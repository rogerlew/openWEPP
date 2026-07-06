# Gate Results

Status: **EXECUTED-HOLD**.

Evidence mode: Ran.

| Gate | Status | Evidence |
|---|---|---|
| `git diff --check` | PASS | Ran at close-out. |
| Markdown/doc lint | PASS | `markdown-doc lint --path ... --no-ignore` validated `19` files, `0` errors, `0` warnings. |
| Focused Lane D / `ofe_routing` tests | PASS | `cargo nextest run -p openwepp-runner hillslope::laned_shadow --no-capture` (`7/7`); `cargo nextest run -p openwepp-hillslope-orchestrator ofe_routing --no-capture` (`67/67`, `522.335 s`). |
| H2637 ignored shadow test | PASS | `cargo nextest run --test laned_shadow_h2637 --run-ignored ignored-only --no-capture` passed after correction (`1/1`, `484.578 s`). |
| H2637 timing path | PASS / HOLD-BUDGET | Release default/off, shadow-on, and shadow-profile all exit `0`; shadow-on is `91.59 s` user / `1:31.67` wall, above D14 budget. |
| Protected-output byte identity | PASS | H2637 ignored test compares shadow-on/off HBP and parquet bytes. No active selector introduced. |
| Active closure / DC01-disable / D13 consumer proof | BLOCKED | Active owner path not implemented; see hold audit. |
| `cargo fmt --check` | PASS | Ran after Rust edit. |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS | Initial run found new-test `float_cmp`; fixed, rerun passed. |
| `cargo nextest run --workspace --profile full` | NOT RUN | Package closes in hold before activation; focused Lane D / H2637 / `ofe_routing` gates were run for the retained terminal-bin fix. |
| `cargo deny check` | NOT RUN | Started after clippy, then interrupted by operator while asking for blocker explanation; not rerun because package is now closing as hold. |
