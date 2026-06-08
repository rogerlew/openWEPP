# REFACTOR020 Kickoff Agent Prompt

Scope: local repository engineering task; flat-file reads/edits only; no
external connectivity.
Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.

Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/work-packages/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/standards/mechanical-refactor-authoring-guide.md`
- `/workdir/openWEPP/docs/standards/kernel-work-package-preparation.md`
- `/workdir/openWEPP/docs/prompt_templates/mechanical-refactor-kickoff-template.md`
- `/workdir/openWEPP/docs/prompt_templates/required-reading-map-template.md`
- `/workdir/openWEPP/docs/work-packages/20260608-refactor020-openwepp-hillslope-orchestrator-runtime-inputs-08-tests-mechanical-modularization-001/package.md`
- `/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs/mod.rs`
- `/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs/08_tests.rs`
- `/workdir/openWEPP/docs/specifications/science-contracts/AGENTS.md` (Conditional only if scope expands into contract or kernel-profile authority)
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md` (Conditional only if kernel authority touches are introduced)

Conditional:
- `/workdir/openWEPP/docs/defect_closure_execplans.md` (if package becomes defect-closure posture)

On-demand:
- phase-relevant canonical `SC-*` contracts (if expanded scope requires contract-level edits)

Required-reading budget:
- map artifact: artifacts/required-reading-map.md
- map template: docs/prompt_templates/required-reading-map-template.md
- Measure local_required_bytes_total (`wc -c` on Core paths) before edits and record
  threshold outcome.

Files:
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/08_tests.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/08_tests/*.rs` (to be created)
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/mod.rs` (seam owner — read only unless required by file move)
- `docs/work-packages/20260608-refactor020-openwepp-hillslope-orchestrator-runtime-inputs-08-tests-mechanical-modularization-001/artifacts/*.md`

Task: execute REFACTOR020 objective end-to-end for declared scope.

Constraints: mechanical modularization only; preserve behavior, test intent, and
symbol contracts; no guard-loosening; no fallback additions; no canonicalize-and-
proceed handling for invalid domain state.

Autonomy: execute package phases end-to-end and update required artifacts without
requesting additional user direction unless hard-blocked.

Outputs: fully modularized `08_tests` seam and complete package artifacts with
`Static`/`Ran` evidence.

Required closure commands (must run; no skip unless hard-blocked):
- cargo fmt --check
- cargo clippy --workspace --all-targets -- -D warnings
- cargo test -p openwepp-hillslope-orchestrator --tests
- cargo test --workspace
- cargo deny check
- Record each command outcome with pass/fail and exit status.

Mandatory execution notes:
- Capture pre/post line counts for all touched `.rs` files.
- Capture any generated module-public surface or test fixture wiring deltas.
- Complete dual review and dual verification artifacts before disposition.
