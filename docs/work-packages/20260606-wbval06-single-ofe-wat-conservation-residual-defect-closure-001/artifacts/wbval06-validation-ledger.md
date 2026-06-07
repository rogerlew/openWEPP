# WBVAL06 Validation Ledger

Status: corrected

Evidence mode: executed

Purpose: record before/after validation for the current WBVAL04 WAT-emitter
population.

Validation target:

- Current emitted WAT population: `p1` through `p22`.
- Years classified: `2..6`.
- Acceptance: `abs(R_with_interception) <= 1.0 mm/year`.

| Hillslope | Years classified | Max abs R mm | Classification | Evidence |
|---|---:|---:|---|---|
| p1 | 5 | 0.000000904 | corrected | prefix summary |
| p2 | 5 | 0.000000937 | corrected | prefix summary |
| p3 | 5 | 0.000000977 | corrected | prefix summary |
| p4 | 5 | 0.000000129 | corrected | prefix summary |
| p5 | 5 | 0.000001037 | corrected | prefix summary |
| p6 | 5 | 0.000001033 | corrected | prefix summary |
| p7 | 5 | 0.000000000010 | corrected | prefix summary |
| p8 | 5 | 0.000000970 | corrected | prefix summary |
| p9 | 5 | 0.000000135 | corrected | prefix summary |
| p10 | 5 | 0.000000000003 | corrected | prefix summary |
| p11 | 5 | 0.000000000005 | corrected | prefix summary |
| p12 | 5 | 0.000000000003 | corrected | prefix summary |
| p13 | 5 | 0.000000904 | corrected | prefix summary |
| p14 | 5 | 0.000000942 | corrected | prefix summary |
| p15 | 5 | 0.000000000004 | corrected | prefix summary |
| p16 | 5 | 0.000000764 | corrected | prefix summary |
| p17 | 5 | 0.000000000003 | corrected | prefix summary |
| p18 | 5 | 0.000000000010 | corrected | prefix summary |
| p19 | 5 | 0.000000000002 | corrected | prefix summary |
| p20 | 5 | 0.000000000008 | corrected | prefix summary |
| p21 | 5 | 0.000000000003 | corrected | prefix summary |
| p22 | 5 | 0.000000000002 | corrected | prefix summary |

Static:

- Corrected schema version is `1.3` and includes `Interception`.

Ran:

- Release binary rerun emitted `22` WAT parquet files under
  `/tmp/wbval06_interception_after_20260607T000000Z/outputs/`.
- Rollup: `wat_emitters=22`, `clean_with_interception=22`,
  `break_with_interception=0`.
- Maximum corrected annual residual: `1.0364184390709852e-06 mm` (`p5`).
