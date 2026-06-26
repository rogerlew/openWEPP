# Contract Amendment Evidence

Evidence class: Static + Ran.

Primary authority: `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`.

## Amendment

- `contract_version: 79`.
- Added variable `snow_melt_shortwave_absorbed_fraction`.
- Added `INV-SNOWFREEZE-055`.
- Added `OBL-SNOWFREEZE-P-030`.
- Added the `SNOWDENSITY-05D Opt-In CoE Melt Implementation Addendum`.
- Added the v79 revision-history row.

## Ratified Behavior

- `legacy_coe` remains default, compatibility comparator, and rollback.
- `coe_shortwave_albedo_v1` is opt-in only.
- The only authorized opt-in melt formula delta is:

```text
amelt = 0.0607 * hrrad * (1 - snow_albedo) * (1 - cancov)
```

- `hrrad` remains the existing `winter.hourly.rad_mj_m2_####` source from 05B.
- `snow_albedo` is the typed `brock2000_temperature_age_v1` state from 05C.
- `bmelt`, `cmelt`, `dmelt`, signed raw melt, daily redistribution, density
  gate, snow storage mutation, WB12 `S`, and WB13 liquid forcing stay on the
  existing algorithmic path.
- Missing or invalid active opt-in albedo state fails closed. It does not fall
  back to `legacy_coe`.

## Contract Test Coverage

The focused 05D contract test verifies the v79 markers, `INV-SNOWFREEZE-055`,
`OBL-SNOWFREEZE-P-030`, the variable name, and the exact opt-in `amelt`
formula string.
