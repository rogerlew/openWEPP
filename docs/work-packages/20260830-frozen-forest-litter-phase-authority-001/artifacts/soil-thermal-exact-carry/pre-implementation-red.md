# Version-15 soil-thermal exact-carry pre-implementation red

Evidence mode: `Static + Ran`

## Contract-first source identities

The pre-amendment identities are retained in `source-snapshot.md`. The amended
contract/test identities at the isolated gate were:

| Surface | SHA-256 |
|---|---|
| `SC-SURFACELIQUID-001.md` v15 / `INV-SURFACELIQUID-022` | `a4bdc8f4bacff930142156ff19a535f6dd7a0d66569c2fb36c658edf913c703a` |
| `SC-LANDSURFACEENERGY-001.md` v15 / `INV-LANDSURFACEENERGY-150` | `af1e0a87ae624dca1c92926e0537855290f09297612d76fe9e3c18b2db381eef` |
| science-contract index | `11d119858f6f19445c0ab23333d65374c802e23defe17b8efb1a183d416cef21` |
| surface-liquid authority test | `0c538c1ada013462b1517bbe95c35f1f864b2b440ad78df0bab956104796e7a9` |
| LSE authority test | `b2183f6148d04890a573d32ef42ee4f73995070fc1964035db37c271d0dbbc31` |

## Isolated expected-red

Ran:

```text
set -o pipefail
env RUST_MIN_STACK=67108864 nix develop -c cargo nextest run \
  --test land_surface_energy_balance_authority_contract \
  --test surface_liquid_hydrology_custody_authority_contract \
  -E 'test(/version_(fifteen|15)/)' --no-fail-fast
```

- nextest run ID: `f2d09db4-cf59-4736-817e-e2fe1dfff57b`
- result: expected failure, command exit `100`
- tests: `4` run, `2` passed, `2` failed, `25` skipped
- retained log: `/tmp/soil_thermal_exact_carry_red/isolated-expected-red.log`
- retained log SHA-256:
  `f5262f6d6b37074d9ce8fa3634b11341098f2c6415f6e32592e7bdc0f28ff0b8`

The two contract/vector tests passed. The only failures were the two intended
unchanged-production gates:

1. `version_fifteen_requires_exact_carry_production_identity` refused because
   production lacks `pub struct ExactDyadicEnthalpy`.
2. `version_15_requires_exact_carry_owner_receipt_and_restart_symbols` refused
   because production lacks `pub struct ExactDyadicEnthalpy`.

No contract assertion, numeric-vector binding, parser/build step, or unrelated
test failed in this isolated gate. Production was not edited by this contract-
first task. The expected red therefore isolates the missing v15 production
symbol family rather than a pre-existing or concurrent failure.

## Required implementation handoff

The production successor must provide the exact public bindings asserted by
the retained red:

- `ExactDyadicEnthalpy`;
- `SoilThermalLayerStateV2`;
- `SoilThermalOwnedStateV2`;
- `SoilThermalOwnerEnvelopeV2`;
- `SoilThermalEnergyCreditReceiptV2`;
- `SoilThermalOwnerRestartV2`;
- `SoilThermalOwnerCheckpointV2`; and
- typed `LSEB-E-049` refusal coverage.

The governing contracts require exact accepted-operand reconstruction, one
nearest-even finite high-term rounding, normalized signed-dyadic carry,
V1-to-V2 zero-carry migration, no production downgrade, exact receipt/restart/
checkpoint identity, complete rollback, and real WAT5/`p61`/native-forest
consumer proof. They authorize no tolerance, `nextafter`, forced ULP,
canonical-zero, constitutive-physics, phase-chronology, or temporal-floor
change.
