# Erod14 conservation vector evidence

Status: completed
Evidence mode: mixed

## Static
- Runtime conservation controls implemented:
  - class-wise cap: `gend_i <= sedmax_i` enforced after iterative reproportioning,
  - normalization: `sum_i(sed_frac_i) ~= 1.0` when `sumg > 0`,
  - explicit domain failure when reproportioning has no feasible mass carrier (`ratbot <= 0`).
- Enrichment export computed as:
  - `ER = (sum_i(sed_frac_i * ssa_class_i) / ssa_soil) + 0.005`.

## Ran
- `cargo test --test erod14_wave2_multiofe_enrichment_kernel_contract`:
  - nominal vector passed with finite `erod14_sumg`, finite positive `ER`, and normalized `sed_frac_*`.
  - case-four/zero-outflow vector passed with zero `sed_frac_*`.
  - unreproportionable-mass vector failed with `HKERNEL-EROD14-WAVE2-E-003` as expected.
