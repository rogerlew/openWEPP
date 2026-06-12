# pre implementation contract gate

Status: satisfied for M-A only

Evidence mode: Static

M-A was characterization/scoping only and did not edit production kernel paths or science contracts.

Read before M-A execution:
- Root `AGENTS.md`.
- `docs/work-packages/AGENTS.md`.
- `docs/specifications/science-contracts/AGENTS.md`.
- `crates/AGENTS.md`.
- `tests/AGENTS.md`.
- Package `package.md`.
- `artifacts/mofe-staged-increment-plan.md`.

M-B trigger:
- Any kernel-affecting implementation must perform a fresh contract gate against the touched `SC-*` authority before edits.
