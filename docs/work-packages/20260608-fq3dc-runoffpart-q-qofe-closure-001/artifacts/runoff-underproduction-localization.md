# Runoff Underproduction Localization

Status: complete

Evidence mode: Static + Ran.

## Symptom

Ran: post-FQ1 outputs in `/tmp/fq1_after/outputs` reproduced near-zero runoff on
affected prefixes:

- `p8`: `sum(Q)=3.930232875259954e-15 mm`
- `p1`: `sum(Q)=3.0643619152587176e-13 mm`

Static: FQ-3 handoff/classification artifacts identify
`FQ3-DC-RUNOFFPART-QQOFE-001` as openWEPP `Q/QOFE` zero or underproduced on the
algebraic-radium single-OFE population, with legacy nonzero as a comparator flag.

## Event Localization

Ran: p8 trace on simulation year 1, Julian day 237 (`1990-08-25`) showed:

- daily rainfall input `24.6 mm`
- pre-fix WB14 same-pass infiltration `24.6 mm`
- pre-fix rainfall-excess residual/runoff `0.0 mm`
- top two WB18 layers at/near upper storage limit, with layer 2 exactly at `ul`

Static: legacy p8 WAT/EBE for the same day has material runoff (`Q ~= 20.4 mm`).
This is not acceptance by comparator magnitude; it proves the same forcing can
reach runoff and that openWEPP was over-absorbing event liquid.

## Mechanism

Static: WB12/WB14 computed coupled infiltration from intensity/capacity but did
not enforce the `SC-RUNOFFPART-001` top-two-layer upper-storage condition before
publishing same-pass infiltration. During the first implementation pass, runoff
was produced but annual WAT closure broke because WB14 recomputed infiltration
after ET/lateral mutation instead of consuming the WB18/percolation-produced
same-pass infiltration that had already updated storage.

Corrected mechanism:

- cap same-pass infiltration by top-two-layer available storage before storage
  ingress
- when WB18/percolation has already produced same-pass `wb12_infiltration`, WB14
  runoff consumes that producer value rather than recomputing from a later state

## Ownership

Static: root cause is inside the declared runoff partition / same-pass
infiltration envelope. No climate disaggregation, WAT publication, snow
magnitude, annual-crop ET, or MOFE routing edit was required.
