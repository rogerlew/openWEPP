# Execute SNOW-SURFACE-EB-04V

Lifecycle: archived after terminal disposition on `2026-08-01`.

Scope: local repository kernel/science diagnostic implementation; flat-file
reads/edits and local model/test execution only; no external systems or network
actions are required.

Execution mode: package-end-to-end.

Autonomy: execute every phase in `package.md` sequentially through disposition
without requesting user intervention unless a declared hard boundary is proven.

## Required Reading

Core: root `AGENTS.md`, `docs/codex_exec_plans.md`,
`docs/work-packages/AGENTS.md`, `docs/work-packages/README.md`, and this
package's `package.md`.

Conditional (triggered): `docs/specifications/science-contracts/AGENTS.md`,
`docs/specifications/science-contract-authoring-procedure.md`,
`docs/specifications/science-contracts/kernel-process-contract-profile.md`,
`docs/specifications/science-contracts/index.md`,
`docs/specifications/unit-governance.md`, `crates/AGENTS.md`, and
`tests/AGENTS.md` because the package amends a kernel-adjacent contract,
runtime state, dimensional trace publication, and tests.

On-demand: `SC-SNOWFREEZE-001`, ADR-0042, EB-04U terminal artifacts, current
density/runtime/trace source, and EB-04S harness/provenance files for the exact
mechanisms and execution surfaces touched.

Required-reading budget: approximately `1.0 MB`,
`REQUIRES-JUSTIFICATION`; map: `artifacts/required-reading-map.md`. The large
contract and catalog cannot be reduced because active density authority is
distributed across the contract's binding invariants/change history and the
catalog is mandatory Core reading.

## Task

Execute EB-04V end-to-end within its declared write set. Freeze operands and
the nine-lane diagnostic population; amend canonical authority; add failing
contract-derived tests; record the pre-implementation contract gate; implement
the typed process ledger through the real JSONL consumer; execute the 36-cell
diagnostic population; analyze mechanisms by observed-anchored phase and bias
direction; produce figures/sidecars; and complete all closure artifacts.

Constraints: contract-first sequencing; typed units/guards; no silent defaults;
no change to density arithmetic, constants, selectors, defaults, cap, CoE
boundary, or non-density processes.

No surrogate physics: production code may expose only the existing canonical
Anderson/SNOBAL process terms. Surrogate, provisional, proxy, heuristic, fitted,
or site-specific physics is forbidden.

Real consumer proof: the real direct-production JSONL trace must read every
new field; producer-only, test-only, shadow, wrapper, or stale compatibility
paths cannot carry closure.

Conservation/output acceptance: record operand lineage, distinguish plausible
aliases, reject omitted-term formulas, independently reconstruct additive
density closure from produced JSONL, run a cohort magnitude audit, and align
contract/units/schema. Self-consistency alone is insufficient.

Subagent requirement: REQUIRED. This prompt explicitly authorizes subagent
spawning/delegation to two independent science/code reviewers, two terminal
verifiers, and `comparator_suite_runner` for the 36-cell population and full
critical suite. Outputs are bounded package artifacts, compact metrics, and log
paths; reviewer/verifier writes are limited to assigned files and the suite
runner may write only package/target evidence outputs.

Every current-scope gate needs direct current evidence. Otherwise hold with the
named blocker; do not reclassify it as future scope after implementation begins.
