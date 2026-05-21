# INIMPL03 Kickoff Agent Prompt

You are executing 20260521-inimpl03-implement-sc-infile-slope-parser-001.

Objectives:
1. Implement parser surface `SC-INFILE-SLOPE-001` in assigned worker worktree.
2. Keep edits within assigned write-set.
3. Add strict/compat and malformed-input tests for `infile-slope-slp (.slp)`.
4. Produce worker handoff + owned-file manifest.
5. Run dual review/disposition/verification gates.

Constraints:
- Work in assigned worktree: `/home/workdir/openWEPP/.worktrees/inimpl03-slope`
- Branch: `inimpl03/slope-parser`
- Do not edit another worker's owned paths.
- Evidence mode: `Ran` for executed checks, `Static` otherwise.
- Correctness over completion; unresolved high-severity gaps remain `HOLD`.

Required outputs:
- parser and tests in assigned write-set
- `artifacts/worker-handoff.md`
- `artifacts/owned-file-manifest.md`
- `artifacts/review_agent_a.md`
- `artifacts/review_agent_b.md`
- `artifacts/inimpl03_disposition.md`
- `artifacts/verification_agent_a.md`
- `artifacts/verification_agent_b.md`
