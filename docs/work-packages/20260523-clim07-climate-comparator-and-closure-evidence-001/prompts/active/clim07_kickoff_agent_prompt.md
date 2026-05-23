# CLIM07 Kickoff Agent Prompt

You are executing
`20260523-clim07-climate-comparator-and-closure-evidence-001` for the
monolithic openWEPP scientific hydrology/erosion model.

Objectives:
1. Implement targeted integration tests and comparator vectors for
   continuous-daily and breakpoint climate modes.
2. Implement parser-to-kernel seam checks and confidence-tier reporting
   evidence for climate runtime closure.
3. Implement required canonical CLIM07 contract updates and contract-derived
   tests/vectors.
4. Preserve ARCH15/ARCH17/ARCH21 typed-seam non-regression posture.

Mandatory sequencing constraints:
- Do not modify production CLIM07 comparator/integration code until:
  1. contract updates are implemented, and
  2. contract-derived tests/vectors are implemented, and
  3. pre-implementation contract-gate evidence is recorded.
- Contract tests/vectors must be authored from canonical SC authority, not from
  current implementation behavior.
- Enforce kernel profile consistency using
  `docs/specifications/science-contracts/kernel-process-contract-profile.md`.
- Do not introduce silent defaults/clamping for seam-check/comparator domain
  violations, missing required symbols, or non-finite values.

Required outputs are listed in `package.md` Deliverables.
