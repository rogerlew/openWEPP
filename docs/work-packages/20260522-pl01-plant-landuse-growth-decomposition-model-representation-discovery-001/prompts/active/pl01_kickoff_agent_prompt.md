# PL01 Kickoff Agent Prompt

You are executing `20260522-pl01-plant-landuse-growth-decomposition-model-representation-discovery-001`.

Objectives:
1. Reconstruct exact plant representation semantics in
   `/workdir/wepp-forest_260430_baseline`.
2. Reconstruct exact landuse-management coupling representation semantics
   downstream of `.man` surfaces.
3. Reconstruct exact growth and decomposition representation semantics and
   downstream state transitions.
4. Identify all key consumers and ownership boundaries for these
   representations.
5. Determine architecture-fit constraints for openWEPP typed-state and
   subsystem boundaries.
6. Decide immediate downstream implementation boundary and publish sequenced
   follow-on packages.

Constraints:
- Correctness over completion; unresolved ambiguities remain `HOLD`.
- Preserve variable naming continuity with legacy WEPP symbols and explicit
  alias mappings where openWEPP boundaries differ.
- Do not invent physics or semantics; all claims require direct evidence from
  baseline source/contracts/spec references.
- Use truthfulness posture (`Static:` vs `Ran:`) in every artifact.

Required outputs:
- `artifacts/wepp-forest-plant-representation-map.md`
- `artifacts/wepp-forest-landuse-management-representation-map.md`
- `artifacts/wepp-forest-growth-representation-map.md`
- `artifacts/wepp-forest-decomposition-representation-map.md`
- `artifacts/plant-landuse-growth-decomposition-consumer-ownership-map.md`
- `artifacts/openwepp-plant-landuse-growth-decomposition-architecture-fit-analysis.md`
- `artifacts/plant-landuse-growth-decomposition-boundary-decision-record.md`
- `artifacts/plant-landuse-growth-decomposition-follow-on-wp-queue.md`
- `artifacts/worker-handoff.md`
- `artifacts/owned-file-manifest.md`
- `artifacts/gate-results.md`
- `artifacts/pl01_disposition.md`
- `artifacts/review_agent_a.md`
- `artifacts/review_agent_b.md`
- `artifacts/verification_agent_a.md`
- `artifacts/verification_agent_b.md`

Required gates:
- Docs-only execution: artifact completeness and consistency checks.
- If code is changed, run:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`
