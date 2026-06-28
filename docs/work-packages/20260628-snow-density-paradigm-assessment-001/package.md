# Snow-Density Paradigm Assessment — climate-class (Paradigm 1) vs multilayer (Paradigm 2)

Status: scaffolded (assessment not yet executed)
Type: **design/architecture decision package** — produces a comparison + recommendation + ADR-candidate. **No production code.**
Owner: TBD (design reasoning is Claude Code's lane; current-implementation grounding may be delegated to Codex)
Date: 2026-06-28
Closure target: `PARADIGM-ASSESSED` (recommendation + ADR-candidate) or named `HOLD-...`.

## Objective

Assess the two known regime-divergent snow-density paradigms — and the
accept-the-floor baseline — for resolving the dominant residual the snow program
now faces, and recommend one, judged against openWEPP's current implementation,
modeling philosophy, and long-term robustness. The output is a decision, not code.

## Context

SNOWDENSITY-10.3.21 decomposed the post-partition residual (default 15 fail / 179
on the cross-SNOTEL forcing-robust rubric, above legacy 16 / 176). The dominant
cluster is **`seasonal_densification_trajectory`, ~60% of the residual, diffuse
across all climates, and split-sign**: the bulk model over-densifies humid/
continental-forest packs and under-densifies deep mountain packs. The
SNOBAL/CoE/Anderson **bulk single-layer** family is exhausted (10.3.16–10.3.20:
sublimation, composition, shallow-pack guard, two-layer Stage B all failed to beat
the default). A single bulk densification curve cannot satisfy both regimes — the
physical control is the vertical temperature gradient (kinetic/depth-hoar vs
equilibrium/melt-freeze metamorphism) plus the overburden load profile a bulk pack
cannot represent.

Regime-divergence research (2026-06-28) found two established paradigms; both are
admissible only under ADR-0028 (observed-data admission when scientific authority
is under-specified) and must improve the cross-SNOTEL forcing-robust rubric
without fixture fitting.

## The options to assess

- **Paradigm 1 — climate-class parameter specialization.** Sturm et al. 1995 (R-59)
  six-class snow classification from {wind, precipitation, air temperature}; Sturm
  et al. 2010 (R-58) class-specific densification parameters. Adopt as
  **regime-adaptive Anderson/SNOBAL densification coefficients keyed by snow class**
  (the clean ADR-0028 form), *not* the raw empirical depth+DOY regression. Lighter
  change; keys off variables openWEPP already has.
- **Paradigm 2 — physics-resolved multilayer.** Crocus (Vionnet 2012, R-40) /
  SNOWPACK / SNTHERM: resolve the vertical profile + grain metamorphism so
  regime-appropriate density emerges (keyed off temperature gradient, liquid water,
  overburden, grain type). The "new density structure." Heavyweight; the capability
  bulk SNOBAL itself lacks.
- **Baseline — accept the bulk floor, proceed to frost.** Treat 15/179 as the
  defensible floor (10.3.21 recommendation), carry the depth/density residual as a
  bounded uncertainty into frost attribution. No new density mechanism.

## Assessment dimensions (the comparison matrix to fill)

For each option, score and justify against:

1. **Fit with current implementation.** Rework required against: bulk single-layer
   density (`crates/openwepp-hillslope-orchestrator/src/hydrology/09_snow_density.rs`),
   the array-native hot-path runtime (ADR-0025) and its perf budget, the stateful
   winter-column sub-solver (ADR-0026), the opt-in `physics_bulk` lane (ADR-0027),
   and the HBP/contract/output surfaces. (Paradigm 2 introduces per-layer state →
   a structural change to the runtime representation; Paradigm 1 is a coefficient
   selector on the existing scalar density state.)
2. **Modeling-philosophy fit.** Contract-first / process-first (ADR-0011);
   ADR-0028 admission (defensible physics + rubric + no overfit); ADR-0017
   (comparators as flags); conservation; no site calibration. Weigh Paradigm 1's
   empirical class regression (regime-grounded but statistical) against Paradigm 2's
   mechanistic resolution (more process-first but heavier and still
   parameter-laden).
3. **Long-term robustness.** Global/hemisphere generality (class drivers
   {wind,precip,temp} + NSIDC-0768 vs prognostic physics); behavior in out-of-sample
   / changing climates; maintainability; reuse for adjacent subsystems (frost soil
   thermal, the cluster-3 depth-SWE geometry, canopy); external dependency footprint
   (a class data product vs added prognostic state).
4. **Cost / effort / risk.** Implementation size, runtime cost under the ADR-0025
   budget, test/validation burden, and the probability each actually clears the
   cross-SNOTEL rubric on the cluster-1 split-sign signature.
5. **Expected payoff.** Which addresses cluster 1 (and 2/3) on the rubric; the
   frost-relevance (depth + density both drive frost insulation — the residual sits
   in exactly the frost-critical dimensions).

## Required reading

- `docs/planning/snow-frost-fidelity-strategy.md` §10.2 item 7, §10.3 steps 8–9
- `docs/work-packages/.../10-3-18-...` and `.../10-3-21-...` artifacts (the rubric
  + residual decomposition); the 10.3.20 review artifact
- ADRs: 0011 (contract-first), 0017 (comparators-as-flags), 0025 (array-native),
  0026 (winter-column sub-solver), 0027 (opt-in physics_bulk), 0028 (observed-data
  admission)
- References: R-58 Sturm 2010, R-59 Sturm 1995, R-40 Vionnet 2012 Crocus, Anderson
  1976 + Marks 1999 (current densification authority); NSIDC-0768 (gridded classes)
- Current code: `09_snow_density.rs` and the winter-column density state

## Deliverables

- `artifacts/paradigm-comparison.md` — the filled assessment matrix (5 dimensions ×
  3 options), each cell justified with evidence/citation.
- `artifacts/recommendation.md` — a single recommendation (Paradigm 1 / Paradigm 2 /
  accept-floor) with the decisive reasoning, and the concrete next package it
  implies (a scoped ADR-0028 candidate package, or the frost-attribution handoff).
- An **ADR-candidate** (draft, if a paradigm is chosen) — domain-scoped to snow
  density-structure, citing ADR-0028 as the admission basis.

## Non-scope

- No production density code, no contract amendment beyond the ADR-candidate draft,
  no fixture/schema/default change. This package decides direction; a later package
  implements the chosen paradigm opt-in under the cross-SNOTEL rubric gate.

## Closure / acceptance

- The comparison matrix and recommendation are complete and evidence-grounded.
- The recommendation names the next package (candidate implementation or frost).
- If no option is clearly superior, close `HOLD-PARADIGM-UNDECIDED` with the
  specific evidence each option still needs.

## Note on sequencing

This package sits **after 10.3.21** in §10.3 as the density-structure decision. It
does **not** block the frost-attribution-threshold decision — the two can run in
parallel: if the recommendation is accept-floor, it converges with proceeding to
frost; if a paradigm is chosen, frost still proceeds with the residual carried as
uncertainty while the density candidate is built and gated separately.
