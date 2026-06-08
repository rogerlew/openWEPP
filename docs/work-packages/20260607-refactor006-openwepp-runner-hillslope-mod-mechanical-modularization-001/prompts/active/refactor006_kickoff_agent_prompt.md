# REFACTOR006 Kickoff Agent Prompt

Scope: local repository engineering task; flat-file reads/edits only; no
external connectivity.
Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.
Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260607-refactor006-openwepp-runner-hillslope-mod-mechanical-modularization-001/package.md`
- `/workdir/openWEPP/crates/openwepp-runner/src/lib.rs`
- `/workdir/openWEPP/crates/openwepp-runner/src/hillslope/mod.rs`
- `/workdir/openWEPP/crates/openwepp-runner/Cargo.toml`
- `/workdir/openWEPP/tests/integration/cli01_runner_hillslope_integration.rs`
- `/workdir/openWEPP/tests/integration/cli03_runner_contract_derived_tests.rs`
- `/workdir/openWEPP/tests/integration/cli04_runner_wat_parquet_contract_derived_tests.rs`

Files:
- `crates/openwepp-runner/src/hillslope/mod.rs`
- `crates/openwepp-runner/src/hillslope/*.rs`
- `tests/integration/cli03_runner_contract_derived_tests.rs`
- `tests/integration/hparity02_profile_capacity_parity_contract.rs`
- `tests/integration/hphys0289_wb13_rm_snowwater_publication_contract.rs`
- `tests/integration/hphys0290_post_winter_rain_publication_contract.rs`
- `tests/integration/hphys0291_snow_publication_lifecycle_contract.rs`
- `tests/integration/hphys0293_winter_melt_timing_contract.rs`
- `tests/integration/hphys0294_post_ingress_storage_retention_contract.rs`
- `tests/integration/hphys0295_cumulative_storage_budget_contract.rs`
- `tests/integration/hphys0296_snow_rm_acceptance_authority_contract.rs`
- `tests/integration/hphys0299_hourly_snow_partition_unit_provenance_contract.rs`
- `tests/integration/hphys0305_paired_melt_term_state_contract.rs`
- `tests/integration/hphys0318_stmtim_control_surface_instrumentation_contract.rs`
- `docs/work-packages/20260607-refactor006-openwepp-runner-hillslope-mod-mechanical-modularization-001/artifacts/*.md`

Task: execute REFACTOR006 objective end-to-end for declared scope.
Constraints: mechanical modularization only; preserve behavior and public API;
no intentional runtime semantic changes; typed errors/guards must remain
intact; no silent fallback additions.
Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.
Outputs: updated modules, updated tests, and complete package artifacts with
`Static`/`Ran` evidence.

Mandatory execution notes:
- Preserve exported symbols currently consumed by bins and integration tests.
- Update brittle source-layout assertions (for example, checks that require
  implementation text to reside in one file) to architecture-stable
  behavior/API checks or module-aware checks.
- Run required validation gates from package exit criteria.
- Complete dual review and dual verification artifacts before disposition.
