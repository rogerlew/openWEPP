# Disposition

Status: complete

Evidence mode: static+ran

Final disposition: legitimate boundary `HOLD`.

Required closure statement:

- Close as corrected, validated non-defect, or legitimate boundary `HOLD`.
- If `HOLD`, first actionable handoff item must be "close defect `<id>`", not
  an inspection step.
- Do not mark complete while known invariant or contract violations remain.

Static:

- In-envelope WB18 percolation defect was corrected: WB18 now consumes
  published `wb12_infiltration` before optional WB14/WB12 recomputation.
- Final p7/p11/p18/p20 validations no longer fail with
  `HKERNEL-WB11-PERC-E-003`.
- Remaining WAT non-emission is outside the WBVAL05 correction authority
  envelope: all four targets now fail first at WB14 runoff with
  `HKERNEL-WB14-RUNOFF-E-003`.
- Temporary attribution evidence showed the upstream domain violation is
  `snow.runtime_swe=-0.006171157610042402`, below the required non-negative
  snow runtime storage domain.
- HOLD legitimacy: remaining correction requires snow/runoff authority, not
  percolation/deep-seepage authority. This is a declared legitimate HOLD
  boundary in `package.md`.

Defect-shaped handoff:

1. Close defect `WBVAL05-J95-HKERNEL-WB11-PERC-E-003` by opening a snow/runoff
   boundary closure for `HKERNEL-WB14-RUNOFF-E-003` on J-95
   `snow.runtime_swe=-0.006171157610042402` for p7/p11/p18/p20.
2. Required first evidence: reproduce final WB14 failure from
   `/tmp/wbval05_j95_perc_20260606T000000Z/final_status.tsv`, then trace the
   snow producer that emitted negative runtime SWE before runoff.

Ran:

- Final p7/p11/p18/p20 validation commands recorded in
  `wbval05-validation-ledger.md`.
