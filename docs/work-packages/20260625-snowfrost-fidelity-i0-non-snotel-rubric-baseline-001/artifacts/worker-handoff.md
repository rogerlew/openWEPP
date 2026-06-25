# Worker Handoff

Current state: I0 is complete as a characterization baseline.

Keep:

- `tools/snowfreeze_observed/non_snotel_rubric_baseline.py`
- `artifacts/non_snotel_rubric_baseline.{json,md}`
- Package evidence under `target/snowfrost_fidelity_i0_non_snotel_rubric_baseline/`
  for local inspection.

Do not:

- Treat observation disagreement alone as `OPENWEPP-DEFECTIVE`.
- Start heat-flow, frozen-K/SFCC, impedance, `Qwet`, or migration/fringe
  remediation from this baseline.
- Tune snow parameters to residuals without contract authority.

Next recommended package:

`SNOWFROST-FIDELITY-I-SNOW-DEPTH-STRUCTURAL-REMEDIATION-001`

Objective:

- Use the H SNOTEL structural density-fork result plus this non-SNOTEL profile to
  remediate snow-depth producer/carry/input/settlement behavior.
- Preserve `INV-SNOWFREEZE-050` profile reporting as the before/after scoring
  surface.
- Rerun both SNOTEL and non-SNOTEL rubric baselines after any correction before
  frost physics attribution.
