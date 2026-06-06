# Worker Handoff

Status: complete

Evidence mode: Static

Next package: `HPHYS0317`

Carry-forward facts:

- HPHYS0315 closed contract/test/artifact obligations for the H1/H7/H39
  spring-2014 hourly snowfall input-lineage route.
- H1/H7/H39 spring-2014 carried rows remain:
  `forcing-input-surface-parity-hold` / ADR0017 `UNRESOLVED`.
- Total carried rows: `24`.
- Baseline key value: `hrsnow = 0.0007454545120708644 m` at 2013 day 11 hour
  11.
- OpenWEPP homologous value: `snow.hourly.snowfall_m_0011 = 0.0 m`.
- Delta: `-0.0007454545120708644 m`.
- Production edit authorization: `false`.

HPHYS0317 should capture or reconstruct paired fixed-baseline/openWEPP values
for `rain`, `stmdur`, `wntdur`, `wnttim`, `hrtemp`, `rst`, `hrsnow`, `hrrain`,
active interval, and branch choice at the same year/day/hour/trace lane.

Do not:

- Reuse stale HPHYS0298 depth-vs-SWE `OPENWEPP-DEFECTIVE` labels.
- Treat source-code resemblance as input-surface parity proof.
- Add snow-drift, WB13, WB17, WB18, WB19, WB12, melt-term, or
  branch-predicate compensation while HPHYS0317 remains open.
- Close the route without ADR0017 same-unit/same-lineage proof and independent
  correctness authority.
