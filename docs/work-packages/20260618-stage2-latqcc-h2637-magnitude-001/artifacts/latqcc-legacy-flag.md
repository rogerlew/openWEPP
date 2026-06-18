# H2637 `latqcc` Legacy Flag

Package:
`20260618-stage2-latqcc-h2637-magnitude-001`

## Evidence

Static:

- ADR-0017 makes the comparator a flag, not a target.
- MAGPARITY01 established the H2637 like-for-like decomposition:
  openWEPP no-UI `runvol = 14,085,670 m3`, OFE19 `sbrunv = 884,950 m3`,
  combined export `14,970,620 m3`; legacy no-UI `runvol = 11,011,152 m3`,
  OFE19 `latqcc = 3,313,841 m3`, combined export `14,324,993 m3`.
- Legacy with-UI is non-conserving and disqualified as a target
  (`127.7%`/`152.6%` of precipitation).

Ran:

- The diagnostic H2637 run reproduced the openWEPP WAT checksum
  `c70af52324b52c89119e57524f75bf4875d2c6a9ff83fe56d239a22082b9b474`.
- PASS numeric totals matched MAGPARITY01:
  `runvol = 14,085,670.078744758 m3`,
  `sbrunv = 884,949.9416133772 m3`.

## Dimensional Consistency

The like-for-like openWEPP lateral conversion is:

```text
latqcc_m3 = latqcc_mm / 1000 * OFE_area_m2
```

MAGPARITY01 geometry:

- 19 OFEs
- `10,869.593 m2` per OFE
- total hillslope area `206,522.267 m2`

PASS `runvol` and `sbrunv` are already volume terms in `m3`. WAT `latqcc` is a
depth in `mm` over the reporting OFE area. The comparison therefore uses
volume-to-volume terms after area scaling; it does not compare a depth to a
volume.

## Like-for-Like Flag

| Run | Outlet `runvol` m3 | OFE19 lateral egress m3 | Combined m3 | Combined % precip |
| --- | ---: | ---: | ---: | ---: |
| openWEPP no-UI | `14,085,670` | `884,950` | `14,970,620` | `75.5%` |
| legacy no-UI bounded comparator | `11,011,152` | `3,313,841` | `14,324,993` | `72.2%` |

The comparator still flags a partition difference:

- openWEPP routes much more lateral water to the outlet as `runvol`.
- legacy leaves more lateral water as terminal OFE19 lateral egress in the
  bounded no-UI comparison.
- combined export is close enough to support the MAGPARITY01 conclusion:
  this is partition/magnitude, not gross mass creation.

## Legacy Flag Verdict

`UNRESOLVED` as a comparator flag, not a defect target. Legacy does not prove
openWEPP wrong, and this package found no WB19 equation or operand-bound defect
that would justify forcing openWEPP toward the no-UI legacy partition.
