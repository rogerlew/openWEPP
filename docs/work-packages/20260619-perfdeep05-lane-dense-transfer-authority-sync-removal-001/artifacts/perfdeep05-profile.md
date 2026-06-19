# PERFDEEP05 Profile

Evidence class: Ran + Static.

## Profile Command

Captured before the final clippy-only helper extraction; the final release
endpoint was rerun afterward and confirmed the same no-go timing disposition.

```text
/usr/bin/time -f "perfdeep05_h2637_optin_perf_record\t%e\t%M" \
  perf record -F 49 --call-graph dwarf,8192 \
  -o /tmp/perfdeep05/profile/perfdeep05-h2637-optin.data \
  -- env OPENWEPP_PERFDEEP03_LANE_DENSE_STATE=1 \
  target/release/openwepp-cli-hill \
  --run-dir /tmp/perfho01/run-dirs/h2637 \
  --run-file docs/work-packages/20260619-perfdeep05-lane-dense-transfer-authority-sync-removal-001/artifacts/runfiles/perfdeep05-h2637.run \
  --output-dir /tmp/perfdeep05/profile-run/h2637_same_manifest \
  --policy compat \
  --legacy-sidecar-discovery
```

Result:

```text
perfdeep05_h2637_optin_perf_record  917.07  409596
[ perf record: Captured and wrote 386.527 MB /tmp/perfdeep05/profile/perfdeep05-h2637-optin.data (48028 samples) ]
Total Lost Samples: 0
```

Kernel symbols were partially unresolved because `/proc/kallsyms` was
restricted, but user-space Rust symbols resolved and are sufficient for the
PERFDEEP05 hotspot question.

Repository text reports:

- `raw/perfdeep05-h2637-optin-header.txt`
- `raw/perfdeep05-h2637-optin-children-report.txt`
- `raw/perfdeep05-h2637-optin-flat-report.txt`
- `raw/perfdeep05-h2637-optin-children-top.txt`
- `raw/perfdeep05-h2637-optin-flat-top.txt`

Raw binary profile remains in `/tmp/perfdeep05/profile/`.

## Hotspot Disposition

PERFDEEP04 measured:

```text
HillslopeLaneDenseState::sync_from_writeback_surface
33.49% children, 14.19% self
```

PERFDEEP05 profile result:

- `sync_from_writeback_surface` is absent from the profile reports.
- `HotSymbolTables::hot_state_symbols` is present only as a `0.00%` report row.
- Direct transfer application is not a material cost:
  `apply_transfer_input_to_lane_dense_state` appears at `0.14% children /
  0.01% self`.

The removed hotspot was replaced by remaining dense-edge costs:

| Children | Self | Symbol |
|---:|---:|---|
| 31.50% | n/a | `execute_persistent_scheduler_kernel_lifecycle` |
| 29.84% | n/a | `HillslopePhaseScheduler::execute_with_kernel_indexed_internal` |
| 16.20% | 9.07% | `HillslopeLaneDenseState::refresh_cached_slots_from_writeback_surface` |
| 10.47% | 0.35% | `HillslopeLaneDenseState::apply_kernel_writeback_payload` |
| 7.72% | 4.59% | `SymbolRegistry::id_of` |
| 6.72% | 0.44% | `HillslopeLaneDenseState::flush_dirty_to_writeback_surface` |

Flat top costs include:

| Self | Symbol |
|---:|---|
| 19.83% | `__memcmp_sse2` |
| 9.07% | `HillslopeLaneDenseState::refresh_cached_slots_from_writeback_surface` |
| 6.08% | `ensure_no_overflow_indexed_symbols_for_decomposition` |
| 4.59% | `SymbolRegistry::id_of` |
| 3.01% | `BTreeMap::insert` |
| 2.97% | `_int_free` |
| 2.96% | `malloc` |
| 2.95% | `HotSymbolTables::state_grid_symbol` |

## Interpretation

PERFDEEP05 disproved the specific PERFDEEP04 full-resync hotspot: the old sync
is gone, and hot-symbol-list rebuild is no longer material.

The opt-in path is still not endpoint-positive because the partial dense island
continues to pay dense-edge compatibility costs:

- daily cached-slot refresh from prepared logical/indexed surfaces;
- logical dense writeback application that still reaches `SymbolRegistry::id_of`;
- dirty dense flush back to logical/indexed surfaces;
- remaining kernel guard and symbol-table work in unmigrated hydrology bodies.

The next package should not revert the lane-owned dense state, but it also
should not expand blindly. The profile points at either removing the daily
cached-slot refresh boundary or moving the remaining logical writeback
application to indexed/dense authority before any larger kernel-body rewrite.
