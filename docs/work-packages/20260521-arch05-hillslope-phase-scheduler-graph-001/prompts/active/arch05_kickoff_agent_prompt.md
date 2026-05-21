# ARCH05 Kickoff Agent Prompt

You are executing `20260521-arch05-hillslope-phase-scheduler-graph-001`.

Concurrency context:
- This package is running concurrently with `ARCH06`.
- You are not alone in the repo. Do not revert edits made by others.
- Stay within ARCH05 write-set; treat shared files as quarantine-owned.
- If shared-file edits are needed, record `shared-change-request` entries in
  `artifacts/worker-handoff.md` instead of modifying those files directly.

Objectives:
1. Implement `openwepp-hillslope-orchestrator` with deterministic phase
   scheduler graph semantics.
2. Enforce topology-validation preconditions before phase execution.
3. Emit typed outcomes/statuses via `openwepp-sim-contract`.
4. Add crate-local tests for ordering and failure classes.
5. Publish scheduler architecture/spec docs.
6. Run worker-local gates and publish evidence.
7. Produce review/disposition/verification artifacts.

Constraints:
- Preserve architecture-first and top-down contract authority posture.
- Scheduler order must be deterministic and explicitly encoded.
- No silent fallback on precondition or closure failures; use typed diagnostics.
- Correctness over completion: unresolved high-severity findings remain `HOLD`.

Required outputs:
- Implementation changes in `crates/openwepp-hillslope-orchestrator/**`.
- Docs:
  - `docs/architecture/hillslope-phase-scheduler-graph.md`
  - `docs/specifications/science-contracts/hillslope-phase-scheduler-contract.md`
- Artifacts:
  - `artifacts/worker-handoff.md`
  - `artifacts/owned-file-manifest.md`
  - `artifacts/gate-results.md`
  - `artifacts/arch05_disposition.md`
  - `artifacts/review_agent_a.md`
  - `artifacts/review_agent_b.md`
  - `artifacts/verification_agent_a.md`
  - `artifacts/verification_agent_b.md`
