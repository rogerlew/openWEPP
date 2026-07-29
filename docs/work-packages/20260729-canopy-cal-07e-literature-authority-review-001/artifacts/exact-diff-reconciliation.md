# CAL-07E Exact-Diff Reconciliation

Evidence class: `Static + Ran`

## Declared write set

The authorized write set was:

1. the new CAL-07E package;
2. `docs/planning/canopy-phenology-assurance-roadmap.md`; and
3. `docs/work-packages/README.md`.

Production code, tests, science contracts, ADRs, and predecessor packages were
read-only.

## Terminal worktree

The terminal diff contains exactly:

- one new CAL-07E package tree;
- one modified canopy assurance roadmap; and
- one modified work-package catalog.

The package contains 28 retained files: protocol, archived prompt, external
source subset and provenance, machine-readable source/claim/product evidence,
synthesis, acquisition and authority decisions, one SVG/sidecar pair, one
validator, and dual review/verification plus closure records.

No path outside the declared write set is changed.

## Intent reconciliation

- No production behavior or science authority changed.
- The review closes without treating inaccessible sources as evidence.
- The provisional PhenoCam rows needed for the product audit are
  checksum-bound and source-validated.
- Roadmap and catalog retain Order 7's hold.
- CAL-07F is authorized only as an observation-product/operator audit.

`git diff --check` passes. Exact-diff reconciliation is `PASS`.
