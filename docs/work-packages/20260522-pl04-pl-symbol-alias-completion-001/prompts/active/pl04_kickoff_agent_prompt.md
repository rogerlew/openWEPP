# PL04 Kickoff Agent Prompt

You are executing `20260522-pl04-pl-symbol-alias-completion-001`.

Objectives:
1. Extend canonical alias registry with PL symbol coverage.
2. Add deterministic template aliases for indexed PL runtime surfaces.
3. Add alias forward/reverse resolution tests and ambiguity guards.
4. Document complete PL canonical-to-boundary alias mapping.
5. Publish implementation evidence and disposition.

Constraints:
- Correctness over completion; unresolved ambiguities remain `HOLD`.
- Canonical WEPP symbols remain authoritative.
- No ambiguous alias back-mapping permitted.
- Use truthfulness posture (`Static:` vs `Ran:`) in every artifact.

Required outputs:
- `artifacts/pl04-symbol-alias-expansion-contract.md`
- `artifacts/pl04-canonical-symbol-alias-table.md`
- `artifacts/pl04-alias-template-validation-notes.md`
- `artifacts/pl04-alias-registry-test-evidence.md`
- `artifacts/worker-handoff.md`
- `artifacts/owned-file-manifest.md`
- `artifacts/gate-results.md`
- `artifacts/pl04_disposition.md`
- `artifacts/review_agent_a.md`
- `artifacts/review_agent_b.md`
- `artifacts/verification_agent_a.md`
- `artifacts/verification_agent_b.md`

Required gates:
- If code is changed, run:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`
