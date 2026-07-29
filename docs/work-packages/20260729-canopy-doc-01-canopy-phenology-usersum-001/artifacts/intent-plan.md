# Intent and Validation Plan

Status: `complete`

Evidence mode: `Static`

## Intent

Author one broad model-science narrative and catalog entry. The narrative
explains the native-forest need, process chain, user coefficients, calibration
sequence, evidence boundary, and interpretation without duplicating the later
assurance report.

## Declared write set

- `usersum/openwepp-canopy-phenology.md`
- `usersum/README.md`
- `docs/planning/canopy-phenology-assurance-roadmap.md`
- `docs/work-packages/README.md`
- this package tree

No conceptual figure is required: the process chain and equations remain
clearer as compact prose and text equations than as a new claim-bearing asset.
Production code, schemas, contracts, prior evidence, assurance surfaces, and
release tooling are read-only.

## Validation intent

Run the four declared Markdown lint scopes, `git diff --check`, changed-link
and usersum-boundary checks, a reviewed `uk2us` preview, coefficient-ledger
completeness checks, dual independent review, and dual verification. Rust,
comparator, empirical, and assurance-publication gates are not applicable to
this documentation-only diff.
