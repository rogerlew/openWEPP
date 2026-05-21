# ARCH04 Kickoff Agent Prompt

You are executing `20260521-arch04-topology-graph-and-validation-gate-001`.

Objectives:
1. Implement a new topology crate (`openwepp-topology`) and wire it into the
   workspace.
2. Implement typed graph model for hillslope/channel/impoundment topology.
3. Implement pre-execution validation gate enforcing topology closure and
   invariant constraints.
4. Integrate gate diagnostics with `openwepp-sim-contract` status/closure
   primitives.
5. Add integration tests for canonical valid and invalid topology scenarios.
6. Run repo gates and publish evidence.
7. Produce review/disposition/verification artifacts.

Constraints:
- Topology validation must happen before timestep execution surfaces.
- Use typed errors/status; no silent fallback for topology closure failures.
- Preserve architecture-first + contract-authority posture from ADR-0011/ARCH02.
- Correctness over completion: unresolved high-severity findings remain `HOLD`.
- Keep scope within the ARCH04 write-set unless a scope amendment is recorded.

Required outputs:
- Implementation changes in `crates/openwepp-topology/**` plus workspace
  integration.
- Integration tests:
  - `tests/integration/topology_graph_validation_gate.rs`
- Fixtures:
  - `tests/fixtures/topology/**`
- Docs:
  - `docs/architecture/topology-graph-model.md`
  - `docs/specifications/science-contracts/topology-validation-gate.md`
- Artifacts:
  - `artifacts/worker-handoff.md`
  - `artifacts/owned-file-manifest.md`
  - `artifacts/gate-results.md`
  - `artifacts/arch04_disposition.md`
  - `artifacts/review_agent_a.md`
  - `artifacts/review_agent_b.md`
  - `artifacts/verification_agent_a.md`
  - `artifacts/verification_agent_b.md`
