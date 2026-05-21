# INIMPL12 Kickoff Agent Prompt

You are executing `20260521-inimpl12-implement-sc-infile-irrigation-depletion-parser-001`.

Objectives:
1. Implement `SC-INFILE-IRRIGATION-DEPLETION-001` for `infile-irrigation-depletion` with strict/compat behavior.
2. Add typed parser errors mapped to contract guard/invariant expectations.
3. Add fixtures/tests for valid and malformed input cases.
4. Run parser-package gates and record evidence.
5. Produce review/disposition/verification closeout artifacts.

Constraints:
- Preserve canonical WEPP/wepp-forest symbol continuity and explicit alias
  mapping where openWEPP boundary names differ.
- Correctness over completion: unresolved high-severity findings remain `HOLD`.
- Do not silently correct malformed input; invariant violations must map to
  typed errors.
- Keep scope within assigned write-set and worktree ownership.

Required outputs:
- Parser/model implementation changes for `infile-irrigation-depletion`.
- Fixture and integration test additions.
- `artifacts/worker-handoff.md`
- `artifacts/owned-file-manifest.md`
- `artifacts/inimpl12_disposition.md`
- `artifacts/review_agent_a.md`
- `artifacts/review_agent_b.md`
- `artifacts/verification_agent_a.md`
- `artifacts/verification_agent_b.md`
