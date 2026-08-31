# WGHL-FULL-001I DirectV10 native soil owner and persisted restart V2

Evidence mode: `Static + Ran`

## Disposition

`CORE/PERSISTENCE CUTOVER PREREQUISITES COMPLETE / FOCUSED + FULL PASS`

This increment retains the distinct
`OPENWEPP_DIRECT_V10_REAL_CONSUMER_CHECKPOINT_V2` schema and completes its
native exact-carry bootstrap, host, and prepared-transaction prerequisites.
No V1 schema, version, tag, canonical digest formula, or accepted wire byte was
changed. The small V1 caller adaptations added only explicit typed refusal of a
V2 resident where the legacy code requires V1.

## Native LSE boundary

- `prepare_soil_thermal_support_v2` validates the accepted native owner,
  predecessor, transaction, half-open support, and the exact
  `60_000_000_000 ns` minimum. One tick below refuses; a stable one-hour
  support admits.
- `PreparedSoilThermalSupportV2` exposes a borrowed
  `SoilThermalPhysicalReadViewV2`. Its layer lookup and exact enthalpy read
  preserve high term plus dyadic carry without constructing a V1 snapshot.
- `advance_soil_thermal_trial_v2` produces an unpublished
  `SoilThermalTrialStateV2` with exact ending state and layer credits but no
  accepted receipt or installation surface. Accepted publication still occurs
  only through the native receipt validator.
- `seal_soil_thermal_receipt_free_owner_v2` and its validator originate and
  cross-check native restart/checkpoint seals from the canonical owner. The
  seal binds state, parent-V1 identity, receipt chain, transaction, and support.
- The existing accepted-credit receiver now reuses the same trial advancement,
  then independently reconstructs and seals its typed receipt.

## Persisted cutover boundary

- `bootstrap_soil_thermal_restart_v1_to_v2` is the sole public V1-to-V2 soil
  bootstrap. It restores V1, copies temperature and high-term bits, installs
  canonical zero carry, binds the parent V1 state/restart digests, and invokes
  native receipt-free sealing. The obsolete caller-supplied seal constructor
  was removed.
- `bootstrap_complete_owner_state_v1_to_v2` substitutes exactly one
  authoritative V2 soil owner while reusing unaffected V1 owner DTOs.
- `DirectV10NativeOwnerHostV2::from_receipt_free_native` accepts native
  prepared custody and seals, validates configuration and canonical persisted
  owner joins, and constructs only after every check passes.
- `DirectV10PreparedDayTransactionV2::prepare_from_native_host` starts from
  that host. `accept_native_soil_candidate` consumes the native beginning,
  accepted candidate, expected-source custody, and orchestrator seals, then
  derives the persisted successor inside the transaction. Caller-fabricated
  DTO prepare/accept helpers are private.
- Candidate construction and checkpoint publication remain clone-only until
  the final assignment. Seal poison and replay refusal preserve checkpoint and
  host state byte-identically.
- V2-to-V1 downgrade remains unconditionally prohibited, including zero carry.
  The V1 restart projection and Stage-3 admission call sites propagate the
  resident mismatch; fixtures and the benchmark use intentional test-only V1
  assertions. No fallback or V2 reinterpretation exists.

Canonical native frames continue to require typed decoding, byte-for-byte
canonical re-encoding, native validation, and restart/checkpoint/receipt/seal
joins. Accepted operands and temperature projections remain separately stored
and digested; independent receipt validation uses that external expected set,
never a list flattened from the receipt.

## Ran gates

| Gate | Result |
|---|---|
| `cargo test -p openwepp-land-surface-energy` | PASS, 116/116 |
| LSE focused native support/trial/seal tests | PASS, 2/2 |
| `cargo clippy -p openwepp-land-surface-energy --all-targets --no-deps -- -D warnings` | PASS |
| `cargo check -p openwepp-persisted-restart-v1` | PASS |
| `cargo nextest run -p openwepp-persisted-restart-v1 --no-fail-fast` | PASS, run `e5ee1fed-890d-468c-9487-90039b567090`, 28/28 |
| `cargo nextest run -p openwepp-persisted-restart-v1 --all-features -E 'test(v2_tests)' --no-fail-fast` | PASS, run `befdbc48-b8d7-419a-a4d9-442bdc8f354f`, 7/7 |
| `cargo nextest run -p openwepp-persisted-restart-v1 --all-features --no-fail-fast` | PASS, run `a0144878-bb55-4438-984e-e7a514651949`, 52/52 in 466.064 s |
| orchestrator V2-resident/V1-projection-refusal focus | PASS, 1/1 |
| `cargo clippy -p openwepp-persisted-restart-v1 --all-targets --all-features --no-deps -- -D warnings` | PASS |
| new/modified LSE and V2-module `rustfmt --check` | PASS; mechanical V1 caller edits retain surrounding baseline formatting |
| `git diff --check` | PASS |
| production V2 key scan for `microstep`, `iteration`, `solver`, `diagnostic`, `carry_diagnostic` | PASS, no matches |

