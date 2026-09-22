# COLD-CANOPY-M1 partial complete-parent body — correctness review 02

**Verdict: HOLD.** The reviewed correction validly isolates support IDs from the `u64` parent namespace, rejects a caller/support mismatch before solving, and preserves typed M1 solver errors. It does not make the parent executable: staged BGC lineage is incompatible with canonical parent finalization, each solve still resets mutable soil beginning temperatures to the original fixture, the native receipt remains bound to bootstrap forcing, and the released raw-outcome/finalization/replay surface is absent.

**Evidence class: Static.** Same-reviewer delta inspection of frozen canonical 760-entry source aggregate `b8fbc11f6bfacf45dc9fc587b7fc2c01a1aa924c325c3ce833dfa79967ba6f0f`. No test, build, or physical command was run by this reviewer. Root reports compile16 expected-red with 56 missing-API errors.

## Findings

1. **High — the high-bit support ID is installed as persistent BGC lineage and cannot pass the existing parent finalizer.** `m1_receiver.rs:2035-2045` now gives every support `TransactionId((1_u128 << 127) | support)`. `prepared_m1_endings` constructs the canonical BGC candidate from that ID and `stage` installs `m1_bgc.ending()` directly (`m1_receiver.rs:650-687,700-716`); canonical BGC construction copies the ID into `ending.last_transaction_id` (`openwepp-biogeochemistry/src/lib.rs:352`). The established V11 finalizer admits the current BGC predecessor only when it equals the prior parent transaction or the final parent transaction (`v9_real_consumer_shadow.rs:3467-3493`), both `u64`-derived and therefore below the high-bit namespace. Parent completion must consequently reject after the first staged support, or an unchecked overwrite would regress lineage. This also supplies the precise field binding for `SC-VEGETATIONTRANSACTION-001.md:350-375`: segment candidates may carry support-local transaction evidence, but cannot install that identity as the persistent parent transaction. Keep the canonical support candidate/receipt validation and physical BGC ending, then normalize only the staged persistent `last_transaction_id` to the admitted parent predecessor. Add a direct staged-BGC-lineage control; current controls inspect staged M1/strata and final BGC, leaving this seam uncovered.

2. **High — every support solve reuses original soil beginning temperatures instead of the current staged soil-thermal owner.** `provider_covered_input` deserializes the entire original `CoveredColumnInputs` (`m1_coupled_expected_red.rs:141-144`). `prepare_fixed_sequence_support` changes the five weather/liquid operands and current M/H only (`m1_fixed_sequence_provider.rs:738-791`). The solver seeds coordinates 15–20 directly from `column.ground.soil_nodes[*].beginning_temperature_k` (`solver_covered_evaluation/m1_coupled.rs:3520-3533`) and uses those values in the coupled residual. The authorization requires an accepted ending to become the next staged beginning, so support two currently resets these six physical operands. Reuse original geometry, topology and immutable material coefficients, but project current staged soil-thermal temperatures before each solve. Audit the other mutable `OpenSurfaceProblem` beginning fields (`surface_liquid`, enthalpy and warm start) against their actual owner; add a consecutive-support assertion that every mutable solver beginning comes from support one's installed owners while immutable original fields remain byte-exact.

3. **High — native execution remains authenticated to bootstrap forcing rather than the admitted provider support.** `advance_fixed_sequence_support` still creates `endpoint_fixture()` on every record (`m1_fixed_sequence_provider.rs:827-837`). `native_fixture_from_current` replaces owners but reconstructs its receipt from the bootstrap forcing SHA/receipt and retains `bootstrap.forcing` (`m1_receiver.rs:564-641`). Authenticated GSI forwarding does not bind the other provider overrides or parent forcing receipt to the native transaction. Construct the native receipt/context from the admitted support and prepared input while retaining only legitimate immutable fixture configuration/geometry.

4. **High — complete-parent production and evidence APIs remain absent.** The released controls still have no real raw support outcome/resource/material/BGC/owner/slab accessors, staged caller/clock evidence, sealed finalized owners, typed replay result, or consuming `complete_parent`. Compile16's 56 missing-method errors are substantive expected-red evidence. Implement those boundaries from producer-owned structures and the existing V11 finalization/ownership-transfer path; do not synthesize reports or expected errors in accessors.

## Accepted corrections

- **Support namespace:** `m1_receiver.rs:2035-2045` uses a checked `u64` support ordinal inside the high half of `u128`. It is monotone across the admitted cycle and disjoint from every `u64` parent revision. The identifier itself is acceptable support-local authority once Finding 1 prevents it from becoming persistent BGC parent lineage.
- **Pre-solve caller binding:** `m1_fixed_sequence_provider.rs:807-827` calls `provider_matches_admitted_support` before preparation. The receiver predicate (`m1_receiver.rs:513-538`) checks bound run and cycle, cursor-derived parent/record ordinal, accepted-until, exact parent interval and exact 60-second record. This closes the reviewed stale/wrong caller entry seam for the bounded provider.
- **Typed solve failure:** `M1ParentExecutionError::Solver(M1CoupledError)` now preserves both `solve_m1_coupled_column` and prepared-context errors (`m1_fixed_sequence_provider.rs:49-55,787-790`). Provider/admission failures remain `VEG-E-144`.

The review01 shared-core, authenticated-GSI propagation, native transaction propagation direction, ledger validation, lineage-only M1 finalizer, and atomic caller-clone/slab-staging findings remain accepted and were not reopened. The current support-ID value and BGC installation must be assessed together as described above.

## Residual risk and missing validation

No complete-parent body compiles, so no provider support, parent completion, replay refusal, rollback, physical balance, or coupled cycle is released. Root separately reports formatter PASS and Clippy failures in introduced vegetation helpers; these quality failures also remain to be corrected before executable review. The ten-member provider extraction manifest was previously rehashed with no mismatch; this review does not call it stale.

## Reviewed identities

- provider driver SHA-256: `b4a29d4cca395962a13fe747187eb21fa48e780c675d66028eb4df6f0fdcbe72`
- receiver SHA-256: `31be6a0877a807f2953742b66bff98160f38b711871c15acb863f836855b23b7`
- source recorder SHA-256: `db3356211c44029b0d0da6348fd90c887ab59a0c8e68ae49ae3601b0e1c67e92`
- observer-relative patch SHA-256: `cbe6bd4dd9a2777cea57a84a199c68cb1518ac2f6f6188734cc59d25ce702b65`

No body or physical execution release is granted.
