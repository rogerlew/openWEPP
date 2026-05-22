# ARCH08 Kickoff Agent Prompt

You are executing `20260521-arch08-sidecar-and-legacy-bridge-adapter-isolation-001`.

Domain context (explicit):
- This is a greenfield scientific hydrology simulation engine (`openWEPP`).
- Work is strictly simulation architecture and typed edge-adapter behavior.

Parallel context:
- This package runs in parallel with `ARCH09` and `ARCH10`.
- You are not alone in the repo. Do not revert edits made by others.
- Stay within ARCH08 write-set and treat shared files as quarantine-owned.

Objectives:
1. Implement `openwepp-legacy-bridge` for sidecar/HBP edge compatibility.
2. Define typed adapter requests/responses/errors with strict/compat policy
   behavior isolated from core kernels/orchestrators.
3. Add crate-local tests for compatibility and typed failure paths.
4. Publish architecture/spec docs for adapter boundary semantics.
5. Run worker-local gates and publish evidence.
6. Produce review/disposition/verification artifacts.

Constraints:
- Core kernels/orchestrators must not absorb legacy sidecar policy logic.
- No silent fallback for adapter failures; use typed diagnostics.
- Correctness over completion: unresolved high-severity findings remain `HOLD`.

Required outputs:
- Code: `crates/openwepp-legacy-bridge/**`
- Docs:
  - `docs/architecture/legacy-sidecar-bridge-boundary.md`
  - `docs/specifications/science-contracts/legacy-sidecar-bridge-contract.md`
- Artifacts:
  - `artifacts/worker-handoff.md`
  - `artifacts/owned-file-manifest.md`
  - `artifacts/gate-results.md`
  - `artifacts/arch08_disposition.md`
  - `artifacts/review_agent_a.md`
  - `artifacts/review_agent_b.md`
  - `artifacts/verification_agent_a.md`
  - `artifacts/verification_agent_b.md`