The full suite includes the frozen V1-byte migration lock, Stage-3 restart
roundtrips, and the feature-only benchmark/fixture surfaces. Focused V2 tests
cover WAT5 nonzero-carry canonical roundtrip, checked zero-carry bootstrap,
unconditional downgrade refusal, native accepted receipt and independent
expected-set replay, real native-host construction, poison rollback, split
before/after credit, replay refusal, canonical checkpoint admission,
cross-version/omission/reorder poisons, and diagnostic-key exclusion.

## Terminal source identities and line counts

| Source | SHA-256 | Lines |
|---|---:|---:|
| `openwepp-land-surface-energy/src/owner_envelope.rs` | `792a109420f160481de32fd0be56a726a1d9c8fc762ce42bf2d54ef0a92e3fb6` | 974 |
| `openwepp-land-surface-energy/src/transaction.rs` | `eec97eaf2794bc246b14f3547a15c7906742ccf00e7f2ffc0ee7848fdc1d5295` | 2,982 |
| `persisted-restart/src/lib.rs` | `857e24e3b96e8230f97a523934e01dafbba37ade8897329ac8bf5ca54d1b4a4b` | unchanged |
| `persisted-restart/src/soil_thermal_v2.rs` | `d84ddb101bd1e7dd2a8c8744a773cfba1836add8c144d7343e78d8c36e196c44` | 597 |
| `persisted-restart/src/scientific_owners_v2.rs` | `6d4427827c441aa9f67d7d5430218153d21500dbeb161f28821918e22fedf85b` | 70 |
| `persisted-restart/src/checkpoint_v2.rs` | `655ef7fdcabb00dd4c67bd84303fa87f156652e520f147f155a952aec09c4f98` | 275 |
| `persisted-restart/src/projection_v2.rs` | `6adee7d8a5bba4e6cd7d93178cffcc26b68e1d212cf291920d519bb452337a0b` | 225 |
| `persisted-restart/src/host_v2.rs` | `1f961642250aaf997f6ecc6373226ddf36fcbe7846e0ba227df1ccbf48f1edd1` | 141 |
| `persisted-restart/src/transaction_v2.rs` | `29bffef5539f76a955f1bb6c4b51eaec00ece12e7b20741df2aaf18ea522dc16` | 247 |
| `persisted-restart/src/v2_tests.rs` | `7ad7092e69d83b9df4d90336cccff3dfef093a388436424c67512a7c1b91fa9a` | 581 |

All V2 production modules remain below the package warning threshold. The
pre-existing LSE transaction file remains below 3,000 lines; its new focused
tests are isolated in `soil_thermal_v2_cutover_tests.rs`.

## Released APIs and remaining integration owner

Released public boundaries are:

- `prepare_soil_thermal_support_v2`
- `PreparedSoilThermalSupportV2::{beginning_owner, physical_read_view}`
- `SoilThermalPhysicalReadViewV2::{owner, layer, exact_layer_enthalpy}`
- `advance_soil_thermal_trial_v2`
- `SoilThermalTrialStateV2::{transaction_id, beginning_state_sha256, ending_state, layer_credits}`
- `seal_soil_thermal_receipt_free_owner_v2`
- `validate_soil_thermal_receipt_free_owner_v2`
- `bootstrap_soil_thermal_restart_v1_to_v2`
- `bootstrap_complete_owner_state_v1_to_v2`
- `project_receipt_free_soil_thermal_owner_state_v2`
- `DirectV10NativeOwnerHostV2::from_receipt_free_native`
- `DirectV10PreparedDayTransactionV2::{prepare_from_native_host, accept_native_soil_candidate, native_soil_thermal}`

The orchestrator now has the matching prepared-owner plus receipt-free-seals
`try_new_v2` constructor and a single V2 resident without a V1 cache. Final
runner seed selection and end-to-end production publication remain the parent
integration slice; this persistence increment does not invent a runner path or
silently downgrade native custody.
