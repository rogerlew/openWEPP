# Stale contract assertion census

Status: `COMPLETE FOR TERMINAL CURRENT-GUARD CENSUS`.

Method: `Static:` searched the seven affected integration guards and the
current contract/index sources. No repository-wide numeric replacement was
performed.

Current canonical guards updated by this package:

| Guard | Old assertion | Current assertion | Classification |
|---|---|---|---|
| `land_surface_energy_balance_authority_contract.rs` | LSE `contract_version: 4` | `contract_version: 7` | current canonical contract guard |
| `surface_liquid_hydrology_custody_authority_contract.rs` | Surface liquid v6 | v7 | current canonical contract guard |
| `snow_stage3_legacy_predecessor_bridge_contract.rs` | SnowFreeze v135 / index v135 | v136 | current canonical contract guard |
| `snow_stage3_persistent_accumulation_shadow_contract.rs` | SnowFreeze v135 / old index phrase | v136 / current terminal-receiver phrase | current canonical contract guard |
| `snow_stage3_terminal_receiver_authority_contract.rs` | removed obsolete v13 requirement and v3 index phrase | v136 terminal receiver and v14 shared-carrier phrases | superseded current guard |
| `snow_stage3_turbulent_operator_reconciliation_contract.rs` | SnowFreeze v135, SnowEnergy v12, old index phrases | v136 and v14 current phrases | current canonical contract guard |
| `snow_stage3_wind_source_custody_contract.rs` | SnowEnergy v12 / SnowFreeze v135 | v14 / v136 | current canonical contract guard |

Historical-artifact guards were not changed: source comments, model registry
records, and protected package evidence that intentionally name earlier
contract generations remain provenance records. They are not current
canonical contract guards and must not be rewritten to the current version.

`Ran:` the terminal workspace quick profile exposed two additional current
guards outside the initial focused list. They were reconciled narrowly:

| Guard | Old assertion | Current assertion | Classification |
|---|---|---|---|
| `snow_surface_eb03_contract.rs` | SnowEnergy v12 | v14 | current canonical contract guard |
| `vegetation_boundary_authority_contract.rs` | current identity generation `90313e...` | current generation `41b142...` | current assurance identity guard; historical receipt assertions retained |

The current Vegetation boundary guard still has unrelated pre-existing
version/vector assertions against the active SC-VEGETATION-001 v26 contract;
those were not rewritten as part of this package because they are outside the
Stage-3 assurance/source change and remain a known workspace baseline failure.

`Ran:`

```text
nix develop --command cargo nextest run --test \
  land_surface_energy_balance_authority_contract \
  surface_liquid_hydrology_custody_authority_contract \
  snow_stage3_legacy_predecessor_bridge_contract \
  snow_stage3_persistent_accumulation_shadow_contract \
  snow_stage3_terminal_receiver_authority_contract \
  snow_stage3_turbulent_operator_reconciliation_contract \
  snow_stage3_wind_source_custody_contract
```

Run ID `a1aba459-59c8-494a-892f-e4076d7c04b0`: `56 passed, 0 skipped`.
