Scope: local repository governance/contract ratification task; flat-file reads/edits only; no external connectivity.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in package.md sequentially through disposition.

Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `docs/work-packages/20260605-adr0017-comparator-distrust-ratification-001/package.md`
- `docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `docs/decisions/0016-promote-260430-baseline-as-canonical-comparator-and-abandon-kernel-rewrite.md`
- `docs/decisions/0017-re-pin-operational-distrust-comparator-is-flag-not-target.md`
- `docs/specifications/correctness-authority-model.md`
- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `docs/specifications/science-contracts/index.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/unit-governance.md`

Files:
- `docs/decisions/0017-re-pin-operational-distrust-comparator-is-flag-not-target.md`
- `docs/decisions/0016-promote-260430-baseline-as-canonical-comparator-and-abandon-kernel-rewrite.md`
- `docs/decisions/README.md`
- `docs/codex_exec_plans.md`
- `docs/specifications/correctness-authority-model.md`
- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/index.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/unit-governance.md`
- `docs/work-packages/README.md`
- `docs/work-packages/20260605-adr0017-comparator-distrust-ratification-001/package.md`
- `docs/work-packages/20260605-adr0017-comparator-distrust-ratification-001/prompts/README.md`
- `docs/work-packages/20260605-adr0017-comparator-distrust-ratification-001/prompts/active/README.md`
- `docs/work-packages/20260605-adr0017-comparator-distrust-ratification-001/prompts/archived/README.md`
- `docs/work-packages/20260605-adr0017-comparator-distrust-ratification-001/prompts/active/20260605-adr0017-comparator-distrust-ratification-001_kickoff_agent_prompt.md`
- `docs/work-packages/20260605-adr0017-comparator-distrust-ratification-001/artifacts/README.md`
- `docs/work-packages/20260605-adr0017-comparator-distrust-ratification-001/artifacts/contract-implementation-evidence.md`
- `docs/work-packages/20260605-adr0017-comparator-distrust-ratification-001/artifacts/contract-test-implementation-evidence.md`
- `docs/work-packages/20260605-adr0017-comparator-distrust-ratification-001/artifacts/pre-implementation-contract-gate.md`
- `docs/work-packages/20260605-adr0017-comparator-distrust-ratification-001/artifacts/implementation-test-evidence.md`
- `docs/work-packages/20260605-adr0017-comparator-distrust-ratification-001/artifacts/kernel-profile-compliance-checklist.md`
- `docs/work-packages/20260605-adr0017-comparator-distrust-ratification-001/artifacts/owned-file-manifest.md`
- `docs/work-packages/20260605-adr0017-comparator-distrust-ratification-001/artifacts/gate-results.md`
- `docs/work-packages/20260605-adr0017-comparator-distrust-ratification-001/artifacts/disposition.md`
- `docs/work-packages/20260605-adr0017-comparator-distrust-ratification-001/artifacts/worker-handoff.md`
- `docs/work-packages/20260605-adr0017-comparator-distrust-ratification-001/artifacts/review_agent_a.md`
- `docs/work-packages/20260605-adr0017-comparator-distrust-ratification-001/artifacts/review_agent_b.md`
- `docs/work-packages/20260605-adr0017-comparator-distrust-ratification-001/artifacts/review-disposition.md`
- `docs/work-packages/20260605-adr0017-comparator-distrust-ratification-001/artifacts/verification_agent_a.md`
- `docs/work-packages/20260605-adr0017-comparator-distrust-ratification-001/artifacts/verification_agent_b.md`
- `Cargo.toml`
- `tests/integration/adr0017_comparator_distrust_ratification_contract.rs`
- `tests/integration/hphys0313_snowpack_settling_carry_recursion_contract.rs`

Task: ratify ADR-0017 end-to-end for declared scope.

Constraints: contract-first sequencing; canonical SC authority; comparator is a flag not a target; typed/explicit governance guards; no silent defaults; no production physics edits; no downstream compensation; dual review/disposition/verification artifacts required.

Autonomy: execute package phases end-to-end and update required artifacts without requesting additional user direction unless hard-blocked.

Outputs: update package artifacts/disposition for all completed phases.
