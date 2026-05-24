# INIMPL07 Kickoff Agent Prompt

Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.
Autonomy: execute package phases end-to-end and update required artifacts without requesting additional user direction unless hard-blocked.
Required reading (read before edits):
- /workdir/openWEPP/AGENTS.md
- /workdir/openWEPP/docs/codex_exec_plans.md
- /workdir/openWEPP/docs/work-packages/README.md
- /workdir/openWEPP/docs/work-packages/20260521-inimpl07-wave1-core-parser-integration-001/package.md


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
