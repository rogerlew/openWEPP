# Worker Handoff

Status: complete

Evidence mode: Static

Next package: `HPHYS0317`

Carry-forward facts:

- HPHYS0316 closed contract/test/artifact obligations for the H1/H7/H39
  spring-2016 2013 terminal carry-recursion route.
- H1/H7/H39 spring-2016 carried rows remain:
  `2013-hourly-snowfall-input-surface-parity-hold` / ADR0017 `UNRESOLVED`.
- Total carried rows: `33`.
- H1 carried rows: `15`.
- H7 carried rows: `9`.
- H39 carried rows: `9`.
- The 2014 day 1 hour 1 deltas match the same hillslope 2013 terminal deltas:
  `0.013144251023522513/0.013144251023522124 m` for H1,
  `0.015279465660242741/0.015279465660242408 m` for H7, and
  `0.0147979087518893/0.014797908751889022 m` for H39.
- Baseline key value at the first material 2013 lane:
  `hrsnow = 0.0007454545120708644 m` at 2013 day 11 hour 11.
- OpenWEPP homologous value: `snow.hourly.snowfall_m_0011 = 0.0 m`.
- Production edit authorization: `false`.

HPHYS0317 should include both the `24` spring-2014 rows from HPHYS0315 and the
`33` spring-2016 inherited rows from HPHYS0316 when closing the paired
fixed-baseline/openWEPP input-surface parity route.

Do not:

- Reuse stale HPHYS0298 depth-vs-SWE `OPENWEPP-DEFECTIVE` labels.
- Treat inherited terminal carry as source-owned production proof.
- Treat source-code resemblance as input-surface parity proof.
- Add snow-drift, WB13, WB17, WB18, WB19, WB12, melt-term, or
  branch-predicate compensation while HPHYS0317 remains open.
- Close the route without ADR0017 same-unit/same-lineage proof and independent
  correctness authority.
