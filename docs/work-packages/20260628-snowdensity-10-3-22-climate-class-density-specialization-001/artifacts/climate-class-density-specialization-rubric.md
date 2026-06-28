# Climate-Class Density Specialization Rubric

Evidence class: `Static + Ran (completed outputs reused)`

## Summary

- Activated default: `15` robust fails, `179` robust score.
- Candidate: `16` robust fails, `168` robust score.
- Activation authorized: `false`.

## Gates

- `cross_snotel_primary`: `FAIL` - candidate robust profile 16/168 vs required better than 15/179; rerun default observed 15/179
- `bidirectional_densification_flip`: `FAIL` - bidirectional densification evidence incomplete; improvements=[]; regressions=['harvard_open:seasonal_densification_trajectory:-1']
- `persistence_guardrail`: `FAIL` - candidate worse robust cells vs activated default: 13
- `whole_model_conservation`: `PASS` - candidate trace rows=159986; max snow-state residual=4.440892098500626e-16; max partition residual=5.551115123125783e-17; tolerance=1e-09

## Protected Boundaries

- Production default, output schemas, fixtures, density cap, frost behavior, parser/runfile/user selectors, and site calibration are unchanged.

## Raw Outputs

- Output directory: `target/snowdensity10_3_22_climate_class_density_specialization`
- JSON artifact: `docs/work-packages/20260628-snowdensity-10-3-22-climate-class-density-specialization-001/artifacts/climate-class-density-specialization-rubric.json`
