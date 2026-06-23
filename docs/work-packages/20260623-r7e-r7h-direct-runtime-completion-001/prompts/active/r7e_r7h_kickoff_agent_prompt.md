# R7E-R7H Kickoff Prompt

Execute `docs/work-packages/20260623-r7e-r7h-direct-runtime-completion-001/package.md`
autonomously.

Required reading:

- Root `AGENTS.md`.
- `docs/work-packages/AGENTS.md`.
- `docs/defect_closure_execplans.md`.
- `docs/codex_exec_plans.md`.
- `docs/specifications/science-contracts/AGENTS.md`.
- `docs/architecture/array-native-runtime-specification.md`, especially R7E,
  R7F, R7G, and R7H.
- R7A-R7D8 package artifacts, especially R7D8 parity, verification, review
  disposition, and worker handoff.

Subagent authorization: this package explicitly authorizes
spawning/delegating to read-only reviewer, verifier, and comparator-runner
subagents for R7 default-selection review, no-compatibility proof review,
benchmark/comparator execution, fixture-matrix audit, gate-evidence audit, and
line-count governance review. Expected outputs are compact Markdown findings
summarized into package artifacts; subagents may not edit files.

Execution rules:

- Start by populating `artifacts/required-reading.md`,
  `artifacts/blocker-ledger.md`, and `artifacts/iteration-log.md`.
- Reproduce the current R7D8 starting state or record why the tree has moved.
- Iterate through R7E, R7F, R7G, and R7H in this package. Do not stop after
  the first blocker, new fail-closed marker, failed benchmark, failed source
  scan, missing fixture, or incomplete release checklist.
- If a blocker is in-envelope, fix it in this package, rerun the focused gate,
  rerun the broader gate that failed, update artifacts, and continue.
- A hold is allowed only for a named out-of-envelope boundary,
  missing/contradictory authority, invalid upstream input with correct typed
  guard, unavailable evidence, or a different process family/contract
  authority. The handoff first action must be `close defect <ID>`.
- Do not activate a slower or output-nonidentical direct path as the normal
  runtime. Do not hide compatibility scheduler/writeback/runtime surfaces
  behind direct-mode wrappers.
- Commit and push only after the package reaches complete or legitimate
  executed-held disposition and artifacts are current, and only when the active
  user instruction authorizes publishing.
