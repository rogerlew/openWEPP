# Implementation/Test Evidence

Status: completed

Evidence mode: static + ran

Static:

- Parsed `pmetpara` is now carried into hillslope runtime setup rather than
  parsed and discarded.
- Legacy sidecar-discovery mode projects PMET mode and selected crop
  coefficients from discovered `pmetpara.txt`; runfile override mode preserves
  explicit override semantics.
- HPHYS trace schema advanced to
  `openwepp-hphys0245-wb11-wb18-wb19-wb17-pmet-branch-trace-v6`.
- Trace rows now emit PMET sidecar presence, `iflget`, selected `kcb/rawp`,
  selected line index, lookup fallback status, WB11 ET demand, and actual seed
  branch.
- Current production ET demand remains truthfully labeled
  `evap_priestley_taylor`; this package did not add proxy `evappm` physics.

Ran:

- H1/H7/H39 classification: all three select PMET/`evappm` by sidecar
  contract (`iflget=2`, `kcb=0.95`, `rawp=0.80`) but still seed WB11 demand
  from the Priestley-Taylor branch.
- H1/H7/H39 day-1 `Ep` residual remains baseline `0.150000 mm` vs candidate
  `0.385294 mm`, diff `+0.235294 mm`.
- Full H1..H39 semantic suite was rerun; semantic pass remains `0/39`.
