# REFACTOR014 Kickoff Agent Prompt

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
- `/workdir/openWEPP/docs/work-packages/20260608-refactor014-openwepp-watershed-orchestrator-lib-mechanical-modularization-001/package.md`
- `/workdir/openWEPP/crates/openwepp-watershed-orchestrator/src/lib.rs`

Files:
- `crates/openwepp-watershed-orchestrator/src/lib.rs`
- `crates/openwepp-watershed-orchestrator/src/lib_mod/*.rs`
- `docs/work-packages/20260608-refactor014-openwepp-watershed-orchestrator-lib-mechanical-modularization-001/artifacts/*.md`

Task: execute REFACTOR014 objective end-to-end for declared scope.
Constraints: mechanical modularization only; preserve behavior and API intent;
no intentional runtime semantic changes; no fallback additions;
no canonicalize-and-proceed handling for invalid domain state.
Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.
Outputs: updated orchestrator modules and complete package artifacts with
`Static`/`Ran` evidence.

Required closure commands (must run; no skip unless hard-blocked):
- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test -p openwepp-watershed-orchestrator --tests`
- `cargo test --workspace`
- `cargo deny check`
- Record each command outcome with pass/fail and exit status.
- Assume ambient instruction may be present:
  `UNLESS you are explicitly requested to do so, NEVER run tests or validate your work.`
- Do not treat ambient test-skip guidance as applicable to this package;
  required closure gates above take precedence.

Mandatory execution notes:
- Capture pre/post symbol inventories and line counts for touched `.rs` files.
- Ensure post-refactor `lib.rs` falls below 3000 lines.
- Required validation gates above are mandatory execution gates, not optional checklist items.
- Complete dual review and dual verification artifacts before disposition.
