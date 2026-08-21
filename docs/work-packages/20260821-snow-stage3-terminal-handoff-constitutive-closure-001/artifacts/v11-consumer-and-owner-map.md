# V11 consumer and owner map

Status: `REAL SNOW-FREE CONSUMER IDENTIFIED / SNOW-COVERED PATH BLOCKED`.

`Static:` The V11 authority and complete manifest are implemented by
`openwepp-vegetation/src/v11.rs`; the current seven owner IDs are vegetation,
snow, land_surface_energy, surface_liquid, hydrology, bgc, and soil_thermal.
`V11ParentTransaction` and `execute_v11_segment` are real typed transaction
surfaces.

`Static:` `DirectV11RealConsumerStack` invokes the actual V10/V9/LSE/hydrology/
BGC/soil-thermal projection and stages typed ending owners, but its lower
boundary is explicitly snow-free. `DirectV11SnowStage3OwnerExecutor` consumes
the existing typed stack only for the accepted snow-free remainder.

`Static:` The new attachment invokes `execute_v11_segment` through that real
stack and has a fixed-point slab receipt construction for the snow-free case,
but `execute_real_v11_parent` rejects a snow-covered `DirectV9ShadowIntervalInput`
before the call. Therefore there is no current proof of actual V11 evolution
over snow, no covered shared-air carrier join, and no complete seven-owner
ending for the requested terminal path. No synthetic/debug owner bytes are
accepted as closure evidence.
