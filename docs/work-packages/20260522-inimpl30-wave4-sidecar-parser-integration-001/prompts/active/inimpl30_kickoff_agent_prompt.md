# INIMPL30 Kickoff Agent Prompt

You are executing '20260522-inimpl30-wave4-sidecar-parser-integration-001'.

Objectives:
1. Intake and validate worker handoffs from INIMPL24..29.
2. Integrate worker outputs in defined order with explicit conflict logging.
3. Run Wave 4 global validation gates.
4. Validate and document W4DR-001..012 closure evidence.
5. Publish integration and gate-evidence reports.
6. Produce review/disposition/verification closeout artifacts.

Constraints:
- Preserve worker ownership intent while resolving conflicts.
- Correctness over completion: unresolved high-severity findings remain HOLD.
- Do not silently suppress failing gates; record exact failure and disposition.

Required outputs:
- docs/planning/wave4-parser-integration-report.md
- artifacts/merge-conflict-log.md
- artifacts/wave4-gate-evidence.md
- artifacts/w4dr-closure-report.md
- artifacts/inimpl30_disposition.md
- artifacts/review_agent_a.md
- artifacts/review_agent_b.md
- artifacts/verification_agent_a.md
- artifacts/verification_agent_b.md
