# Gate Results

Status: **EXECUTED-HOLD**.

Evidence mode: Ran + Static.

| Gate | Status | Evidence |
|---|---|---|
| `git diff --check` | PASS | Ran locally; clean |
| Markdown/doc lint for touched docs | PASS | `markdown-doc lint --path ... --no-ignore`; 16 files, 0 errors, 0 warnings |
| Contract/profile/BEI checks for touched `SC-*` contracts | NOT RUN | No `SC-*` contract was modified; not triggered |
| Focused Lane D / `ofe_routing` tests | FAIL / BLOCKED | `cargo nextest run --test laned_shadow_h2637 --run-ignored ignored-only --no-capture` failed with `NegativeOutletBin`; this is the hold evidence |
| H2637 timing run with opt-in routed path enabled | FAIL / BLOCKED | Shadow-on timing failed with `NegativeOutletBin` before endpoint; profile run failed before emitting slot JSON |
| Protected-output byte identity with subsystem off | PARTIAL PASS | Default/off H2637 timing run completed; no active selector exists, so active-selector off identity is not runnable |
| Active-mode closure evidence for `INV-OFEROUTE-012` | BLOCKED | No active production path invokes seam closure hard-fail |
| DC01-disable / no-double-feed proof | BLOCKED | Production path still calls DC01 runon admission; no active selector |
| Routed-hydrograph-to-erosion consumer proof | BLOCKED | D13 candidate consumer tests exist, but no active routed producer feeds it |
| `cargo fmt --check` | NOT RUN | Held before code implementation; no Rust changes retained |
| `cargo clippy --workspace --all-targets -- -D warnings` | NOT RUN | Held before code implementation; no Rust changes retained |
| `cargo nextest run --workspace --profile full` | NOT RUN | Held before implementation; focused H2637 gate already fails |
| `cargo deny check` | NOT RUN | Held before implementation; no dependency changes |
| Authority anti-evasion guards if triggered | NOT RUN | No required-case binding, cohort fixture, or external-authority suite posture touched |

## Subagents

- `comparator_suite_runner`: dispatched for H2637 timing refresh.
- `explorer`: dispatched for read-only activation readiness audit.

## Comparator Timing Summary

The package-authorized timing subagent ran the release H2637 path read-only.
Default/off completed near the old D14 budget; shadow-on and shadow-on-profile
both failed with `NegativeOutletBin`. Solver counters, step counts, and slot
JSON are unavailable because the routed path aborts before finalize.

Copied logs are under `artifacts/logs/`.
