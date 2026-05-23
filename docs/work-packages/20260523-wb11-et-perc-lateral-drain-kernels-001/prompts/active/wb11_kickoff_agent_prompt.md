# WB11 Kickoff Agent Prompt

You are executing `20260523-wb11-et-perc-lateral-drain-kernels-001` for the
monolithic openWEPP scientific hydrology/erosion model.

Objectives:
1. Implement ET/percolation/lateral/drainage production kernels with typed
   invariant checks and deterministic state/flux updates.
2. Implement required canonical kernel-contract amendments for WB11 authority.
3. Implement contract-derived WB11 tests and run pre-implementation gate
   evidence before kernel code edits.
4. Preserve ARCH15/ARCH21 typed-seam non-regression posture.

Mandatory sequencing constraints:
- Do not modify production WB11 kernel code until:
  1. contract amendments are implemented, and
  2. contract-derived tests are implemented, and
  3. pre-implementation contract-gate evidence is recorded.
- Enforce kernel profile consistency using
  `docs/specifications/science-contracts/kernel-process-contract-profile.md`.
- Do not introduce silent defaults/clamping for invalid phase routing or kernel
  state domains.

Required outputs are listed in `package.md` Deliverables.
