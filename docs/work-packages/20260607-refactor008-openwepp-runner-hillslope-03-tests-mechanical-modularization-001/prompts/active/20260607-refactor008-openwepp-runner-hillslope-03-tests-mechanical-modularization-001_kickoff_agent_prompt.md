# REFACTOR008 Kickoff Agent Prompt

Scope: local repository engineering task; flat-file reads/edits only; no
external connectivity.
Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.
Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/standards/mechanical-refactor-authoring-guide.md`
- `/workdir/openWEPP/docs/prompt_templates/mechanical-refactor-kickoff-template.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/work-packages/20260607-refactor008-openwepp-runner-hillslope-03-tests-mechanical-modularization-001/package.md`
- `/workdir/openWEPP/crates/openwepp-runner/src/hillslope/03_tests.rs`

Files:
- `crates/openwepp-runner/src/hillslope/03_tests.rs`
- `crates/openwepp-runner/src/hillslope/tests03/*.rs`
- `tests/integration/cli03_runner_contract_derived_tests.rs` (if required)
- `tests/integration/hphys*.rs` only for layout-coupled assertion updates
- `docs/work-packages/20260607-refactor008-openwepp-runner-hillslope-03-tests-mechanical-modularization-001/artifacts/*.md`

Task: execute REFACTOR008 objective end-to-end for declared scope.
Constraints: mechanical modularization only; preserve behavior and guard/test
intent; no intentional runtime semantic changes; no fallback additions;
no canonicalize-and-proceed handling for invalid domain state.
Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.
Outputs: updated test modules and complete package artifacts with
`Static`/`Ran` evidence.

Mandatory execution notes:
- Capture pre/post test symbol inventories and line counts for touched `.rs` files.
- Ensure post-refactor `03_tests.rs` falls below 3000 lines.
- Update layout-coupled integration assertions only when needed for
  module-aware source aggregation checks.
- Run required validation gates from package exit criteria.
- Complete dual review and dual verification artifacts before disposition.
