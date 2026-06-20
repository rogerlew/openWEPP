# Kickoff Prompt - R5B Direct Normalization And Storage Bounds

Execution mode: package-end-to-end.

Autonomy: execute this package from scaffold through disposition without
additional user intervention unless a declared hard blocker is reached.

Objective: implement the R5B direct `Normalization` and `StorageBounds` phases
from `docs/work-packages/20260620-r5b-normalization-storage-bounds-direct-phases-001/package.md`.

Required reading:

Core:

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/work-packages/r5-burndown-execplan.md`
- `docs/work-packages/20260620-r5b-normalization-storage-bounds-direct-phases-001/package.md`
- `docs/specifications/science-contracts/AGENTS.md`

Conditional:

- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `docs/architecture/array-native-runtime-specification.md`

On-demand:

- `crates/AGENTS.md`
- `tests/AGENTS.md`
- R5A package artifacts for lifecycle counter evidence.

Implementation boundaries:

- Do not change public output authority or default runtime activation.
- Do not add compatibility request/writeback/symbol access to direct-runtime
  files.
- Do not hide decomposition, residue, or growth migration inside R5B.

Required closeout:

- focused direct-runtime and runner tests;
- no-compatibility scan;
- scheduler/API diff review;
- full Rust gates;
- scoped docs lint and `git diff --check`;
- default-disabled H2637 reps;
- local dual review, disposition, dual verification, line-count governance,
  worker handoff, burn-down progress update, commit, and push.
