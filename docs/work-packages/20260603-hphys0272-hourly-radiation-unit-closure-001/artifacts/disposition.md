# Disposition

Status: completed/HOLD
Evidence mode: static + ran

Static:

- HPHYS0272 completed the declared radiation-unit correction.
- The corrected behavior is baseline-authoritative: daily `rad`/`radly` is
  Langleys/day and is converted exactly once to `radmj = radly * 0.04184`.
- No heuristic radiation clipping, snowmelt equation change, WB13 publication
  compensation, WB17 `Ep` edit, or aggregate-storage compensation was made.

Ran:

- H1 day-36 max hourly radiation: `59.258047 -> 2.388678 MJ m^-2 h^-1`.
- H1 day-36 candidate `RM`: `28.175296 -> 0.000000 mm`; baseline remains
  `0.000000 mm`.
- H1 day-36 `Snow-Water` diff: `-28.904465 -> -0.729168 mm`.
- H1/H7/H39 targeted traces all classify as
  `HOURLY_RADIATION_MJ_SCALE_CONFIRMED`.
- Full H1..H39 runtime: `39/39 rc=0`; semantic parity: `0/39`.

Decision: `completed/HOLD`. The radiation-unit defect is fixed. Continue on
remaining snowpack/ET/storage residuals using the new HPHYS0272 metrics; do not
reopen radiation unless a new trace shows MJ-unit violation.
