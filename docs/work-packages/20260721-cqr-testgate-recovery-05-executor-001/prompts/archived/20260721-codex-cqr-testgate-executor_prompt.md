# Active Prompt: Executor CQR

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
- `docs/work-packages/20260721-cqr-testgate-recovery-05-executor-001/package.md`
- `docs/work-packages/cqr-nightly-burndown-execplan.md`
- `docs/standards/mechanical-refactor-authoring-guide.md`
- `docs/standards/code-quality-refactor-authoring-guide.md`
- `docs/decisions/0021-module-coverage-closure-thresholds.md`

Conditional:

- `docs/standards/AGENTS.md` and
  `docs/standards/testing-and-gate-strategy.md` when selecting, reusing, or
  escalating gates.
- `docs/standards/prompt-wording-guidance.md` when maintaining this prompt.
- `docs/work-packages/20260721-cqr-testgate-recovery-closeout-execplan.md` for
  the seven-package campaign boundary.

On-demand:

- `crates/openwepp-gate-planner/src/executor.rs`
- `crates/openwepp-gate-planner/src/executor_coverage_tests.rs`

Required-reading budget: 347,441 local bytes, REQUIRES-JUSTIFICATION because
the trust-bearing executor and canonical gate standard must be read in full;
map: `artifacts/required-reading-map.md`.

Files:

- `crates/openwepp-gate-planner/src/executor.rs`
- `crates/openwepp-gate-planner/src/executor_coverage_tests.rs`
- `docs/work-packages/20260721-cqr-testgate-recovery-05-executor-001/**`
- `docs/work-packages/20260721-cqr-testgate-recovery-closeout-execplan.md`

Task: execute package `20260721-cqr-testgate-recovery-05-executor-001`
end-to-end under the master TESTGATE recovery closeout ExecPlan. Preserve
execution and receipt semantics, use one matching-module measurement per
changed head, obtain fresh dual review and terminal verification, and do not
launch campaign-global TESTGATE.

Constraints: behavior-preserving CRAP decomposition only. Do not change public
APIs, schemas, thresholds, execution order, error precedence, resource
contracts, receipt meaning, or fail-closed behavior. Do not rerun unchanged
expensive gates.

Subagent requirement: REQUIRED: spawn `comparator_suite_runner` for every heavy
batch, closure, comparator, or campaign-global gate selected by the terminal
plan; do NOT run those gates on the parent model unless the subagent is
unavailable and command-level evidence records that fact. This prompt
explicitly authorizes subagent spawning/delegation to metric, review,
verification, comparator, and closure-runner roles for package-scoped evidence
and final campaign qualification. Outputs are compact metrics, package-local
review/verification artifacts, and retained log paths. Write access is
read-only unless a bounded package write is explicitly assigned.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: update every required package artifact, disposition all findings, and
leave a committed COMPLETE or HOLD package before rank 6 begins.
