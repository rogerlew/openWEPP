# INT10 Kickoff Agent Prompt

Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.
Autonomy: execute package phases end-to-end and update required artifacts without requesting additional user direction unless hard-blocked.
Required reading (read before edits):
- /workdir/openWEPP/AGENTS.md
- /workdir/openWEPP/docs/codex_exec_plans.md
- /workdir/openWEPP/docs/work-packages/README.md
- /workdir/openWEPP/docs/work-packages/20260523-int10-plant-water-coupling-validation-001/package.md


You are executing `20260523-int10-plant-water-coupling-validation-001` for the
monolithic openWEPP scientific hydrology/erosion model.

Objectives:
1. Validate coupled daily execution ordering (`decomp -> growth -> watbal`) and
   coupled state-transfer semantics across plant and hydrology lanes.
2. Implement required canonical contract amendments for INT10 coupling
   authority and guard behavior.
3. Implement contract-derived INT10 tests and run pre-implementation gate
   evidence before production integration code edits.
4. Preserve ARCH15/ARCH21 typed-seam non-regression posture.

Mandatory sequencing constraints:
- Do not modify production INT10 integration code until:
  1. contract amendments are implemented, and
  2. contract-derived tests are implemented, and
  3. pre-implementation contract-gate evidence is recorded.
- Enforce kernel profile consistency using
  `docs/specifications/science-contracts/kernel-process-contract-profile.md`.
- Do not introduce silent defaults/clamping for ordering violations, missing
  coupled-state symbols, or non-finite coupled values.

Required outputs are listed in `package.md` Deliverables.
