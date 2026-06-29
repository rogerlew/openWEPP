# FROST STEP 3 Residue Parameterization Diagnostic

Evidence mode: Ran.

- Decision branch: `A` Residue parameterization is a partial contributor
- Justification: Seasonal Dec_* residue reduced candidate-defect timing cells from 18 to 13; residual cells remain for follow-up frost attribution.
- GAP-SNOWFREEZE-002: GAP-SNOWFREEZE-002 remains open. Seasonal residue is a confirmed partial contributor: it cleared 5 of 18 candidate-defect timing cells, while 13 cells remain for follow-up frost attribution.
- Step 2 analyzer: `docs/work-packages/20260629-frost-step2-sleepers-attribution-001/artifacts/attribute_sleepers.py`

## Entry Gate

- Fixture: `tests/fixtures/cancov_forest/hubbardbrook_deciduous_nh`
- Trace summary: `docs/work-packages/20260629-frost-step3-residue-parameterization-001/artifacts/entry_gate_residue_trace_summary.json`
- Monthly trajectory: `docs/work-packages/20260629-frost-step3-residue-parameterization-001/artifacts/entry_gate_residue_monthly_trajectory.csv`
- Seasonal: `True`
- Physically reasonable: `True`
- Residue depth min/max m: `0.0210945` / `0.197088`
- Autumn mean m: `0.165028`
- Spring mean m: `0.15991`
- Max monthly mean month: `10`

## A-vs-B Timing Comparison

| Site | Baseline candidate defects | Seasonal candidate defects | Delta |
| --- | ---: | ---: | ---: |
| `site1_sleepers_south_field_vt` | `4` | `2` | `-2` |
| `site2_sleepers_w9_hardwood_vt` | `14` | `11` | `-3` |

## site1_sleepers_south_field_vt

- Seasonal fixture: `target/frost_step3_residue_parameterization/fixtures/site1_sleepers_south_field_vt_seasonal_dec`
- Seasonal report: `docs/work-packages/20260629-frost-step3-residue-parameterization-001/artifacts/site_reports/site1_sleepers_south_field_vt.seasonal_dec.comparison_report.json`
- Seasonal trace: `docs/work-packages/20260629-frost-step3-residue-parameterization-001/artifacts/site_reports/site1_sleepers_south_field_vt.seasonal_dec.residue_trace_summary.json`
- Candidate defects baseline -> seasonal: `4` -> `2`

### Baseline Candidate Cells After Seasonal Run

| WY | Signature | Baseline residual | Seasonal residual | Seasonal attribution |
| ---: | --- | ---: | ---: | --- |
| `1986` | `thaw` | `19` | `19` | `candidate-frost-model-defect` |
| `1987` | `thaw` | `20` | `20` | `candidate-frost-model-defect` |
| `2007` | `thaw` | `17` | `0` | `agrees-within-tolerance` |
| `2015` | `thaw` | `20` | `0` | `agrees-within-tolerance` |

## site2_sleepers_w9_hardwood_vt

- Seasonal fixture: `target/frost_step3_residue_parameterization/fixtures/site2_sleepers_w9_hardwood_vt_seasonal_dec`
- Seasonal report: `docs/work-packages/20260629-frost-step3-residue-parameterization-001/artifacts/site_reports/site2_sleepers_w9_hardwood_vt.seasonal_dec.comparison_report.json`
- Seasonal trace: `docs/work-packages/20260629-frost-step3-residue-parameterization-001/artifacts/site_reports/site2_sleepers_w9_hardwood_vt.seasonal_dec.residue_trace_summary.json`
- Candidate defects baseline -> seasonal: `14` -> `11`

### Baseline Candidate Cells After Seasonal Run

| WY | Signature | Baseline residual | Seasonal residual | Seasonal attribution |
| ---: | --- | ---: | ---: | --- |
| `1994` | `onset` | `-22` | `12` | `agrees-within-tolerance` |
| `1994` | `thaw` | `35` | `35` | `candidate-frost-model-defect` |
| `1995` | `thaw` | `18` | `18` | `candidate-frost-model-defect` |
| `1996` | `onset` | `-19` | `-19` | `candidate-frost-model-defect` |
| `1996` | `thaw` | `39` | `39` | `candidate-frost-model-defect` |
| `1997` | `onset` | `-23` | `0` | `agrees-within-tolerance` |
| `1997` | `thaw` | `84` | `84` | `candidate-frost-model-defect` |
| `1998` | `onset` | `-48` | `-48` | `candidate-frost-model-defect` |
| `2004` | `thaw` | `41` | `41` | `candidate-frost-model-defect` |
| `2006` | `thaw` | `17` | `17` | `candidate-frost-model-defect` |
| `2009` | `thaw` | `111` | `111` | `candidate-frost-model-defect` |
| `2010` | `thaw` | `50` | `50` | `candidate-frost-model-defect` |
| `2011` | `thaw` | `20` | `20` | `candidate-frost-model-defect` |
| `2013` | `onset` | `-22` | `13` | `agrees-within-tolerance` |

## Step 4 Note

The Step 1 >0.25 systematic-timing-fraction cutoff is diagnostic-script-local; only TOLERANCE_DAYS=14 is inherited by this package.
