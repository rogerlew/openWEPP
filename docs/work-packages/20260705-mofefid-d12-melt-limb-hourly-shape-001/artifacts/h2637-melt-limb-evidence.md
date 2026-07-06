# H2637 Melt-Limb Evidence

Status: **COMPLETE**.

Ran:

- Command:
  `cargo test --test laned_shadow_h2637 h2637_native_shadow_classifies_uniform_shape_after_d12 -- --ignored --nocapture`
- Result: PASS, 324.83 s.
- Manifest path: `/tmp/laned_shadow_h2637_native_on_10668/manifest.json`.
- Comparator worker: Popper (`comparator_suite_runner`), no files edited.

Metrics:

| Metric | Value |
|---|---:|
| `days_seen` | 731 |
| `days_routed` | 622 |
| `days_uniform_shape` | 6 |
| `days_uniform_shape_with_routed_melt` | 0 |
| `days_uniform_shape_without_routed_melt` | 6 |
| `max_supply_reconstruction_rel` | `5.434281268840262e-16` |
| `total_source_m3` | `1769606.816247753` |
| `total_routed_outlet_m3` | `1678721.4446571462` |
| `aggregate_router_conservation_rel` | `0.08236358856103747` |
| `max_router_conservation_rel` | `0.5049051203739849` |
| `transfer_identity_status` | `pass-published-per-ofe-wb13-records` |
| `per_element_identity_status` | `pass-published-per-ofe-wb13-records` |
| `aggregate_identity_status` | `pass-published-per-ofe-wb13-records` |
| `transfer_identity_max_abs_mm` | `0.0` |
| `per_element_identity_max_abs_mm` | `0.0` |
| `hillslope_total_identity_max_abs_mm` | `0.0` |

Disposition:

- Before D12/D11 evidence, H2637 recorded `10/731` uniform-shape days in the
  old two-limb source-shape path.
- After D12, no uniform fallback day has routed melt.
- The remaining `6` days are the no-authorized-source-shape residual class:
  no WB14 excess, no saturation carry, and no routed-melt limb. Uniform fallback
  remains diagnostic-only and cannot support activation.
- Protected HBP and pass parquet bytes are identical between native shadow-off
  and native shadow-on runs.
