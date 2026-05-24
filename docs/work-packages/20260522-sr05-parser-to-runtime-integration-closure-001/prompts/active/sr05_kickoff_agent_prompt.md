# SR05 Kickoff Agent Prompt

Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.
Autonomy: execute package phases end-to-end and update required artifacts without requesting additional user direction unless hard-blocked.
Required reading (read before edits):
- /workdir/openWEPP/AGENTS.md
- /workdir/openWEPP/docs/codex_exec_plans.md
- /workdir/openWEPP/docs/work-packages/README.md
- /workdir/openWEPP/docs/work-packages/20260522-sr05-parser-to-runtime-integration-closure-001/package.md


You are executing `20260522-sr05-parser-to-runtime-integration-closure-001`.

Objective:
Close parser-to-runtime integration coverage by proving slope and expanded soil
parser outputs propagate through hillslope scheduler runtime surfaces with
explicit typed failures and no silent defaults.

Constraints:
- Preserve architecture-first + contract-first posture.
- Preserve SR02/SR03 seam ownership boundaries and SR04 alias continuity
  assumptions.
- Do not introduce fallback wrappers or default substitution for required
  slope/soil seam fields.
- Use evidence-mode labeling (`Static:` vs `Ran:`) in all artifacts.
- Correctness over completion; unresolved high-severity integration ambiguity
  remains `HOLD`.

Required outputs:
- `artifacts/parser-runtime-integration-closure-matrix.md`
- `artifacts/parser-runtime-integration-implementation-evidence.md`
- `artifacts/runtime-scheduler-symbol-coverage-matrix.md`
- `artifacts/sr05_disposition.md`
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
