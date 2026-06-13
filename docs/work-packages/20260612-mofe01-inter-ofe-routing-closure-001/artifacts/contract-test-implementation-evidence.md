# contract test implementation evidence

Status: M-B contract tests implemented

Evidence mode: Ran + Static

M-B added `tests/integration/mofe01_inter_ofe_route_contract.rs` and registered it as an explicit test target. It asserts that `SC-RUNOFFPART-001`, `SC-WATBAL-001`, and the science-contract index carry the M-B route-closure authority.

## Ran

- `cargo test --test mofe01_inter_ofe_route_contract --test wb11_hydrology_kernel_contract --test wb14_infiltration_hyetograph_kernel_contract mofe01_mb -- --nocapture`
  - PASS.
- `cargo test --workspace`
  - PASS.

## Covered M-B contract behaviors

- Multi-OFE upstream carry preserves separated surface runoff and lateral runon components.
- Downstream `SubRIn` receives upstream lateral flow.
- Positive top-layer saturation excess routes into the current saturation carry before WB14 runoff reconciliation.
- Stale aggregate daily `wb12_runoff_carryover` is purged before MOFE hourly-array execution.
