# CLIM11 Kickoff Agent Prompt

You are executing
`20260522-clim11-climate-ownership-boundary-reconciliation-001`.

Objective:
Resolve climate forcing ownership boundaries between hillslope and watershed
orchestration surfaces with an explicit, testable routing authority contract.

Constraints:
- Preserve architecture-first and science-contract authority posture.
- Do not use implicit ownership assumptions; document explicit boundaries.
- Use evidence-mode labeling (`Static:` vs `Ran:`) in all artifacts.
- Correctness over completion; unresolved high-severity ambiguity remains
  `HOLD`.

Required outputs:
- `artifacts/climate-ownership-boundary-contract.md`
- `artifacts/clim11_disposition.md`
- `artifacts/worker-handoff.md`
- `artifacts/owned-file-manifest.md`
- `artifacts/gate-results.md`
- `artifacts/review_agent_a.md`
- `artifacts/review_agent_b.md`
- `artifacts/verification_agent_a.md`
- `artifacts/verification_agent_b.md`

Required gates (when code changes are in scope):
1. `cargo fmt --check`
2. `cargo clippy --workspace --all-targets -- -D warnings`
3. `cargo test --workspace`
4. `cargo deny check`
