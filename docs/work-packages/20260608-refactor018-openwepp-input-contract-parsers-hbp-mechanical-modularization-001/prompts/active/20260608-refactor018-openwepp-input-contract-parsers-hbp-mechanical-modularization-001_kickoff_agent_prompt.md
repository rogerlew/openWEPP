# REFACTOR018 Kickoff Agent Prompt

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
  - /workdir/openWEPP/docs/work-packages/20260608-refactor018-openwepp-input-contract-parsers-hbp-mechanical-modularization-001/package.md
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
- Measure local_required_bytes_total (`wc -c` on Core paths) and record
  threshold outcome in the map before any edits.

Files:
- crates/openwepp-input-contract/src/parsers/hbp.rs
- crates/openwepp-input-contract/src/parsers/hbp/mod.rs  (to be created)
- crates/openwepp-input-contract/src/parsers/hbp/*.rs  (to be created)
- crates/openwepp-input-contract/src/parsers/mod.rs  (seam owner — read only; no edit needed)
- docs/work-packages/20260608-refactor018-openwepp-input-contract-parsers-hbp-mechanical-modularization-001/artifacts/*.md

Task: execute REFACTOR018 objective end-to-end for declared scope.

Constraints: mechanical modularization only; preserve public API surface at
existing import paths; no intended behavior or logic changes; no fallback
additions; no canonicalize-and-proceed handling for invalid domain state.

Key mechanical note: the `pub mod hbp;` declaration in `parsers/mod.rs` resolves
transparently to either `hbp.rs` or `hbp/mod.rs` — no change to `parsers/mod.rs`
is required. The Phase B opening step is: move `hbp.rs` → `hbp/mod.rs`, confirm
compilation, then proceed with concern extraction.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: updated module seam under `hbp/` and complete package artifacts with
`Static`/`Ran` evidence.

Required closure commands (must run; no skip unless hard-blocked):
- cargo fmt --check
- cargo clippy --workspace --all-targets -- -D warnings
- cargo test -p openwepp-input-contract
- cargo test --workspace
- cargo deny check
- Record each command outcome with pass/fail and exit status.

Mandatory execution notes:
- Capture pre/post public API symbol inventories and line counts for touched
  `.rs` files.
- Verify all 4 public entry-point functions and all `pub` Hbp* types remain
  accessible at their original import paths after modularization.
- Ensure each new `.rs` file under `hbp/` is within `.rs` line-count governance.
- `parse_layout` (~843 lines) and `validate_payload` (~384 lines) are the two
  largest extraction targets; treat each as its own module.
- Required validation gates above are mandatory execution gates, not optional
  checklist items.
- Complete dual review and dual verification artifacts before disposition.
