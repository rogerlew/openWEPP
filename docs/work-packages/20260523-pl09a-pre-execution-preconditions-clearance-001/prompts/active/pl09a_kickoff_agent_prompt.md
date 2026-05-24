# PL09A Kickoff Agent Prompt

Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.
Autonomy: execute package phases end-to-end and update required artifacts without requesting additional user direction unless hard-blocked.
Required reading (read before edits):
- /workdir/openWEPP/AGENTS.md
- /workdir/openWEPP/docs/codex_exec_plans.md
- /workdir/openWEPP/docs/work-packages/README.md
- /workdir/openWEPP/docs/work-packages/20260523-pl09a-pre-execution-preconditions-clearance-001/package.md


You are executing `20260523-pl09a-pre-execution-preconditions-clearance-001`.

Objectives:
1. Clear preconditions 1/2/3 from Claude PL09 pre-execution review.
2. Record explicit acknowledgement for secondary findings.
3. Patch PL09 queue gating so PL10/WB10 execution is conditioned on clearance.

Constraints:
- No kernel implementation in this package.
- Evidence-mode truthfulness: `Ran` means commands/data extraction actually run.
- Preserve confidence-tier and correctness-over-completion governance posture.

Required outputs are listed in `package.md` Deliverables.
