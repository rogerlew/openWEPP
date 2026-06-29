# Paradigm 2 Stage 1 Layered Density Rubric

Evidence class: `Static + Ran`

## Summary

- Activated default: `15` robust fails, `179` robust score.
- Candidate: `16` robust fails, `177` robust score.
- Activation authorized: `false`.
- Real-run elapsed seconds: `712.821`.

## Gates

- `cross_snotel_primary`: `FAIL` - candidate robust profile 16/177 vs required better than 15/179; rerun default observed 15/179
- `bidirectional_densification_and_persistence`: `FAIL` - bidirectional evidence incomplete; improvements=[]; regressions=[]; worse robust cells=6
- `whole_model_conservation`: `PASS` - candidate trace rows=159986; max snow-state residual=8.881784197001252e-16; max partition residual=5.551115123125783e-17; tolerance=1e-09
- `layer_persistence_and_closure`: `PASS` - layer rows=65459/159986; max layer SWE residual=4.440892098500626e-16; max layer depth residual=0.0; tolerance=1e-09

## Protected Boundaries

- Production default, output schemas, fixtures, density cap, frost behavior, parser/runfile/user selectors, and site calibration are unchanged.

## Raw Outputs

- Output directory: `target/paradigm2_stage1_layered_density`
- JSON artifact: `docs/work-packages/20260628-paradigm-2-stage-1-layered-snow-density-001/artifacts/paradigm2-stage1-layered-density-rubric.json`
