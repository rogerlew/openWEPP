# Operand Lineage

Evidence mode: Static + Ran.

## Accepted Retained Operand Lineage

Static:

| Retained field | R6D source | Forbidden source avoided |
|---|---|---|
| `run_id` | `output_hillslope_id` converted to `u64` | WB13 rows, manifest JSON |
| `hillslope_id` | `output_hillslope_id` | WB13 rows, manifest JSON |
| `lane_id` / `ofe_id` | `lane_index + 1` checked into `u32` | PASS/WAT rows |
| `lane_index` | current OFE lane iteration | compatibility row order |
| `day_index` / `sim_day_index` | climate loop index checked into `i32` | WB13 `sim_day` |
| `calendar` | parsed `ClimateDayProjection` year/month/day/julian day | HBP/WAT/loss rows |
| `area_m2` | `per_ofe_lane_areas_m2` from parsed slope geometry | WB13 run volume scaling |
| `climate.precipitation_mm` | parsed `ClimateDayProjection.precipitation_mm` | runtime precipitation symbols |

## Missing Parity-Grade Families

Static:

R6D intentionally leaves the following direct publication families as zero or
absent until production direct producers exist:

- liquid input;
- runoff;
- evaporation;
- subsurface;
- upstream transfer;
- storage;
- profile capacity;
- interception;
- erosion/sediment;
- direct PASS projection authority;
- direct loss projection authority;
- direct manifest/checksum publication authority.

The cutover gate rejects this retained frame at
`HOLD-R6D-PARITY-GRADE-PUBLICATION-PRODUCERS-ABSENT`.
