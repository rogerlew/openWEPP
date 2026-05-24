# CLIM12 Kickoff Agent Prompt

Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.
Autonomy: execute package phases end-to-end and update required artifacts without requesting additional user direction unless hard-blocked.
Required reading (read before edits):
- /workdir/openWEPP/AGENTS.md
- /workdir/openWEPP/docs/codex_exec_plans.md
- /workdir/openWEPP/docs/work-packages/README.md
- /workdir/openWEPP/docs/work-packages/20260522-clim12-shared-climate-runtime-adapter-extraction-001/package.md


You are executing
`20260522-clim12-shared-climate-runtime-adapter-extraction-001`.

Objective:
Remove duplicated climate runtime seam logic by extracting a shared
implementation consumed by both hillslope and watershed orchestrators.

Constraints:
- Follow CLIM11 ownership contract as authoritative.
- Preserve current validated runtime behavior while deduplicating logic.
- Use evidence-mode labeling (`Static:` vs `Ran:`) in all artifacts.
- Correctness over completion; unresolved divergence risk remains `HOLD`.

Required outputs:
- `artifacts/shared-climate-runtime-adapter-contract.md`
- `artifacts/shared-adapter-parity-evidence.md`
- `artifacts/clim12_disposition.md`
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
