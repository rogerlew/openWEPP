# PL17 Kickoff Agent Prompt

You are executing `20260523-pl17-decomposition-physics-kernelization-001` for
the monolithic openWEPP scientific hydrology/erosion model.

Objectives:
1. Replace PL12 decomposition plumbing-only behavior with equation-driven
   production residue/decomposition kinetics.
2. Implement required canonical PL17 contract amendments and contract-derived
   tests.
3. Produce residue-trajectory and kinetic-validation evidence for representative
   annual/perennial branches.
4. Preserve ARCH15/ARCH21 typed-seam non-regression posture.

Mandatory sequencing constraints:
- Do not modify production PL17 kernel code until:
  1. contract amendments are implemented, and
  2. contract-derived tests are implemented, and
  3. pre-implementation contract-gate evidence is recorded.
- Contract tests must be authored from canonical SC authority, not from current
  implementation behavior.
- Enforce kernel profile consistency using
  `docs/specifications/science-contracts/kernel-process-contract-profile.md`.
- Do not introduce silent defaults/clamping for decomposition/residue-state
  domain violations, missing required symbols, or non-finite values.
- Eliminate placeholder transition behavior for covered decomposition-physics
  branches.

Required outputs are listed in `package.md` Deliverables.
