# `ksatadj` OpenWEPP vs Source Intent

Package:
`20260618-refimpl-intent-authority-ksatadj-subhyd-001`

## Verdict

`OPENWEPP-DEFECTIVE`.

OpenWEPP mirrors the 9001/9002+/9003 branch formulas closely, but the current
`wb14` implementation does not form the source-intent saturation fraction. It
uses a storage-over-upper-limit aggregate instead of the legacy-intent
top-two-layer total-water over averaged porosity and rock correction.

## Source Intent

Static:

- `infpar.for:237-260` computes `avpor`, `avcpm`, `avsm15`,
  `avthetafc`, and `avthetadr` as top-two tillage-layer weighted averages.
- `infpar.for:293-296` computes `avsat` as storage plus residual water:
  `(st(1)+st(2))/tillay(2) + avsm15`, capped by `avpor * 0.98`.
- `infpar.for:606-621` applies the second cap against `avpor * avcpm` and then
  computes `sat_frac = avsat / (avpor * avcpm)`.
- `infpar.for:625-638` applies the 9002+ Saxton-Rawls exponent or 9001
  exponential recovery formula.

## OpenWEPP Match Points

Static:

- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/02_soil_slope.rs`
  projects `ksatadj`, `ksatfac`, `ksatrec`, and `lkeff` policy symbols.
- `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_lateral_drainage/02_ksat_adjustment.rs`
  implements:
  - the `ksatadj` flag guard,
  - the 9001 exponential branch,
  - the 9002+ exponent branch,
  - the 9003 `lkeff` lower floor, and
  - `mm h^-1`/`m s^-1` conversion.

These formula-level pieces match the extracted source intent.

## OpenWEPP Divergence

Static:

- `wb14_accumulate_ksatadj_layer` accumulates
  `theta_sum += theta.max(0.0)` and `ul_sum += ul`.
- `wb14_ksatadj_saturation_fraction` computes
  `sat_frac = theta_sum / ul_sum`.
- The function does not load or use `por_####`, `cpm_####`, `tillay(2)`,
  `avsm15`, or the source-intent `avpor * avcpm` denominator.
- It does not implement the two source-intent `avsat` caps:
  `avpor * 0.98` and `avpor * avcpm * 0.99`.

This is not an equivalent algebraic rewrite in general. The reference intent is
based on total water and an averaged porosity/rock-correction denominator; the
current OpenWEPP implementation is based on storage over upper-limit storage.
Those only coincide in restricted degenerate cases.

## Disposition

Static:

- `SC-SUBHYD-001#INV-SUBHYD-032` is the governing contract hook for the defect.
- No production Rust fix is in this package by design.
- The follow-on defect should replace or extend the `wb14` `ksatadj` operand
  lineage so it can compute the source-intent `sat_frac`, including non-aliased
  tests where the surrogate and intended formula differ.

Ran:

- No Rust tests were run for this artifact; no Rust code was changed.
