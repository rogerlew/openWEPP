# WGHL-FULL-001I LSE owner core implementation

Evidence mode: `Static + Ran`

## Scope and source identity

Implementation followed the committed contract-first red at `020b5138b` and
is limited to the LSE-owned exact arithmetic, V2 owner/envelope schemas,
typed accepted-energy receipt, independent reconstruction, and clone-only
candidate transaction. It does not edit constitutive physics, temperature,
support/event chronology, closure tolerances, persisted restart adoption,
or orchestration. No microstepping or exact-carry diagnostic is persisted.

Current source SHA-256 identities:

| Source | SHA-256 |
|---|---|
| `src/exact_dyadic_enthalpy.rs` | `c3193d7bb90c3208575ee5236e2ac44e5260e24e21cf153ac07bdeb741901be9` |
| `src/owner_envelope.rs` | `28af5505b3a044c8f5d7cc8b9fd2c2a2d6c9046747e30f326a0c371ba524d6fc` |
| `src/transaction.rs` | `2a85707215dbbd065f86ade4c10716eaaaf1be31ab42b8d81a5e0da65250154b` |
| `src/transaction_native_v2_finalization_tests.rs` | `003b7afe4c49150e7e3ad6fdec7c4c767e65d15bf2a82b9866ec0d3bd7502218` |
| `src/lib.rs` | `2d0426b0693cfe7477cb23e7b079ea78dfb59e86806026bc94eeb76ad9158d74` |

The embedded immutable V2 owner-schema definition digest is
`7877f2a227b0fa98c0c92ae2fb7397744857555fc2f2f77d91a6de327ca88be4`;
the exact-dyadic definition digest is
`7ceb6e80567a05625b0ac7c33fc8c48ac9a776bab8f9863e02e5a87696714014`.

## Implemented custody

- dependency-free arbitrary-precision signed-magnitude integer arithmetic;
- canonical unique zero and normalized odd lowercase-hex signed-dyadic wire;
- exact binary64 decode, exact order-independent addition, one finite
  nearest-even high-term selection, and exact remainder reconstruction;
- finite/domain/resource/overflow refusals under typed `LSEB-E-049` errors;
- `SoilThermalLayerStateV2`, ordered OFE/owned-state/snapshot surfaces, tagged
  `SoilThermalOwnerEnvelopeV2`, restart/checkpoint core seals, and explicit
  V1-to-V2 exact-zero-carry migration;
- unconditional production V2-to-V1 downgrade refusal, including zero carry;
- one typed receipt binding model/configuration/run/owner, transaction and
  predecessor, half-open support, beginning/ending `(H_hi,R)`, canonical
  ordered physical `Q` operands, state identities, and receipt-chain digest;
- independent receipt reconstruction and exact high/carry comparison; and
- one authoritative per-layer ending temperature plus unchanged finite
  positive heat capacity, independently checked by exact rational evaluation
  of `T_end=T_begin+(E_end-E_begin)/C` and one nearest-even binary64 rounding;
- clone-only candidate construction, so every refusal leaves the complete
  beginning bytes unchanged.

The canonical WAT5 vector retains high term
`-34315.42154113602 J m^-2` bit-for-bit after accepted infiltration energy
`-8.0670339832330148e-19 J m^-2`; the exact nonzero remainder is
`(-1,"1dc319224e55f",-109)`. Exact reconstruction residual is mathematical
zero. No tolerance, `nextafter`, forced ULP, producer residual, subnormal
flush, surrogate temperature rule, or high-term signed-zero rewrite is used.
The sub-ULP WAT5 projection retains the beginning temperature bits. A normal
`+1000 J m^-2` credit with unchanged `C=2000 J m^-2 K^-1` changes the receiver
temperature from `273.15 K` to exactly `273.65 K`; stale or substituted
temperature/capacity refuses with byte-exact rollback.

## Ran gates

