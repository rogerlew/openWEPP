# PL16 Kickoff Agent Prompt

You are executing `20260523-pl16-growth-physics-kernelization-001` for the
monolithic openWEPP scientific hydrology/erosion model.

Objectives:
1. Replace PL13 growth plumbing-only behavior with equation-driven production
   growth physics updates.
2. Implement required canonical PL16 contract amendments and contract-derived
   tests.
3. Produce growth state-trajectory and regression parity-trace evidence for
   representative annual/perennial branches.
4. Preserve ARCH15/ARCH21 typed-seam non-regression posture.

Mandatory sequencing constraints:
- Do not modify production PL16 kernel code until:
  1. contract amendments are implemented, and
  2. contract-derived tests are implemented, and
  3. pre-implementation contract-gate evidence is recorded.
- Contract tests must be authored from canonical SC authority, not from current
  implementation behavior.
- Enforce kernel profile consistency using
  `docs/specifications/science-contracts/kernel-process-contract-profile.md`.
- Do not introduce silent defaults/clamping for growth-state domain
  violations, missing required symbols, or non-finite values.
- Eliminate default skip/zero-reset fallback behavior for covered active
  growth branches.

Required outputs are listed in `package.md` Deliverables.
