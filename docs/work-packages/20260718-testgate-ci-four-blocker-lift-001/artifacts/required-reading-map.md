# Required Reading Map

Evidence class: `Ran` and `Static`

## Instruction Discovery

Ran:

    tools/agents/find-agents --for crates/openwepp-gate-planner/src/executor.rs tests/integration/testgate_ci_executor_contract.rs tools/local_ci/testgate_shadow.py .github/workflows/testgate-shadow.yml docs/work-packages docs/standards/testing-and-gate-strategy.md

Applicable chains:

- Rust executor/planner/verifier: root `AGENTS.md`, then `crates/AGENTS.md`.
- Integration tests: root `AGENTS.md`, then `tests/AGENTS.md`.
- Tooling/workflow: root `AGENTS.md`.
- Package artifacts: root `AGENTS.md`, then `docs/work-packages/AGENTS.md`.
- Gate standard: root `AGENTS.md`, then `docs/standards/AGENTS.md`.

## Authority Routing

- Core process: `docs/work-packages/AGENTS.md` and
  `docs/codex_exec_plans.md`.
- Gate selection, receipts, coverage, CI, failure, anti-evasion, review, and
  transition: `docs/standards/testing-and-gate-strategy.md` sections 8-10, 12,
  and 14-19.
- Prompt contract: `docs/standards/prompt-wording-guidance.md`.
- Blocker evidence:
  `20260718-testgate-ci-shadow-executor-001/artifacts/review-a.md`,
  `review-b.md`, `review-disposition.md`, and `final-disposition.md`.
- Conditional Rust/test authoring: `crates/AGENTS.md`, `tests/AGENTS.md`.

No kernel science contract is triggered: this package changes gate planning,
execution, evidence, and tests, not process physics or published science.
