# CLIM02 Kickoff Agent Prompt

Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.
Autonomy: execute package phases end-to-end and update required artifacts without requesting additional user direction unless hard-blocked.
Required reading (read before edits):
- /workdir/openWEPP/AGENTS.md
- /workdir/openWEPP/docs/codex_exec_plans.md
- /workdir/openWEPP/docs/work-packages/README.md
- /workdir/openWEPP/docs/work-packages/20260522-clim02-climate-parser-to-runtime-seam-adapters-001/package.md


You are executing `20260522-clim02-climate-parser-to-runtime-seam-adapters-001`.

Objectives:
1. Implement `HS-CLIM-SEAM-001` and `WS-CLIM-SEAM-001` climate
   parser-to-runtime adapters.
2. Define and wire typed `CLIM-RUNTIME-E-*` failure taxonomy for seam/adapter
   failures.
3. Enforce ratified CLIM01 policy at seam boundaries:
   - support explicit `datver=0.0` override (`iclig=0`)
   - support CLIGEN `4.0+` branch (`iclig=1`)
   - reject pre-4 nonzero branch (`0.0<datver<4.0`, `iclig=2`) via explicit typed guard.
   - reject duplicate/decreasing breakpoint times with strict `dtime>0` guard
     for all intervals, regardless of `drain`.
4. Add integration tests proving climate parser outputs are consumed through
   runtime seams in orchestrator surfaces.
5. Produce dual review/disposition/verification artifacts.

Constraints:
- Preserve typed seam and unit-boundary direction from ARCH15/ARCH17.
- Do not introduce silent compatibility fallbacks for missing required climate
  runtime inputs.
- Keep canonical WEPP symbol continuity in seam/request structures.
- Correctness over completion: unresolved high-severity seam ambiguity remains
  `HOLD`.
- Use truthfulness posture (`Static:` vs `Ran:`) in all artifacts.

Required outputs:
- `artifacts/climate-seam-adapter-ownership-contract.md`
- `artifacts/climate-runtime-error-taxonomy.md`
- `artifacts/climate-parser-runtime-seam-integration-evidence.md`
- `artifacts/worker-handoff.md`
- `artifacts/owned-file-manifest.md`
- `artifacts/gate-results.md`
- `artifacts/clim02_disposition.md`
- `artifacts/review_agent_a.md`
- `artifacts/review_agent_b.md`
- `artifacts/verification_agent_a.md`
- `artifacts/verification_agent_b.md`

Required gates:
1. `cargo fmt --check`
2. `cargo clippy --workspace --all-targets -- -D warnings`
3. `cargo test --workspace`
4. `cargo deny check`
