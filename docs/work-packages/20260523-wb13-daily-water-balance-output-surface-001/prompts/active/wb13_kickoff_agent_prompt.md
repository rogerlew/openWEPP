# WB13 Kickoff Agent Prompt

Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.
Autonomy: execute package phases end-to-end and update required artifacts without requesting additional user direction unless hard-blocked.
Required reading (read before edits):
- /workdir/openWEPP/AGENTS.md
- /workdir/openWEPP/docs/codex_exec_plans.md
- /workdir/openWEPP/docs/work-packages/README.md
- /workdir/openWEPP/docs/work-packages/20260523-wb13-daily-water-balance-output-surface-001/package.md


You are executing `20260523-wb13-daily-water-balance-output-surface-001` for
the monolithic openWEPP scientific hydrology/erosion model.

Objectives:
1. Implement comparator-ready daily water-balance output emission (`H5.wat.dat`
   equivalent surface) with deterministic schema, units, and row ordering.
2. Implement required canonical WB13 contract amendments for output-surface
   authority and guard behavior.
3. Implement contract-derived WB13 tests and run pre-implementation gate
   evidence before production output-surface code edits.
4. Preserve ARCH15/ARCH21 typed-seam non-regression posture.

Mandatory sequencing constraints:
- Do not modify production WB13 output-surface code until:
  1. contract amendments are implemented, and
  2. contract-derived tests are implemented, and
  3. pre-implementation contract-gate evidence is recorded.
- Enforce kernel profile consistency using
  `docs/specifications/science-contracts/kernel-process-contract-profile.md`.
- Do not introduce silent defaults/clamping for invalid output domains,
  missing required fields, or non-finite values.

Required outputs are listed in `package.md` Deliverables.
