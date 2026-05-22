# ARCH09 Kickoff Agent Prompt

You are executing `20260521-arch09-unit-safe-boundary-types-001`.

Domain context (explicit):
- This is a greenfield scientific hydrology simulation engine (`openWEPP`).
- Work is strictly simulation architecture and typed numerical boundary safety.

Parallel context:
- This package runs in parallel with `ARCH08` and `ARCH10`.
- You are not alone in the repo. Do not revert edits made by others.
- Stay within ARCH09 write-set and treat shared files as quarantine-owned.

Objectives:
1. Implement `openwepp-unit-boundary` for unit-safe boundary wrappers.
2. Define typed runoff/flow/storage/rate boundary constructors and conversion
   helpers with domain guards.
3. Add crate-local tests for valid/invalid boundary construction and
   conversion behavior.
4. Publish architecture/spec docs for unit-safe boundary policy.
5. Run worker-local gates and publish evidence.
6. Produce review/disposition/verification artifacts.

Constraints:
- No silent coercion of invalid/non-finite boundary values.
- Preserve ARCH03/ARCH07 status and ownership semantics.
- Correctness over completion: unresolved high-severity findings remain `HOLD`.

Required outputs:
- Code: `crates/openwepp-unit-boundary/**`
- Docs:
  - `docs/architecture/unit-safe-boundary-types.md`
  - `docs/specifications/science-contracts/unit-safe-boundary-types-contract.md`
- Artifacts:
  - `artifacts/worker-handoff.md`
  - `artifacts/owned-file-manifest.md`
  - `artifacts/gate-results.md`
  - `artifacts/arch09_disposition.md`
  - `artifacts/review_agent_a.md`
  - `artifacts/review_agent_b.md`
  - `artifacts/verification_agent_a.md`
  - `artifacts/verification_agent_b.md`
