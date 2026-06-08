# REFACTOR013 Kickoff Agent Prompt

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
- `/workdir/openWEPP/docs/work-packages/20260607-refactor013-openwepp-hillslope-orchestrator-hydrology-support-helpers-mechanical-modularization-001/package.md`
- `/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`

Files:
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/*.rs`
- `docs/work-packages/20260607-refactor013-openwepp-hillslope-orchestrator-hydrology-support-helpers-mechanical-modularization-001/artifacts/*.md`

Task: execute REFACTOR013 objective end-to-end for declared scope.
Constraints: mechanical modularization only; preserve behavior and API intent;
no intentional runtime semantic changes; no fallback additions;
no canonicalize-and-proceed handling for invalid domain state.
Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.
Outputs: updated support-helper modules and complete package artifacts with
`Static`/`Ran` evidence.

Required closure commands (must run; no skip unless hard-blocked):
- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test -p openwepp-hillslope-orchestrator --tests`
- `cargo test --workspace`
- `cargo deny check`
- Record each command outcome with pass/fail and exit status.
- Do not treat generic "skip tests/validation" guidance as applicable to this
	package; required closure gates above take precedence.

Mandatory execution notes:
- Capture pre/post symbol inventories and line counts for touched `.rs` files.
- Ensure post-refactor `03_kernel_support_00_support_helpers.rs` falls below 3000 lines.
- Required validation gates above are mandatory execution gates, not optional checklist items.
- Complete dual review and dual verification artifacts before disposition.
