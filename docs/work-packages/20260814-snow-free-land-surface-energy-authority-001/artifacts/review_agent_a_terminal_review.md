# Terminal Land-Surface Science And Numerics Review

Evidence class: `Static + Ran`; fresh independent review of the exact stable
Child-1 candidate bytes. Earlier failed reviews were read as immutable
historical evidence and were not reused as acceptance evidence.

Verdict: **PASS / GO**. No material science or numerical finding remains.

## Exact Reviewed Bytes

- LSE definition:
  `e1736b8c77d13d6fb12fb97a6f747e54eea877edf237817b6c6e8954cff8332f`;
- V8 definition:
  `622bc900a08bd4c70e67c09e1fa113a9de24c48afce3b145a494bb76f6dcbe9b`;
- top-level calculator:
  `1156fa88a6d7e4dd98f6dd70fe5b891f69e0b6825694179ac4d687a38907c859`;
- exact joint canopy-ground core:
  `c9555b2dd02a5d6f11d71eb923fb60bc882e9638ec20eb79accc96cec9018be5`;
- frozen vectors:
  `7b6a303ae434ca6ad59c7082ebf486300214427d6abe20c36bfaa9b8cbdab91c`;
- diagnostics schema:
  `41fb7909d073b4fdf4e59c9fa7da26b9a965ad916688b7867a56525d1bf1460c`;
- coupled-transaction schema:
  `02dfa522b7d070df9a7d3e904d4f538a7f734eb6c8315fcbf033b7628b28e07f`;
- `SC-LANDSURFACEENERGY-001.md`:
  `67b51fde024e85668d1bb605bbb54fd58ea6b7a0e798b68db1293ebbb93a0a62`;
- `SC-VEGETATION-001.md`:
  `1d7ec3699085fdf5d2f29e01b3c1d76b8a2a5ad8ce22340df2e066cb39f1fb1a`;
- `SC-VEGETATIONTRANSACTION-001.md`:
  `44b77e5d9854ce9c4b64214f2a669e1b09e2fdc5bdb6d1391ef25a909f186e73`.

The remaining schema hashes match the definition exactly: configuration
`6499b98c...`, forcing `f1fb785e...`, state `91243e40...`, and water protocol
`2e5ade75...`.

## Independent Regeneration And Focused Gate

Ran exactly one generator process to completion in the absolute temporary
directory `/tmp/lse-child1-science-terminal-MnGmAQ`:

```text
.venv/bin/python .../reference_calculator.py \
  --write /tmp/lse-child1-science-terminal-MnGmAQ/vectors.json
sha256sum /tmp/lse-child1-science-terminal-MnGmAQ/vectors.json \
  .../openwepp_snow_free_lse_v1_vectors.json
cmp -s /tmp/lse-child1-science-terminal-MnGmAQ/vectors.json \
  .../openwepp_snow_free_lse_v1_vectors.json
```

The generated and committed files both had SHA-256 `7b6a303a...`; `cmp`
returned zero. No concurrent retry was started.

Ran:

```text
cargo nextest run \
  --test land_surface_energy_balance_authority_contract \
  --profile quick
```

Result: **7 passed, 0 skipped**.

I additionally validated all eleven failure diagnostics directly against the
Draft 2020-12 diagnostics schema and checked their semantic error-code/kind
pairing, attempted transaction, null candidate, six-member rollback envelope,
and byte-identical before/after hashes. All passed. All 76 executed poisons
reject with null candidates.

## `A6-CRITICAL-001` Confirmation

The accepted infiltration enthalpy is now consumed by the identified first
soil-thermal node rather than merely retained as a receipt. The independent
reconstruction is:

```text
T1_end
  = 291.5
    + (4766.427866774984 + 53529.68923674751)
      / (120000 * 0.62)
  = 292.28354996106884 K.
```

The frozen physical candidate contains that exact temperature. Ground heat and
infiltration energy remain distinct typed receipts; the stand-ground amounts
are divided by the tile fraction exactly once and by the node's tile-ground
areal heat capacity exactly once. The second soil node receives no infiltration
energy.

The four required counterfactuals execute and reject:

- omitted infiltration receipt;
- duplicated infiltration receipt;
- wrong receiving soil node; and
- wrong area basis.

The complete candidate-body reconstruction also rejects any independently
altered soil-thermal ending state. Therefore receipt presence cannot substitute
for physical state consumption.

## Prior Science Finding Reassessment

| Finding family | Terminal assessment |
|---|---|
| `A-CRITICAL-001..007`, `A-HIGH-008..011`, `A-MEDIUM-012` | corrected in the admitted equations, strict schemas, exact coupled execution, source custody, and typed evidence |
| `A2-CRITICAL-001..002`, `A2-HIGH-003..005` | corrected by the complete joint oracle, physical owner transaction, semantic poisons, equilibrium-zero rule, and schema-bound vectors |
| `A3-CRITICAL-001..002`, `A3-HIGH-003..004` | corrected by complete dry-plus-liquid post-ingress enthalpy, exact lower-boundary two-stream coupling, family-specific step evidence, frozen active sets, and complete failure DTOs |
| `A4-HIGH-001` | corrected: the equality/active ground cap retains its fixed value and zero generalized derivative through centered differences, multirank reconstruction, and line search |
| `A4-HIGH-002`, `A5-HIGH-001` | corrected: all failures have complete canonical diagnostics and the exact `LSEB-E-030` unsupported-domain or `LSEB-E-034` numerical pairing |
| `A6-CRITICAL-001` | corrected: infiltration enthalpy enters node-1 accepted energy and ending temperature exactly once |

The 22 mandatory exact scenarios continue to exercise bare-soil and litter day/
night states, open and multirank covered tiles, source-resolved shortwave,
reciprocal longwave, canopy-air heat/vapor feedback, signed evaporation and
condensation, finite and equilibrium-zero surface storage, arbitrary-layer
Crank--Nicolson heat, beginning-snapshot water arbitration, fixed-cap rebuild,
post-solve advection, alternate starts, and natural numerical failures. The
exact ground-cap review probe remains nondegenerate: cap
`0.00015581562596770875` versus perturbed law
`0.00015581538850274556 kg m^-2 tile s^-1`.

## Scope And Conclusion

This PASS releases only the selected contract-first implementation authority.
It does not claim a Rust LSE runtime, real hydrology consumer, scheduler
integration, selector activation, production cutover, calibration, empirical
validation, or transferability.

No finding is accepted, rejected, deferred, or assigned to follow-up by this
terminal science review. The exact reviewed Child-1 science/numerics bytes are
eligible for the package's remaining independent ownership, heavy-gate, and
terminal-verification steps.
