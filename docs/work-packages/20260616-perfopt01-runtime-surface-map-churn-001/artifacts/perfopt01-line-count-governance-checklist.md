# PERFOPT01 Line-Count Governance Checklist

Status: PASS WITH EXISTING WARN 2026-06-16
Evidence mode: **Ran** (`wc -l`) + **Static** (mechanical-refactor scope review)

## Touched Rust Files

| File | Before lines | After lines | Delta | Governance disposition |
|---|---:|---:|---:|---|
| `crates/openwepp-runner/src/hillslope/intake_lane_setup/mod.rs` | 22 | 23 | +1 | OK |
| `crates/openwepp-runner/src/hillslope/intake_lane_setup/runtime_surface_helpers.rs` | 135 | 153 | +18 | OK |
| `crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_seed_and_runtime.rs` | 2584 | 2632 | +48 | WARN: already above 2000, below 3000 |
| `crates/openwepp-kernel-contract/src/lib_mod/writeback.rs` | 217 | 238 | +21 | OK |

Ran:

```text
wc -l crates/openwepp-runner/src/hillslope/intake_lane_setup/mod.rs \
  crates/openwepp-runner/src/hillslope/intake_lane_setup/runtime_surface_helpers.rs \
  crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_seed_and_runtime.rs \
  crates/openwepp-kernel-contract/src/lib_mod/writeback.rs
```

After result:

```text
    23 crates/openwepp-runner/src/hillslope/intake_lane_setup/mod.rs
   153 crates/openwepp-runner/src/hillslope/intake_lane_setup/runtime_surface_helpers.rs
  2632 crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_seed_and_runtime.rs
   238 crates/openwepp-kernel-contract/src/lib_mod/writeback.rs
  3046 total
```

## Disposition

`scheduler_seed_and_runtime.rs` remains in WARN territory. PERFOPT01 did not perform a structural split because the work package was a behavior-preserving optimization with strict bit-identity gates, and the change was localized to an existing hot-path lifecycle function. The file remains below the 3000-line refactor threshold.

No new abstraction broadening, physics movement, or contract authority change was introduced.

