# Contract-Test Implementation Evidence

Status: `executed`

Evidence mode: `static + ran`

No contract amendment was required, but deleted old-surface contract tests were
replaced with typed-route tests derived from existing contract vectors:

- `SC-ROUTE-001` WS11 ipeak branch and closure vectors:
  `typed_frame_dispatch_executes_ws11_ipeak_branches_with_closure`.
- `SC-ROUTE-001` sediment capacity anti-surrogate vector:
  `typed_frame_channel_transport_capacity_responds_to_particle_diameter`.
- `SC-IMPOUND-001` WS12 nominal/inactive and surrogate-deauthorization vector:
  `typed_frame_dispatch_records_and_publishes_direct_routed_state`.
- `SC-IMPOUND-001` active projection/min-controller vector:
  `typed_frame_active_impoundment_matches_drop_spillway_min_controller_composition`.
- `SC-IMPOUND-001` non-finite/domain guard taxonomy:
  `typed_frame_impoundment_projection_preserves_non_finite_guard_class` and
  `typed_frame_impoundment_projection_preserves_domain_guard_class`.

Focused result: `8 passed`.
