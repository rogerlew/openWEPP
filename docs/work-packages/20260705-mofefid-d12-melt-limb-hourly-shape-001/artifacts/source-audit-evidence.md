# Source Audit Evidence

Status: **COMPLETE**.

Static:

| Candidate | Classification | Disposition |
|---|---|---|
| `snow_liquid.raw_melt_m` / `melt_raw_m` | diagnostic only | Signed raw melt before daily redistribution; cannot carry routed-liquid magnitude alone. |
| `snow_liquid.redistributed_melt_m` | daily magnitude only | Source-authorized daily routed snowpack melt after redistribution, but no independent hourly vector. |
| `snow_liquid.routed_melt_m` | daily scalar authority | Consumed by WB12/WB14/WB13; D12 preserves this magnitude. |
| `snow.hourly.melt_m` after redistribution/rain release | source-authorized timing shape | `SC-RUNOFFPART-001#INV-RUNOFFPART-022` requires producer hourly melt shape while conserving daily routed scalar. |
| `snow.hourly_routed_melt_m[h]` | accepted D12 surface | Built by producer from hourly melt shape and daily `snow.routed_melt_m`, then hard-validated downstream. |
| Uniform spread across 24 hours | rejected as authority | Diagnostic fallback only for positive-runoff days with no authorized source profile. |

Ran:

- Producer allocation test proves hourly shape preservation with daily-scalar
  closure.
- Runtime R4G test rejects downstream hourly/daily nonclosure.
