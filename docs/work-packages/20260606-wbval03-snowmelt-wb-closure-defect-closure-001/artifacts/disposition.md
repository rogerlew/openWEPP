# Disposition

Status: executed-hold

Evidence mode: mixed `Static:` and `Ran:`

Final disposition: legitimate `HOLD` at upstream climate source boundary.

Static:

- Historical WBVAL01 evidence still anchors the original WBVAL03 defects:
  - `HKERNEL-WB11-PERC-E-003` on `p7`, `p11`, `p18`, `p20` at J-95.
  - Complete-identity WAT residuals above `1.0 mm/year` for the 12 prior WAT
    emitters in years `2..6`.
- WBVAL03 completed the full identity audit. The omitted terms do not explain
  the residual.
- Current post-WBVAL02 execution cannot reach J-95 percolation or WAT
  publication for these targets because the shared DRIGGS climate source fails
  first at `CLIM-RUNTIME-E-017`, `radly=486`.

Ran:

- Release build passed.
- Current four J-95 target reruns all returned `CLIM-RUNTIME-E-017`,
  `radly=486`.
- Current 12 prior WAT-emitter reruns all returned `CLIM-RUNTIME-E-017`,
  `radly=486`.
- Saved WBVAL01 WAT parquet files were audited for the complete identity.

Closure:

- WBVAL03 is not complete as a correction package.
- WBVAL03 is legitimately held because the validation surface is blocked by
  upstream defect `WBVAL04-CLIMATE-RADLY-RAMAX-INPUT-BOUNDARY`.
- No WBVAL03 in-envelope, authority-backed, testable, measurable fix was
  identified and deferred.
