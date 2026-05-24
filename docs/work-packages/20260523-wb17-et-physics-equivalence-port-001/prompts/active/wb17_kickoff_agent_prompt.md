# WB17 Kickoff Agent Prompt

Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.
Autonomy: execute package phases end-to-end and update required artifacts without requesting additional user direction unless hard-blocked.
Required reading (read before edits):
- /workdir/openWEPP/AGENTS.md
- /workdir/openWEPP/docs/codex_exec_plans.md
- /workdir/openWEPP/docs/work-packages/README.md
- /workdir/openWEPP/docs/work-packages/20260523-wb17-et-physics-equivalence-port-001/package.md


You are executing
`20260523-wb17-et-physics-equivalence-port-001` for the monolithic openWEPP
scientific hydrology/erosion model.

Objectives:
1. Replace WB11 ET surrogate behavior with equation-driven ET physics
   implementation in production runtime execution.
2. Implement required canonical WB17 contract amendments and contract-derived
   tests.
3. Produce ET equation-vector and partition-trajectory closure evidence.
4. Preserve ARCH15/ARCH21 typed-seam non-regression posture.

Mandatory sequencing constraints:
- Do not modify production WB17 ET kernel code until:
  1. contract amendments are implemented, and
  2. contract-derived tests are implemented, and
  3. pre-implementation contract-gate evidence is recorded.
- ET physics authority must be explicit in canonical science contracts
  (`SC-EVAP-001` and required companion `SC-*` files): equations, symbols,
  units, guards, and partition semantics. Package-local notes are not
  sufficient for closure.
- Legacy ET migration authority must be sourced from pinned
  `/workdir/wepp-forest_260430_baseline` (commit
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`) and mapped into WB17 provenance
  artifacts.
- Contract tests must be authored from canonical SC authority, not from current
  implementation behavior.
- Enforce kernel profile consistency using
  `docs/specifications/science-contracts/kernel-process-contract-profile.md`.
- Do not introduce silent defaults/clamping for ET domain violations, missing
  required symbols, or non-finite values.
- Complete dual code review and dual verification artifacts before final
  disposition.

Required outputs are listed in `package.md` Deliverables.
