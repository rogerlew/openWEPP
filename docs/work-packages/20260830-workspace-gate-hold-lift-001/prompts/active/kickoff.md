# Execute the workspace gate hold-lift

Scope: local repository engineering work using flat-file reads/edits and local
commands only; no external systems or network actions are required.

Execution mode: package-end-to-end.

Phase plan: execute every phase in `package.md` sequentially through truthful
disposition without requesting user intervention unless a genuine authority,
safety, external-state, or operator-decision blocker is reached.

Required reading:

- Core: `AGENTS.md`, `docs/codex_exec_plans.md`,
  `docs/defect_closure_execplans.md`, `docs/work-packages/AGENTS.md`, this
  package, and `artifacts/required-reading-map.md`.
- Conditional: `crates/AGENTS.md` and `tests/AGENTS.md` before Rust/test edits;
  nearest nested `AGENTS.md` for every classified path; science-contract
  governance before any kernel/contract-affecting amendment.
- On demand: owning module README/contracts and the predecessor fixed-point
  package's review/verification/gate artifacts.

Required-reading budget: 137,872 bytes for the initial governance map,
`REQUIRES-JUSTIFICATION`; the full testing/work-package standards are required
because this package owns a critical cross-workspace hold-lift. Map:
`artifacts/required-reading-map.md`.

Task: close `WGHL-CLIPPY-001` and `WGHL-FULL-001` end-to-end. Record each
failure family prospectively before its implementation edit, preserve all
semantic assertions and failure policies, and obtain exact-clean passing
mandatory commands.

Constraints: no test weakening, silent waiver, retry-to-pass, surrogate
physics, tolerance relaxation, authority bypass, or TESTGATE/planner work.
Contract-first sequencing applies if triage reaches a governed kernel surface.

Subagent requirement: REQUIRED. This prompt explicitly authorizes subagent
spawning/delegation to `comparator_suite_runner` subagents for all heavy
baseline/current/full/Clippy commands, bounded triage/worker subagents for
classified failure families, two independent reviewers, and two independent
verifiers. Comparator outputs are compact metrics and retained logs with
read-only source access; worker writes are limited to assigned prospective
paths; reviewer/verifier writes are limited to their assigned artifacts. The
parent must not execute heavy full-workspace closure runs while the comparator
runner is available. Standing user authorization was supplied on 2026-08-30.

Outputs: update every package artifact, archive this prompt after final
disposition, and commit/push only stable increments.
