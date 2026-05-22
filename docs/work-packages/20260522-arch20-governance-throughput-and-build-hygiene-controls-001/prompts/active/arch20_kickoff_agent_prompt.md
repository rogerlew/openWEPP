# ARCH20 Kickoff Agent Prompt

You are executing `20260522-arch20-governance-throughput-and-build-hygiene-controls-001`.

Objectives:
1. Author governance throughput rubric tied to engine capability outcomes.
2. Author work-package WIP/closure policy that controls churn and false closeout.
3. Author workspace build-discipline policy for consistent gate execution.
4. Author evidence/gate policy clarifying docs-only vs code-touch package
   obligations.
5. Produce dual review/disposition/verification artifacts.

Constraints:
- Preserve correctness-over-completion posture.
- Keep policy language explicit and auditable (normative requirement style).
- Avoid speculative redesign; anchor controls to observed ARCH14 findings.
- Use truthfulness posture (`Static:` vs `Ran:`) in artifacts.

Required outputs:
- `artifacts/governance-throughput-rubric.md`
- `artifacts/work-package-wip-and-closure-policy.md`
- `artifacts/workspace-build-discipline-policy.md`
- `artifacts/evidence-and-gate-policy.md`
- `artifacts/worker-handoff.md`
- `artifacts/owned-file-manifest.md`
- `artifacts/gate-results.md`
- `artifacts/arch20_disposition.md`
- `artifacts/review_agent_a.md`
- `artifacts/review_agent_b.md`
- `artifacts/verification_agent_a.md`
- `artifacts/verification_agent_b.md`

Required gates:
- Docs-only scope: static validation and artifact completeness checks.
- If code is changed, run:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`
