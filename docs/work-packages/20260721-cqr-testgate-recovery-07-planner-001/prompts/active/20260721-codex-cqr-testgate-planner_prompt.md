# Active Prompt: Planner CQR

Scope: local behavior-preserving CQR work inside `/home/workdir/openWEPP`; no
external connectivity is required.

Execution mode: package-end-to-end.

Phase plan: execute all phases in `package.md` sequentially through
disposition.

Required reading:

Core:

- `AGENTS.md`
- `crates/AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/work-packages/20260721-cqr-testgate-recovery-07-planner-001/package.md`
- `docs/work-packages/cqr-nightly-burndown-execplan.md`
- `docs/standards/mechanical-refactor-authoring-guide.md`
- `docs/standards/code-quality-refactor-authoring-guide.md`
- `docs/decisions/0021-module-coverage-closure-thresholds.md`

Conditional:

- `docs/standards/AGENTS.md` and
  `docs/standards/testing-and-gate-strategy.md` for gate selection and evidence.
- `docs/standards/prompt-wording-guidance.md` for prompt maintenance.
- `docs/work-packages/20260721-cqr-testgate-recovery-closeout-execplan.md` for
  the master boundary.

On-demand:

- `crates/openwepp-gate-planner/src/planner.rs`
- `crates/openwepp-gate-planner/src/planner_coverage_tests.rs`

Required-reading budget: 318,598 local bytes, REQUIRES-JUSTIFICATION; map:
`artifacts/required-reading-map.md`.

Files:

- `crates/openwepp-gate-planner/src/planner.rs`
- `crates/openwepp-gate-planner/src/planner_coverage_tests.rs`
- `docs/work-packages/20260721-cqr-testgate-recovery-07-planner-001/**`
- `docs/work-packages/20260721-cqr-testgate-recovery-closeout-execplan.md`

Task: execute rank 7 end-to-end, preserving planner behavior and reducing every
owned eligible function to CRAP at most 30.

Constraints: behavior-preserving decomposition only. Do not change public APIs,
schemas, selection/reconciliation meanings, validation order, typed errors, or
fail-closed behavior. Do not rerun unchanged expensive gates.

Subagent requirement: REQUIRED: delegate every heavy batch, comparator, or
campaign-global gate selected by the terminal plan. This prompt explicitly
authorizes metric, review, verification, comparator, and closure-runner roles.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting direction unless hard-blocked.

Outputs: disposition every finding and leave a committed COMPLETE or HOLD
package before campaign closeout.
