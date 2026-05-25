# REFACTOR001 Kickoff Agent Prompt

Scope: local repository engineering task; flat-file reads/edits only; no
external connectivity.
Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.
Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260525-refactor001-openwepp-runner-lib-mechanical-modularization-001/package.md`
- `/workdir/openWEPP/crates/openwepp-runner/src/lib.rs`
- `/workdir/openWEPP/crates/openwepp-runner/Cargo.toml`
- `/workdir/openWEPP/tests/integration/cli01_runner_hillslope_integration.rs`
- `/workdir/openWEPP/tests/integration/cli03_runner_contract_derived_tests.rs`
- `/workdir/openWEPP/tests/integration/cli04_runner_wat_parquet_contract_derived_tests.rs`
- `/workdir/openWEPP/tests/integration/pl14s_tier_a_candidate_emission_and_replay_contract.rs`

Files:
- `crates/openwepp-runner/src/lib.rs`
- `crates/openwepp-runner/src/*.rs`
- `crates/openwepp-runner/src/hillslope/*.rs`
- `tests/integration/cli03_runner_contract_derived_tests.rs`
- `docs/work-packages/20260525-refactor001-openwepp-runner-lib-mechanical-modularization-001/artifacts/*.md`

Task: execute REFACTOR001 objective end-to-end for declared scope.
Constraints: mechanical modularization only; preserve behavior and public API;
no intentional runtime semantic changes; typed errors/guards must remain
intact; no silent fallback additions.
Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.
Outputs: updated modules, updated tests, and complete package artifacts with
`Static`/`Ran` evidence.

Mandatory execution notes:
- Preserve exported symbols currently consumed by bins and integration tests.
- Update brittle source-layout assertions (for example `include_str!` on a
  single file) to behavior/API oriented checks or module-tree aware checks.
- Run required validation gates from package exit criteria.
- Complete dual review and dual verification artifacts before disposition.
