# Required Reading Map

Status: `SCAFFOLD`

Evidence class: `Static`

Core reading before edits totals 55,809 bytes and is `OK`: `AGENTS.md`
(12,488), `docs/work-packages/AGENTS.md` (27,243), `tests/AGENTS.md` (4,684),
`package.md` (10,045), and this map (1,349).

Conditional reading is `docs/standards/testing-and-gate-strategy.md` (99,370)
before gate planning/execution and `docs/defect_closure_execplans.md` (24,803)
before HOLD or disposition. These are tiered because they govern later phases,
not the two-line edit.

On-demand mechanism reading is
`docs/standards/prompt-wording-guidance.md`,
`docs/work-packages/20260727-assurance-v2-amendment-contract-clippy-line-disposition-001/artifacts/implementation-gates.md`,
`tests/integration/testgate_ci_executor_contract.rs`, and
`crates/openwepp-gate-planner/src/main.rs`.

Applicable instruction chains were enumerated with:

    tools/agents/find-agents --for \
      tests/integration/testgate_ci_executor_contract.rs \
      docs/work-packages/20260727-testgate-bound-ledger-source-contract-alignment-001/package.md \
      docs/work-packages/README.md \
      docs/planning/canopy-phenology-assurance-roadmap.md

The resulting chains are root plus `tests/AGENTS.md` for the Rust test, root
plus `docs/work-packages/AGENTS.md` for work-package files/catalog, and root
only for the roadmap.
