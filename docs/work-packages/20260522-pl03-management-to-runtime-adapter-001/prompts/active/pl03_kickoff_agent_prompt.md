# PL03 Kickoff Agent Prompt

You are executing `20260522-pl03-management-to-runtime-adapter-001`.

Objectives:
1. Implement strict typed parser-to-runtime PL adapter (`PL-MAN-SEAM-001`).
2. Project required PL schedule/growth/decomposition runtime surfaces.
3. Implement typed seam error taxonomy and test negative paths.
4. Preserve deterministic scheduler ordering preconditions from PL contracts.
5. Publish implementation evidence and disposition.

Constraints:
- Correctness over completion; unresolved ambiguities remain `HOLD`.
- No silent defaults for required projected fields.
- Preserve canonical symbol continuity constraints from PL02.
- Use truthfulness posture (`Static:` vs `Ran:`) in every artifact.

Required outputs:
- `artifacts/pl03-runtime-adapter-contract.md`
- `artifacts/pl03-runtime-surface-projection-map.md`
- `artifacts/pl03-typed-error-taxonomy.md`
- `artifacts/pl03-parser-to-runtime-integration-evidence.md`
- `artifacts/pl03-scheduler-ordering-compliance.md`
- `artifacts/worker-handoff.md`
- `artifacts/owned-file-manifest.md`
- `artifacts/gate-results.md`
- `artifacts/pl03_disposition.md`
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
