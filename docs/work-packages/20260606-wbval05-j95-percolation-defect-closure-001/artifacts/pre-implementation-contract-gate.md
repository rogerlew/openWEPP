# Pre-Implementation Contract Gate

Status: complete

Evidence mode: static+ran

Required gate:

- Contracts amended or explicitly confirmed sufficient.
- Contract-derived tests added and failing for the reproduced defect.
- Pre-fix validation evidence recorded.
- No production edits made before this gate.

Static:

- Gate authority existed only after `SC-PERC-001` v29 clarified published
  `wb12_infiltration` precedence for WB18 same-pass ingress.
- Pre-fix code path was inspected before production correction: WB18
  percolation called `compute_same_pass_wb14_infiltration_lineage` whenever
  `management.initial.params.tillay2_m` existed, so the code revalidated snow
  even when `wb12_infiltration=0` was already present.

Ran:

- Pre-fix repro of p7/p11/p18/p20 produced `HKERNEL-WB11-PERC-E-003`.
- Temporary local attribution print before correction showed the failing
  symbol was `snow.runtime_swe=-0.006171157610042402`.
