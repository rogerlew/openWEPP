# Conservation / Output Lineage

Status: **COMPLETE** (Static + Ran).

## Identities

- Water shape: `w_h` is a fraction.
- Positive-runoff candidate routed shape: `sum_h w_h = 1.0` within
  `1.0e-9`, every `w_h` finite and non-negative.
- No-runoff candidate routed shape: `sum_h w_h = 0.0` within `1.0e-12`,
  every `w_h` finite and non-negative.
- HBP water surface: `V_h = runvol_m3 * w_h`; `sum_h V_h = runvol_m3`.
- Sediment surface: `sum_h S_h = exported_sediment_kg` to the existing
  `SC-SED-001#INV-SED-014` f64-rounding closure.

## Output Lineage

The D13 implementation target is upstream of the existing publication writer:
`DirectPublicationErosionOperands.hourly_runoff_fraction` receives the selected
shape, and the runner already forms HBP minor-1 `V_h` and `S_h` from that
paired surface.

## Evidence

- Focused routed-shape selection and invalid-surface tests passed.
- Adjacent Wave-1 continuity suite passed.
- H2637 final-code ignored integration gate passed with shadow-on/off byte
  identity and preserved D12 uniform-shape counts.
- Gate command output is summarized in `gate-results.md`.
