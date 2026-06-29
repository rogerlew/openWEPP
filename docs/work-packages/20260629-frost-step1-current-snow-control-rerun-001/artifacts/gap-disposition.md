# GAP-SNOWFREEZE-002 Disposition Input

Evidence mode: Static + Ran.

`GAP-SNOWFREEZE-002` remains open. This package does not perform frost
magnitude attribution or ratify `INV-SNOWFREEZE-047/048/050`.

## Site Routing

| Site | Route | Step 2 Status | Reason |
| --- | --- | --- | --- |
| `site1_sleepers_south_field_vt` | `FORCING-LIMITED` | Unblocked for frost timing; magnitude attribution must carry snow-depth forcing uncertainty | Paired scalar snow-depth failures remain, but cover agreement is `0.940`; snow timing failures are sparse (`2` checks failed), not systematic. |
| `site2_sleepers_w9_hardwood_vt` | `FORCING-LIMITED` | Unblocked for frost timing; magnitude attribution must carry snow-depth forcing uncertainty | Paired scalar snow-depth failures remain, but cover agreement is `0.959`; snow timing failures are non-systematic (`5` checks failed over the multi-year record). |
| `site3_scan_mandan_nd` | `INCONCLUSIVE-NO-PAIRED-SNOW` | Not eligible for snow-controlled attribution | The isotherm source has no paired observed snow-depth rows. |
| `site4_ggd498_morris_mn` | `BLOCKED` | Not eligible for frost attribution | The paired snow record shows systematic snow-cover timing/regime mismatch over the short overlap (`0.759` cover agreement; `5` snow timing failures). |
| `site5_reynolds_creek_us_rls_id` | `INCONCLUSIVE-NO-PAIRED-SNOW` | Not eligible for snow-controlled attribution | The isotherm source has no paired observed snow-depth rows. |

## Updated Harness Disposition

The Step 1 current-default rerun changes the standing frost-blocker from
"three paired sites fail snow control" to:

- two Sleepers frost-tube sites are available for Step 2 frost timing and
  uncertainty-carrying magnitude analysis;
- Morris remains a true snow-control blocker;
- Mandan and Reynolds Creek remain snow-control inconclusive because the corpus
  has no paired snow-depth observations.

No frost-model mechanism is authorized by this package.
