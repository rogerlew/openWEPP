# Frost Activation Localization

Status: complete

Evidence mode: Static + Ran.

## FQ-2 Ledger Fix

Static: FROSTVAL01's `frost-break` classification was a validation-ledger
artifact. The old ledger summed flux terms over a tiny day subset while applying
full-scale storage deltas. This package withdraws that verdict and uses the
full WAT complete identity with `Interception` and `SoilWaterTotal`
(`Total-Soil + frozwt`) for closure-under-frost.

## Symptom Reproduction

Ran: pre-fix p8 diagnostic under `/tmp/fq4_pre`.

- Manifest `winter.active=true`.
- Manifest `frsoil.wint_red_enabled=true`.
- Manifest `frsoil.frost_file_present=false`.
- Manifest `frsoil.active=false`.
- WAT `frozwt`: sum `0`, max `0`, nonzero days `0`.
- p8 climate has repeated cold days, including January-February mean
  temperatures below `0 degC`.

## Root Cause

Static: `resolve_active_frost_coupling` returned inactive whenever
`frost.options.frost_file_present` was absent or `0`, before checking
`frost.options.wintRed`. The algebraic-radium TOML runfiles do not provide an
explicit `frost.txt` or inline frost block, so the parser supplies valid
missing-file default controls (`wintRed=1`, default kfactors) but marks
`frost_file_present=false`. That provenance flag was incorrectly promoted into
an activation gate.

This is in-envelope:

- It is a standard frost activation/control handoff defect.
- Temperature forcing was cold and winter processing was active.
- The kernel's synthetic CLIM06 frost vectors already showed the frost routine
  can produce depth, frozen water, and conductivity reduction when activated.
- No snow magnitude, runoff partition, ET, p11 percolation, or MOFE mechanism
  was needed.

## Ownership

Ownership is openWEPP frost-control runtime activation. `SC-SNOWFREEZE-001`
already held analogous snow-sidecar presence guidance; v53 extends that posture
to defaulted frost controls. Frost file presence is provenance only. Standard
`wintRed=1` controls plus active thermal/runtime triggers activate `frsoil`.

## Paired On/Off Evidence

Ran: p8 paired run under `/tmp/fq4_pair`.

- Frost-on default controls: `sum(frozwt)=28902.293333333757 mm-day`,
  `max(frozwt)=30.399999999999995 mm`.
- Frost-off inline `wintRed=0`: `sum(frozwt)=0`, `max(frozwt)=0`.
- Output deltas, on minus off:
  - `Q`: `+393.2886145503722 mm`
  - `Dp`: `0.0 mm`
  - `latqcc`: `-62.19133256957724 mm`

The gate is no longer numerically identical.
