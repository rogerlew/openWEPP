# ARCH13 Kickoff Agent Prompt

Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.
Autonomy: execute package phases end-to-end and update required artifacts without requesting additional user direction unless hard-blocked.
Required reading (read before edits):
- /workdir/openWEPP/AGENTS.md
- /workdir/openWEPP/docs/codex_exec_plans.md
- /workdir/openWEPP/docs/work-packages/README.md
- /workdir/openWEPP/docs/work-packages/20260522-arch13-wave4-hold-ratification-checklist-001/package.md


You are executing `20260522-arch13-wave4-hold-ratification-checklist-001`.

Domain context (explicit):
- This is a greenfield scientific hydrology simulation engine (`openWEPP`).
- Work is strictly governance/specification ratification for sidecar/parser
  HOLD closure.
- This package is documentation workflow and does not implement parser code.

Objectives:
1. Encode 12 Wave 4 HOLD decision points as explicit decision records.
2. Map each decision to contract HOLD gap IDs and required evidence.
3. Define explicit per-decision and global Wave 4 kickoff acceptance criteria.
4. Produce review/disposition/verification artifacts.

Constraints:
- Preserve canonical `wepp-forest` symbol naming continuity in decision text.
- Use truthfulness posture (`Ran:` vs `Static:`) in artifacts.
- Correctness over completion: unresolved high-severity ambiguity remains `HOLD`.

Required outputs:
- `artifacts/wave4-hold-ratification-checklist.md`
- `artifacts/wave4-kickoff-acceptance-criteria.md`
- `artifacts/worker-handoff.md`
- `artifacts/owned-file-manifest.md`
- `artifacts/gate-results.md`
- `artifacts/arch13_disposition.md`
- `artifacts/review_agent_a.md`
- `artifacts/review_agent_b.md`
- `artifacts/verification_agent_a.md`
- `artifacts/verification_agent_b.md`
