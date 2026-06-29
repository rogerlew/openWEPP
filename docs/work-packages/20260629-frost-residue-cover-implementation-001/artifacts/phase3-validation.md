# Phase 3 Validation

Evidence class: Ran.

Command:

```sh
.venv/bin/python docs/work-packages/20260629-frost-step3-residue-parameterization-001/artifacts/run_residue_parameterization.py --binary target/debug/openwepp-cli-hill
```

## Entry Gate

Source artifact:
`docs/work-packages/20260629-frost-step3-residue-parameterization-001/artifacts/entry_gate_residue_trace_summary.json`

- Seasonal residue depth: `true`.
- Physically reasonable: `true`.
- Residue depth min/max: `0.0210945 m` / `0.197088 m`.
- Autumn mean: `0.165028 m`.
- Spring mean: `0.159910 m`.
- Max monthly mean month: `10`.

## Sleepers A-vs-B Result

Source artifact:
`docs/work-packages/20260629-frost-step3-residue-parameterization-001/artifacts/residue_parameterization_diagnostic.json`

Decision branch: `A` residue parameterization is a partial contributor.

| Site | Baseline candidate defects | Seasonal candidate defects | Delta |
| --- | ---: | ---: | ---: |
| `site1_sleepers_south_field_vt` | 4 | 2 | -2 |
| `site2_sleepers_w9_hardwood_vt` | 14 | 11 | -3 |
| Total | 18 | 13 | -5 |

## Disposition

The seasonal residue trajectory exists and shrinks the Step 2 candidate-defect
set. Residual timing cells remain, so `GAP-SNOWFREEZE-002` stays open for later
frost attribution. The tested residue-lifecycle pointer is confirmed as a
partial contributor: it clears 5 of 18 candidate-defect cells, while 13 remain.
