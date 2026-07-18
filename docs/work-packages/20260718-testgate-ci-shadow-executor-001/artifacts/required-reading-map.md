# Required Reading Map

Evidence class: `Ran` and `Static`

`tools/agents/find-agents --for` was run before edits. Applicable chains are:

- repository root, Cargo, policy, release tooling, local-CI tooling, and GitHub
  workflows: `AGENTS.md`;
- crate source: `AGENTS.md`, then `crates/AGENTS.md`;
- integration tests: `AGENTS.md`, then `tests/AGENTS.md`;
- work-package files: `AGENTS.md`, then `docs/work-packages/AGENTS.md`;
- standards: `AGENTS.md`, then `docs/standards/AGENTS.md`.

Core authority read before scaffolding:

- `AGENTS.md`;
- `crates/AGENTS.md`;
- `docs/work-packages/AGENTS.md`;
- `docs/standards/AGENTS.md`;
- `docs/standards/testing-and-gate-strategy.md`, especially sections 8–13 and
  18–19;
- `docs/standards/prompt-wording-guidance.md`;
- `docs/standards/local-ci-gate-selection.md`;
- ADR-0039;
- `docs/ROADMAP.md`;
- TESTGATE-ALIGN-01's `artifacts/implementation-handoff.md`; and
- TESTGATE-PLAN-01's package, planner source, schemas, and tests.

The package does not touch kernel physics, so science-contract instruction and
legacy comparator authority do not apply. `tests/AGENTS.md` must be read before
creating the declared integration guard.
