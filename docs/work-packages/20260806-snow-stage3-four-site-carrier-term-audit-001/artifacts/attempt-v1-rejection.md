# Attempt v1 Rejection

Status: `REJECTED BEFORE RESULT PRODUCTION`

Evidence mode: `Ran` at exact reviewed commit
`73ca62bd1df026401681d28d422deca79fd51859`.

The release build, four controls, and four paired lanes completed in 122 seconds.
All four control/paired WAT and HBP pairs are byte-identical. Analysis then
stopped on the first Mica Creek trace row, before any water-year statistic,
site summary, carrier screen, result JSON, execution receipt, or retained
manifest was written:

```text
same-state daily zero
stage3_evaluation_complete_arm_component_residual_j_m2
1986-01-01 residual -2.79396772e-09 exceeds 0.0
```

The defect is package-validator-only. The frozen protocol already assigned
producer reconstruction residuals the daily `1e-6 J m^-2` tolerance; the v1
code mistakenly grouped this applicable floating residual with exact-zero
same-state N/A fields. No cohort, forcing, support, aggregation, context,
tolerance, or screen changed. The v1 namespace is retained and must not be
rerun or admitted.

Protocol v2 removes only the component residual from the exact-zero family,
applies the already-frozen daily tolerance, adds acceptance/rejection unit
tests, and uses a fresh target namespace.
