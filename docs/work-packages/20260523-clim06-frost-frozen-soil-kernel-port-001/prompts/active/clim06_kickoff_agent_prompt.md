# CLIM06 Kickoff Agent Prompt

Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.
Autonomy: execute package phases end-to-end and update required artifacts without requesting additional user direction unless hard-blocked.
Required reading (read before edits):
- /workdir/openWEPP/AGENTS.md
- /workdir/openWEPP/docs/codex_exec_plans.md
- /workdir/openWEPP/docs/work-packages/README.md
- /workdir/openWEPP/docs/work-packages/20260523-clim06-frost-frozen-soil-kernel-port-001/package.md


You are executing `20260523-clim06-frost-frozen-soil-kernel-port-001` for the
monolithic openWEPP scientific hydrology/erosion model.

Objectives:
1. Implement frozen-soil/frost runtime kernel behavior and infiltration coupling
   from parsed frost controls into hydrology boundary surfaces.
2. Implement required canonical CLIM06 contract amendments and
   contract-derived tests.
3. Produce cold-season fixture replay evidence and infiltration/runoff branch
   coupling evidence.
4. Preserve ARCH15/ARCH21 typed-seam non-regression posture.

Mandatory sequencing constraints:
- Do not modify production CLIM06 kernel code until:
  1. contract amendments are implemented, and
  2. contract-derived tests are implemented, and
  3. pre-implementation contract-gate evidence is recorded.
- Contract tests must be authored from canonical SC authority, not from current
  implementation behavior.
- Enforce kernel profile consistency using
  `docs/specifications/science-contracts/kernel-process-contract-profile.md`.
- Do not introduce silent defaults/clamping for frozen-soil/frost state domain
  violations, missing required symbols, or non-finite values.

Required outputs are listed in `package.md` Deliverables.
