# Active Prompt: Verifier CQR

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
- `docs/work-packages/20260721-cqr-testgate-recovery-06-verifier-001/package.md`
- `docs/work-packages/cqr-nightly-burndown-execplan.md`
- `docs/standards/mechanical-refactor-authoring-guide.md`
- `docs/standards/code-quality-refactor-authoring-guide.md`
- `docs/decisions/0021-module-coverage-closure-thresholds.md`

Conditional:

- `docs/standards/AGENTS.md` and
  `docs/standards/testing-and-gate-strategy.md` for gate selection, failure,
  correction, and evidence reuse.
- `docs/standards/prompt-wording-guidance.md` for prompt maintenance.
- `docs/work-packages/20260721-cqr-testgate-recovery-closeout-execplan.md` for
  the master boundary.

On-demand:

- `crates/openwepp-gate-planner/src/verifier.rs`
- `crates/openwepp-gate-planner/src/verifier_coverage_tests.rs`

Required-reading budget: 323,088 local bytes, REQUIRES-JUSTIFICATION
because the trust-bearing verifier and full gate standard must be read in full;
map: `artifacts/required-reading-map.md`.

Files:

- `crates/openwepp-gate-planner/src/verifier.rs`
- `crates/openwepp-gate-planner/src/verifier_coverage_tests.rs`
- `docs/work-packages/20260721-cqr-testgate-recovery-06-verifier-001/**`
- `docs/work-packages/20260721-cqr-testgate-recovery-closeout-execplan.md`

Task: execute rank 6 end-to-end, preserving READY-audit receipt verification
semantics and reducing every owned eligible function to CRAP at most 30.

Constraints: behavior-preserving decomposition only. Do not change public APIs,
schemas, receipt/audit meanings, execution-context policy, validation order,
typed errors, or fail-closed behavior. Do not rerun unchanged expensive gates.

Subagent requirement: REQUIRED: spawn `comparator_suite_runner` for every heavy
batch, closure, comparator, or campaign-global gate selected by the terminal
plan; do NOT run those gates on the parent model unless the subagent is
unavailable and command-level evidence records that fact. This prompt
explicitly authorizes subagent spawning/delegation to metric, review,
verification, comparator, and closure-runner roles. Outputs are compact
metrics, package-local review/verification artifacts, and retained log paths.
Write access is read-only unless a bounded package write is explicitly
assigned.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: update every required package artifact, disposition all findings, and
leave a committed COMPLETE or HOLD package before rank 7 begins.
