# INIMPL02 Kickoff Agent Prompt

Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.
Autonomy: execute package phases end-to-end and update required artifacts without requesting additional user direction unless hard-blocked.
Required reading (read before edits):
- /workdir/openWEPP/AGENTS.md
- /workdir/openWEPP/docs/codex_exec_plans.md
- /workdir/openWEPP/docs/work-packages/README.md
- /workdir/openWEPP/docs/work-packages/20260521-inimpl02-wave1-worktree-orchestration-001/package.md


You are executing 20260521-inimpl02-wave1-worktree-orchestration-001.

Objectives:
1. Produce a canonical Wave 1 worktree execution plan.
2. Define disjoint worker ownership and branch/worktree registry.
3. Define integration sequencing and hard quality gates.
4. Run dual review/disposition/verification for this governance package.

Constraints:
- Evidence mode: `Static` unless execution is explicitly run.
- Preserve correctness-over-completion posture.
- Any unresolved high-severity governance ambiguity remains `HOLD`.

Required outputs:
- `docs/planning/wave1-parser-worktree-execution-plan.md`
- `artifacts/worktree-ownership-manifest.md`
- `artifacts/worktree-branch-registry.md`
- `artifacts/wave1-integration-sequence.md`
- `artifacts/review_agent_a.md`
- `artifacts/review_agent_b.md`
- `artifacts/inimpl02_disposition.md`
- `artifacts/verification_agent_a.md`
- `artifacts/verification_agent_b.md`
