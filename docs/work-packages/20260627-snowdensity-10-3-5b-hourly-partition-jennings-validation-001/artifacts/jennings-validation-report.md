# Jennings Phase Validation Report

Status: complete
Evidence mode: Ran

## Inputs

- Observations: `tests/fixtures/precip_phase_observed/jennings2018/jennings_et_al_2018_file2_ppt_phase_met_observations.csv`
- Thresholds: `tests/fixtures/precip_phase_observed/jennings2018/jennings_et_al_2018_file3_temp50_observed_by_station.csv`
- Rows read: `17810805`
- Rows scored: `11711058`
- Rows skipped: `6099747`
- Stations scored: `6883`
- RH values normalized to saturation: `0`

## Scores

| Model | Accuracy | RR | RS | SR | SS |
|---|---:|---:|---:|---:|---:|
| `harder_pomeroy_hourly` | 0.903141 | 3396066 | 498328 | 635996 | 7180668 |
| `legacy_rst_0c` | 0.858331 | 3574653 | 319741 | 1339353 | 6477311 |

## Threshold Summary

- Station count: `6883`
- Mean predicted temp50 C: `1.527938`
- Mean observed temp50 C: `0.973472`
- Mean bias C: `0.554467`
- Mean absolute error C: `0.944264`
- Max absolute error C: `8.478333`

## Humidity Contrast

- Low-RH station count: `688`
- High-RH station count: `688`
- Observed high-minus-low temp50 C: `-0.883105`
- Predicted high-minus-low temp50 C: `-0.770058`
