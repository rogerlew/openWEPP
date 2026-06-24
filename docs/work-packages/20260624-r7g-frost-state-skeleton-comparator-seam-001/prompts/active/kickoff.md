# Kickoff: R7G Frost State Skeleton and Comparator Seam

Autonomy: execute this package end-to-end without asking for next steps unless
a declared blocker is reached.

Subagent authorization: none.

Required reading:

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/decisions/0026-stateful-winter-column-sub-solver.md`
- `docs/architecture/coupled-frost-sub-solver-specification.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/work-packages/20260624-r7g-frost-state-skeleton-comparator-seam-001/package.md`

Execution:

1. Preserve physics behavior. Do not change frost equations, thresholds, or
   contract authority.
2. Make `DirectWinterColumnState.frost` the canonical direct lane/day state
   skeleton and keep `DirectFrostRuntimeCarry` only as a temporary mirror.
3. Move direct publication prior-frost reads to `lane.winter_column.frost`.
4. Isolate remaining `DirectFrostRunoffSurface` construction/seeding behind
   named comparator-seam helpers.
5. Add tests/source scans for skeleton authority and seam isolation.
6. Run the required closure gates and record exact evidence.
7. If the typed solver extraction is still required, record that as follow-up;
   do not claim full frost bridge deletion.
