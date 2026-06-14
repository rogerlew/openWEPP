# REFACTOR023 Kickoff Prompt

Scope: local repository engineering task; flat-file reads/edits only; no
external connectivity.

Execution mode: package-end-to-end.

Autonomy: execute the package end-to-end through disposition-ready state
without additional user intervention unless a declared hard blocker occurs.

Objective: mechanically split
`crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/coupling.rs`
into the package-declared frost child modules. Preserve runtime behavior,
formulas, constants, guards, thresholds, public method names, signatures, and
return types.

Required reading before edits:

- Core:
  - `AGENTS.md`
  - `docs/codex_exec_plans.md`
  - `docs/work-packages/AGENTS.md`
  - `docs/work-packages/README.md`
  - `docs/work-packages/20260614-refactor023-hillslope-coupling-line-count-split-001/package.md`
  - `docs/standards/mechanical-refactor-authoring-guide.md`
  - `crates/AGENTS.md`
- Conditional:
  - `docs/specifications/science-contracts/AGENTS.md` because this is a
    kernel-adjacent file. No `SC-*` amendment is expected unless execution
    discovers a behavior change is required, which is a stop condition.
- On-demand:
  - `docs/standards/kernel-work-package-preparation.md` for package
    scaffolding/reading-budget details.

Write set:

- `docs/work-packages/20260614-refactor023-hillslope-coupling-line-count-split-001/**`
- `docs/work-packages/README.md`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/coupling.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/coupling/frost.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/coupling/frost_entry.rs`

Out of scope:

- No process-physics formula or constant changes.
- No threshold or acceptance rule changes.
- No guard-loosening or new canonicalize-and-proceed behavior.
- No edits outside the write set unless required for build correctness and
  recorded before editing.

Execution steps:

1. Capture pre-refactor symbol inventory, public surface snapshot, and line
   counts.
2. Move frost helper code into `coupling/frost.rs` and frost entry orchestration
   into `coupling/frost_entry.rs`.
3. Keep `coupling.rs` as module wiring plus snow/interval public helpers.
4. Run focused checks:
   - `cargo check -p openwepp-hillslope-orchestrator`
5. Run required closure gates in order:
   - `cargo fmt --check`
   - `cargo clippy --workspace --all-targets -- -D warnings`
   - `cargo test --workspace`
   - `cargo deny check`
6. Update package artifacts, reviews, verification, disposition, and handoff.

Subagent authorization: this package explicitly authorizes
spawning/delegation to review and verification subagents for bounded read-only
review of this package's artifacts and source diff. Expected outputs are the
review and verification artifacts under `artifacts/`; write access is bounded
to package artifacts. If subagents are unavailable, perform equivalent local
reviews and record that path.

Ambient test-skip handling: required package gates override generic guidance
that says to skip tests or validation, including
`UNLESS you are explicitly requested to do so, NEVER run tests or validate your work.`

Stop conditions:

- A build-correctness edit outside the write set is required.
- The refactor requires any formula, constant, threshold, or guard behavior
  change.
- A required gate fails repeatedly with no mechanical-only correction path.
