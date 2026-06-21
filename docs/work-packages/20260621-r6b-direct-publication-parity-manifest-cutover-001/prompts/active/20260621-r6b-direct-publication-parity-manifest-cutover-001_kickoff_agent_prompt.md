# R6B Direct Publication Parity and Manifest Cutover Kickoff Prompt

Scope: local repository science-contract/kernel migration task; flat-file
reads/edits only; no external connectivity.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in
`docs/work-packages/20260621-r6b-direct-publication-parity-manifest-cutover-001/package.md`
sequentially through disposition.

Required reading:

Core:

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/standards/prompt-wording-guidance.md`
- `docs/architecture/array-native-runtime-specification.md`
- `docs/work-packages/20260621-r6-direct-publication-cutover-001/package.md`
- `docs/work-packages/20260621-r6-direct-publication-cutover-001/artifacts/worker-handoff.md`
- `docs/work-packages/20260621-r6a-run-bound-direct-publication-frame-001/package.md`
- `docs/work-packages/20260621-r6b-direct-publication-parity-manifest-cutover-001/package.md`
- `crates/AGENTS.md`
- `tests/AGENTS.md`

Conditional:

- Read the contract authoring/profile/index docs before any `SC-*` amendment.
- Read touched `SC-*` contracts before changing output meaning, units,
  metadata, aliases, guard semantics, or conservation authority.

On-demand:

- Read direct runtime, runner, output writer, test, fixture, and benchmark
  source files only for touched mechanisms.

Required-reading budget: approximately `119494` bytes, `WARN`; map:
`artifacts/required-reading-map.md`.

Files: use the intended write set in `package.md`. Amend the package before
editing outside it.

Task: close `HOLD-R6-DIRECT-PUBLICATION-PARITY-AND-MANIFEST-CUTOVER` by
executing all five worker-handoff items: parity-grade frame population,
anti-alias fixtures, independent reconstruction, direct manifest cutover, and
cutover reruns plus default-disabled/endpoint benchmarks.

Constraints: contract-first sequencing; canonical `SC-*` authority; pinned
baseline provenance when migration authority applies; typed guards; no silent
defaults; no compatibility WB13/runtime/writeback wrappers in accepted direct
publication; no default activation; no R7 deletion.

Conservation/output acceptance: record operand lineage; separate plausible
aliases in fixtures; reject known wrong formulas; run independent
reconstruction plus real closure/magnitude audit; align metadata/schema; do
not close on one-sided bounds or self-consistency.

Subagent requirement: REQUIRED for heavy closure/comparator/benchmark runs when
a suitable subagent is available. This prompt explicitly authorizes subagent
spawning/delegation to read-only operand-lineage, anti-alias, independent
reconstruction, no-compatibility source-scan, benchmark runner, reviewer, and
verifier subagents for the scopes in `package.md`; outputs: compact metrics,
command logs, and findings; write access: package artifacts only unless
explicitly amended. If no suitable subagent is available, record that evidence
before running heavy commands locally.

Autonomy: execute package phases end to end and update required artifacts
without requesting additional user direction unless hard-blocked. Commit and
push only when the user asks or an enclosing autonomous ExecPlan explicitly
requires it.

Outputs: update package artifacts, reviews, verification, disposition, roadmap,
and work-package catalog for all completed phases.
