# INIMPL06 Kickoff Agent Prompt

Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.
Autonomy: execute package phases end-to-end and update required artifacts without requesting additional user direction unless hard-blocked.
Required reading (read before edits):
- /workdir/openWEPP/AGENTS.md
- /workdir/openWEPP/docs/codex_exec_plans.md
- /workdir/openWEPP/docs/work-packages/README.md
- /workdir/openWEPP/docs/work-packages/20260521-inimpl06-implement-sc-infile-management-parser-001/package.md


You are executing 20260521-inimpl06-implement-sc-infile-management-parser-001.

Objectives:
1. Implement parser surface `SC-INFILE-MANAGEMENT-001` in assigned worker worktree.
2. Keep edits within assigned write-set.
3. Add strict/compat and malformed-input tests for `infile-management-man (.man)`.
4. Produce worker handoff + owned-file manifest.
5. Run dual review/disposition/verification gates.

Constraints:
- Work in assigned worktree: `/workdir/openWEPP/.worktrees/inimpl06-management`
- Branch: `inimpl06/management-parser`
- Do not edit another worker's owned paths.
- Evidence mode: `Ran` for executed checks, `Static` otherwise.
- Correctness over completion; unresolved high-severity gaps remain `HOLD`.

Required outputs:
- parser and tests in assigned write-set
- `artifacts/worker-handoff.md`
- `artifacts/owned-file-manifest.md`
- `artifacts/review_agent_a.md`
- `artifacts/review_agent_b.md`
- `artifacts/inimpl06_disposition.md`
- `artifacts/verification_agent_a.md`
- `artifacts/verification_agent_b.md`
