# R6A Run-Bound Direct Publication Frame Kickoff Prompt

You are executing
`docs/work-packages/20260621-r6a-run-bound-direct-publication-frame-001/package.md`.

Read and follow:

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/architecture/array-native-runtime-specification.md`
- `docs/work-packages/20260621-r6-direct-publication-cutover-001/package.md`
- `docs/work-packages/20260621-r6a-run-bound-direct-publication-frame-001/package.md`
- `crates/AGENTS.md`
- `tests/AGENTS.md`

Execution constraints:

- Close the direct-publication-frame blocker, not the full R6 output cutover.
- Do not use `DirectSkeletonNoop`, `DirectSkeletonShadowOnly`, counters, or
  shadow-only evidence as acceptance evidence.
- Do not build direct-named wrappers around compatibility WB13 rows, runtime
  surfaces, writeback payloads, stale logical state, or diagnostic
  compatibility ledgers.
- Prove the producer, in-memory frame, runner handoff, downstream consumers, and
  old-path negative scan before closure.
- Add anti-alias fixtures and independent reconstruction before accepting each
  projection consumer.
- Preserve public compatibility output identity until a later package explicitly
  authorizes production output writer cutover.

Subagent authorization: this package explicitly authorizes
spawning/delegating to read-only consumer-path, no-compatibility, anti-alias,
reconstruction, reviewer, verifier, and benchmark-runner subagents for the
scopes declared in `package.md`.

Run the package end to end. Commit and push only when the user asks or an
enclosing autonomous ExecPlan explicitly requires it.
