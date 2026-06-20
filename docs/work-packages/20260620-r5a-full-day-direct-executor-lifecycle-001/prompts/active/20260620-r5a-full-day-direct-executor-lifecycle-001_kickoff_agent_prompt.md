# Kickoff Prompt - R5A Full-Day Direct Executor Lifecycle

Execution mode: package-end-to-end.

Autonomy: execute this package from scaffold through disposition without
additional user intervention unless a declared hard blocker is reached.

Objective: implement the R5A direct-runtime lifecycle prerequisite from
`docs/work-packages/20260620-r5a-full-day-direct-executor-lifecycle-001/package.md`.

Required reading:

Core:

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/work-packages/r5-burndown-execplan.md`
- `docs/work-packages/20260620-r5a-full-day-direct-executor-lifecycle-001/package.md`
- `docs/specifications/science-contracts/AGENTS.md`

Conditional:

- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `docs/architecture/array-native-runtime-specification.md`

On-demand:

- `crates/AGENTS.md`
- `tests/AGENTS.md`
- R4 direct-runtime package artifacts for adjacent evidence patterns.

Implementation boundaries:

- Touch only the intended write set unless package evidence justifies a narrow
  addition.
- Do not change public output authority, schemas, scheduler phase order, or
  default runtime selection.
- Do not add compatibility request/writeback/symbol access to direct-runtime
  files.

Required closeout:

- focused direct-runtime and runner tests;
- no-compatibility source scan;
- scheduler diff review;
- full Rust gates unless explicitly held with blocker evidence;
- scoped docs lint and `git diff --check`;
- default-disabled H2637 reps or a package-level HOLD if the required fixture
  is unavailable;
- local dual review, disposition, dual verification, line-count governance,
  worker handoff, burn-down progress update, commit, and push.
