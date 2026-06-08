# REFACTOR016 Kickoff Agent Prompt

Scope: local repository engineering task; flat-file reads/edits only; no
external connectivity.
Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.

Required reading (read before edits):
- Core:
  - /workdir/openWEPP/AGENTS.md
  - /workdir/openWEPP/docs/work-packages/AGENTS.md
  - /workdir/openWEPP/docs/codex_exec_plans.md
  - /workdir/openWEPP/docs/work-packages/README.md
  - /workdir/openWEPP/docs/standards/mechanical-refactor-authoring-guide.md
  - /workdir/openWEPP/docs/work-packages/20260608-refactor016-openwepp-kernel-contract-lib-mechanical-modularization-001/package.md
- Conditional (read only when applicable):
  - /workdir/openWEPP/docs/defect_closure_execplans.md (if package converts to defect-closure posture)
  - docs/specifications/science-contract-authoring-procedure.md,
    docs/specifications/science-contracts/kernel-process-contract-profile.md,
    docs/specifications/science-contracts/index.md
    (if execution discovers contract/kernel authority touch)
  - docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md
    (if legacy migration/parity scope becomes applicable)
- On-demand (load only for touched mechanisms):
  - phase-relevant canonical SC-* contracts
  - queue/hold-lift/disposition artifacts
  - baseline source files when parity/legacy provenance is in-scope

Required-reading budget:
- local_bytes_total: 69417
- threshold: OK
- map artifact: artifacts/required-reading-map.md
- map template (canonical): docs/prompt_templates/required-reading-map-template.md

Files:
- crates/openwepp-kernel-contract/src/lib.rs
- crates/openwepp-kernel-contract/src/lib_mod/*.rs
- docs/work-packages/20260608-refactor016-openwepp-kernel-contract-lib-mechanical-modularization-001/artifacts/*.md

Task: execute REFACTOR016 objective end-to-end for declared scope.

Constraints: mechanical modularization only; preserve behavior and API intent;
no intentional runtime semantic changes; no fallback additions;
no canonicalize-and-proceed handling for invalid domain state.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: updated module seam and complete package artifacts with `Static`/`Ran`
evidence.

Required closure commands (must run; no skip unless hard-blocked):
- cargo fmt --check
- cargo clippy --workspace --all-targets -- -D warnings
- cargo test -p openwepp-kernel-contract --tests
- cargo test --workspace
- cargo deny check
- Record each command outcome with pass/fail and exit status.

Mandatory execution notes:
- Capture pre/post symbol inventories and line counts for touched `.rs` files.
- Ensure post-refactor `lib.rs` falls below 2000 lines.
- Required validation gates above are mandatory execution gates, not optional checklist items.
- Complete dual review and dual verification artifacts before disposition.
