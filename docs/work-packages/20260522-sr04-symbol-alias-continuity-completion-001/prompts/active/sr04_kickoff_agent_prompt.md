# SR04 Kickoff Agent Prompt

Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.
Autonomy: execute package phases end-to-end and update required artifacts without requesting additional user direction unless hard-blocked.
Required reading (read before edits):
- /workdir/openWEPP/AGENTS.md
- /workdir/openWEPP/docs/codex_exec_plans.md
- /workdir/openWEPP/docs/work-packages/README.md
- /workdir/openWEPP/docs/work-packages/20260522-sr04-symbol-alias-continuity-completion-001/package.md


You are executing `20260522-sr04-symbol-alias-continuity-completion-001`.

Objective:
Expand `openwepp-sim-contract` symbol alias continuity to include slope runtime
surfaces delivered by SR02 and expanded soil runtime surfaces delivered by
SR03.

Constraints:
- Preserve architecture-first + contract-first posture.
- Preserve canonical variable/symbol continuity with explicit alias mapping
  where openWEPP boundary names differ.
- Avoid alias ambiguity and silent fallback behavior for required symbols.
- Use evidence-mode labeling (`Static:` vs `Ran:`) in all artifacts.
- Correctness over completion; unresolved high-severity alias ambiguity remains
  `HOLD`.

Required outputs:
- `artifacts/slope-soil-symbol-alias-continuity-table.md`
- `artifacts/symbol-alias-registry-implementation-evidence.md`
- `artifacts/symbol-alias-consumer-coverage-matrix.md`
- `artifacts/sr04_disposition.md`
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
