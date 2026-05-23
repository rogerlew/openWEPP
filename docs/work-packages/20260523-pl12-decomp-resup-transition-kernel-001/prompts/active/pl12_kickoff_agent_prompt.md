# PL12 Kickoff Agent Prompt

You are executing `20260523-pl12-decomp-resup-transition-kernel-001` for the
monolithic openWEPP scientific hydrology/erosion model.

Objectives:
1. Implement production decomposition/residue transition execution against typed
   contexts and projected PL11 control payloads.
2. Enforce typed guard/failure behavior for invalid transition domains.
3. Preserve ARCH15/ARCH21 typed-seam non-regression posture.
4. Keep contract authority as the source of truth for algorithm intent.

Mandatory sequencing constraints:
- Do not modify production kernel code until:
  1. contract authority updates are drafted, and
  2. pre-implementation contract-gate evidence is recorded.
- Enforce kernel profile consistency using
  `docs/specifications/science-contracts/kernel-process-contract-profile.md`.
- Do not introduce silent defaults/clamping for invalid state domains.

Required outputs are listed in `package.md` Deliverables.
