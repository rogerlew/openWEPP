# INIMPL01 Kickoff Agent Prompt

You are executing 20260521-inimpl01-prioritize-parser-implementation-order.

Objectives:
1. Prioritize implementation order for all `active` `SC-INFILE-*` surfaces.
2. Produce an evidence-tagged prioritization rubric and scored matrix.
3. Produce dependency-aware implementation waves with acceptance checks.
4. Produce a canonical plan at `docs/planning/parser-implementation-order.md`.
5. Propose follow-on implementation work-package queue entries.
6. Run dual-agent review/disposition/verification gates.

Constraints:
- Evidence mode: `Static` unless execution is explicitly run.
- Use `[DIRECT]` and `[INFERENCE]` tags per claim.
- Treat correctness over completion as normative: unresolved high-severity
  correctness risks remain `HOLD`.
- Preserve canonical WEPP/wepp-forest symbol continuity when referencing
  contract/state surfaces.

Required outputs:
- `artifacts/parser-implementation-prioritization-rubric.md`
- `artifacts/parser-implementation-priority-matrix.csv`
- `artifacts/parser-implementation-wave-plan.md`
- `docs/planning/parser-implementation-order.md`
- `artifacts/follow-on-parser-implementation-wp-queue.md`
- `artifacts/review_agent_a.md`
- `artifacts/review_agent_b.md`
- `artifacts/inimpl01_disposition.md`
- `artifacts/verification_agent_a.md`
- `artifacts/verification_agent_b.md`
