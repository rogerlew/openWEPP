# INIMPL09 Kickoff Agent Prompt

You are executing `20260521-inimpl09-management-full-typed-datamodel-001`.

Objectives:
1. Update management spec + parser contract so typed datamodel requirements are
   executable and unambiguous.
2. Implement full non-zero section parsing for `.man` with typed scenario
   registries and schedule output.
3. Add fixtures/tests that cover valid and malformed canonical management files
   across supported datver branches.
4. Run parser-package gates and record evidence.
5. Produce review/disposition/verification closeout artifacts.

Constraints:
- Preserve canonical WEPP/wepp-forest symbols; use alias mapping where openWEPP
  boundary names differ.
- Correctness over completion: unresolved high-severity findings remain `HOLD`.
- Do not silently correct malformed input; invariant violations must map to
  typed errors.
- Keep scope on parser + typed read model; do not introduce unrelated runtime
  mutation APIs.
- Treat `/home/workdir/wepppy/wepppy/wepp/management/data` as the primary
  seed corpus for fixture selection, and preserve provenance for curated
  fixture files.
- openWEPP does not implement rangeland simulation behavior; ensure spec,
  contract, and parser outcomes make this explicit via typed unsupported policy
  (reject or non-executable marking), not implicit partial support.

Required outputs:
- Updated spec and contract docs.
- Parser/model implementation changes.
- Fixture and integration test additions.
- `artifacts/management-typed-datamodel-decomposition.md`
- `artifacts/wave-gate-evidence.md`
- `artifacts/review_agent_a.md`
- `artifacts/review_agent_b.md`
- `artifacts/inimpl09_disposition.md`
- `artifacts/verification_agent_a.md`
- `artifacts/verification_agent_b.md`
