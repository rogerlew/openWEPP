# CLIM15 Kickoff Agent Prompt

You are executing
`20260522-clim15-climate-runtime-error-taxonomy-reachability-reconciliation-001`.

Objective:
Reconcile climate runtime error taxonomy with actual reachable guard paths,
including removal/redesign of unreachable or misnamed variants.

Constraints:
- Build on CLIM12/CLIM14 runtime surfaces.
- Treat taxonomy coverage as valid only when exercised via real guards.
- Use evidence-mode labeling (`Static:` vs `Ran:`) in all artifacts.
- Correctness over completion; unresolved taxonomy reachability remains `HOLD`.

Required outputs:
- `artifacts/climate-runtime-error-taxonomy-reachability.md`
- `artifacts/guard-path-taxonomy-evidence.md`
- `artifacts/clim15_disposition.md`
- `artifacts/worker-handoff.md`
- `artifacts/owned-file-manifest.md`
- `artifacts/gate-results.md`
- `artifacts/review_agent_a.md`
- `artifacts/review_agent_b.md`
- `artifacts/verification_agent_a.md`
- `artifacts/verification_agent_b.md`

Required gates:
1. `cargo fmt --check`
2. `cargo clippy --workspace --all-targets -- -D warnings`
3. `cargo test --workspace`
4. `cargo deny check`
