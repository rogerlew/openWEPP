# ARCH06 Kickoff Agent Prompt

Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.
Autonomy: execute package phases end-to-end and update required artifacts without requesting additional user direction unless hard-blocked.
Required reading (read before edits):
- /workdir/openWEPP/AGENTS.md
- /workdir/openWEPP/docs/codex_exec_plans.md
- /workdir/openWEPP/docs/work-packages/README.md
- /workdir/openWEPP/docs/work-packages/20260521-arch06-watershed-dispatch-scheduler-graph-001/package.md


You are executing `20260521-arch06-watershed-dispatch-scheduler-graph-001`.

Concurrency context:
- This package is running concurrently with `ARCH05`.
- You are not alone in the repo. Do not revert edits made by others.
- Stay within ARCH06 write-set; treat shared files as quarantine-owned.
- If shared-file edits are needed, record `shared-change-request` entries in
  `artifacts/worker-handoff.md` instead of modifying those files directly.

Objectives:
1. Implement `openwepp-watershed-orchestrator` with deterministic dispatch
   scheduler graph semantics.
2. Enforce topology-validation preconditions before dispatch execution.
3. Emit typed outcomes/statuses via `openwepp-sim-contract`.
4. Add crate-local tests for dispatch ordering and failure classes.
5. Publish scheduler architecture/spec docs.
6. Run worker-local gates and publish evidence.
7. Produce review/disposition/verification artifacts.

Constraints:
- Preserve architecture-first and top-down contract authority posture.
- Dispatch order must be deterministic and explicitly encoded.
- No silent fallback on precondition or closure failures; use typed diagnostics.
- Correctness over completion: unresolved high-severity findings remain `HOLD`.

Required outputs:
- Implementation changes in `crates/openwepp-watershed-orchestrator/**`.
- Docs:
  - `docs/architecture/watershed-dispatch-scheduler-graph.md`
  - `docs/specifications/science-contracts/watershed-dispatch-scheduler-contract.md`
- Artifacts:
  - `artifacts/worker-handoff.md`
  - `artifacts/owned-file-manifest.md`
  - `artifacts/gate-results.md`
  - `artifacts/arch06_disposition.md`
  - `artifacts/review_agent_a.md`
  - `artifacts/review_agent_b.md`
  - `artifacts/verification_agent_a.md`
  - `artifacts/verification_agent_b.md`
