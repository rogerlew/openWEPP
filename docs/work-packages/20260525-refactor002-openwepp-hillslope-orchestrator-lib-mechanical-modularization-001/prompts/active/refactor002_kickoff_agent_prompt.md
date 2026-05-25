# REFACTOR002 Kickoff Agent Prompt

Scope: local repository engineering task; flat-file reads/edits only; no
external connectivity.
Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.
Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260525-refactor002-openwepp-hillslope-orchestrator-lib-mechanical-modularization-001/package.md`
- `/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/Cargo.toml`
- `/workdir/openWEPP/tests/integration/arch22_typed_state_surface_contract.rs`
- `/workdir/openWEPP/tests/integration/hillslope_consumer_boundary_integration.rs`
- `/workdir/openWEPP/tests/integration/kernel_writeback_contract.rs`

Files:
- `crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `crates/openwepp-hillslope-orchestrator/src/*.rs`
- `tests/integration/arch22_typed_state_surface_contract.rs`
- `docs/work-packages/20260525-refactor002-openwepp-hillslope-orchestrator-lib-mechanical-modularization-001/artifacts/*.md`

Task: execute REFACTOR002 objective end-to-end for declared scope.
Constraints: mechanical modularization only; preserve behavior and public API;
no intentional runtime semantic changes; typed errors/guards must remain
intact; no silent fallback additions.
Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.
Outputs: updated modules, updated tests, and complete package artifacts with
`Static`/`Ran` evidence.

Mandatory execution notes:
- Preserve exported symbols currently consumed by integration tests.
- Update brittle source-layout assertions (for example direct reads of a
  single `lib.rs` file) to behavior/API oriented checks or module-tree aware
  checks.
- Run required validation gates from package exit criteria.
- Complete dual review and dual verification artifacts before disposition.
