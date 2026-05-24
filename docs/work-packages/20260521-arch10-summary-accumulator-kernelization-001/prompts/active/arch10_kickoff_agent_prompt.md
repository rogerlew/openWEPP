# ARCH10 Kickoff Agent Prompt

Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.
Autonomy: execute package phases end-to-end and update required artifacts without requesting additional user direction unless hard-blocked.
Required reading (read before edits):
- /workdir/openWEPP/AGENTS.md
- /workdir/openWEPP/docs/codex_exec_plans.md
- /workdir/openWEPP/docs/work-packages/README.md
- /workdir/openWEPP/docs/work-packages/20260521-arch10-summary-accumulator-kernelization-001/package.md


You are executing `20260521-arch10-summary-accumulator-kernelization-001`.

Domain context (explicit):
- This is a greenfield scientific hydrology simulation engine (`openWEPP`).
- Work is strictly simulation architecture and typed summary accumulation.

Parallel context:
- This package runs in parallel with `ARCH08` and `ARCH09`.
- You are not alone in the repo. Do not revert edits made by others.
- Stay within ARCH10 write-set and treat shared files as quarantine-owned.

Objectives:
1. Implement `openwepp-summary-accumulator` for daily/monthly/yearly/EOS
   rollup kernelization.
2. Define typed accumulator inputs/outputs and status outcomes.
3. Add crate-local tests for window transitions and accumulation correctness.
4. Publish architecture/spec docs for summary kernel semantics.
5. Run worker-local gates and publish evidence.
6. Produce review/disposition/verification artifacts.

Constraints:
- Accumulator behavior must be deterministic and typed.
- No silent fallback for invalid accumulation inputs.
- Correctness over completion: unresolved high-severity findings remain `HOLD`.

Required outputs:
- Code: `crates/openwepp-summary-accumulator/**`
- Docs:
  - `docs/architecture/summary-accumulator-kernelization.md`
  - `docs/specifications/science-contracts/summary-accumulator-contract.md`
- Artifacts:
  - `artifacts/worker-handoff.md`
  - `artifacts/owned-file-manifest.md`
  - `artifacts/gate-results.md`
  - `artifacts/arch10_disposition.md`
  - `artifacts/review_agent_a.md`
  - `artifacts/review_agent_b.md`
  - `artifacts/verification_agent_a.md`
  - `artifacts/verification_agent_b.md`
