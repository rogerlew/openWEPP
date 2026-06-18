# MAGPARITY01 INV-028 Closure Check

Evidence mode: **Ran** (openWEPP H2637 parquet + manifest checks) + **Static**
(`SC-RUNOFFPART-001`, `SC-WATBAL-001`).

## Contract Focus

`SC-RUNOFFPART-001#INV-RUNOFFPART-028` and
`SC-WATBAL-001#INV-WATBAL-096` require MOFE hourly carry to preserve:

- adjacent surface carry: downstream `UpStrmQ` equals upstream `QOFE`, scaled by
  `A_upstream / A_current`;
- adjacent lateral carry: downstream `SubRIn` equals upstream `latqcc`, scaled by
  the same area basis;
- per-element water closure:
  `local_liquid + UpStrmQ + SubRIn = infiltration + Q_partition + Δdepression + ε`;
- hillslope-total cancellation of internal inter-OFE transfers.

## Emitted Structural Checks

All 19 H2637 OFEs have the same published area, so
`A_upstream / A_current = 1.0` for every adjacent pair.

| Check | Max residual |
|---|---:|
| `UpStrmQ_i - QOFE_(i-1)` for OFE2-OFE19 | `2.27e-13 mm` |
| `SubRIn_i - latqcc_(i-1)` for OFE2-OFE19 | `0.0 mm` |
| `QOFE - Q * OFE` | `6.82e-13 mm` |
| PASS `runvol - Q_outlet * A_hillslope` | `5.46e-12 m3` |
| PASS `sbrunv - latqcc_OFE19 * A_outlet` | `0.0 m3` |

The `QOFE/Q` duality is sound: `QOFE` is local-length normalized, `Q` is
cumulative-length normalized, and the outlet area pairing reconstructs PASS
`runvol`.

## Manifest Closure Evidence

The H2637 run manifest records the internal per-OFE WB13 identity gates:

| Manifest field | Value |
|---|---:|
| `transfer_identity_max_abs_mm` | `0.0` |
| `per_element_identity_max_abs_mm` | `7.958078640513122e-13` |
| `aggregate_transfer_cancellation_max_abs_mm` | `0.0` |
| `hillslope_total_identity_max_abs_mm` | `1.623438148979705e-13` |
| `per_ofe_record_count` | `235,961` |
| `per_ofe_expected_record_count` | `235,961` |

The public WAT file does not expose standalone infiltration or depression
storage terms, so this artifact does not reconstruct the full event identity
from WAT aliases alone. The closure evidence is the in-runner manifest gate over
the per-OFE dynamic water-balance records, plus the independent adjacent-transfer
and export reconstructions above.

## Finding

`INV-RUNOFFPART-028` is **not** the source of the H2637 magnitude difference.
There is no evidence of under-absorbed run-on caused by stale aggregate carry,
collapsed component arrays, or area-ratio inflation.
