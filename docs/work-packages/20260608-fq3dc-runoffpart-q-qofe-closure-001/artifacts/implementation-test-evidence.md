# Implementation Test Evidence

Status: complete

Evidence mode: Static + Ran.

## Implementation

Static: production changes are limited to the declared WB12/WB14/WB18 seam:

- `03_kernel_support_00_support_helpers.rs`
  - resolves top-two-layer storage availability from `wb18_perc_theta_*` and
    `wb18_perc_ul_*`
  - caps computed same-pass infiltration by that available storage
  - resolves WB18/percolation-published same-pass infiltration only when the
    percolation producer path has run
- `03_kernel_support_01_kernel_phases.rs`
  - applies the storage cap when percolation reconstructs same-pass infiltration
  - makes WB14 runoff consume producer-published same-pass infiltration when
    present, otherwise computes/caps from the hyetograph

No runner publication, annual-crop ET, snow magnitude, or MOFE code was edited.

## Test Evidence

Ran:

- `cargo test --test wb14_infiltration_hyetograph_kernel_contract -- --nocapture`
  - result: `13 passed`
- `cargo build -p openwepp-runner --bin openwepp-cli-hill`
  - result: passed

Ran: first population validation attempt produced runoff but broke annual closure
(`max_abs=242.69382156404856 mm`). That finding was accepted and fixed by
making WB14 consume WB18/percolation-published same-pass infiltration.
