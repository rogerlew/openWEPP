# Kickoff: Native-Forest Ecohydrology Authority Reframe

Execute
`docs/work-packages/20260809-native-forest-ecohydrology-authority-reframe-001/package.md`
autonomously to terminal disposition.

## Mission

Correct `SC-VEGETATION-001` so site-specific stratum parameters and compatible
initial state are caller-supplied values governed by an A0 schema rather than
values openWEPP must select. Replace the future native-forest implementation
target's agricultural `Kcb`/LAI PMET redistribution with independently closed
canopy transpiration, wet-canopy evaporation, forest-floor evaporation, and
layer-resolved root uptake obligations. Do not implement production Rust.

## Required Reading

Read fully before edits:

- root `AGENTS.md` and the applicable chains returned by
  `tools/agents/find-agents`;
- `docs/work-packages/AGENTS.md`;
- `docs/specifications/science-contracts/AGENTS.md`;
- `docs/standards/AGENTS.md`;
- `docs/standards/testing-and-gate-strategy.md`;
- `docs/standards/kernel-work-package-preparation.md`;
- `docs/standards/prompt-wording-guidance.md`;
- `docs/specifications/science-contract-authoring-procedure.md`;
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`;
- `docs/specifications/unit-governance.md`;
- `tests/AGENTS.md`;
- this package, `SC-VEGETATION-001`, the held coupled-vegetation package, and
  the Stevens Canyon investigation README plus its ET-option, PMET calibration,
  legacy-ablation, and water-balance artifacts.

## Execution Rules

- Write failing contract assertions before canonical contract edits.
- Preserve the historical predecessor disposition; supersede its premise by
  explicit forward links.
- Treat literature as process authority candidates only when the complete
  equation, operands, constants, domains, guards, ownership, and vectors are
  admitted. Citation presence alone is not closure.
- `AUTH-RHEC-007` does not mandate porting RHESSys PM and does not forbid all PM
  component equations. It forbids the agricultural PMET partition as the
  native-forest target and requires independent component flux closure.
- Demonstration fixtures use explicit `ASSUMED_FOR_EXECUTION` values and make
  no site-suitability claim.
- No production edits, branches, pushes, fallback defaults, or surrogate
  physics.

## Delegation Authorization

You are explicitly authorized and required to spawn two independent science
reviewers, two independent terminal verifiers, and a
`comparator_suite_runner` for the Critical full-workspace gate. Reviewers and
verifiers must inspect the exact current diff and write separate artifacts.

## Completion

Reconcile the exact terminal diff, record all commands and outcomes, disposition
all review findings, run post-fix verification, archive this prompt, update
lifecycle catalogs, and close only when every exit criterion in `package.md`
is satisfied.

