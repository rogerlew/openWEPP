# Canonical-JSON Correction Terminal Verifier B

Evidence class: `Static + Ran`

Verdict: `PASS / GO`

Scope: bounded verification of the canonical-JSON exponent portability and
strict-state digest-order correction against historical Child-1 release commit
`3f1cf8ee3`. This report does not review or qualify concurrent Child-3 runtime
implementation.

## Exact Bytes

| Artifact | Historical SHA-256 | Verified correction SHA-256 |
|---|---|---|
| `reference_calculator.py` | `1156fa88a6d7e4dd98f6dd70fe5b891f69e0b6825694179ac4d687a38907c859` | `9278be79b1a74d4d609ab5857d00071b1e5717e036cc7323cbfcbf970795666c` |
| vector fixture | `7b6a303ae434ca6ad59c7082ebf486300214427d6abe20c36bfaa9b8cbdab91c` | `9f171b0fd0e9a9a2e40d6ea8773d120b961c343e2aad6ad951ae705c8d683f3b` |
| joint core | `c9555b2dd02a5d6f11d71eb923fb60bc882e9638ec20eb79accc96cec9018be5` | unchanged |
| LSE definition | `e1736b8c77d13d6fb12fb97a6f747e54eea877edf237817b6c6e8954cff8332f` | unchanged |
| V8 definition | `622bc900a08bd4c70e67c09e1fa113a9de24c48afce3b145a494bb76f6dcbe9b` | unchanged |

All six schema artifacts compare byte-for-byte with `3f1cf8ee3`; their
verified hashes are `6499b98c...`, `02dfa522...`, `41fb7909...`,
`f1fb785e...`, `91243e40...`, and `2e5ade75...` in configuration,
coupled-transaction, diagnostics, forcing, state, and water-protocol order.

## Independent Assessment

1. Rust canonical hashing serializes the typed JSON value first and applies
   exponent padding only to the hash input. The scanner tracks quoted and
   escaped-string state, recognizes only lowercase signed one-digit exponents
   outside strings, pads them to two digits, and leaves signed multi-digit
   exponents unchanged. The poison covers both ordinary and escaped
   exponent-like strings. This is a representation rule, not a binary64,
   solver, tolerance, or branch change.
2. The state generator now completes the forest-tile projection before
   replacing `state_sha256` with the digest of the final state with only that
   digest field blank. Independent reconstruction produced
   `6ff22f0d72b6c4fdad3c0d8a0b2947571191e48213635609af8f3b951c07abf1`,
   exactly matching both the standalone strict state and the coupled
   transaction's `beginning_lse_state`. Those two state objects are identical.
3. Independent recursive parsed comparison with the historical fixture found
   exactly four changed leaves: the two copies of `state_sha256` and their two
   dependent schema-instance hashes. No model input, physical operand,
   accepted value, request, authorization, finalized use, candidate, ledger,
   diagnostic, failure, poison, or invariant value changed.
4. The package and final-disposition surfaces truthfully remain `executing` /
   `in progress` while this bounded verification is active. Historical release
   hashes are labeled historical, current correction hashes are labeled as the
   correction candidate, and no runtime, selector, cutover, calibration, or
   empirical-validation claim is added.

## Ran Evidence

| Check | Result |
|---|---|
| Independent Python fixture regeneration and byte comparison | PASS; regenerated `9f171b0f...` is byte-identical to the committed fixture |
| Independent strict-state digest reconstruction | PASS; embedded and reconstructed `6ff22f0d...` match in both locations |
| Recursive parsed comparison against `3f1cf8ee3` | PASS; exactly four dependent digest leaves changed |
| Historical/current definition, schema, and joint-core byte comparison | PASS; all unchanged |
| `cargo nextest run -p openwepp-land-surface-energy --profile quick` | PASS; 21/21 |
| `cargo nextest run --test land_surface_energy_balance_authority_contract --profile quick` | PASS; 7/7 |

## Conclusion

The bounded correction is exact, portable for the frozen CPython lexical
authority, and isolated from constitutive physics and solver acceptance. No
material finding remains within this verification scope.
