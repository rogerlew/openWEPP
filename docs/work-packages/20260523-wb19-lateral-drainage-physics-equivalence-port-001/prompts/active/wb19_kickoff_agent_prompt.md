# WB19 Kickoff Agent Prompt

You are executing
`20260523-wb19-lateral-drainage-physics-equivalence-port-001` for the
monolithic openWEPP scientific hydrology/erosion model.

Scope note:
- This task is local repository science-contract and Rust model migration work.
- Operate on flat files in this repository/worktree only.

Objectives:
1. Replace WB11 lateral/drainage surrogate behavior with layer-aware,
   equation-driven lateral/drainage physics implementation in production runtime
   execution.
2. Implement required canonical WB19 contract amendments and contract-derived
   tests.
3. Produce hydraulic-vector and branch-response closure evidence.
4. Preserve ARCH15/ARCH21 typed-seam non-regression posture.

Mandatory sequencing constraints:
- Do not modify production WB19 lateral/drainage kernel code until:
  1. contract amendments are implemented, and
  2. contract-derived tests are implemented, and
  3. pre-implementation contract-gate evidence is recorded.
- Lateral/drainage physics authority must be explicit in canonical science
  contracts (`SC-SUBHYD-001` and required companion `SC-*` files): equations,
  symbols, units, guards, branch and layer semantics. Package-local notes are
  not sufficient for closure.
- Legacy lateral/drainage migration authority must be sourced from pinned
  `/workdir/wepp-forest_260430_baseline` (commit
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`) and mapped into WB19 provenance
  artifacts.
- Contract tests must be authored from canonical SC authority, not from current
  implementation behavior.
- Enforce kernel profile consistency using
  `docs/specifications/science-contracts/kernel-process-contract-profile.md`.
- Do not introduce silent defaults/clamping for lateral/drainage domain
  violations, missing required symbols, or non-finite values.
- Complete dual code review and dual verification artifacts before final
  disposition.

Required outputs are listed in `package.md` Deliverables.
