# EROD11 Kickoff Agent Prompt

You are executing
`20260523-erod11-alias-and-boundary-ownership-closure-001` for the monolithic
openWEPP scientific hydrology/erosion model.

Objectives:
1. Close Wave-0 alias/ownership ambiguity identified by EROD10.
2. Publish canonical-to-runtime alias ownership across required companion
   erosion contracts.
3. Publish explicit EROD12 entry verdict (`GO` or `HOLD`) with rationale.
4. Preserve kernel governance posture (procedure/profile + truthfulness labels).

Constraints:
- This package is governance/contracts scoped; do not implement production
  erosion kernel physics in EROD11.
- Kernel-affecting code edits are prohibited before all contract-first gates
  are complete in order:
  1. canonical contract updates,
  2. contract-derived tests,
  3. pre-implementation contract gate evidence.
- Canonical authority must be updated in `SC-*` files when closure claims are
  made; package-local notes are not authority.
- For migration/parity authority, default provenance is
  `/workdir/wepp-forest_260430_baseline` at commit
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70` unless explicitly justified.
- Prohibit scaffolded/placeholder physics acceptance: unresolved scaffolded
  behavior must remain `HOLD` with explicit disposition.
- Enforce kernel profile consistency using
  `docs/specifications/science-contracts/kernel-process-contract-profile.md`.
- Do not silently default/clamp domain violations; require typed errors/guards
  and explicit invariant mapping.
- Use truthfulness posture (`Static:` vs `Ran:`) in all artifacts.
- Correctness over completion: unresolved high-severity alias/ownership
  ambiguity remains `HOLD`.

Required outputs are listed in `package.md` Deliverables.
