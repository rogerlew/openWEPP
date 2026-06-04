# Disposition

Status: complete
Evidence mode: Static + Ran

## Outcome

HPHYS0283 is complete for the active-snowmelt runoff/infiltration partition seam.

The original spring 2014 `Total-Soil` collapse rows no longer collapse to roughly `30..45 mm` after routed melt is offered to infiltration and applied to WB18 layer storage. The final targeted rows are:

- H1 Julian 145: candidate `Total-Soil` moved from `33.747 mm` to `343.986 mm`.
- H7 Julian 146: candidate `Total-Soil` moved from `31.793 mm` to `296.668 mm`.
- H39 Julian 145: candidate `Total-Soil` moved from `45.485 mm` to `303.333 mm`.

## Residual

Semantic parity is not closed. Final full-suite metrics remain `0/39` semantic pass, with `Total-Soil` mean abs diff `83.841688`.

The remaining residual now localizes away from the original melt-only runoff bypass and toward:

- snowpack timing/retention, because `Snow-Water` metrics did not move;
- earlier-season storage divergence before the largest spring melt rows;
- runoff/storage magnitude remaining in the spring snow window.

## Closure Basis

- Contract-first sequence completed.
- Focused and workspace Rust gates passed.
- Full H1..H39 suite rerun on final code state.
- Dual review and dual verification artifacts completed.
- No accepted review finding remains unresolved.
