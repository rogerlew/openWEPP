# contract test implementation evidence

Status: M-H executed; no new M-H contract tests required beyond the existing
MOFE01 contract-derived test suite and full-ladder acceptance audit.

Evidence mode: Ran + Static

## M-H

M-H made no test-source edits. It accepts package closure by running the
current contract-backed runtime across the full ladder:

- Existing M-E4-REDO internal WB13 identity tests remain the authority for
  non-tautological per-OFE storage/transfer identity validation.
- Existing M-F-REDO2 tests remain the authority for public per-OFE WAT row
  cardinality, active handoff, anti-clone behavior, and public `QOFE`/`Q`
  normalization.
- Existing M-G tests remain the authority for the erosion `qin`/sediment
  boundary and manifest provenance.

Ran:

- Full H1-H36 acceptance audit under `/tmp/openwepp_mofe01_mh_final`: PASS.
- Local full-ladder `owcmp` execution without comparator subagent: PASS
  execution and row-key alignment; semantic values remain investigation fail.
- Watershed-output `totalwatsed3` attempt: BLOCKED / RESTATED at
  `CLIWAT-E-010` / `IMP-E-004`, `jpond=0`.
- Final post-documentation gates: PASS; see `gate-results.md`.

## M-G

M-G adds contract-derived coverage for the decision boundary:

- `mofe01_mg_erosion_qin_boundary_contract_authority_is_present`
  - Asserts `SC-RUNOFFPART-001`, `SC-WATBAL-001`, `SC-SED-001`, and
    `SC-SYSTEM-001` carry the M-G invariant/addendum authority.
  - Asserts the runner manifest source contains `erod14_qin_source_policy`,
    `erod14_qin_sediment_coupled`, and the M-G water-transfer-only policy.
- `cli03_mofe03_multiofe_runfile_executes_wave2_without_manual_symbol_injection`
  - Now also asserts active multi-OFE Wave-2 manifests report
    `erod14_qin_source_policy =
    "water-transfer-only-mofe01-mg-sediment-coupling-follow-on"` and
    `erod14_qin_sediment_coupled = false`.
- `cli03_mofe03_single_ofe_policy_disables_wave2_by_default`
  - Asserts disabled single-OFE Wave-2 manifests report
    `erod14_qin_source_policy = "wave2-disabled"` and no sediment coupling
    claim.

Ran:

- `cargo test --test mofe01_inter_ofe_route_contract -- --nocapture`
  - PASS.
- `cargo test --test cli03_runner_contract_derived_tests cli03_mofe03 -- --nocapture`
  - PASS.

## M-F-REDO2

M-F-REDO2 expands publication coverage without weakening M-E4-REDO identity
tests:

- `cli03_mf_multiofe_publication_emits_public_per_ofe_wat_rows`
  - Asserts multi-OFE WAT output emits `days * nofe` rows, grouped OFE keys,
    active nonzero surface handoff, anti-clone publication vectors, downstream
    `QOFE != Q` on routed rows, and the local/cumulative QOFE-to-Q ratio.
- `mofe04_publication_contract_authority_closure_contract`
  - Asserts `SC-WATBAL-001`/`SC-SYSTEM-001` carry M-F-REDO2 authority for
    `runoff * efflen / slplen`, `runoff * efflen / totlen`, and downstream
    `QOFE == Q` rejection.
- Existing M-E4-REDO internal WB13 tests continue to require
  non-tautological transfer/storage identity evidence.

Ran:

- `cargo test --test cli03_runner_contract_derived_tests cli03_mf_multiofe_publication_emits_public_per_ofe_wat_rows -- --nocapture`
  - PASS.
- `cargo test -p openwepp-runner --lib mofe01_me4_redo_internal_wb13_records -- --nocapture`
  - PASS.
- `cargo test --test mofe04_publication_contract_authority_closure_contract -- --nocapture`
  - PASS.
- `cargo test --test mofe01_per_ofe_state_contract -- --nocapture`
  - PASS.
- `cargo test --workspace`
  - PASS.
- Required local H1/H6/H9/H11 smoke audit
  - PASS active handoff, anti-clone, internal identity, and `QOFE/Q`
    geometry gates.

Runtime semantic comparison still fails on broader value families, but
M-F-REDO2's test and audit gates close the publication-normalization blocker.

## M-F-REDO

M-F-REDO expands publication coverage without weakening M-E4-REDO identity
tests:

- `cli03_mf_multiofe_publication_emits_public_per_ofe_wat_rows`
  - Asserts multi-OFE WAT output emits `days * nofe` rows, grouped OFE keys,
    active nonzero surface handoff, and anti-clone publication vectors.
- `mofe04_publication_contract_authority_closure_contract`
  - Asserts contract authority for `INV-WATBAL-098` and `INV-SYSTEM-031`,
    including active handoff and anti-clone language.
- Existing fixture-family tests were updated for the now-active same-pass runon
  storage path.

Ran:

- `cargo test --workspace`
  - PASS.
- Required local H1/H6/H9/H11 smoke audit
  - PASS active surface/lateral handoff and anti-clone gates.
  - FAIL `QOFE != Q` geometry gate; this blocker is runtime acceptance, not
    missing test registration.

## M-F

M-F adds public per-OFE publication coverage without weakening the M-E4-REDO
identity tests:

- `cli03_mf_multiofe_publication_emits_public_per_ofe_wat_rows`
  - Asserts multi-OFE WAT output emits `days * nofe` rows, grouped OFE keys,
    M-F manifest policy/storage markers, and the public per-OFE publication
    seam. M-F-REDO later tightened this to active handoff and anti-clone
    coverage.
- `watershed_cli_mf_accepts_valid_per_ofe_publication_metadata`
  - Asserts watershed contributor manifest validation accepts the M-F per-OFE
    metadata shape when counts and keys are coherent.
- `mofe01_mf_current_architecture_requires_public_per_ofe_wat_publication`
  - Source guard requiring the M-F publication policy and public per-OFE WAT
    publication seam.

Ran:

- `cargo test --workspace`
  - PASS; includes the three M-F coverage points above.

Runtime acceptance remains held by the real-run `UpStrmQ` audit, not by missing
test registration.

## M-E4-REDO

M-E4-REDO replaces the tautological M-E4 identity tests with focused internal
WB13 conservation tests in
`crates/openwepp-runner/src/hillslope/tests03/per_ofe_state.rs`:

- `mofe01_me4_redo_internal_wb13_records_close_true_transfer_and_storage_identities`,
- `mofe01_me4_redo_internal_wb13_records_reject_storage_delta_mismatch`,
- `mofe01_me4_redo_internal_wb13_records_reject_cross_ofe_transfer_mismatch`,
- `mofe01_me4_redo_internal_wb13_records_include_frost_storage_delta_per_ofe`.

The integration contract test also adds
`mofe01_me4_redo_current_architecture_requires_non_tautological_internal_wb13_checks`.

Ran:

- `cargo test -p openwepp-runner mofe01_me4_redo -- --nocapture`
  - PASS; 4 passed.
- `cargo test -p openwepp-runner mofe01 -- --nocapture`
  - PASS; 12 passed.
- `cargo test --test mofe01_per_ofe_state_contract -- --nocapture`
  - PASS; 5 passed.

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
