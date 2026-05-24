# INIMPL18 Kickoff Agent Prompt

Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.
Autonomy: execute package phases end-to-end and update required artifacts without requesting additional user direction unless hard-blocked.
Required reading (read before edits):
- /workdir/openWEPP/AGENTS.md
- /workdir/openWEPP/docs/codex_exec_plans.md
- /workdir/openWEPP/docs/work-packages/README.md
- /workdir/openWEPP/docs/work-packages/20260521-inimpl18-wave3-worktree-orchestration-001/package.md


You are executing '20260521-inimpl18-wave3-worktree-orchestration-001'.

Objectives:
1. Publish Wave 3 worktree execution governance for concurrent parser work.
2. Define disjoint ownership for INIMPL19..21.
3. Define deterministic integration order into INIMPL22.
4. Define Wave 3 gate/promotion criteria and blocker policy.
5. Produce review/disposition/verification closeout artifacts.

Constraints:
- Preserve correctness-over-completion posture; unresolved high-severity
  findings remain HOLD.
- Do not redefine science contracts in this package; reference existing
  SC-INFILE authorities.
- Keep governance explicit and executable (no ambiguous ownership boundaries).

Required outputs:
- docs/planning/wave3-parser-worktree-execution-plan.md
- artifacts/worktree-ownership-manifest.md
- artifacts/worktree-branch-registry.md
- artifacts/wave3-integration-sequence.md
- artifacts/inimpl18_disposition.md
- artifacts/review_agent_a.md
- artifacts/review_agent_b.md
- artifacts/verification_agent_a.md
- artifacts/verification_agent_b.md
