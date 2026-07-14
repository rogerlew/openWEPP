# Kickoff: Growth-State CRAP Regression Closure

Execution mode: `package-end-to-end`

Autonomy: execute the bounded behavior-preserving CQR package through focused
verification, delegated heavy closure, dual review, finding disposition, and
final package status. Do not change equations or create an adjudication.

## Required Reading

Tier 1:

- `AGENTS.md`
- `crates/AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- package-local `package.md`

Tier 2:

- `docs/specifications/science-contracts/contracts/SC-PLANT-001.md`
- `docs/decisions/0021-module-coverage-closure-thresholds.md`
- `docs/standards/code-quality-refactor-authoring-guide.md`
- `docs/standards/mechanical-refactor-authoring-guide.md`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/growth.rs`

Tier 3:

- prior row-6 growth CQR package and retained fresh CRAP artifact named in
  `package.md`.

Required-reading budget: read Tier 1 and directly applicable Tier 2 material
before editing. Use Tier 3 only for provenance and comparator context.

## Implementation Boundary

Extract only the perennial/annual root mass and root-depth candidate block into
one private helper. Preserve all arithmetic expression grouping, branch order,
and validation order. Reuse existing tests; add a test only if a branch lacks
characterization. No contract amendment is authorized because behavior must not
change.

## Subagents

Use the package-authorized comparator runner for fresh CRAP and full closure.
After terminal implementation, dispatch two independent reviewers. They must
submit findings before reading each other's work and then verify accepted
dispositions.
