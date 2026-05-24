# INIMPL24 Kickoff Agent Prompt

Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.
Autonomy: execute package phases end-to-end and update required artifacts without requesting additional user direction unless hard-blocked.
Required reading (read before edits):
- /workdir/openWEPP/AGENTS.md
- /workdir/openWEPP/docs/codex_exec_plans.md
- /workdir/openWEPP/docs/work-packages/README.md
- /workdir/openWEPP/docs/work-packages/20260522-inimpl24-implement-sc-infile-chaninp-parser-001/package.md


You are executing '20260522-inimpl24-implement-sc-infile-chaninp-parser-001'.

Objectives:
1. Implement 'SC-INFILE-CHANINP-001' for 'infile-chaninp (chan.inp)' with strict/compat behavior.
2. Add typed parser errors mapped to contract guard/invariant expectations.
3. Add fixtures/tests for valid and malformed input cases.
4. Verify and document applicable Wave 4 decision checks: W4DR-003, W4DR-004, W4DR-005, W4DR-006, W4DR-008.
5. Run parser-package gates and record evidence.
6. Produce review/disposition/verification closeout artifacts.

Constraints:
- Preserve canonical WEPP/wepp-forest symbol continuity and explicit alias
  mapping where openWEPP boundary names differ.
- Correctness over completion: unresolved high-severity findings remain HOLD.
- Do not silently correct malformed input; invariant violations must map to
  typed errors.
- Keep scope within assigned write-set and worktree ownership.
- You are not alone in the codebase: do not revert others' edits; adapt to
  concurrent changes and keep your write-set isolated.

Required outputs:
- Parser/model implementation changes for 'infile-chaninp (chan.inp)'.
- Fixture and integration test additions.
- artifacts/worker-handoff.md
- artifacts/owned-file-manifest.md
- artifacts/inimpl24_disposition.md
- artifacts/review_agent_a.md
- artifacts/review_agent_b.md
- artifacts/verification_agent_a.md
- artifacts/verification_agent_b.md
