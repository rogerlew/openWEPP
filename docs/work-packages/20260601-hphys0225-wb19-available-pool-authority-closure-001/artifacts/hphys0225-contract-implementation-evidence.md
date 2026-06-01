# HPHYS0225 Contract Implementation Evidence

Status: completed  
Evidence mode: Static

## Contract-first Updates

1. `SC-SUBHYD-001`
   - Added `INV-SUBHYD-017` for WB19 layer-pool available-cap authority.
   - Added `HPHYS0225 WB19 Layer-Pool Available-Cap Authority Addendum`
     prohibiting `max(layer_pool, legacy_term)` expansion.
2. `SC-WATBAL-001`
   - Added `HPHYS0225 WB19 Layer-Pool Available-Cap Authority Addendum`
     linked to `SC-SUBHYD-001#INV-SUBHYD-017`.
3. `docs/specifications/science-contracts/index.md`
   - Updated `SC-SUBHYD-001` and `SC-WATBAL-001` notes with HPHYS0225 scope.

## External-authority Contract Surfaces

1. Added suite specification:
   - `docs/specifications/external-authority/suites/cas_l4_subhyd_layer_pool_withdrawal_cap_001.md`
2. Registered suite in:
   - `docs/specifications/external-authority/registry.yaml`
3. Added fixture provenance + lock sidecars:
   - `tests/fixtures/constitutive/cas_l4_subhyd_layer_pool_withdrawal_cap_001/fixtures.sha256`
   - `tests/fixtures/constitutive/cas_l4_subhyd_layer_pool_withdrawal_cap_001/fixtures.provenance.yaml`

## Closure Measure Mapping

- `MEASURE-HP225-001`: satisfied (canonical contracts amended).
- `MEASURE-HP225-002`: satisfied (required Level-4 suite + registry + fixture integrity metadata landed).
