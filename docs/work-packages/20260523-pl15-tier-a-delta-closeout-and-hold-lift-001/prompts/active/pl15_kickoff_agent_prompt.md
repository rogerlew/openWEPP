# PL15 Kickoff Agent Prompt

Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.
Autonomy: execute package phases end-to-end and update required artifacts without requesting additional user direction unless hard-blocked.
Required reading (read before edits):
- /workdir/openWEPP/AGENTS.md
- /workdir/openWEPP/docs/codex_exec_plans.md
- /workdir/openWEPP/docs/work-packages/README.md
- /workdir/openWEPP/docs/work-packages/20260523-pl15-tier-a-delta-closeout-and-hold-lift-001/package.md


You are executing `20260523-pl15-tier-a-delta-closeout-and-hold-lift-001` for
the monolithic openWEPP scientific hydrology/erosion model.

Objectives:
1. Disposition residual Tier-A deltas from direct PL14 replay evidence.
2. Update comparator confidence-tier disposition and semantic parity direction
   assessment.
3. Issue final PL08 hold-lift verdict with explicit criteria outcomes and
   conditional risk-acceptance reference when blockers remain.
4. Implement required canonical PL15 contract/spec authority updates and
   contract-derived tests before production closeout logic edits.

Mandatory sequencing constraints:
- Do not modify production closeout logic or decision-surface code until:
  1. contract/spec amendments are implemented, and
  2. contract-derived tests are implemented, and
  3. pre-implementation contract-gate evidence is recorded.
- Enforce kernel profile consistency using
  `docs/specifications/science-contracts/kernel-process-contract-profile.md`.
- Do not introduce silent down-classification of Tier-A blockers or implicit
  risk-acceptance; all exceptions must have explicit approval references.

Required outputs are listed in `package.md` Deliverables.
