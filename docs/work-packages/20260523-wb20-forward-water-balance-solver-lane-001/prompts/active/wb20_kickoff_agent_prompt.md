# WB20 Kickoff Agent Prompt

You are executing
`20260523-wb20-forward-water-balance-solver-lane-001` for the monolithic
openWEPP scientific hydrology/erosion model.

Scope note:
- This task is local repository science-contract and Rust model migration work.
- Operate on flat files in this repository/worktree only.

Objectives:
1. Establish forward-solver parity lane behavior where closure signals are
   solver-output-derived and not observed-target-driven.
2. Implement required canonical WB20 contract amendments and contract-derived
   tests.
3. Produce lane-manifest, no-substitution, and replay-trace closure evidence.
4. Preserve ARCH15/ARCH21 typed-seam non-regression posture.

Mandatory sequencing constraints:
- Do not modify production WB20 lane/runtime code until:
  1. contract amendments are implemented, and
  2. contract-derived tests are implemented, and
  3. pre-implementation contract-gate evidence is recorded.
- Forward-solver lane authority must be explicit in canonical science contracts
  (`SC-WATBAL-001` and required companion `SC-*` files), including
  observed-target exclusion semantics. Package-local notes are not sufficient
  for closure.
- Legacy migration/provenance authority must be sourced from pinned
  `/workdir/wepp-forest_260430_baseline` (commit
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`) where applicable and mapped into
  WB20 provenance artifacts.
- Contract tests must be authored from canonical SC authority, not from current
  implementation behavior.
- Enforce kernel profile consistency using
  `docs/specifications/science-contracts/kernel-process-contract-profile.md`.
- Do not introduce silent defaults/substitution for excluded observed targets,
  missing required symbols, or non-finite values.
- Complete dual code review and dual verification artifacts before final
  disposition.

Required outputs are listed in `package.md` Deliverables.
