# CLIM04 Kickoff Agent Prompt

Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.
Autonomy: execute package phases end-to-end and update required artifacts without requesting additional user direction unless hard-blocked.
Required reading (read before edits):
- /workdir/openWEPP/AGENTS.md
- /workdir/openWEPP/docs/codex_exec_plans.md
- /workdir/openWEPP/docs/work-packages/README.md
- /workdir/openWEPP/docs/work-packages/20260522-clim04-breakpoint-runtime-port-and-policy-reconciliation-001/package.md


You are executing
`20260522-clim04-breakpoint-runtime-port-and-policy-reconciliation-001`.

Objectives:
1. Port breakpoint (`ibrkpt=1`) legacy climate runtime behavior into openWEPP
   runtime forcing surfaces.
2. Implement breakpoint event semantics:
   - `stmstr` capture from first breakpoint time,
   - elapsed-time normalization from absolute breakpoint times,
   - interval-intensity/event-shape projection from cumulative breakpoint
     depth.
3. Implement ratified breakpoint policies:
   - parser/runtime alignment to `1500` breakpoint capacity target,
   - strict `dtime>0` interval guard for all intervals (duplicate/decreasing
     breakpoint times are errors).
4. Implement explicit compatibility controls for any non-default legacy
   behavior without weakening strict defaults.
5. Curate and document breakpoint `.cli` fixture corpus from
   `/wc1/runs/**/wepp/runs/*.cli` and add integration/parity coverage.
6. Produce dual review/disposition/verification artifacts.

Constraints:
- Preserve typed seam and unit-boundary direction from ARCH15/ARCH17/CLIM02.
- Do not introduce silent fallback behavior for invalid breakpoint forcing.
- Maintain canonical WEPP variable/symbol continuity in adapter/runtime
  mapping.
- Correctness over completion: unresolved high-severity breakpoint parity gaps
  remain `HOLD`.
- Use truthfulness posture (`Static:` vs `Ran:`) in all artifacts.

Required outputs:
- `artifacts/breakpoint-kernel-port-contract.md`
- `artifacts/clim04-cli-fixture-corpus-manifest.md`
- `artifacts/breakpoint-runtime-parity-evidence.md`
- `artifacts/breakpoint-policy-compat-controls.md`
- `artifacts/worker-handoff.md`
- `artifacts/owned-file-manifest.md`
- `artifacts/gate-results.md`
- `artifacts/clim04_disposition.md`
- `artifacts/review_agent_a.md`
- `artifacts/review_agent_b.md`
- `artifacts/verification_agent_a.md`
- `artifacts/verification_agent_b.md`

Required gates:
1. `cargo fmt --check`
2. `cargo clippy --workspace --all-targets -- -D warnings`
3. `cargo test --workspace`
4. `cargo deny check`
