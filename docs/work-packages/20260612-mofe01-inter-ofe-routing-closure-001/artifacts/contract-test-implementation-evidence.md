# contract test implementation evidence

Status: M-E3 dynamic persistence tests green

Evidence mode: Ran + Static

## M-E3

M-E3 adds focused persistence tests in
`crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/writeback.rs`:

- `mofe01_me3_persistent_sequence_carries_lane_state_across_days_without_bleed`,
- `mofe01_me3_persistent_sequence_keeps_prior_state_when_day_fails`,
- `mofe01_me3_persistent_sequence_rejects_nonsequential_initial_state`.

Ran:

- `cargo test -p openwepp-hillslope-orchestrator mofe01_me3 -- --nocapture`
  - PASS; 3 passed.
- `cargo test --test mofe01_per_ofe_state_contract -- --nocapture`
  - PASS; 4 passed.

## M-E2

M-E2 adds focused contract-derived executor tests in
`crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/writeback.rs`:

- `mofe01_me2_sequential_executor_carries_first_ofe_arrays_to_second_lane`,
- `mofe01_me2_sequential_executor_applies_downstream_area_ratio`,
- `mofe01_me2_sequential_executor_rejects_stale_current_output_arrays`,
- `mofe01_me2_sequential_executor_rejects_malformed_transfer_arrays`,
- `mofe01_me2_sequential_executor_rejects_transfer_total_overflow`,
- `mofe01_me2_sequential_executor_rejects_nonsequential_lane_ids`.

Ran:

- `cargo test -p openwepp-hillslope-orchestrator mofe01_me2 -- --nocapture`
  - PASS; 6 passed.
- `cargo test -p openwepp-hillslope-orchestrator --lib writeback:: -- --nocapture`
  - PASS; 10 passed.
- `cargo test --test mofe01_per_ofe_state_contract -- --nocapture`
  - PASS; 4 passed.

## M-E1

M-E1 made `mofe01_per_ofe_state_contract` green by implementing the structural
surfaces it required:

- `PerOfeDailyWaterBalanceCollection` and
  `PerOfeDailyWaterBalanceRecord`;
- `TransferInput` and `TransferOutput`;
- publication manifest tokens for aggregate-vs-per-OFE policy, record
  cardinality, storage-lineage policy, and identity status.

Additional focused runner tests in
`crates/openwepp-runner/src/hillslope/tests03/per_ofe_state.rs` cover:

- N=1 aggregate adapter round-trip;
- multi-OFE aggregate adapter rejection;
- transfer output recipient validation;
- source/recipient mismatch rejection;
- valid 2-OFE transfer-chain append;
- static slice cardinality and geometry validation.

Ran:

- `cargo test --test mofe01_per_ofe_state_contract -- --nocapture`
  - PASS; all 4 contract tests passed.
- `cargo test -p openwepp-runner mofe01_me1 -- --nocapture`
  - PASS; all 7 focused M-E1 tests passed.

## M-E0

M-E0 added and registered
`tests/integration/mofe01_per_ofe_state_contract.rs`.

Test coverage:

- `mofe01_me0_contract_authority_is_present`
  - Asserts M-E0 authority in `SC-RUNOFFPART-001`, `SC-WATBAL-001`,
    `SC-SYSTEM-001`, and exact science-contract registry rows without pinning
    future-sensitive global review dates.
  - PASS under focused execution.
- `mofe01_me0_current_architecture_requires_structural_per_ofe_state_collection`
  - Strips comments, string literals, and char literals from current runtime
    sources, then requires structural `PerOfeDailyWaterBalanceCollection` and
    `PerOfeDailyWaterBalanceRecord` definitions, an impl block, and core OFE
    transfer fields.
  - FAILS intentionally on the current aggregate architecture.
- `mofe01_me0_current_architecture_requires_structural_transfer_payloads`
  - Requires structural `TransferInput`/`TransferOutput` payload definitions
    with OFE identity and separated surface/lateral carry tokens.
  - FAILS intentionally on the current aggregate architecture.
- `mofe01_me0_current_architecture_requires_publication_policy_manifest_gate`
  - Requires source-level publication policy, record-cardinality,
    identity-status, and per-OFE storage-lineage manifest tokens.
  - FAILS intentionally on the current aggregate architecture.

M-E0 also updated
`tests/integration/mofe01_inter_ofe_route_contract.rs` to stop pinning the
registry to the old M-B review date. The test now asserts registry exposure of
the M-B authority rows, allowing later legitimate contract review dates.

Ran:

- `cargo test --test mofe01_per_ofe_state_contract mofe01_me0_contract_authority_is_present -- --nocapture`
  - PASS.
- `cargo test --test mofe01_inter_ofe_route_contract -- --nocapture`
  - PASS.
- `cargo test --test mofe01_per_ofe_state_contract -- --nocapture`
  - FAIL by design: 1 authority test passed and 3 structural red gates failed.

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
