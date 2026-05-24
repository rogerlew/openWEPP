# SR01 Kickoff Agent Prompt

Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.
Autonomy: execute package phases end-to-end and update required artifacts without requesting additional user direction unless hard-blocked.
Required reading (read before edits):
- /workdir/openWEPP/AGENTS.md
- /workdir/openWEPP/docs/codex_exec_plans.md
- /workdir/openWEPP/docs/work-packages/README.md
- /workdir/openWEPP/docs/work-packages/20260522-sr01-slope-soils-model-representation-discovery-001/package.md


You are executing `20260522-sr01-slope-soils-model-representation-discovery-001`.

Objectives:
1. Reconstruct exact slope representation semantics in
   `/workdir/wepp-forest_260430_baseline`.
2. Reconstruct exact soil representation semantics in
   `/workdir/wepp-forest_260430_baseline`.
3. Identify all key consumers and ownership boundaries for these
   representations.
4. Determine architecture-fit constraints for openWEPP typed-state and
   subsystem boundaries.
5. Decide whether slope+soil is the current boundary or whether a larger
   extension boundary should be executed as a series.

Constraints:
- Do not edit CLIM01 queue artifacts in this package.
- Correctness over completion; unresolved ambiguities remain `HOLD`.
- Preserve variable naming continuity with legacy WEPP symbols and explicit
  alias mappings where openWEPP boundaries differ.
- Do not invent physics or semantics; all claims require direct evidence from
  baseline source/contracts/spec references.
- Use truthfulness posture (`Static:` vs `Ran:`) in every artifact.

Required outputs:
- `artifacts/wepp-forest-slope-representation-map.md`
- `artifacts/wepp-forest-soil-representation-map.md`
- `artifacts/slope-soil-consumer-ownership-map.md`
- `artifacts/openwepp-slope-soil-architecture-fit-analysis.md`
- `artifacts/slope-soil-boundary-decision-record.md`
- `artifacts/slope-soil-follow-on-wp-queue.md`
- `artifacts/worker-handoff.md`
- `artifacts/owned-file-manifest.md`
- `artifacts/gate-results.md`
- `artifacts/sr01_disposition.md`
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
