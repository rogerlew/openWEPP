# Execute Global CRAP Output Relocation

Scope: close `TESTGATE-GLOBAL-CRAP-OUTPUT-01` end-to-end inside the package's
declared write set. Do not use GitHub, forest1, workflows, or production hosts.

Execution mode: package-end-to-end.

Required reading:

- Core: `AGENTS.md`, `docs/work-packages/AGENTS.md`,
  `docs/defect_closure_execplans.md`, `docs/codex_exec_plans.md`,
  `docs/standards/testing-and-gate-strategy.md`, this package, and
  `artifacts/required-reading-map.md`.
- Conditional: `tests/AGENTS.md` before editing or interpreting the integration
  contract; `crates/AGENTS.md` before the integrated planner correction.
- On demand: the runner, gate definitions, executor contract, and predecessor
  receipt artifacts named by the package.

Task: preserve all confinement, coverage, CRAP, fail-closed, and explicit-path
semantics. Keep the default output relative through executor relocation, resolve
it against the repository only in standalone mode, update exact contract
assertions, and rebind only the two derived adapter SHA fields. Run focused
checks. Also close `TESTGATE-ENV-PROJECTION-DETERMINISM-01`: bind only the union
of policy-declared gate environment keys, prove undeclared invoker noise cannot
alter identity, and preserve all other execution-context identities. Commit and
require two independently generated exact plans plus reconciliation to agree
before the exact planner-selected critical terminal plan runs once. Do not rerun
passing nodes separately or substitute an ad hoc broad suite.

Subagent requirement: two independent read-only reviewer/verifier roles. This
prompt explicitly authorizes subagent spawning/delegation by the parent for the
scope and expected outputs defined in `package.md`; the parent retains all write
access and finding patches.

Autonomy: execute through disposition unless a declared hard boundary is
proven. Patch accepted findings inside the bounded write set, archive this
prompt at terminal disposition, and report every gate as PASS, FAIL, BLOCKED, or
NOT RUN without relabeling.
