# CLIM01 Kickoff Agent Prompt

You are executing
`20260522-clim01-wepp-forest-climate-model-discovery-and-specification-001`.

Objectives:
1. Determine exactly what `wepp-forest_260430_baseline` climate modeling does for
   continuous-daily and breakfile execution paths.
2. Author an openWEPP-owned detailed climate model specification from that
   evidence.
3. Define explicit downstream consumer requirements and integration
   constraints.
4. Map climate modeling requirements to parser contracts and architecture
   boundaries.
5. Produce dual review/disposition/verification artifacts.

Mandatory scope boundaries:
- Include: continuous-daily and breakfile climate behavior.
- Exclude: single-storm modeling and single-storm climates.

Constraints:
- Correctness over completion; unresolved ambiguities remain `HOLD`.
- Use `/workdir/wepp-forest_260430_baseline` as the canonical legacy
  authority path for behavior reconstruction in this package.
- Preserve variable naming continuity with legacy WEPP symbols and explicit
  alias mappings where openWEPP boundaries differ.
- Do not invent physics or constants; all claims must trace to references,
  static code analysis, or recorded execution evidence.
- Use truthfulness posture (`Static:` vs `Ran:`) in every artifact.

Required outputs:
- `artifacts/wepp-forest-climate-model-behavior-map.md`
- `artifacts/openwepp-climate-model-detailed-specification.md`
- `artifacts/climate-consumer-requirements.md`
- `artifacts/climate-parser-architecture-integration-map.md`
- `artifacts/climate-coverage-and-exclusions-matrix.md`
- `artifacts/climate-implementation-wp-queue.md`
- `artifacts/worker-handoff.md`
- `artifacts/owned-file-manifest.md`
- `artifacts/gate-results.md`
- `artifacts/clim01_disposition.md`
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
