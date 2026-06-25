# Evidence Reconciliation

Evidence mode: Static + Ran.

## Inputs

- SNOWFROST-FIDELITY-E: paired non-SNOTEL snow-depth failures are
  like-for-like physical depth failures and mostly modeled-over-observed.
- SNOWFROST-FIDELITY-F: pinned legacy WEPP fails the same paired snow-depth
  control; openWEPP SWE is close to legacy SWE.
- SNOWFROST-FIDELITY-H: SNOTEL SWE/depth/density comparison routes all five
  sites `STRUCTURAL`; observed-density `ssd` arms do not close depth.
- SNOWFROST-FIDELITY-I0: current non-SNOTEL rubric baseline still has three
  paired-snow sites failing snow control and two sites without paired observed
  snow depth.
- `SC-SNOWFREEZE-001`: `INV-SNOWFREEZE-048/049/050` define snow-depth
  correspondence, SNOTEL density correspondence, and the rubric profile.

## Findings

1. **This is not an array-native regression.**
   H pins the maximum absolute openWEPP-minus-legacy mean-signed density
   residual at `4.351 kg m^-3`, while observation residuals are much larger.
   F also shows both models fail paired snow-depth control.

2. **This is not an `ssd` tuning problem.**
   The H observed-density arm used SNOTEL peak-SWE-period density before
   residual comparison and still routed every site `STRUCTURAL`. The arm often
   worsened depth MAE.

3. **This is not a PySnobal adoption result.**
   PySnobal is useful reference/profile evidence, but H does not show it beating
   WEPP variants under the current bridge. CSS Lab WY2017 is unavailable due to
   upstream thin-snow instability.

4. **The actionable next scope is contract/ADR preparation for an opt-in
   physics candidate.**
   The candidate needs state-evolved density/depth, not a site-specific
   empirical settling-density parameter.

5. **Frost-depth physics remains blocked.**
   Snow insulation is still uncontrolled on the current observed corpus.
   Heat-flow, frozen-K/SFCC, impedance, or migration/fringe production work is
   not authorized from these residuals.

## Reconciled Route

Close SNOWDENSITY-01 as complete evidence reconciliation and route the next work
to `SNOWDENSITY-02 Contract + ADR`.

SNOWDENSITY-02 should be contract-first and should not edit production runtime
physics. It should:

- define the `snow_model = legacy_wepp | physics_bulk` authority envelope;
- ratify no-site-tuning and opt-in-only constraints;
- select or narrow fresh-snow-density and densification candidate equations;
- define state variables and conservation obligations;
- draft the deliberate legacy-divergence ADR;
- create red contract tests for the selected envelope.
