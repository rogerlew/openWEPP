# Execute TESTGATE Pre-Heavy Closure Audit And Structural Repair

Scope: local openWEPP repository engineering; flat-file worktree reads and
package-bounded edits only. No external connectivity, GitHub dispatch, runner
registration, forest1 mutation, release, or publication action is required.

Execution mode: package-end-to-end. Execute all phases in `package.md`
sequentially through truthful disposition unless a declared hard boundary is
reached.

Required reading:

- Core: `AGENTS.md`, `docs/work-packages/AGENTS.md`,
  `docs/standards/testing-and-gate-strategy.md`,
  `docs/standards/prompt-wording-guidance.md`, `tools/local_ci/README.md`, this
  package, this prompt, `artifacts/defect-inventory.md`, and
  `artifacts/pre-heavy-audit-contract.md`, and
  `artifacts/acceptance-matrix.md`.
- Conditional: `crates/AGENTS.md`, `tests/AGENTS.md`,
  `tests/fixtures/AGENTS.md`, and every applicable file returned by
  `tools/agents/find-agents`; read before editing its governed path.
- On-demand: planner/executor/verifier/ledger source, gate-policy schemas and
  fixtures, helper tests, workflow definitions, and retained historical failure
  evidence named in `artifacts/required-reading-map.md`.

Required-reading budget: 170,034 local bytes, `OK` (`<=400000`); recalculate
after scaffold-review edits and immediately before execution. Map:
`artifacts/required-reading-map.md`.

Files: only paths under `## Declared Write Set` in `package.md`.

Task: close `TGCA-001` through `TGCA-011` end-to-end. Freeze failing fixtures
and the typed report contract first; implement one planner-owned pre-heavy audit
consumed unchanged by executor and verifier; repair scaffold validation and
base-commit admission sequencing without self-authorizing uncommitted bytes;
front-load cheap blockers; bind identities, immutable attempts, safe caches,
and persistent ledgers; enforce a machine-owned light/audit/heavy state machine;
resume late failures from verified current per-node receipts; retain trusted-run
attempt evidence across runner resets; eliminate duplicate full/coverage
execution after parity; integrate local and trusted workflow paths; update
evidence as work proceeds.

Constraints: do not weaken selection, thresholds, authority, trust, test
membership, or anti-evasion behavior. Do not add a fallback that bypasses the
audit. After one policy-permitted infrastructure retry, recurrence of the same
cause blocks another heavy attempt until its tooling defect is fixed or bounded
external-outage authority exists. Run successful nodes once unless a bound
input changes.

Subagent requirement: REQUIRED. This prompt explicitly authorizes subagent
spawning/delegation to two independent read-only reviewers, two independent
read-only terminal verifiers, and `comparator_suite_runner` for all selected
heavy batch/closure/comparator runs. Heavy-run output is limited to ignored
execution/evidence roots. Outputs: compact findings, dispositions, commands,
timings, artifact paths, and `PASS`/`HOLD`/`FAIL` verdicts. The parent must not
run or repeat heavy nodes while that role is available.

Autonomy: execute the package through final disposition without requesting
additional user direction unless a declared hard boundary blocks truthful
progress.

Outputs: keep package progress and decision logs current; produce audit,
intent/terminal plan, receipt, timing, review, disposition, dual verification,
line-count, worker-handoff, and final-disposition artifacts; archive this prompt
at closure.
