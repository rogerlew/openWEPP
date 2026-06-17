# PERFIDX01 Line-Count Governance

Status: PASS WITH EXISTING WARN 2026-06-16
Evidence mode: **Ran** (`wc -l`) + **Static**

## Touched Rust Files

| File | Before lines | After lines | Delta | Governance disposition |
|---|---:|---:|---:|---|
| `crates/openwepp-kernel-contract/src/lib_mod/core_types.rs` | 1498 | 1806 | +308 | OK, below 2000 |
| `crates/openwepp-kernel-contract/src/lib.rs` | 345 | 418 | +73 | OK |
| `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs` | 2360 | 2366 | +6 | WARN, already above 2000 and below 3000 |
| `crates/openwepp-runner/src/hillslope/mod.rs` | 6 | 7 | +1 | OK |
| `crates/openwepp-runner/src/hillslope/symbol_registry_audit.rs` | 0 | 931 | +931 | OK |

Ran:

```text
wc -l crates/openwepp-kernel-contract/src/lib_mod/core_types.rs \
  crates/openwepp-kernel-contract/src/lib.rs \
  crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs \
  crates/openwepp-runner/src/hillslope/mod.rs \
  crates/openwepp-runner/src/hillslope/symbol_registry_audit.rs
```

After result:

```text
  1806 crates/openwepp-kernel-contract/src/lib_mod/core_types.rs
   418 crates/openwepp-kernel-contract/src/lib.rs
  2366 crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs
     7 crates/openwepp-runner/src/hillslope/mod.rs
   931 crates/openwepp-runner/src/hillslope/symbol_registry_audit.rs
  5528 total
```

## Disposition

`00_runner_intake_and_lane_setup.rs` remains in WARN territory. PERFIDX01 added
only a six-line env-gated audit hook to the existing orchestration file. The
new registry enumeration logic is isolated in `symbol_registry_audit.rs` and is
below the 2000-line warning threshold.

No touched file reaches the 3000-line required-refactor threshold.

