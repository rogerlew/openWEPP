# Current Scheduler And Cadence Map

Status: `Static production trace complete`

The runner builds climate/snow, frost, canopy interception and liquid input,
then the executor applies normalization, storage bounds, residue/decomposition,
growth, beginning-storage capture, frost ingress, local liquid and runon,
WB14 infiltration/depression, WB18 percolation, surface ET, retained-liquid
projection, lateral/drainage, root uptake, snow coupling, saturation addback,
runoff partition, peak flow, WAT5, storage reconciliation and hydrology
projection.

The exact immutable same-day shadow hook is after `seed_day_frame` and
`apply_publication_day_input`, before `run_day_spans_hydrology`. At that point
infiltration and runoff have not occurred. `DirectDayFrame`, `DirectLaneFrame`
and `DirectRunFrame` are cloneable and comparable in memory.

The input builder holds additional mutable residue/canopy/snow state in
`RefCell`s and is neither cloneable nor serializable. A same-day clone is
therefore lossless; longitudinal paired shadow execution needs a complete new
snapshot envelope or independent full runs.

Legacy native ET is consumed in `run_r4n_surface_et_span` and
`run_r4n_root_uptake_span`. The native shadow must replace/bypass both spans on
its clone. Zeroing only one scalar is insufficient because PMET component
fields independently select the legacy branch.
