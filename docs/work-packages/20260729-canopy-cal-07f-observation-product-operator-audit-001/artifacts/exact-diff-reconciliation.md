# CAL-07F Exact-Diff Reconciliation

Evidence class: `Static + Ran`

## Declared write set

The authorized write set is:

1. the new CAL-07F package;
2. CAL-07E `artifacts/acquisition-needed.md`;
3. the canopy assurance roadmap; and
4. the work-package catalog.

Production code, tests, contracts, ADRs, and all other predecessor evidence
were read-only.

## Terminal worktree

The terminal diff contains exactly:

- one new 38-file CAL-07F package tree;
- one modified CAL-07E acquisition record;
- one modified canopy assurance roadmap; and
- one modified work-package catalog.

No path outside the declared write set is modified or untracked.

## Intent reconciliation

- No parameter was fitted or selected.
- No production, forcing, process, contract, or source-role change occurred.
- Both observation products and missing crossings remain visible.
- The decision follows the prospective six-criterion screen.
- Acquisition and further canopy work are deferred under the user stop-loss.
- The limitation is scoped to the assessed Bezà tropical dry-forest lane.

`git diff --check` passes. Exact-diff reconciliation is `PASS`.
