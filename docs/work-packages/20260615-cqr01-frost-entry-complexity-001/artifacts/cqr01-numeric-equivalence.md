# CQR01 Numeric Equivalence

Status: complete

Evidence mode: static-and-ran

## Static

The production edit is a helper extraction only. Formula constants, thresholds,
state symbol names, typed error variants, unit assumptions, and publication
surface names were preserved.

Specific equivalence checks reviewed in the source diff:

- `legacy_tmpadj_surface_temperature_c` still receives the original snow
  density and `ksnowf` inputs.
- Freeze/thaw branch dispatch remains driven by the same `inv_snowfreeze`
  branch values.
- The thaw path still uses the depth snapshot from the start of the hour when
  calculating thaw-front recession.
- Fine-layer canonicalization and bottom-overflow routing remain ordered after
  the hourly freeze/thaw step.
- Final `frwatc` storage reconciliation still derives from the same layer and
  fine-layer storage operands.

## Ran

- Pre-refactor focused frost characterization:
  `cargo test --test clim06_frost_frozen_soil_kernel_contract`
  - exit_code: 0
  - result: `46 passed`
- Post-refactor focused frost characterization:
  `cargo test --test clim06_frost_frozen_soil_kernel_contract`
  - exit_code: 0
  - result: `46 passed`
- `cargo test --workspace`
  - exit_code: 0
