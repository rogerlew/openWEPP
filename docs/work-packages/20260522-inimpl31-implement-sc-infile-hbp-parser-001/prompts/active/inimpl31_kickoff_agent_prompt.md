# INIMPL31 Kickoff Agent Prompt

You are executing `20260522-inimpl31-implement-sc-infile-hbp-parser-001`.

Objectives:
1. Author canonical HBP surface specification and parser contract (`SC-INFILE-HBP-001`).
2. Integrate exported HBP parser surface in `openwepp-input-contract` with typed strict/compat behavior.
3. Add deterministic integration tests for schema1/schema2 success and typed failure branches.
4. Register HBP in the parser input-surface registry.
5. Run all required gates and produce closeout artifacts.

Constraints:
- Preserve canonical WEPP/wepp-forest symbols for HBP fields and invariants.
- Use explicit typed errors/warnings; no silent fallback to legacy text pass files.
- Keep parser-local responsibilities separate from downstream shard-set orchestration responsibilities.
- Correctness over completion: unresolved high-severity findings remain HOLD.

Required outputs:
- Parser wiring and test additions.
- `docs/specifications/wepp-input-files/specs/hbp-file.spec.md`
- `docs/specifications/science-contracts/contracts/SC-INFILE-HBP-001.md`
- INIMPL31 artifact bundle files in `artifacts/`.
