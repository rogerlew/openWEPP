# CAL-07B Exact-Diff Reconciliation

Evidence class: `Static`

## Declared write set

CAL-07B declared this write set:

- `docs/work-packages/20260728-canopy-cal-07b-hourly-vpd-aggregation-diagnostic-001/`;
- `docs/planning/canopy-phenology-assurance-roadmap.md`; and
- `docs/work-packages/README.md`.

No production Rust, science contract, fixture, test, CAL-07 input, or runtime
publication path is in scope.

## Current worktree context

The terminal worktree also contains the previously authorized, untracked
CAL-07 package:

- `docs/work-packages/20260728-canopy-cal-07-southern-hemisphere-robustness-001/`.

That directory is retained as separate CAL-07 hold evidence. CAL-07B did not
change production paths or claim ownership of CAL-07's package-local results.
The local CAL-07 package has ignored Cargo `target/` build output on disk;
`git status --untracked-files=all` reports 62 nonignored CAL-07 package files
as add candidates.

## CAL-07B package inventory

`git status --untracked-files=all` reports 40 nonignored CAL-07B package files
as add candidates. The package contains:

- eight retained source objects under `inputs/source/`;
- hourly reconstruction, daily decomposition, attribution, source manifest,
  and result manifest CSVs;
- three plot-only SVG figures and three Markdown sidecars;
- prospective and terminal review/verification artifacts;
- science summary, gate evidence, line-count governance, finding
  disposition, exact-diff reconciliation, and final disposition; and
- package-local analyzer, renderer, and independent validator tools.

## Shared tracked files

The shared tracked-file delta is:

| File | Insertions | Deletions | Purpose |
| --- | ---: | ---: | --- |
| `docs/planning/canopy-phenology-assurance-roadmap.md` | 31 | 10 | Record CAL-07 hold and CAL-07B diagnostic result. |
| `docs/work-packages/README.md` | 18 | 0 | Add CAL-07 and CAL-07B catalog entries. |

These tracked changes are in the declared shared documentation write set. They
do not modify production behavior.

## Disposition

The exact diff supports `COMPLETE / DIAGNOSTIC PASS / CAL-07 HOLD RETAINED`
for CAL-07B. It does not support CAL-07 resumption, Order 7 advancement,
input clipping, production canonicalization, or OBL-PLANT-P-013 replacement.
