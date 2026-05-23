# WB14 Kickoff Agent Prompt

You are executing
`20260523-wb14-infiltration-and-subdaily-hyetograph-kernel-001` for the
monolithic openWEPP scientific hydrology/erosion model.

Objectives:
1. Implement production infiltration kernel behavior using canonical
   Green-Ampt lineage authority.
2. Implement within-day hyetograph integration in hydrology runtime execution.
3. Implement required canonical WB14 contract amendments and contract-derived
   tests.
4. Preserve ARCH15/ARCH21 typed-seam non-regression posture.

Mandatory sequencing constraints:
- Do not modify production WB14 kernel code until:
  1. contract amendments are implemented, and
  2. contract-derived tests are implemented, and
  3. pre-implementation contract-gate evidence is recorded.
- Contract tests must be authored from canonical SC authority, not from current
  implementation behavior.
- Enforce kernel profile consistency using
  `docs/specifications/science-contracts/kernel-process-contract-profile.md`.
- Do not introduce silent defaults/clamping for infiltration/hyetograph domain
  violations, missing required symbols, or non-finite values.

Required outputs are listed in `package.md` Deliverables.
