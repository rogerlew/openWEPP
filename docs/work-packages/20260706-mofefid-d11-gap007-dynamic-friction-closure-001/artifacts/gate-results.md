# Gate Results

Status: **PASS**.

| Gate | Result | Evidence |
| --- | --- | --- |
| Contract-first gate | PASS | Static: `SC-OFEROUTE-001` rev 21 amended before code edits. |
| Focused runner Lane D tests | PASS | Ran: `cargo test -q -p openwepp-runner laned_shadow` (`6` passed). |
| Dynamic operand guard subset | PASS | Ran: `cargo test -q -p openwepp-runner laned_shadow_dynamic_operands` (`3` passed). |
| Routed intensity subset | PASS | Ran: `cargo test -q -p openwepp-runner dynamic_rainfall_intensity_changes_routed_cascade_result` (`1` passed). |
| H2637 missing-extension fail-closed guard | PASS | Ran locally: `cargo test -q --test laned_shadow_h2637 h2637_legacy_shadow_fails_closed_without_routing_coefficients` (`1` passed, `35.86 s`). Heavy runner repeated it (`1` passed, `26.47 s`, `artifacts/runner-logs/h2637_lane_d_shadow_gate.log`). |
| `cargo fmt --check` | PASS | Ran against final tree (`1.66 s`, `artifacts/runner-logs/cargo_fmt_check.log`). |
| `git diff --check` | PASS | Ran locally after final artifact reconciliation. |
| Markdown lint | PASS | Ran after final reconciliation: `markdown-doc lint --path docs/work-packages/README.md --path docs/work-packages/20260706-mofefid-d11-gap007-dynamic-friction-closure-001 --path docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md` (`19` files validated, `0` errors, `0` warnings). |
| Clippy | PASS | Ran against final tree: `cargo clippy --workspace --all-targets -- -D warnings` (`1.72 s`, `artifacts/runner-logs/cargo_clippy.log`). |
| Full nextest | PASS | Ran serialized against final tree after resolving an overlapping-run collision: `cargo nextest run --workspace --profile full` (`1372` passed, `1` skipped, `570.20 s`, `artifacts/runner-logs/cargo_nextest_full.log`). |
| `cargo deny check` | PASS | Ran against final tree (`0.87 s`, `artifacts/runner-logs/cargo_deny_check.log`). |

## Subagent Dispatch

Ran:

- Rust correctness review: `019f354b-7e1b-7910-87fa-fe43d064fbff`.
- Rust QA review: `019f354b-a0c6-7681-bb3c-b24a44a18cb9`.
- Heavy gate runner: `019f354b-b6ae-7432-8d55-636fd9b94598`.

## Gate Hygiene Note

Ran: An overlapping full-nextest attempt failed four snowbench tests while a
second full-nextest process was also generating shared `target/snowdensity*`
fixtures. No Lane D tests failed. After serialization, the current-tree full
nextest gate passed with `1372` tests.
