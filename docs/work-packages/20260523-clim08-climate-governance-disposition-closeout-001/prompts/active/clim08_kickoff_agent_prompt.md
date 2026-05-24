# CLIM08 Kickoff Agent Prompt

Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.
Autonomy: execute package phases end-to-end and update required artifacts without requesting additional user direction unless hard-blocked.
Required reading (read before edits):
- /workdir/openWEPP/AGENTS.md
- /workdir/openWEPP/docs/codex_exec_plans.md
- /workdir/openWEPP/docs/work-packages/README.md
- /workdir/openWEPP/docs/work-packages/20260523-clim08-climate-governance-disposition-closeout-001/package.md


You are executing
`20260523-clim08-climate-governance-disposition-closeout-001` for the
monolithic openWEPP scientific hydrology/erosion model.

Objectives:
1. Close remaining CLIM HOLD items (`parser/runtime seam`, climate seam
   integration-test closure) using existing CLIM evidence.
2. Update canonical climate contracts/specifications to promotable governance
   status where closure evidence supports promotion.
3. Reconcile climate governance vocabulary/status semantics across CLIM
   packages and canonical contract surfaces.
4. Publish final CLIM08 governance disposition (`GO` or `HOLD`) with explicit
   rationale.

Constraints:
- This package is governance/contracts scoped; do not introduce new climate
  runtime algorithm branches in CLIM08.
- Canonical authority must be updated in `SC-*` files when promotion/closure
  claims are made.
- Enforce kernel profile consistency using
  `docs/specifications/science-contracts/kernel-process-contract-profile.md`.
- Use truthfulness posture (`Static:` vs `Ran:`) in all artifacts.
- Correctness over completion: unresolved high-severity governance gaps remain
  `HOLD`.

Required outputs are listed in `package.md` Deliverables.
