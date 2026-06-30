# Verification

Evidence mode: Static/Ran.

## Static Verification

Static:

- Verified the implemented sub-computations are pure typed projection functions
  with `HillslopeWritebackSurface` limited to writer/reader adapters around
  them.
- Verified production direct execution calls
  `DirectProductionSeedAuthority::from_typed_inputs`.
- Verified the snowbench diagnostic replay also uses the typed seed authority.
- Verified no production caller reaches `from_day_zero_seed_surfaces` or
  `direct_publication_day_zero_seed_surface`.
- Verified the direct-production setup branch no longer calls static
  `HillslopeWritebackSurface` seed construction, persistent lane symbol-map
  state, symbol registry, or hot-table construction.
- Verified snowbench/PySnobal export diagnostics source initial canopy cover,
  winter hourly context, and snow density from the typed seed carrier rather
  than from the deleted setup `runtime_surface`.
- Verified retained symbol-map scheduler/runtime helpers are confined to tests
  and non-direct compatibility selection; the explicit deprecated
  `--compatibility-runtime` seam remains in scope to retain.

## Ran Verification

```text
cargo fmt --check
cargo check -p openwepp-runner
cargo nextest run -p openwepp-runner publication_wb11_seed publication_wb19_wb12_wb16
cargo nextest run r7f_typed_day_input_hot_loop_excludes_runtime_surface_reads physics_bulk_snowbench_runs_offline_for_snotel_fixture coe_melt_snowbench_runs_both_models_as_diagnostic_only snowdensity05g_coe_melt_replay_uses_configured_canopy_and_proven_shortwave density_compaction_snowbench_runs_offline_with_closure coe_bound_density_replay_preserves_coe_swe_and_changes_density_surface snowdensity10_3_1a_coe_melt_consumes_daily_canopy_sidecar g0_exporter_emits_pysnobal_schema_and_required_anti_alias_lineage --no-fail-fast
cargo nextest run -p openwepp-runner r7f_typed_day_input_hot_loop_excludes_runtime_surface_reads typed_seed_authority_direct_setup_skips_symbol_map_seed_surface
cargo test -p openwepp --test cli03_runner_contract_derived_tests cli03_mofe03_multiofe_runfile_executes_wave2_without_manual_symbol_injection -- --nocapture
cargo clippy --workspace --all-targets -- -D warnings
cargo nextest run --workspace --profile full
cargo deny check
bash tools/release/check_authority_suite_antievasion.sh
cargo test --test auth11_required_suite_obligation_guards_contract -- --nocapture
markdown-doc lint --path docs/work-packages/README.md --path docs/work-packages/20260630-typed-day-zero-seed-computation-001 --format json
markdown-doc validate --path docs/work-packages/README.md --path docs/work-packages/20260630-typed-day-zero-seed-computation-001
git diff --check
```

Results:

- `cargo fmt --check`: pass.
- `cargo check -p openwepp-runner`: pass.
- Focused seed tests: `41` run, `41` passed.
- Snowbench/PySnobal diagnostic rerun: `7` run, `7` passed (`633.961s`,
  `1` slow).
- Setup/source guards: `2` run, `2` passed.
- Multi-OFE/Wave-2 focused test: pass.
- `cargo clippy --workspace --all-targets -- -D warnings`: pass.
- H2637 protected output identity: HBP/loss/PASS/WAT/plot byte-identical
  against clean `5b139058`.
- H2637 RSS/time: clean baseline `1:09.02`, `113268 KiB`; current `1:08.62`,
  `91692 KiB`.
- H2637 after direct setup seed-surface deletion and snowbench diagnostic
  cutover: HBP/loss/PASS/WAT/plot byte-identical; `1:07.35`, `84776 KiB`;
  `direct_runtime_counters.compatibility_edge_invocations=0`.
- cli01 protected output identity: HBP/loss/WAT/plot byte-identical.
- `cargo nextest run --workspace --profile full`: `1880` passed, `1` skipped,
  `1` slow, `638.672s`.
- `cargo deny check`: pass.
- Authority anti-evasion: pass.
- Required-suite obligation guard: `2` passed.
- `markdown-doc lint`: `10` files scanned, `0` errors, `0` warnings.
- `markdown-doc validate`: `10` files, `0` errors.
- `git diff --check`: no findings.

## Pending

- Phase 3 broader deletion of scheduler/day-frame/carrier code that is no
  longer needed outside tests or the explicit deprecated
  `--compatibility-runtime` seam.
- Phase 4 full no-compatibility static call-graph proof after the broader
  deletion.
