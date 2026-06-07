# Verification Agent A

Status: complete

Evidence mode: Static + Ran.

## Verification

Static:

- Confirmed annual pre-plant filtering now preserves the PL activation sentinel
  for later days.
- Confirmed scheduler calendar projection now uses Julian day for the scheduler
  `day` symbol.
- Confirmed WB15 accepts finite non-negative plant `vdmt` and caps only the
  interception equation biomass input.

Ran:

- `cargo test -p openwepp-runner fq3dc_ -- --nocapture`: passed.
- p8 Corn spot run: nonzero `Ep=1938.103398211248` and
  `Interception=615.0133788383012`.
- p1 perennial spot run: nonzero `Ep=5475.201811235968` and
  `Interception=643.3614332068395`.

Result: verified.
