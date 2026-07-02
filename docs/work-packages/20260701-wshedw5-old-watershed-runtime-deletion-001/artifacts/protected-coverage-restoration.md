# Protected Coverage Restoration

Status: `executed`

Evidence mode: `static + ran`

Deleted old-surface coverage was restored on the typed route rather than by
retaining symbol-map tests.

| Deleted coverage | Typed replacement |
| --- | --- |
| WS10 dispatch over `WatershedWritebackSurface`, finite channel/impoundment route, typed publication | `typed_frame_dispatch_records_and_publishes_direct_routed_state` |
| WS10 channel non-finite contributor guard | `typed_frame_dispatch_fails_closed_on_non_finite_hillslope_payload` |
| WS10/WS12 impoundment domain guard | `typed_frame_dispatch_fails_closed_on_impoundment_domain_violation` |
| WS11 `ipeak` branch vectors and closure | `typed_frame_dispatch_executes_ws11_ipeak_branches_with_closure` |
| WS18/WS20 transport-capacity anti-surrogate behavior | `typed_frame_channel_transport_capacity_responds_to_particle_diameter` |
| WS12 inactive-structure surrogate deauthorization | `typed_frame_dispatch_records_and_publishes_direct_routed_state` asserts zero inactive outflow |
| WS12 active projection / 15-family min-controller composition | `typed_frame_active_impoundment_matches_drop_spillway_min_controller_composition` |
| WS12 non-finite projection guard taxonomy | `typed_frame_impoundment_projection_preserves_non_finite_guard_class` |
| WS12 domain projection guard taxonomy | `typed_frame_impoundment_projection_preserves_domain_guard_class` |

Focused gate:

```text
cargo nextest run --test wshedw5_typed_watershed_runtime_contract
Summary: 8 tests run: 8 passed, 0 skipped
```
