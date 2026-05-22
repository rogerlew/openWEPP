# ARCH07 Kickoff Agent Prompt

You are executing `20260521-arch07-kernel-trait-boundary-and-writeback-contract-001`.

Domain context (explicit):
- This is a greenfield scientific hydrology simulation engine (`openWEPP`).
- Work is limited to simulation architecture, numerical/state contracts, and
  Rust implementation quality for erosion/runoff/watershed modeling.

Objectives:
1. Implement `openwepp-kernel-contract` with explicit trait signatures for
   hillslope and watershed kernel invocation.
2. Define typed state-update writeback records and decision outcomes
   (`accept`, `reject`, `apply`) with deterministic status/diagnostic mapping.
3. Integrate both orchestrators to consume kernel contract boundaries while
   retaining orchestrator ownership of state mutation.
4. Add integration tests for writeback success, rejection, and typed failure
   propagation.
5. Publish architecture/spec docs for kernel boundary and writeback semantics.
6. Run full repository gates and publish evidence.
7. Produce review/disposition/verification artifacts.

Constraints:
- Kernels must remain pure transform surfaces; they do not own final state
  mutation.
- Orchestrators are the only mutation-commit authority.
- No silent fallback for rejected/non-finite/out-of-domain writeback records.
- Preserve ARCH03 status taxonomy and canonical symbol continuity policy.
- Correctness over completion: unresolved high-severity findings remain `HOLD`.

Required outputs:
- Code:
  - `crates/openwepp-kernel-contract/**`
  - orchestrator integration updates in
    - `crates/openwepp-hillslope-orchestrator/src/lib.rs`
    - `crates/openwepp-watershed-orchestrator/src/lib.rs`
  - workspace wiring updates (`Cargo.toml`, `Cargo.lock` as needed)
- Tests:
  - `tests/integration/kernel_writeback_contract.rs`
- Docs:
  - `docs/architecture/kernel-trait-boundary-and-writeback.md`
  - `docs/specifications/science-contracts/kernel-writeback-contract.md`
- Artifacts:
  - `artifacts/worker-handoff.md`
  - `artifacts/owned-file-manifest.md`
  - `artifacts/gate-results.md`
  - `artifacts/arch07_disposition.md`
  - `artifacts/review_agent_a.md`
  - `artifacts/review_agent_b.md`
  - `artifacts/verification_agent_a.md`
  - `artifacts/verification_agent_b.md`
