# Erod13 core kernel invariant coverage map

Status: completed
Evidence mode: mixed

## Static
- `INV-SED-001` continuity (`dGdx = Df + Di`):
  - runtime check in `run_erod13_wave1_core` continuity residual guard.
  - covered by test `erod13_contract_vector_rejects_continuity_residual_violation`.
- `INV-SED-002` detachment branch (`tau_f > taucn` and `G < Tc`):
  - runtime branch computes `Dc` and positive `Df`.
  - covered by `erod13_contract_vector_nominal_detachment_emits_core_outputs`.
- `INV-SED-003` deposition branch (`G > Tc`, `q > 0`):
  - runtime branch computes negative `Df` with typed domain guard on `q`.
  - covered by `erod13_contract_vector_deposition_branch_emits_negative_df`.
- `INV-SED-004` hydrologic input continuity (`Q`, `peakro`, `watdur`, `Ie`, `te`):
  - runtime gate checks positivity/finite domains and `watdur ~= Q/peakro`.
  - covered by nominal vectors and continuity-violation guard vector.
- `INV-SED-005` shear-partition (`fs/ft`, `tau_f`):
  - runtime domain guards enforce `ft > 0`, `0 <= fs <= ft`, finite/non-negative `taufe`.
  - covered by nominal and domain-failure vectors.
- `INV-SED-006` transport capacity (`tcadjf >= 0.30`, `Tc` finite/non-negative):
  - runtime guards on `tcadjf`, `erod13_tc_k`, `erod13_tc_m`, and computed `Tc`.
  - covered by `erod13_contract_vector_rejects_domain_violation`.
- `INV-SED-007` normalized parameters (`eta`, `taucn`, `theta`, `phi`):
  - runtime computes and domain-validates normalized outputs.
  - covered by nominal vector and non-finite/missing symbol guard vectors.

## Ran
- `cargo test --test erod13_wave1_core_kernel_contract` passed all 7 contract-derived vectors.
