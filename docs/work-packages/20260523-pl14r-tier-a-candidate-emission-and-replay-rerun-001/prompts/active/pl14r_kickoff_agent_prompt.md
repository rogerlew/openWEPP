# PL14R Kickoff Agent Prompt

Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.
Autonomy: execute package phases end-to-end and update required artifacts without requesting additional user direction unless hard-blocked.
Required reading (read before edits):
- /workdir/openWEPP/AGENTS.md
- /workdir/openWEPP/docs/codex_exec_plans.md
- /workdir/openWEPP/docs/work-packages/README.md
- /workdir/openWEPP/docs/work-packages/20260523-pl14r-tier-a-candidate-emission-and-replay-rerun-001/package.md


You are executing `20260523-pl14r-tier-a-candidate-emission-and-replay-rerun-001`
for the monolithic openWEPP scientific hydrology/erosion model.

Objectives:
1. Re-run strict Tier-A comparator replay using direct openWEPP candidate output
   versus pinned legacy baseline after post-PL15 closure-wave completion.
2. Implement required canonical PL14R contract amendments for replay authority
   and guard behavior.
3. Implement contract-derived PL14R tests and run pre-implementation gate
   evidence before production replay/harness code edits.
4. Persist comparator JSON artifacts, command traces, and provenance hashes for
   reproducibility and PL15R decision consumption.

Mandatory sequencing constraints:
- Do not modify replay/harness production code until:
  1. contract amendments are implemented, and
  2. contract-derived tests are implemented, and
  3. pre-implementation contract-gate evidence is recorded.
- Enforce kernel profile consistency using
  `docs/specifications/science-contracts/kernel-process-contract-profile.md`.
- Do not introduce silent defaults/clamping for comparator failures,
  provenance gaps, missing required replay artifacts, or missing include
  surfaces.

Required outputs are listed in `package.md` Deliverables.
