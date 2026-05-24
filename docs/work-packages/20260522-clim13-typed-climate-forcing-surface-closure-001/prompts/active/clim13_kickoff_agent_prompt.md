# CLIM13 Kickoff Agent Prompt

Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.
Autonomy: execute package phases end-to-end and update required artifacts without requesting additional user direction unless hard-blocked.
Required reading (read before edits):
- /workdir/openWEPP/AGENTS.md
- /workdir/openWEPP/docs/codex_exec_plans.md
- /workdir/openWEPP/docs/work-packages/README.md
- /workdir/openWEPP/docs/work-packages/20260522-clim13-typed-climate-forcing-surface-closure-001/package.md


You are executing
`20260522-clim13-typed-climate-forcing-surface-closure-001`.

Objective:
Implement typed climate forcing surfaces for breakpoint/runtime payloads and
remove dynamic runtime string-key synthesis from hot-path behavior.

Constraints:
- Build on CLIM12 shared extraction surfaces.
- Preserve canonical WEPP symbol alias continuity at seam boundaries.
- Use evidence-mode labeling (`Static:` vs `Ran:`) in all artifacts.
- Correctness over completion; unresolved typed-surface drift remains `HOLD`.

Required outputs:
- `artifacts/typed-climate-forcing-surface-contract.md`
- `artifacts/typed-forcing-migration-evidence.md`
- `artifacts/clim13_disposition.md`
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
