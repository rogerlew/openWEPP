# WB16 Kickoff Agent Prompt

Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.
Autonomy: execute package phases end-to-end and update required artifacts without requesting additional user direction unless hard-blocked.
Required reading (read before edits):
- /workdir/openWEPP/AGENTS.md
- /workdir/openWEPP/docs/codex_exec_plans.md
- /workdir/openWEPP/docs/work-packages/README.md
- /workdir/openWEPP/docs/work-packages/20260523-wb16-peak-runoff-kernel-001/package.md


You are executing
`20260523-wb16-peak-runoff-kernel-001` for the monolithic openWEPP scientific
hydrology/erosion model.

Objectives:
1. Implement production peak runoff kernel behavior with documented method
   branches and typed guards.
2. Implement required canonical WB16 contract amendments and contract-derived
   tests.
3. Produce peak-flow trace evidence and downstream coupling-readiness evidence.
4. Preserve ARCH15/ARCH21 typed-seam non-regression posture.

Mandatory sequencing constraints:
- Do not modify production WB16 kernel code until:
  1. contract amendments are implemented, and
  2. contract-derived tests are implemented, and
  3. pre-implementation contract-gate evidence is recorded.
- Contract tests must be authored from canonical SC authority, not from current
  implementation behavior.
- Enforce kernel profile consistency using
  `docs/specifications/science-contracts/kernel-process-contract-profile.md`.
- Do not introduce silent defaults/clamping for peak-flow domain violations,
  missing required symbols, or non-finite values.

Required outputs are listed in `package.md` Deliverables.
