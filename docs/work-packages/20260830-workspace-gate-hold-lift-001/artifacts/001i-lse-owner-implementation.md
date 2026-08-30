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
| `src/exact_dyadic_enthalpy.rs` | `da172a40b2fd81e70a1ab16c2aa81bcd315fba4387a2e76780a5d2b6bdce6135` |
| `src/owner_envelope.rs` | `142d2a0c653179b2d00037da86e3720c5cd75e048553a6b94e5cddbfa2dc8315` |
| `src/transaction.rs` | `c1ea38f349d5950740e2de62a607742931e0344c51359c36acbe0269612a0980` |
| `src/lib.rs` | `2d0426b0693cfe7477cb23e7b079ea78dfb59e86806026bc94eeb76ad9158d74` |

The embedded immutable V2 owner-schema definition digest is
`e9b9e7f181e0abf1d28fd7e4a3b15f6c00158781acd7d3d0f9a5b5559eb4d09c`;
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
- clone-only candidate construction, so every refusal leaves the complete
  beginning bytes unchanged.

The canonical WAT5 vector retains high term
`-34315.42154113602 J m^-2` bit-for-bit after accepted infiltration energy
`-8.0670339832330148e-19 J m^-2`; the exact nonzero remainder is
`(-1,"1dc319224e55f",-109)`. Exact reconstruction residual is mathematical
zero. No tolerance, `nextafter`, forced ULP, producer residual, subnormal
flush, temperature update, or high-term signed-zero rewrite is used.

## Ran gates

| Gate | Result |
|---|---|
| `cargo nextest run -p openwepp-land-surface-energy -E 'test(exact_dyadic_enthalpy)' --no-fail-fast` | PASS, run `338a4c0a-8646-4679-8144-f1ad1fb2ad08`, 9/9 |
| `cargo nextest run -p openwepp-land-surface-energy --no-fail-fast` | PASS, run `5dd772f0-0e94-4051-b13c-c6e338d2f705`, 113/113, 0 skipped |
| both v15 authority tests with `-E 'test(/version_(fifteen|15)/)'` | PASS, run `6420d594-5e17-461c-a6f0-25f4a7593bdf`, 4/4 |
| `cargo clippy -p openwepp-land-surface-energy --all-targets --all-features -- -D warnings` | PASS |
| `cargo fmt --all -- --check` | PASS |
| owned-file `git diff --check` | PASS |
| added-line scan for `microstep`, persisted `diagnostic`, `nextafter`, forced ULP, or tolerance path | PASS, no match |

Focused vectors cover WAT5, positive/negative carry, even/odd halfway ties,
adjacent crossing, exact cancellation, repeated multi-operand permutations,
minimum signed subnormals, the normal/subnormal boundary, largest-finite
rounding and overflow refusal, all canonical-wire poison classes, V1 byte and
bit preservation, V2 roundtrip/tag/digest, downgrade refusal, signed-zero
no-op preservation, receipt omission/duplication/reorder and support/layer/
beginning/Q/carry/predecessor substitutions, nonfinite Q, exact independent
reconstruction, and byte-exact rollback.

## Line-count disposition

`exact_dyadic_enthalpy.rs` is 1,314 lines, `owner_envelope.rs` is 745,
`lib.rs` is 340, and `transaction.rs` is 2,869. The transaction file remains
below the 3,000-line closure limit but retains the pre-existing 2,000-line
`WARN`; the exact-carry increment is 109 added / 2 import lines replaced and
does not cross the mandatory-refactor threshold. The new arithmetic module is
below `WARN`.

## Disposition

`CORE IMPLEMENTED / PASS`. Persisted restart/checkpoint adoption,
orchestrator receiver wiring, and real WAT5/`p61`/native-forest consumers are
deliberately outside this bounded core handoff and remain parent-package
closure obligations.
