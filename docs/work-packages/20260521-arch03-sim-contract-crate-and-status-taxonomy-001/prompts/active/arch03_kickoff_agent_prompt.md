# ARCH03 Kickoff Agent Prompt

You are executing `20260521-arch03-sim-contract-crate-and-status-taxonomy-001`.

Objectives:
1. Implement a new simulation contract crate (`openwepp-sim-contract`) and wire
   it into the workspace.
2. Implement unified typed status taxonomy for simulation phases and kernel
   outcomes.
3. Implement closure/invariant check primitives with typed violation results.
4. Implement canonical WEPP/wepp-forest symbol alias registry with explicit
   openWEPP boundary-name mappings.
5. Add tests for taxonomy behavior, closure checks, and alias resolution.
6. Run repo gates and publish evidence.
7. Produce review/disposition/verification artifacts.

Constraints:
- Preserve architecture-first and top-down science-contract posture.
- Canonical WEPP/wepp-forest variable names remain authoritative in alias
  tables; openWEPP names must be mapped explicitly.
- Correctness over completion: unresolved high-severity findings remain `HOLD`.
- No silent fallback for failed closure/invariant checks; violations must be
  typed and explicit.
- Keep scope within the ARCH03 write-set unless a scope amendment is recorded.

Required outputs:
- Implementation changes in `crates/openwepp-sim-contract/**` plus workspace
  integration.
- Integration tests:
  - `tests/integration/sim_contract_status_taxonomy.rs`
  - `tests/integration/sim_contract_closure_checks.rs`
  - `tests/integration/sim_contract_symbol_alias_registry.rs`
- Docs:
  - `docs/specifications/science-contracts/status-taxonomy.md`
  - `docs/specifications/science-contracts/closure-check-primitives.md`
  - `docs/specifications/science-contracts/symbol-alias-registry.md`
- Artifacts:
  - `artifacts/worker-handoff.md`
  - `artifacts/owned-file-manifest.md`
  - `artifacts/gate-results.md`
  - `artifacts/arch03_disposition.md`
  - `artifacts/review_agent_a.md`
  - `artifacts/review_agent_b.md`
  - `artifacts/verification_agent_a.md`
  - `artifacts/verification_agent_b.md`
