# Field Ownership And Public API Inventory

Status: `complete / terminal implementation reconciled`

Evidence mode: `Static: field and workspace-consumer inventory`

## Field Classification

All fields of `DirectSnowAccumulationMeltDiagnostics` are verbose trace fields:
daily wind/dewpoint/canopy and the hourly precipitation, phase, temperature,
radiation, cloud, melt-component, routed-liquid, capacity/store/release,
sublimation, pack geometry/density, and modeled-redistribution arrays. None is
read by production runtime after snow partition.

`DirectSnowStage3Diagnostics` is split as follows:

- production outcome: `enabled`, `meltwater_temperature_c`, `sublimation_m`;
- compact liquid ledger: `incoming_liquid_m`, `routed_liquid_m`, signed
  `retained_liquid_m`, `refrozen_liquid_m`, and
  `liquid_closure_residual_m`;
- verbose trace: cold-content, surface/conduction/refreeze/closure energy,
  shortwave/longwave/latent/vapor and latent-mass energy, cold-content export,
  mass-latent residual, unused positive energy, suspension/collapse extrema,
  and all 24 `hourly_surface_energy` records.

No field is obsolete. Existing top-level `raw_melt_m`,
`redistributed_melt_m`, `snowpack_swe_loss_m`, `rain_released_m`, and
`routed_melt_m` become the owned upstream compact ledger rather than remaining
duplicate partition fields.

## Public Surface Inventory

- The orchestrator crate is `publish = false` at version `0.1.0`.
- The three diagnostic/result types are exported through `src/lib.rs`, but
  repository search found consumers only in the orchestrator, runner, and
  workspace integration tests.
- CLI, runfile, WAT, HBP/PASS, loss, manifest, watershed, and schema-v4 JSONL
  surfaces are unchanged.
- `compute_direct_snow_liquid_partition_from_typed` remains available and
  preserves verbose-diagnostic behavior for workspace/API compatibility.
- A new explicit capture-aware compute method is additive. The runner alone
  moves to it.
- Direct field users migrate mechanically to owned ledger/outcome fields or
  the optional verbose payload. Since the crate is non-published and every
  workspace consumer is reconciled in the same package, this is a reviewed
  internal diagnostic API decomposition, not an external breaking release.

## Pre-Edit Source Consumers

`DirectSnowAccumulationMeltDiagnostics` is read only by the schema-v4 formatter
and diagnostic tests. `DirectSnowStage3Diagnostics` is read by the formatter,
Stage-3 tests, and direct-runtime meltwater-temperature projection. The latter
is the production consumer that requires the separate outcome. No downstream
consumer requires the verbose energy or hourly arrays.

Any newly discovered consumer outside this inventory is a hold until migrated
without restoring eager payload carriage.

## Terminal Reconciliation

The runner's Snowbench CoE-melt diagnostic was an additional repository-owned
consumer of the five upstream scalars; it migrated mechanically to
`DirectSnowSolidToLiquidLedger` and the package write set records that
discovery. Direct-runtime tests and snow science integration tests were also
mechanically migrated to the new typed ownership. No external crate, CLI,
runfile, serializer, or downstream publication consumer was discovered.

The final direct runtime carries only both compact ledgers and
`DirectSnowStage3Outcome`; it does not carry `DirectSnowVerboseDiagnostics`.
The existing compatibility compute entry requests verbose capture for
repository tests, while the production runner resolves capture before its
solve. Workspace all-target compilation and exact real-output identity close
the inventory.
