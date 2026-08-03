# HF237 Observation Identity Audit

Status: `FAIL / PROVIDER_IDENTITY_CONTRADICTION`

Evidence mode: **Ran**

Under the frozen identity `density_kg_m3 = observed_swe_mm / depth_m`, none of
the complete nonzero HF237 rows closes within `1 kg m^-3`.

| Stratum | Complete rows | Closing | Nonclosing | Maximum absolute residual (kg m^-3) |
|---|---:|---:|---:|---:|
| open | 336 | 0 | 336 | 5,417.0 |
| hardwood | 410 | 0 | 410 | 4,296.727 |

The discrepancy is consistent with an unresolved scale/unit identity issue,
but this package does not normalize it by an inferred factor. Depth and density
remain independently usable; supplied SWE cannot carry a geometry, magnitude,
interception, or promotion claim until its provider identity is corrected.
