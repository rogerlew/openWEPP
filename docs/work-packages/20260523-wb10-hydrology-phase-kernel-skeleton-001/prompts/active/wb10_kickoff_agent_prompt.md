# WB10 Kickoff Agent Prompt

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
