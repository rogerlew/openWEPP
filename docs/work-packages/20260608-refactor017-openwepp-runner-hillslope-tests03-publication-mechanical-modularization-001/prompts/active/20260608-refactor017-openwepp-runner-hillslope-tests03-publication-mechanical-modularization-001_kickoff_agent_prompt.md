# REFACTOR017 Kickoff Agent Prompt

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
  - /workdir/openWEPP/docs/work-packages/20260608-refactor017-openwepp-runner-hillslope-tests03-publication-mechanical-modularization-001/package.md
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
- map artifact: artifacts/required-reading-map.md
- map template (canonical): docs/prompt_templates/required-reading-map-template.md
- Measure local_required_bytes_total and record threshold outcome in the map.

Files:
- crates/openwepp-runner/src/hillslope/tests03/publication.rs
- crates/openwepp-runner/src/hillslope/tests03/publication/*.rs
- crates/openwepp-runner/src/hillslope/03_tests.rs (include-wiring seam owner)
- docs/work-packages/20260608-refactor017-openwepp-runner-hillslope-tests03-publication-mechanical-modularization-001/artifacts/*.md

Task: execute REFACTOR017 objective end-to-end for declared scope.

Constraints: mechanical test modularization only; preserve test intent,
assertion strength, and test names; no production (`src/`) behavior changes;
no fallback additions; no canonicalize-and-proceed handling for invalid domain
state.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: updated test-module seam under `tests03/publication/` and complete
package artifacts with `Static`/`Ran` evidence.

Required closure commands (must run; no skip unless hard-blocked):
- cargo fmt --check
- cargo clippy --workspace --all-targets -- -D warnings
- cargo test -p openwepp-runner --tests
- cargo test --workspace
- cargo deny check
- Record each command outcome with pass/fail and exit status.

Mandatory execution notes:
- Capture pre/post test inventories (`#[test]` roster) and line counts for
  touched `.rs` files.
- Ensure post-refactor `tests03/publication.rs` falls below 2000 lines and each
  new module file stays within `.rs` line-count governance.
- Confirm all 49 tests remain present and pass under their new module residency.
- Required validation gates above are mandatory execution gates, not optional
  checklist items.
- Complete dual review and dual verification artifacts before disposition.
