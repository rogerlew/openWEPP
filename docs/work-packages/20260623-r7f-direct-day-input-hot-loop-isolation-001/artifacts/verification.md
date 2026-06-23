# Verification

Status: complete.

## Static

- `r7c_direct_production_source_excludes_compatibility_entrypoints` proves the
  production direct function body does not reference compatibility scheduler,
  kernel, day-input builder, or compatibility-edge counter entrypoints.
- `r7f_production_direct_uses_typed_day_input_builder` proves production direct
  instantiates `DirectProductionDayInputBuilder` and not
  `DirectPublicationDayInputBuilder`.
- `r7f_typed_day_input_hot_loop_excludes_runtime_surface_reads` proves the
  typed production builder hot-loop body does not reference runtime-surface
  maps or compatibility builders.

## Ran

- `cargo test -p openwepp-runner r7 -- --nocapture`:
  passed, 12 tests.
- `cargo test -p openwepp-runner r6 -- --nocapture`:
  passed, 11 lib tests plus
  `r6_direct_publication_cutover_cli_flag_writes_direct_outputs_and_manifest`.
- `cargo fmt --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `cargo deny check`: passed (`advisories ok, bans ok, licenses ok, sources
  ok`).
- `cargo clippy -p openwepp-runner --all-targets -- -D warnings`: passed.
- `git diff --check`: passed.
- `markdown-doc lint --path docs/work-packages/20260623-r7f-direct-day-input-hot-loop-isolation-001 --path docs/work-packages/README.md --format json`:
  passed with 13 files scanned, 0 errors, 0 warnings.
- `wc -l ...`: recorded in `line-count.md`.

## Not Run

- None required for R7F closure.
