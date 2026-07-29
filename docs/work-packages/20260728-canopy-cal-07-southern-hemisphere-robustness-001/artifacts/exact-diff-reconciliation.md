# CAL-07 Exact-Diff Reconciliation

Evidence class: `Static`

## Declared versus actual write set

The terminal worktree contains changes only in:

- `docs/work-packages/20260728-canopy-cal-07-southern-hemisphere-robustness-001/`;
- `docs/planning/canopy-phenology-assurance-roadmap.md`; and
- `docs/work-packages/README.md`.

These are exactly the package's declared write set. Production crates,
canonical contracts, tests, fixtures, and prior package evidence are
unchanged.

## Tracked-file delta outside the new package

- Roadmap: 18 inserted lines and 10 removed lines.
- Work-package catalog: 9 inserted lines.

## New package inventory

The package contains 62 retained files after terminal review and closure
artifacts. Ignored Cargo build output is not part of the evidence set.
The package includes:

- 13 immutable source objects and their digest manifest;
- normalized ensemble, forcing, observation, and transition inputs;
- prospective and terminal review/verification artifacts;
- fail-closed incident, diagnostics, manifests, summaries, and disposition;
- four plot-only SVGs and four detailed Markdown sidecars; and
- deterministic preparation, execution, analysis, rendering, and validation
  tools.

No forbidden partial canopy output, gate receipt, score table, or verdict
matrix exists.

## Disposition

The exact diff supports `HOLD / FORCING AUTHORITY INCOMPATIBLE / NO CANOPY
RESULT`. It does not support Order 7 advancement or a completed robustness
claim.
