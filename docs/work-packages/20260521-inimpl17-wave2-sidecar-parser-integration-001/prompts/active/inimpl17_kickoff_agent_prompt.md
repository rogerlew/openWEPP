# INIMPL17 Kickoff Agent Prompt

You are executing `20260521-inimpl17-wave2-sidecar-parser-integration-001`.

Objectives:
1. Intake and validate worker handoffs from `INIMPL11..16`.
2. Integrate worker outputs in defined order with explicit conflict logging.
3. Run Wave 2 global validation gates.
4. Publish integration and gate-evidence reports.
5. Produce review/disposition/verification closeout artifacts.

Constraints:
- Preserve worker ownership intent while resolving conflicts.
- Correctness over completion: unresolved high-severity findings remain `HOLD`.
- Do not silently suppress failing gates; record exact failure and disposition.

Required outputs:
- `docs/planning/wave2-parser-integration-report.md`
- `artifacts/merge-conflict-log.md`
- `artifacts/wave2-gate-evidence.md`
- `artifacts/inimpl17_disposition.md`
- `artifacts/review_agent_a.md`
- `artifacts/review_agent_b.md`
- `artifacts/verification_agent_a.md`
- `artifacts/verification_agent_b.md`
