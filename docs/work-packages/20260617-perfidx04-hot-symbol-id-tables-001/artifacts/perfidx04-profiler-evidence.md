# PERFIDX04 Profiler Evidence

Ran:
- Command: `perf record -F 99 --call-graph fp -o /tmp/perfidx04/perf-h2637-current.data -- timeout --signal=INT 90s /tmp/perfidx04/current/bin/openwepp-cli-hill --run-dir /tmp/perfho01/run-dirs/h2637 --run-file /tmp/perfidx04/runfiles/h2637_same_current.run --output-dir /tmp/perfidx04/current/h2637_perf_manifest --policy compat --legacy-sidecar-discovery`
- Exit code: `124` from expected timeout.
- Samples: 9,495.
- Reports:
  - `/tmp/perfidx04/perf-h2637-current-report-nochildren.txt`
  - `/tmp/perfidx04/perf-h2637-current-report-children.txt`

Ran:
- `perf_event_paranoid` no longer blocked recording. `perf` warned only about restricted kernel symbol maps via `kptr_restrict`; user-space symbols resolved.
- Final no-children report shows direct `hourly_symbol` at 0.01% self.
- Hot paths now show `hourly_symbol_for_request`, `frost_fine_layer_symbol_for_request`, `HotSymbolTables::state_grid_symbol`, and `require_integral_pl_dispatch_symbol_ref_in_range` rather than per-access direct `format!` symbol construction for the named hot families.

Static:
- Remaining `alloc::fmt::format::format_inner` samples are not from the old direct hot hourly formatter. They are residual cold/logical-name construction paths including layer-symbol/export and PL payload areas that are outside Stage-4 read-id migration or deferred to Stage 5 writeback/guard-by-id work.
