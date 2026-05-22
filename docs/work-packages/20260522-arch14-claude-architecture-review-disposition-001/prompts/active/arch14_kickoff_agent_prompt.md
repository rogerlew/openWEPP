# ARCH14 Kickoff Agent Prompt

You are executing `20260522-arch14-claude-architecture-review-disposition-001`.

Objectives:
1. Normalize the external review into stable finding IDs (`CRF-001..010`).
2. Assign severity and impact surfaces per finding.
3. Produce a disposition register with explicit decisions and closure criteria.
4. Produce a remediation work-package queue with dependency order.
5. Produce dual review/disposition/verification artifacts.
6. Explicitly state that openWEPP is moving to typed kernel state surfaces and
   unit-boundary wiring at the kernel seam.

Constraints:
- This is a greenfield scientific hydrology simulation architecture.
- This package is governance/disposition only; do not implement code fixes.
- Correctness over completion: unresolved high-severity findings remain `HOLD`.
- Preserve canonical WEPP/wepp-forest naming continuity in architecture and
  contract references.
- Use truthfulness posture (`Static:` vs `Ran:`) in artifacts.
- `CRF-001` and `CRF-002` must not be dispositioned as `reject`.

Required outputs:
- `artifacts/claude-review-findings-register.md`
- `artifacts/disposition-register.md`
- `artifacts/remediation-work-package-queue.md`
- `artifacts/architecture-review-disposition-acceptance-criteria.md`
- `artifacts/worker-handoff.md`
- `artifacts/owned-file-manifest.md`
- `artifacts/gate-results.md`
- `artifacts/arch14_disposition.md`
- `artifacts/review_agent_a.md`
- `artifacts/review_agent_b.md`
- `artifacts/verification_agent_a.md`
- `artifacts/verification_agent_b.md`
