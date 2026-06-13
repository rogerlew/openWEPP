# contract test implementation evidence

Status: M-D design complete; M-B contract tests implemented

Evidence mode: Ran + Static

## M-D

No contract tests were added in M-D. The design artifact defines the red-test
surface for M-E0:

- policy-transition tests must fail on the current aggregate WB13/WAT
  architecture,
- transfer identity tests must require sent/received hourly arrays by adjacent
  OFE pair,
- per-element identity tests must require OFE-local dynamic state,
- publication tests must reject single aggregate `OFE=1` rows for multi-OFE
  hillslopes under the new policy,
- single-OFE anchor tests must keep N=1 behavior identical or at-noise.

## M-C2

No contract tests were added in M-C2. The increment stopped at the
pre-implementation contract gate because current architecture has no real
per-OFE daily WB state surface to assert.

Permanent M-C2 tests should be added with the next implementation increment
after an authoritative per-OFE state design exists. The temporary red evidence
for this increment is the direct publication audit and local owcmp output:

- all 29 multi-OFE surfaces remain single-`OFE=1` on WAT output,
- downstream `UpStrmQ` remains zero on WAT output,
- `QOFE` remains aliased to `Q`,
- per-element and transfer identities remain unmeasurable on daily output
  surfaces.

Existing M-B tests were rerun and passed:

- `cargo test --test wb11_hydrology_kernel_contract mofe01_mb -- --nocapture`
- `cargo test -p openwepp-runner mofe01_mb_wb11_seed_purges_stale_daily_carryover_for_mofe_hourly_arrays -- --nocapture`

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
