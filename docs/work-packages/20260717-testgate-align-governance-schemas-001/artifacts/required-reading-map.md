# Required Reading Map

Evidence class: `Ran` and `Static`

`tools/agents/find-agents --for` was run before edits against every declared
write-set class. Applicable chains are:

- repository-root governance, Cargo, and `gate-policy/v1/**`: `AGENTS.md`;
- crate guidance: `AGENTS.md` then `crates/AGENTS.md`;
- work-package files, templates, and catalog: `AGENTS.md` then
  `docs/work-packages/AGENTS.md`;
- standards: `AGENTS.md` then `docs/standards/AGENTS.md`;
- integration tests: `AGENTS.md` then `tests/AGENTS.md`.

Required authority read before scaffolding:

- `AGENTS.md`;
- `crates/AGENTS.md`;
- `docs/work-packages/AGENTS.md`;
- `docs/standards/AGENTS.md`;
- `tests/AGENTS.md`;
- `docs/decisions/0039-campaign-scoped-risk-based-testing-and-assurance-gates.md`;
- `docs/standards/testing-and-gate-strategy.md`, especially sections 6–13;
- `docs/decisions/0021-module-coverage-closure-thresholds.md`;
- `docs/specifications/correctness-authority-model.md`;
- `docs/work-packages/20260717-test-gate-authority-001/package.md`; and
- `docs/work-packages/20260717-test-gate-authority-001/artifacts/implementation-handoff.md`.

The work is governance/test-contract scope and does not affect kernel physics,
so `docs/specifications/science-contracts/AGENTS.md` is not applicable.

Required-reading budget: 183,315 local bytes (`OK`, at or below 400,000 bytes).
Schema bodies, fixtures, and changed guidance sections were read on demand
rather than inflating the mandatory intake set.
