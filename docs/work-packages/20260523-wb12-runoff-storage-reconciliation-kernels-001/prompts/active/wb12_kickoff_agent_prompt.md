# WB12 Kickoff Agent Prompt

Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.
Autonomy: execute package phases end-to-end and update required artifacts without requesting additional user direction unless hard-blocked.
Required reading (read before edits):
- /workdir/openWEPP/AGENTS.md
- /workdir/openWEPP/docs/codex_exec_plans.md
- /workdir/openWEPP/docs/work-packages/README.md
- /workdir/openWEPP/docs/work-packages/20260523-wb12-runoff-storage-reconciliation-kernels-001/package.md


You are executing `20260523-wb12-runoff-storage-reconciliation-kernels-001` for
the monolithic openWEPP scientific hydrology/erosion model.

Objectives:
1. Implement runoff and storage reconciliation production kernels with typed
   closure diagnostics and deterministic state/flux updates.
2. Implement required canonical kernel-contract amendments for WB12 authority.
3. Implement contract-derived WB12 tests and run pre-implementation gate
   evidence before kernel code edits.
4. Preserve ARCH15/ARCH21 typed-seam non-regression posture.

Mandatory sequencing constraints:
- Do not modify production WB12 kernel code until:
  1. contract amendments are implemented, and
  2. contract-derived tests are implemented, and
  3. pre-implementation contract-gate evidence is recorded.
- Enforce kernel profile consistency using
  `docs/specifications/science-contracts/kernel-process-contract-profile.md`.
- Do not introduce silent defaults/clamping for invalid closure diagnostics or
  kernel state domains.

Required outputs are listed in `package.md` Deliverables.
