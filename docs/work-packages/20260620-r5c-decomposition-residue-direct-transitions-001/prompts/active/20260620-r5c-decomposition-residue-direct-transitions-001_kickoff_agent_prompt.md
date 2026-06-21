# R5C Kickoff Agent Prompt

Execute `docs/work-packages/20260620-r5c-decomposition-residue-direct-transitions-001/package.md`.

Required reading:

- root `AGENTS.md`;
- `docs/work-packages/AGENTS.md`;
- `docs/specifications/science-contracts/AGENTS.md`;
- `docs/work-packages/r5-burndown-execplan.md`;
- `docs/architecture/array-native-runtime-specification.md`;
- decomposition/residue science contracts discovered during scope selection.

Objective: implement the R5C direct `DecompositionTransition` and
`ResiduePartitionTransition` phases with typed inputs, direct compute, state
mutation, downstream operands, and shadow projection while preserving
no-publication, no-default-activation, and no-scheduler boundaries.

Execution rules:

- Do not create or switch branches.
- Do not add compatibility request/writeback/symbol access to direct-runtime
  modules.
- Do not invent decomposition or residue physics; if canonical authority is
  missing or contradictory, record `HOLD` with a defect-shaped handoff.
- Commit and push after package closure, then update
  `docs/work-packages/r5-burndown-execplan.md` with the pushed commit SHA.

Minimum gates:

- focused R5C direct-runtime tests;
- `cargo test -p openwepp-hillslope-orchestrator direct_runtime -- --nocapture`;
- `cargo test -p openwepp-runner r2a_ -- --nocapture`;
- no-compatibility source scan;
- scheduler/API diff review;
- `cargo fmt --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- `cargo deny check`;
- scoped markdown lint;
- `git diff --check`;
- release build and three default-disabled H2637 reps;
- protected output comparison against retained PERFDEEP07 baseline.