| Gate | Result |
|---|---|
| `cargo nextest run -p openwepp-land-surface-energy -E 'test(exact_dyadic_enthalpy)' --no-fail-fast` | PASS, run `5f90d8f4-f445-4c6e-a2f3-68272b742e38`, 10/10 |
| `cargo nextest run -p openwepp-land-surface-energy --no-fail-fast` | PASS, run `6642ed52-cc82-4c31-bba2-5e1f5c8190bc`, 119/119, 0 skipped |
| both v15 authority tests with `-E 'test(/version_(fifteen|15)/)'` | PASS, run `c55f6717-6454-41b8-9f9c-bf42da0e8dc6`, 4/4 |
| `cargo clippy -p openwepp-land-surface-energy --all-targets --all-features -- -D warnings` | PASS |
| `cargo fmt --all -- --check` | PASS |
| owned-file `git diff --check` | PASS |
| added-line scan for `microstep`, persisted `diagnostic`, `nextafter`, forced ULP, or tolerance path | PASS, no match |

## Native V2 finalization seam

The later canonical-DFF integration correction adds a read-only typed
`SoilThermalFinalizationBeginning` over either the unchanged V1 snapshot or a
borrowed `SoilThermalPhysicalReadViewV2`. The legacy open and covered
finalization signatures remain wrappers over new typed entry points. V2
finalization consumes the native layer high term and temperature in the
unchanged constitutive solve and retains the exact carry plus complete owner,
schema, exact-definition, parent, contract/model/run/configuration,
transaction/predecessor/support, receipt-chain, state, OFE, layer, and order
identity in the provisional candidate. It performs no V2-to-V1 projection,
receipt sealing, installation, or mutation.

`build_soil_thermal_passthrough_candidate` provides the authenticated
zero-change candidate required by an authoritative branch with no soil
constitutive solve. It copies high term, carry, and temperature bit-for-bit and
publishes exact zero physical credits.

Current seam validation:

| Gate | Result |
|---|---|
| `cargo test -p openwepp-land-surface-energy native_v2_finalization_tests -- --nocapture` | PASS, 3/3 |
| `cargo test -p openwepp-land-surface-energy` | PASS, 119/119 plus doc tests |
| `cargo clippy -p openwepp-land-surface-energy --all-targets -- -D warnings` | PASS |
| LSE v15 authority filter `test(/version_fifteen/)` | PASS, nextest run `aeb5961a-810d-4a27-9ae6-0a53fdbd668a`, 2/2 |
| surface-liquid v15 authority filter `test(/version_15/)` | PASS, nextest run `47b50450-0ad9-402b-8a24-264e74494542`, 2/2 |
| owned-file rustfmt and `git diff --check` | PASS |

Focused seam vectors prove V1 wrapper parity; zero and nonzero minimum-
subnormal V2 carry retention; unchanged high/temperature constitutive output;
complete V2 identity binding; pass-through exact-zero credits; stale
transaction/state, reordered/duplicate layer, physical-temperature, and
noncanonical-carry refusal; and byte-equivalent clone-only rollback.

Focused vectors cover WAT5, positive/negative carry, even/odd halfway ties,
adjacent crossing, exact cancellation, repeated multi-operand permutations,
minimum signed subnormals, the normal/subnormal boundary, largest-finite
rounding and overflow refusal, all canonical-wire poison classes, V1 byte and
bit preservation, V2 roundtrip/tag/digest, downgrade refusal, signed-zero
no-op preservation, receipt omission/duplication/reorder and support/layer/
beginning/Q/carry/predecessor/temperature/heat-capacity substitutions,
nonfinite Q, exact independent
reconstruction, and byte-exact rollback.

## Line-count disposition

`exact_dyadic_enthalpy.rs` is 1,672 lines, `owner_envelope.rs` is 1,380,
`lib.rs` is 340, and `transaction.rs` is 2,905. The transaction file remains
below the 3,000-line closure limit but retains the pre-existing 2,000-line
`WARN` and does not cross the mandatory-refactor threshold. The new arithmetic
module remains below `WARN`.

## Disposition

`CORE IMPLEMENTED / PASS`. Persisted restart/checkpoint adoption,
orchestrator receiver wiring, and real WAT5/`p61`/native-forest consumers are
deliberately outside this bounded core handoff and remain parent-package
closure obligations.
