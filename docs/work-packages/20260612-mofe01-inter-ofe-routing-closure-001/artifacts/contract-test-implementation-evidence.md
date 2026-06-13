# contract test implementation evidence

Status: M-C held; M-B contract tests implemented

Evidence mode: Ran + Static

## M-C

No contract tests were added in M-C. The direct publication audit served as the
red-test execution surface and failed before production edits:

- multi-OFE row cardinality remains single-row aggregate,
- H1 day 1 emits only `OFE=1`,
- downstream handoff terms cannot be observed on WAT rows,
- `QOFE` remains aliased to `Q`.

Adding permanent failing tests without an implementable per-OFE state surface
would leave the workspace red. The next increment should add contract tests once
the per-OFE publication design is authoritative.

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
