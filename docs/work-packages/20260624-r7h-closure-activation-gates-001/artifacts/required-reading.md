# Required Reading

Evidence class: Static.

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `crates/AGENTS.md`
- `docs/ROADMAP.md`
- `docs/architecture/array-native-runtime-specification.md`
- `docs/architecture/coupled-frost-sub-solver-specification.md`
- `docs/decisions/0026-stateful-winter-column-sub-solver.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/work-packages/20260624-r7g-consumer-cutover-deletion-001/package.md`
- `docs/work-packages/20260623-r7g-iterative-completion-001/package.md`

Key binding:

- `docs/ROADMAP.md` item 7 requires the R7G closure matrix before R7H release
  readiness or direct-default activation.
- `docs/architecture/array-native-runtime-specification.md` forbids default
  activation of a slower or output-nonidentical direct path.
- `docs/architecture/coupled-frost-sub-solver-specification.md` requires H2637
  performance, protected-output parity, manifest parity, direct counters,
  no-compatibility source scans, anti-alias fixtures, and independent operand
  reconstruction after the winter-column migration.
- `docs/work-packages/20260624-r7g-consumer-cutover-deletion-001/package.md`
  completed typed frost consumer cutover and bridge deletion but explicitly did
  not claim terminal protected-output parity, default activation, release
  readiness, or H2637 performance.

