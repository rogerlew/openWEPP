# Execute Stage 3 authentic coverage and memory attribution

Scope: local openWEPP repository engineering; flat-file reads/edits, local
builds, local instrumentation, and local tests only. No deployment, remote
service changes, infrastructure changes, or network actions. Do not push.

Execution mode: `package-end-to-end`.

Execute all phases in `package.md` sequentially through review, verification,
and truthful disposition. The predecessor package remains terminal HOLD. This
package does not restore revision 31 or revision 61, change their historical
acceptance rules, claim throughput qualification, begin a solver redesign, or
revise the runtime budget.

Required reading before edits:

- Core: `AGENTS.md`, `docs/work-packages/AGENTS.md`,
  `docs/codex_exec_plans.md`, `docs/standards/prompt-wording-guidance.md`,
  `docs/standards/testing-and-gate-strategy.md`.
- Predecessor: `package.md`, `artifacts/final-disposition.md`,
  `artifacts/hold-legitimacy-audit.md`, `artifacts/gate-results.md`, and
  `artifacts/science-contracts/component-temperature-dependency-replay/contract_ref.md`.
- Conditional before Rust/authority edits: `docs/specifications/science-contracts/AGENTS.md`,
  `docs/standards/kernel-work-package-preparation.md`,
  `docs/standards/numerical-solver-architecture.md`, relevant `SC-*`,
  `crates/AGENTS.md`, `tests/AGENTS.md`, and every nearest `AGENTS.md` for the
  exact path.
- On-demand: predecessor performance/workload/tolerance/implementation/review
  artifacts and the four named raw logs, only for touched mechanisms.

Required-reading budget: record exact local bytes and threshold disposition in
`artifacts/required-reading-map.md`; do not silently use the predecessor's
older budget.

Subagent requirement: REQUIRED. This package explicitly authorizes
spawning/delegating to two independent reviewers, two independent verifiers,
and `comparator_suite_runner` for review, verification, heavy release/parity,
and batch gates. Reviewers/verifiers have bounded artifact-only writes;
comparator writes only package logs/results. Do not run heavy gates on the
parent model while the comparator is available; record any spawn/tool failure
before an authorized local fallback.

Before source edits, preserve every touched file with content-addressed
snapshots, resolve exact paths, run `tools/agents/find-agents --for` for each,
and amend the package write set. Keep coverage, historical reproduction, and
memory controls as separate evidence surfaces. Never convert eligibility into
executed replay, treat endpoint RSS as peak/live heap, or infer causality from
an incomplete trace.

Required outputs are the package artifacts named in `package.md`, current
catalog/roadmap status, exact commands and identities, raw logs, independent
stream/output reconstruction, two reviews, two terminal verifications, and a
specific recommendation for each mechanism. Do not stop at scaffolding,
expected-red, first trace, or first inconvenient result.
