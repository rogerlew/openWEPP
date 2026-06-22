# Verification

Status: executed-held.

## Static

- Static: reviewed `execute_hillslope_direct_production_days` and confirmed
  direct production invokes `DirectFrameExecutor::run_publication_capture_with_interleaved_day_inputs`
  and returns empty compatibility `wb13_rows`/`pass_rows`.
- Static: reviewed `build_direct_publication_artifacts` and confirmed the
  `DirectProductionExecutor` branch uses retained direct publication artifacts,
  not the compatibility WB13 adapter.
- Static: reviewed `build_direct_production_run_frame` and confirmed lane
  constructor inputs are topology/area only.
- Static: reviewed `DirectPublicationDayInputBuilder` and confirmed production
  direct day-input construction still clones a single aggregate
  `HillslopeWritebackSurface`.
- Static: reviewed `OfeLanePersistentStateSequence` and confirmed per-OFE
  compatibility scheduler state exists, but no typed direct constructor bridge
  is wired into production direct execution.

## Ran

- Ran: focused fixture default and production-direct CLI executions. HBP, loss,
  PASS, and WAT checksums matched.
- Ran: separated H2637 default and production-direct CLI executions. HBP, PASS,
  and WAT checksums differed; loss and plot checksums matched.
- Ran: H2637 DuckDB `except all` comparisons. WAT differed in both directions
  by `235961` rows; PASS differed in both directions by `12419` rows.
- Ran: H2637 joined field-difference summaries. WAT hydrology/storage/ET fields
  differed across most rows; PASS `runvol` differed on `12372` rows and
  `sbrunv` on `12419` rows.
- Ran: H2637 manifest inspection. Production direct reports
  `execution_provenance.scheduler_kernel_executed=false`,
  `execution_provenance.publication_source=direct-publication-frame`,
  `wb13_publication.source=direct-publication-frame`, `row_count=235961`, and
  `direct_runtime_counters.compatibility_edge_invocations=0`.
- Ran: `git diff --check` passed.
- Ran: `markdown-doc lint --path docs/work-packages/20260622-r7d-direct-publication-producer-authority-001 --no-ignore`
  validated 8 files with 0 errors and 0 warnings.
- Ran: `markdown-doc lint --path docs/work-packages/README.md --no-ignore`
  validated 1 file with 0 errors and 0 warnings.
- Ran: `markdown-doc lint --path docs/architecture/array-native-runtime-specification.md --no-ignore`
  validated 1 file with 0 errors and 0 warnings.
- Not run: full Rust closure gates. No production code edits were made after
  the blocker was classified, and the package intentionally closes in a named
  hold before full implementation closure.
