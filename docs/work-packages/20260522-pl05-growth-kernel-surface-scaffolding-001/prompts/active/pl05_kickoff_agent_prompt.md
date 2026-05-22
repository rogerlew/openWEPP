# PL05 Kickoff Agent Prompt

You are executing `20260522-pl05-growth-kernel-surface-scaffolding-001`.

Objectives:
1. Add typed growth-kernel interfaces for scheduler-facing PL growth surfaces.
2. Add placeholder annual/perennial growth scheduler phases with deterministic
   ordering semantics.
3. Integrate scheduler-to-kernel growth boundary wiring with typed failures
   only.
4. Add integration/test evidence for interface shape and phase-order guards.
5. Publish PL05 implementation evidence and disposition.

Constraints:
- Preserve architecture-first and science-contract governance posture.
- Do not introduce silent defaults for missing/non-finite required growth
  runtime inputs.
- Use evidence-mode labeling (`Static:` vs `Ran:`) in all artifacts.
- Correctness over completion; unresolved phase-ordering ambiguity remains
  `HOLD`.

Required outputs:
- `artifacts/pl05-growth-kernel-surface-contract.md`
- `artifacts/pl05-growth-scheduler-phase-scaffold.md`
- `artifacts/pl05-annual-perennial-transition-state-map.md`
- `artifacts/pl05-growth-surface-test-evidence.md`
- `artifacts/worker-handoff.md`
- `artifacts/owned-file-manifest.md`
- `artifacts/gate-results.md`
- `artifacts/pl05_disposition.md`
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
