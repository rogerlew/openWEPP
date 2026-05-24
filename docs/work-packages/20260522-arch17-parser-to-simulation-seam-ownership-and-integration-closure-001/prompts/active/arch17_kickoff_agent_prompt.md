# ARCH17 Kickoff Agent Prompt

Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.
Autonomy: execute package phases end-to-end and update required artifacts without requesting additional user direction unless hard-blocked.
Required reading (read before edits):
- /workdir/openWEPP/AGENTS.md
- /workdir/openWEPP/docs/codex_exec_plans.md
- /workdir/openWEPP/docs/work-packages/README.md
- /workdir/openWEPP/docs/work-packages/20260522-arch17-parser-to-simulation-seam-ownership-and-integration-closure-001/package.md


You are executing `20260522-arch17-parser-to-simulation-seam-ownership-and-integration-closure-001`.

Objectives:
1. Define parser-to-simulation seam ownership and authoritative runtime input
   boundaries.
2. Implement integration wiring so selected parser outputs are consumed by
   runtime/orchestrator surfaces through explicit typed adapters/contracts.
3. Add integration acceptance tests proving parser-to-runtime closure.
4. Ensure integration closure is not masked by root-crate re-export-only
   dependency relationships.
5. Produce dual review/disposition/verification artifacts.

Constraints:
- Preserve typed seam invariants introduced in ARCH15.
- Do not introduce silent compatibility fallbacks that hide missing required
  runtime inputs.
- Keep canonical WEPP variable/symbol continuity at boundary definitions.
- Correctness over completion: unresolved seam ownership ambiguity remains
  `HOLD`.
- Use truthfulness posture (`Static:` vs `Ran:`) in artifacts.

Required gates:
1. `cargo fmt --check`
2. `cargo clippy --workspace --all-targets -- -D warnings`
3. `cargo test --workspace`
4. `cargo deny check`

Required outputs:
- `artifacts/parser-to-simulation-seam-ownership-contract.md`
- `artifacts/runtime-input-surface-classification.md`
- `artifacts/parser-to-runtime-integration-evidence.md`
- `artifacts/worker-handoff.md`
- `artifacts/owned-file-manifest.md`
- `artifacts/gate-results.md`
- `artifacts/arch17_disposition.md`
- `artifacts/review_agent_a.md`
- `artifacts/review_agent_b.md`
- `artifacts/verification_agent_a.md`
- `artifacts/verification_agent_b.md`
