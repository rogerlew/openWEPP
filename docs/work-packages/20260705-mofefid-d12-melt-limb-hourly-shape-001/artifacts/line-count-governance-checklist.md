# Line-Count Governance Checklist

Status: **COMPLETE**.

Ran: `wc -l` on touched Rust files.

Warnings:

| File | Lines | Disposition |
|---|---:|---|
| `crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs` | 2528 | Existing large production module; D12 touched one shared helper and did not broaden module ownership. Split follow-up deferred to runtime refactor work. |
| `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/infiltration_reconciliation.rs` | 2033 | Existing large production module; D12 added one producer helper and focused test. Split follow-up deferred to hydrology helper refactor work. |

Resolved governance item:

- `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs`
  was reduced from `3243` to `2988` lines by moving the adjacent DC01 tests to
  `direct_runtime_dc01.rs` (`257` lines). No touched Rust file remains at or
  above the 3000-line hard threshold.

All remaining touched Rust files are below 2000 lines.
