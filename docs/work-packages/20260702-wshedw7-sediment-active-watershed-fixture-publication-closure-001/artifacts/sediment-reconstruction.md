# Sediment Reconstruction

Status: `passed`

Evidence mode: `Ran:` HBP parser/public parquet reconstruction plus `Static:`
source-lineage review.

## HBP to Public Detachment/Deposition

The focused W7R test parses generated
`/tmp/wshedw7r_p102_fixture_jobs1/hillslope-jobs/H1/H1.hbp` with
`parse_hbp_from_path_with_latest_event_payload` and compares the payload to
public parquet output.

Release public output:

| Public field | Value |
| --- | ---: |
| `totalwatsed3.tdet` | `584.2332653870001 kg` |
| `totalwatsed3.tdep` | `282.14618621700004 kg` |
| `tdet - tdep` | `302.08707917000004 kg` |

The test proves:

- HBP `schema_major = 1`, `schema_minor = 1`
- `nofe = 2`
- `npart = 5`
- HBP hourly runoff and sediment arrays both have `24` slots
- `sum(hourly_sediment_mass_kg) == total_detachment_kg - total_deposition_kg`
- public `tdet` equals HBP `total_detachment_kg`
- public `tdep` equals HBP `total_deposition_kg`

## Routed Sediment Yield

Release public output:

| Public field | Value |
| --- | ---: |
| `ebe_pw0.sediment_yield` | `0.08391307754719238 kg` |
| `totalwatsed3.sed_del` | `0.08391307754719238 kg` |

The test proves `sed_del` matches typed routed sediment yield and is not
`tdet - tdep`; the difference from HBP exported mass is greater than `1 kg`.

## Rejected Aliases

- zero-fill: rejected because all accepted sediment fields above are nonzero.
- producer-only proof: rejected because public watershed parquets are checked.
- `tdet - tdep` as `sed_del`: rejected by the focused test.
- byte-level parquet identity only: rejected as sufficient evidence because
  metadata hashes can differ; decoded schema and rows are compared instead.
