# Base Conductivity Plausibility

Evidence class: Static + Ran

Verdict: the raw H2637 conductivity values are not rejected as physically
impossible under current authority. The defect is algorithmic: openWEPP's
vertical `ssc` layer normalization uses the wrong mean for split layers.

## Raw Conductivity Plausibility

Static:

- H2637 carries a high second-layer `ksat` of `330.2755 mm/h`
  (`9.174319444444445e-5 m/s`).
- No package-local evidence establishes a site-specific upper bound that would
  let openWEPP reject that `.sol` value.
- `SC-INFILE-SOIL-001` treats the `.sol` `ksat` value as an input hydraulic
  state, not as a value to silently cap.
- The baseline source lower-conductivity guard in `input.for` addresses
  near-zero conductivity, not high conductivity.

Plausibility conclusion:

- Do not classify H2637 as defective merely because the source soil has a high
  `ksat` layer.
- ADR-0024 source-intent authority applies to the transform from source layer
  to runtime layer, not to inventing a new empirical cap for H2637.

## Split-Layer Plausibility

Static:

The 400-600 mm runtime layer mixes:

- 160 mm from the `330.2755 mm/h` source layer; and
- 40 mm from the `33.0275 mm/h` source layer.

For vertical flow through serial layers, the source-intent harmonic value is
`117.955408163210 mm/h`, not the arithmetic value `270.8259 mm/h`.

This is also physically coherent: the lower-conductivity 40 mm segment should
materially throttle vertical percolation through the combined 200 mm layer.
Arithmetic averaging overstates that vertical pass-through by `2.296x`.

## Lateral Plausibility

Static + Ran:

- H2637 is a modern hourly lane: `solwpv=9002`, `lane_substeps=24`.
- `SC-SUBHYD-001` HPHYS0257 says modern hourly lateral conductivity is
  `ui_ssh`, layer-normalized from `ssc2 * ui_anisrt`.
- H2637 `ui_anisrt = 1.0`, so the hourly horizontal split-layer value is the
  arithmetic `270.8259 mm/h`.
- The `ksat_x0.9` probe reduced peak WAT `latqcc` by `6.32961016857 mm`
  (`-8.83726326408%`), confirming that the lateral magnitude is sensitive to
  the base conductivity surface.

Plausibility conclusion:

- The H2637 lateral magnitude remains large, but this package does not find an
  equation-level reason to reject the hourly `ui_ssh` surface for H2637.
- The broad base-conductivity lineage is still not defensible as correct until
  vertical `wb18_perc_ssc` is fixed and H2637 is rerun.

## Final Plausibility Judgment

The correct classification is not "H2637 soil input impossible" and not
"WB19 lateral equation defective." It is:

`OPENWEPP-DEFECTIVE`: vertical `wb18_perc_ssc_####` 200 mm normalization is
arithmetically averaged where source intent requires inverse-conductivity
normalization.
