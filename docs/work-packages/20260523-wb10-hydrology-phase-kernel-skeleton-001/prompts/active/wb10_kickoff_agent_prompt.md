# WB10 Kickoff Agent Prompt

Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.
Autonomy: execute package phases end-to-end and update required artifacts without requesting additional user direction unless hard-blocked.
Required reading (read before edits):
- /workdir/openWEPP/AGENTS.md
- /workdir/openWEPP/docs/codex_exec_plans.md
- /workdir/openWEPP/docs/work-packages/README.md
- /workdir/openWEPP/docs/work-packages/20260523-wb10-hydrology-phase-kernel-skeleton-001/package.md


You are executing `20260523-wb10-hydrology-phase-kernel-skeleton-001` for the
monolithic openWEPP scientific hydrology/erosion model.

Objectives:
1. Add production hydrology kernel entry scaffolding for ET/perc/lateral/
   drainage/runoff/storage phase classes.
2. Wire scaffolding through scheduler phase-class dispatch with typed routing
   failures for unsupported classes.
3. Preserve ARCH15/ARCH21 typed-seam non-regression posture.
4. Keep canonical science-contract authority as source of truth.

Mandatory sequencing constraints:
- Do not modify production routing code until:
  1. contract authority updates are drafted, and
  2. pre-implementation contract-gate evidence is recorded.
- Enforce kernel profile consistency using
  `docs/specifications/science-contracts/kernel-process-contract-profile.md`.
- Do not introduce silent defaults/clamping for invalid phase routing states.

Required outputs are listed in `package.md` Deliverables.
