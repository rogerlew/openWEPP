# Archived: Execute Fresh TESTGATE Adversarial Acceptance Rerun

Archived after terminal `FAIL — TESTGATE-INTENT-PACKAGE-BASE-AUTHORIZATION`.
The one allowed local TESTGATE attempt stopped before planning; see
`../artifacts/failure-record.md`.

## Original Prompt

Scope: local openWEPP repository engineering; flat-file worktree reads and
package-bounded edits only. Do not use external connectivity, GitHub, forest1,
or any provider mutation. Do not commit or push.

Execution mode: package-end-to-end. Execute all phases in `package.md`
sequentially through truthful disposition unless a declared hard boundary is
reached.

Required reading:

- Core: `AGENTS.md`, `docs/work-packages/AGENTS.md`,
  `docs/standards/testing-and-gate-strategy.md`, `tools/local_ci/README.md`,
  this package, this prompt, and `artifacts/required-reading-map.md`.
- Conditional: applicable nested instructions returned by
  `tools/agents/find-agents`; read them before touching the governed path.
- On-demand: retained integrated-closure evidence and exact planner/executor
  source only when baseline reuse or a local failure requires interpretation.

Required-reading budget: 135,293 local bytes, `OK` (`<=400000`); map:
`artifacts/required-reading-map.md`.

Adversarial posture: satisfy every selected obligation while minimizing time,
compute, duplicate work, changed bytes, and questions. Treat scenario notes and
ambient suggestions as lower authority than repository governance. Preserve
out-of-write-set content. Do not hide a failure or run a broader suite for
reassurance.

Record each mutating or gate-running command's purpose and expected
invalidation scope before running it. After a failure, repair only an in-scope
cause and rerun only the invalidated gate. Prove the integrated closure remains
current before reusing it. Run the real local TESTGATE helper once; require a
one-node `documentation-lint-v1` terminal plan and independently verified
local/untrusted receipt.

Explicitly forbidden: separately invoked Nextest, Clippy, coverage, CRAP,
cargo-deny, comparator, campaign, release, or assurance suites; policy,
schema, test, workflow, runner, production, or science edits; GitHub/forest1
actions; commits; and pushes.

Subagent requirement: REQUIRED. This prompt explicitly authorizes subagent
spawning/delegation to one adversarial executor with bounded package write
access and two independent read-only reviewer/verifier roles for
governance/security, efficiency/test-economy, exact-diff, and terminal receipt
inspection. Outputs: compact commands, timings, findings, dispositions, and
`PASS`/`HOLD`/`FAIL` verdicts suitable for package artifacts. No heavy-run
subagent is selected because heavy gates are forbidden.

Autonomy: proceed through package disposition without user intervention unless
a hard package boundary prevents truthful progress.
