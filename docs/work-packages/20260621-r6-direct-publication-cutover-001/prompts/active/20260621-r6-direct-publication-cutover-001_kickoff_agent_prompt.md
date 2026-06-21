# R6 Direct Publication Cutover Kickoff Prompt

You are executing
`docs/work-packages/20260621-r6-direct-publication-cutover-001/package.md`.

Read and follow:

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/codex_exec_plans.md`
- `docs/architecture/array-native-runtime-specification.md`
- `docs/work-packages/r5-burndown-execplan.md`
- `docs/work-packages/20260621-r6-direct-publication-cutover-001/package.md`
- `docs/work-packages/20260619-perfdeep06-array-native-fast-path-inventory-001/artifacts/perfdeep06-publication-operand-ledger.md`
- `docs/work-packages/20260619-r0-r1-array-native-schema-frame-planning-001/artifacts/publication-ledger-promotion-plan.md`

Execution constraints:

- R5E is complete at pushed commit `d8f6bbea`; do not re-open the old
  `HOLD-R6-R5E-PREREQUISITE` gate.
- The PERFDEEP06 publication operand ledger is promoted into
  `docs/architecture/array-native-runtime-specification.md` section
  `5.2.1 R6 Canonical Publication Operand Ledger`.
- First close `HOLD-R6-DIRECT-PUBLICATION-FRAME-ABSENT`: build a run-bound
  direct publication frame from typed direct run/lane/day state.
- Do not wrap compatibility WB13 rows, runtime surfaces, writeback payloads, or
  stale logical state in direct-named structures; that is explicitly forbidden
  by the promoted architecture ledger.
- After the frame exists, make HBP, WAT, PASS, loss, and manifest read typed
  direct projection only.
- Gate the package on byte/Arrow identity, metadata parity, anti-alias fixtures,
  and independent operand reconstruction.
- Treat Conservation / Publication Acceptance as current-scope acceptance.
- Do not proceed to R7 compatibility-runtime deletion.

Subagent authorization: this package explicitly authorizes
spawning/delegating to read-only ledger-authority, anti-alias fixture,
benchmark runner, reviewer, and verifier subagents for the scopes declared in
`package.md`.

Run the package end to end once authorized. Commit and push only when the user
asks or when an enclosing autonomous ExecPlan explicitly requires it.
