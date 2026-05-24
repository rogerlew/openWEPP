# ARCH11 Kickoff Agent Prompt

Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.
Autonomy: execute package phases end-to-end and update required artifacts without requesting additional user direction unless hard-blocked.
Required reading (read before edits):
- /workdir/openWEPP/AGENTS.md
- /workdir/openWEPP/docs/codex_exec_plans.md
- /workdir/openWEPP/docs/work-packages/README.md
- /workdir/openWEPP/docs/work-packages/20260522-arch11-comparator-tier-routing-metadata-integration-001/package.md


You are executing `20260522-arch11-comparator-tier-routing-metadata-integration-001`.

Domain context (explicit):
- This is a greenfield scientific hydrology simulation engine (`openWEPP`).
- Work is strictly simulation architecture and comparator-governance metadata.
- This package does not create harmful capabilities; it implements typed
  metadata/routing for scientific model validation workflows.

Objectives:
1. Implement a typed comparator-tier metadata surface (`ARCH11`) aligned with
   ADR-0011 confidence-tier governance.
2. Define deterministic routing metadata and message-id classes for:
   - higher-confidence surfaces (single OFE + daily water-balance)
   - investigation-tier surfaces (hourly and watershed)
3. Integrate metadata propagation through summary/reporting outputs.
4. Add integration tests for deterministic tier mapping and typed invalid-path
   behavior.
5. Publish architecture/spec docs for comparator-tier routing governance.
6. Run workspace gates and publish evidence.
7. Produce review/disposition/verification artifacts.

Constraints:
- Legacy binary comparison is a flagging mechanism, not universal oracle.
- No silent fallback/default tier assignment on invalid metadata.
- Preserve ARCH03 status-taxonomy semantics and ARCH10 summary semantics.
- Correctness over completion: unresolved high-severity findings remain `HOLD`.

Required outputs:
- Code:
  - `crates/openwepp-comparator-metadata/**`
  - `crates/openwepp-summary-accumulator/src/lib.rs`
  - `tests/integration/comparator_tier_routing_metadata.rs`
- Docs:
  - `docs/architecture/comparator-tier-routing-metadata.md`
  - `docs/specifications/science-contracts/comparator-tier-routing-metadata-contract.md`
- Artifacts:
  - `artifacts/worker-handoff.md`
  - `artifacts/owned-file-manifest.md`
  - `artifacts/gate-results.md`
  - `artifacts/arch11_disposition.md`
  - `artifacts/review_agent_a.md`
  - `artifacts/review_agent_b.md`
  - `artifacts/verification_agent_a.md`
  - `artifacts/verification_agent_b.md`
