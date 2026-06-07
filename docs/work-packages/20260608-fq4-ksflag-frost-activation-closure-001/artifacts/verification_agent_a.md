# Verification Agent A

Status: complete

Evidence mode: Static + Ran.

## Verification

Static:

- Confirmed `resolve_active_frost_coupling` validates `frost_file_present` as a
  binary provenance flag when present but no longer requires it for activation.
- Confirmed activation now depends on `frost.options.wintRed`.
- Confirmed no snow magnitude or `ksatadj` code was edited.

Ran:

- `cargo test --test clim06_frost_frozen_soil_kernel_contract -- --nocapture`:
  passed.
- p8 post-fix run: `frsoil.active=true`, `dfrost=0.2`, `frozwt` nonzero on
  `1017` days.

Result: verified.
