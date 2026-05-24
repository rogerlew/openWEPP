# CLIM03 Kickoff Agent Prompt

Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.
Autonomy: execute package phases end-to-end and update required artifacts without requesting additional user direction unless hard-blocked.
Required reading (read before edits):
- /workdir/openWEPP/AGENTS.md
- /workdir/openWEPP/docs/codex_exec_plans.md
- /workdir/openWEPP/docs/work-packages/README.md
- /workdir/openWEPP/docs/work-packages/20260522-clim03-continuous-daily-climate-runtime-kernel-port-001/package.md


You are executing `20260522-clim03-continuous-daily-climate-runtime-kernel-port-001`.

Objectives:
1. Port continuous-daily (`ibrkpt=0`) legacy climate runtime behavior into
   openWEPP runtime forcing surfaces.
2. Implement ratified version policy branches:
   - support explicit `datver=0.0` override (`iclig=0`)
   - support `datver>=4.0` branch (`iclig=1`)
   - reject pre-4 nonzero branch (`0.0<datver<4.0`, legacy `iclig=2`).
3. Implement continuous-daily normalization, unit conversion, and
   disaggregation/event-shape behavior with typed guard surfaces.
4. Curate and document `.cli` fixture corpus from `/wc1/runs/**/wepp/runs/*.cli`
   for integration/parity coverage.
5. Add integration tests and produce dual review/disposition/verification
   artifacts.

Constraints:
- Preserve typed seam and unit-boundary direction from ARCH15/ARCH17/CLIM02.
- Do not introduce silent fallback behavior for invalid climate forcing.
- Maintain canonical WEPP variable/symbol continuity in adapter/runtime mapping.
- Correctness over completion: unresolved high-severity runtime parity gaps
  remain `HOLD`.
- Use truthfulness posture (`Static:` vs `Ran:`) in all artifacts.

Required outputs:
- `artifacts/continuous-daily-kernel-port-contract.md`
- `artifacts/clim03-cli-fixture-corpus-manifest.md`
- `artifacts/continuous-daily-runtime-parity-evidence.md`
- `artifacts/worker-handoff.md`
- `artifacts/owned-file-manifest.md`
- `artifacts/gate-results.md`
- `artifacts/clim03_disposition.md`
- `artifacts/review_agent_a.md`
- `artifacts/review_agent_b.md`
- `artifacts/verification_agent_a.md`
- `artifacts/verification_agent_b.md`

Required gates:
1. `cargo fmt --check`
2. `cargo clippy --workspace --all-targets -- -D warnings`
3. `cargo test --workspace`
4. `cargo deny check`
