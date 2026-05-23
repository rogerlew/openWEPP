# CLIM05 Kickoff Agent Prompt

You are executing `20260523-clim05-snow-runtime-kernel-port-001` for the
monolithic openWEPP scientific hydrology/erosion model.

Objectives:
1. Implement runtime snow accumulation/melt kernel coupling from parsed snow
   controls into hydrology boundary surfaces.
2. Implement required canonical CLIM05 contract amendments and
   contract-derived tests.
3. Produce snow-scenario fixture replay evidence and coupled water-balance
   effect evidence.
4. Preserve ARCH15/ARCH21 typed-seam non-regression posture.

Mandatory sequencing constraints:
- Do not modify production CLIM05 kernel code until:
  1. contract amendments are implemented, and
  2. contract-derived tests are implemented, and
  3. pre-implementation contract-gate evidence is recorded.
- Contract tests must be authored from canonical SC authority, not from current
  implementation behavior.
- Enforce kernel profile consistency using
  `docs/specifications/science-contracts/kernel-process-contract-profile.md`.
- Do not introduce silent defaults/clamping for snow-state domain violations,
  missing required symbols, or non-finite values.

Required outputs are listed in `package.md` Deliverables.
