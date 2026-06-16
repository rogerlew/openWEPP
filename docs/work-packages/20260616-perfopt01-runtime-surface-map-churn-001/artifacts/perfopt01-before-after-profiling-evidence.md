# PERFOPT01 Before/After Profiling Evidence

Status: IMPLEMENTATION GATES PASSED 2026-06-16
Evidence mode: **Ran** (release build, fixture timings, GDB sampling) + **Static** (source-path attribution)

## Build

Ran:

```text
RUSTFLAGS='-C force-frame-pointers=yes -C debuginfo=1' cargo build --release -p openwepp-runner --bin openwepp-cli-hill
```

Result:

```text
Finished release profile [optimized] target(s) in 1m 00s
```

## Implemented Hot-Path Changes

Static:

- `prepare_persistent_lane_inputs` now extends each cloned lane surface from the shared daily climate surface by reference, avoiding a cloned climate overlay per lane/day.
- `execute_persistent_scheduler_kernel_lifecycle` now captures sequence summary fields before state replacement, then consumes the sequence report and moves each lane `writeback_surface` into persistent state. This removes the previous report-to-persistent-state surface clone.
- `collect_field_violations` now returns immediately for already-valid fields and only constructs the diagnostic `subject` string on potential failure. The failure path still calls the same `check_finite`, `check_range`, `check_min`, and `check_max` helpers with the same invariant/message IDs.

## Timing Results

Baselines for OFE1-OFE5 and H2637 without UI came from PERFHO01 M1 evidence. The H2637 with-UI baseline was captured before source edits for this package.

| Case | OFEs | Before elapsed s | After elapsed s | Speedup | Reduction |
|---|---:|---:|---:|---:|---:|
| ofe1 / p15 | 1 | 6.15 | 5.53 | 1.112x | 10.1% |
| ofe2 / p11 | 2 | 13.90 | 11.80 | 1.178x | 15.1% |
| ofe3 / p12 | 3 | 20.67 | 16.91 | 1.222x | 18.2% |
| ofe4 / p25 | 4 | 32.47 | 27.84 | 1.166x | 14.3% |
| ofe5 / p1 | 5 | 32.30 | 27.04 | 1.195x | 16.3% |
| h2637 / p2637 | 19 | 978.55 | 849.86 | 1.151x | 13.2% |
| h2637 with UI / p2637 | 19 | 968.73 | 851.40 | 1.138x | 12.1% |

Ran timing lines:

```text
PERFOPT01_BASELINE case=h2637_with_ui source=p2637 ofe_count=19 elapsed_s=968.73 user_s=968.25 sys_s=0.37 maxrss_kb=229036
PERFOPT01_AFTER case=ofe1 source=p15 ofe_count=1 elapsed_s=5.53 user_s=5.48 sys_s=0.02 maxrss_kb=20352
PERFOPT01_AFTER case=ofe2 source=p11 ofe_count=2 elapsed_s=11.80 user_s=11.78 sys_s=0.01 maxrss_kb=22096
PERFOPT01_AFTER case=ofe3 source=p12 ofe_count=3 elapsed_s=16.91 user_s=16.84 sys_s=0.05 maxrss_kb=22868
PERFOPT01_AFTER case=ofe4 source=p25 ofe_count=4 elapsed_s=27.84 user_s=27.79 sys_s=0.04 maxrss_kb=24928
PERFOPT01_AFTER case=ofe5 source=p1 ofe_count=5 elapsed_s=27.04 user_s=27.01 sys_s=0.02 maxrss_kb=25728
PERFOPT01_AFTER case=h2637 source=p2637 ofe_count=19 elapsed_s=849.86 user_s=849.30 sys_s=0.42 maxrss_kb=235792
PERFOPT01_AFTER case=h2637_with_ui source=p2637 ofe_count=19 elapsed_s=851.40 user_s=850.83 sys_s=0.44 maxrss_kb=236352
```

## Optimized GDB Re-Check

Ran: GDB user-space stack sampling on optimized H2637 without UI, using the PERFHO01 method. Raw log:

```text
/tmp/perfopt01/gdb-after-h2637.txt
```

Sample window: 10 manual samples during steady-state H2637 daily execution. Percentages are stack-sample shares, not hardware-cycle measurements.

| Category | Samples | Share | Representative stack |
|---|---:|---:|---|
| Runtime-surface lifecycle / lane surface clone | 3/10 | 30.0% | `drop_in_place<HillslopeWritebackSurface>`, `BTreeMap::clone_subtree` under `execute_persistent_scheduler_kernel_lifecycle` |
| Writeback validation / apply detail construction | 0/10 | 0.0% | No `collect_field_violations` or `apply_kernel_writeback` sample |
| Hydrology / transfer guards and symbol formatting | 7/10 | 70.0% | `ensure_no_overflow_indexed_symbols_for_decomposition`, `wb16_publish_ofe_frcteq`, `validate_transfer_array`, `require_erod14_state_scalar`, `run_wb18_percolation_routing`, `require_integral_pl_dispatch_symbol_in_range` |
| Output writers / Parquet | 0/10 | 0.0% | No output writer sample |

Static interpretation: PERFOPT01 shrank the named sampled hot path. Remaining clone samples are from unavoidable per-lane daily input surface construction, not the removed report-to-persistent-state clone. The residual dominant samples now point to hydrology guard/symbol paths, making PERFHO02 the appropriate next characterization.

