# Verification Agent A

Status: `complete / PASS`

Evidence mode: `Static + Ran`

Verdict: PASS for truthful negative-result administrative closure. Scientific
disposition remains `HOLD / CLOSE_AS_MODEL_LIMITATION`.

Verified:

- exact 98-path inventory: 56 modified and 42 new;
- focused suite 13/13, formatting, and diff hygiene pass;
- consumer JSON, figures, sidecars, assurance state, readiness fields, and
  stop-loss narrative agree;
- dual review, exact-diff reconciliation, prompt archive, and exit criteria are
  complete; and
- EB-04 remains blocked with no clamp, fitted limiter, or calibration retry.

## Independent Bulk-Density Reproduction

Ran:

```bash
env \
  OPENWEPP_SNOWDENSITY09_DENSITY_MODEL=physics_bulk_density_compaction_v1 \
  OPENWEPP_PARADIGM2_STAGE3_LIQUID_MODEL=layered_thermal_liquid_v1 \
  OPENWEPP_SNOW_SURFACE_LONGWAVE_MODEL=disabled \
  OPENWEPP_SNOW_SURFACE_SUBLIMATION_MODEL=neutral_bulk_stage3_v1 \
  OPENWEPP_R7H_SNOW_TRACE_PATH=/home/workdir/openWEPP/target/snow_surface_eb03_verify_a_bulk_s/eb03-verify-a-bulk-S.snow.jsonl \
  /home/workdir/openWEPP/target/debug/openwepp-cli-hill \
  --run-dir /home/workdir/openWEPP/tests/fixtures/cancov_forest/hjandrews_conifer_or \
  --run-file /home/workdir/openWEPP/target/snow_surface_eb03_verify_a_bulk_s/eb03-verify-a-bulk-S.run \
  --output-dir /home/workdir/openWEPP/target/snow_surface_eb03_verify_a_bulk_s \
  --legacy-sidecar-discovery \
  --direct-production-executor
```

Result: exit 1 at typed `surface_temperature_k=0`; 18 trace rows, 12
active rows, `0.00140457118515381 m` cumulative sublimation,
`0.03601285123347629 m` final SWE, `522 kg m^-3` final density, and trace
SHA-256
`fa6e9ede41204c10ab327929f666c558d7927a29274a7e85e55d667cb786f2b8`.

This independently excludes multilayer-density geometry as the cause.
