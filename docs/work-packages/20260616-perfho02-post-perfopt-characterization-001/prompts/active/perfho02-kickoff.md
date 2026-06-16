# PERFHO02 Kickoff

Scope: local repository performance-characterization task; flat-file reads/edits and local command execution only; no external connectivity.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in `docs/work-packages/20260616-perfho02-post-perfopt-characterization-001/package.md` sequentially through disposition.

Required reading (read before edits):

Core:
- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/standards/AGENTS.md`
- `docs/standards/prompt-wording-guidance.md`
- `docs/codex_exec_plans.md`
- `docs/work-packages/20260616-perfho02-post-perfopt-characterization-001/package.md`
- `docs/work-packages/20260616-perfho02-post-perfopt-characterization-001/artifacts/required-reading-map.md`

Conditional:
- `crates/AGENTS.md` only if production Rust edits become necessary. They should not be necessary for PERFHO02.
- `docs/standards/mechanical-refactor-authoring-guide.md` only if the package is amended into an optimization package. It should not be amended that way.

On-demand:
- PERFHO01 and PERFOPT01 artifacts named in `artifacts/required-reading-map.md`.
- Source files named by GDB stacks, for static attribution only.

Required-reading budget: `65731` bytes for the core plus PERFHO/PERFOPT evidence read during scaffold, `OK`; map: `artifacts/required-reading-map.md`.

Files:
- `docs/work-packages/20260616-perfho02-post-perfopt-characterization-001/**`
- `docs/work-packages/README.md`
- `docs/ROADMAP.md`

Task: execute package objective end-to-end for the declared characterization scope.

Constraints: no production Rust edits; no science-contract edits; no physics, formula, threshold, typed-guard, output-schema, or behavior change; no branch creation.

Conservation/output acceptance: not applicable because PERFHO02 does not modify outputs. If any full fixture run is used as supporting evidence, record only exit status/timing unless output identity is explicitly in scope.

Subagent requirement: none. The user did not explicitly request subagents. Do not claim delegated review or verification occurred.

Autonomy: execute package phases end-to-end and update required artifacts without requesting additional user direction unless hard-blocked.

Outputs: update package artifacts and disposition for all completed phases.

