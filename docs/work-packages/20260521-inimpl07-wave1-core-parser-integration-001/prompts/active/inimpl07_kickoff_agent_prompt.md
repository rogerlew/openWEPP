# INIMPL07 Kickoff Agent Prompt

You are executing 20260521-inimpl07-wave1-core-parser-integration-001.

Objectives:
1. Integrate worker outputs from INIMPL03..INIMPL06.
2. Resolve conflicts with explicit logs.
3. Run Wave 1 global gates and record `Ran` evidence.
4. Publish integration report and close disposition.

Constraints:
- Use integration order defined in package.
- Do not silently drop worker changes.
- Any unresolved high-severity integration defect remains `HOLD`.

Required outputs:
- `docs/planning/wave1-parser-integration-report.md`
- `artifacts/merge-conflict-log.md`
- `artifacts/wave1-gate-evidence.md`
- `artifacts/review_agent_a.md`
- `artifacts/review_agent_b.md`
- `artifacts/inimpl07_disposition.md`
- `artifacts/verification_agent_a.md`
- `artifacts/verification_agent_b.md`
