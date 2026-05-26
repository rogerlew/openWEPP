# REFACTOR003 Kickoff Agent Prompt

Scope: local repository engineering task; flat-file reads/edits only; no
external connectivity.
Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.
Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260525-refactor003-openwepp-hillslope-orchestrator-hydrology-mechanical-modularization-001/package.md`
- `/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/hydrology.rs`
- `/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/Cargo.toml`

Files:
- `crates/openwepp-hillslope-orchestrator/src/hydrology.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/mod.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/*.rs`
- `docs/work-packages/20260525-refactor003-openwepp-hillslope-orchestrator-hydrology-mechanical-modularization-001/artifacts/*.md`

Task: execute REFACTOR003 objective end-to-end for declared scope.
Constraints: mechanical modularization only; preserve behavior and public API;
no intentional runtime semantic changes; typed errors/guards must remain
intact; no silent fallback additions.
Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.
Outputs: updated modules and complete package artifacts with `Static`/`Ran`
evidence.

Mandatory execution notes:
- Preserve exported symbols consumed by integration tests.
- Preserve canonical hydrology dispatch/guard/kernel behavior without formula
  rewrites.
- Run required validation gates from package exit criteria.
- Complete dual review and dual verification artifacts before disposition.
