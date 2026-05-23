# ARCH22 Kickoff Agent Prompt

You are executing
`20260523-arch22-typed-state-surface-closure-001` for the monolithic
openWEPP scientific hydrology/erosion model.

Objectives:
1. Close CRF-001 carry-forward by migrating covered production kernel
   interfaces away from stringly `BoundarySymbol(String)` surfaces.
2. Implement required canonical typed-surface contract updates and
   contract-derived migration proof tests.
3. Produce typed-surface migration closure evidence and ARCH22 closure artifact.
4. Preserve ARCH15/ARCH21 typed-seam non-regression posture.

Mandatory sequencing constraints:
- Do not modify production ARCH22 migration code until:
  1. contract updates are implemented, and
  2. contract-derived migration proof tests are implemented, and
  3. pre-implementation contract-gate evidence is recorded.
- Contract tests must be authored from canonical SC authority, not from current
  implementation behavior.
- Enforce kernel profile consistency using
  `docs/specifications/science-contracts/kernel-process-contract-profile.md`.
- Do not retain stringly symbol-key dependency in migrated production
  interfaces when typed surfaces exist.

Required outputs are listed in `package.md` Deliverables.
