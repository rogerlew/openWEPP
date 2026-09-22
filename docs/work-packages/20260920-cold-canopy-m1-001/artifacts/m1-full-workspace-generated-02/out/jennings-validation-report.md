# Jennings Phase Validation Report

Status: complete
Evidence mode: Ran

## Inputs

- Observations: `target/openwepp_snowbench_cli_tests/jennings_minimal/file2.csv`
- Thresholds: `target/openwepp_snowbench_cli_tests/jennings_minimal/file3.csv`
- Rows read: `2`
- Rows scored: `2`
- Rows skipped: `0`
- Stations scored: `1`
- RH values normalized to saturation: `0`

## Scores

| Model | Accuracy | RR | RS | SR | SS |
|---|---:|---:|---:|---:|---:|
| `harder_pomeroy_hourly` | 1.000000 | 1 | 0 | 0 | 1 |
| `legacy_rst_0c` | 1.000000 | 1 | 0 | 0 | 1 |

## Threshold Summary

- Station count: `1`
- Mean predicted temp50 C: `1.000000`
- Mean observed temp50 C: `0.000000`
- Mean bias C: `1.000000`
- Mean absolute error C: `1.000000`
- Max absolute error C: `1.000000`

## Humidity Contrast

- Low-RH station count: `1`
- High-RH station count: `1`
- Observed high-minus-low temp50 C: `0.000000`
- Predicted high-minus-low temp50 C: `0.000000`
