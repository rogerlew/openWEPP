# HPHYS0224 Contract Implementation Evidence

Status: completed  
Evidence mode: Static

## Canonical Contract Amendments

1. `SC-SUBHYD-001`
   - Added invariant `INV-SUBHYD-016` for WB19 realized-withdrawal soil-water
     cap authority.
   - Added invariant guard-map row requiring typed hard-fail behavior on
     over-withdrawal and prohibiting silent clamping/flooring.
   - Added `HPHYS0224 WB19 Realized-Withdrawal Soil-Water Cap Addendum` with
     explicit subtraction law and Level-4 suite linkage.

2. `SC-WATBAL-001`
   - Added `HPHYS0224 WB19 Realized-Withdrawal Soil-Water Cap Addendum`.
   - Declared typed domain-failure posture
     (`HKERNEL-WB11-LAT-E-003`/`HKERNEL-WB11-DRAIN-E-003`) for over-withdrawal
     relative to pre-phase `wb11_soil_water`.

3. `docs/specifications/science-contracts/index.md`
   - Updated `SC-SUBHYD-001` and `SC-WATBAL-001` notes and `last_updated`
     values to include HPHYS0224 authority additions.

## Contract-First Sequencing

- HPHYS0224 executed contracts before production kernel edits.
