# PL07 Kickoff Agent Prompt

You are executing `20260522-pl07-parser-to-runtime-integration-tests-001`.

Objectives:
1. Add integration tests asserting full PL runtime surface projection from
   `.man` fixtures.
2. Add typed reject-path assertions for required PL seam inputs.
3. Validate runtime surface projection coverage across schedule, growth, and
   decomposition/resup families.
4. Publish integration evidence, coverage maps, and disposition outputs.

Constraints:
- Preserve strict seam behavior: no silent defaults on missing/invalid inputs.
- Maintain canonical symbol continuity assumptions established in PL04.
- Use evidence-mode labeling (`Static:` vs `Ran:`) in all artifacts.
- Correctness over completion; unresolved high-severity coverage gaps remain
  `HOLD`.

Required outputs:
- `artifacts/pl07-fixture-runtime-projection-coverage-matrix.md`
- `artifacts/pl07-typed-reject-path-catalog.md`
- `artifacts/pl07-runtime-surface-assertion-map.md`
- `artifacts/pl07-parser-to-runtime-integration-evidence.md`
- `artifacts/worker-handoff.md`
- `artifacts/owned-file-manifest.md`
- `artifacts/gate-results.md`
- `artifacts/pl07_disposition.md`
- `artifacts/review_agent_a.md`
- `artifacts/review_agent_b.md`
- `artifacts/verification_agent_a.md`
- `artifacts/verification_agent_b.md`

Required gates:
- If code is changed, run:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`
