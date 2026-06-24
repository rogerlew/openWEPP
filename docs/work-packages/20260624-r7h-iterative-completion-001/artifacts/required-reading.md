# Required Reading

Evidence class: Static.

Read before execution:

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/defect_closure_execplans.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `crates/AGENTS.md`
- `docs/ROADMAP.md`
- `docs/architecture/array-native-runtime-specification.md`
- `docs/architecture/coupled-frost-sub-solver-specification.md`
- `docs/decisions/0026-stateful-winter-column-sub-solver.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/work-packages/20260624-r7h-closure-activation-gates-001/package.md`
- `docs/work-packages/20260624-r7h-closure-activation-gates-001/artifacts/blocker-ledger.md`
- `docs/work-packages/20260624-r7h-closure-activation-gates-001/artifacts/worker-handoff.md`

Summary:

- The active R7H defect is performance plus protected-output parity on H2637
  direct production.
- Current direct endpoint reaches `compatibility_edge_invocations=0`.
- Activation is blocked until performance, parity, no-compatibility,
  reconstruction, and rollback gates are all green on current code.
