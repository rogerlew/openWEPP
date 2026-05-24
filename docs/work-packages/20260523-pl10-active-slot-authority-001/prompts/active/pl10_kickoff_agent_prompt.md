# PL10 Kickoff Agent Prompt

Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.
Autonomy: execute package phases end-to-end and update required artifacts without requesting additional user direction unless hard-blocked.
Required reading (read before edits):
- /workdir/openWEPP/AGENTS.md
- /workdir/openWEPP/docs/codex_exec_plans.md
- /workdir/openWEPP/docs/work-packages/README.md
- /workdir/openWEPP/docs/work-packages/20260523-pl10-active-slot-authority-001/package.md


You are executing `20260523-pl10-active-slot-authority-001`.

Objectives:
1. Remove first-slot/crop placeholder authority from PL growth/decomposition
   transition dispatch paths.
2. Implement deterministic day-aware active slot/crop resolution per OFE.
3. Add typed failure behavior for invalid/ambiguous active-slot conditions.
4. Deliver integration evidence for multi-slot and rotation-boundary cases.

Constraints:
- Preserve ordering invariants and existing typed guard posture.
- Do not implement PL11+ event payload expansion or process kinetics.
- Maintain typed-seam non-regression posture per ARCH15/ARCH21 evidence.

Required outputs are listed in `package.md` Deliverables.
