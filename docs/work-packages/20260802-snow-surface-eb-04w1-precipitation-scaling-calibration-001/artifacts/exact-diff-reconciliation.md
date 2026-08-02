# Exact-Diff Reconciliation

Status: `PASS / TERMINAL RECONCILIATION`

Evidence mode: **Static + Ran**.

## Intended Versus Actual

| Declared surface | Actual terminal path class | Reconciliation |
|---|---|---|
| package tree | new `20260802-...-eb-04w1.../` plan, tool, evidence, figures, prompts | expected |
| campaign roadmap | `docs/planning/snow-surface-energy-balance-roadmap.md` | expected; records W1 result and W2 next gate |
| root roadmap | `docs/ROADMAP.md` | expected; updates current priority only |
| work-package catalog | `docs/work-packages/README.md` | expected; records executed evidence and claim boundary |
| target runtime output | ignored `target/snow_surface_eb04w1_precipitation_scaling/` | expected; retained locally, not committed |

## Protected Surfaces

`git diff --name-only`, untracked-path inventory, and status inspection show no
changes under production Rust, canonical science contracts, manifests, tests,
source fixtures, source observations, public schemas, selectors, defaults,
assurance reports, or historical work-package evidence. The only tracked
predecessor files changed are the three declared roadmap/catalog documents.

No source branch was created or switched. EB-04W, EB-04R, and EB-04S evidence
is unchanged. Both reviewer and both verifier artifacts are confined to the
declared package tree. The execution prompt is archived and the final package,
roadmap, catalog, SVG, and whitespace checks pass.
