# Pre-Implementation Contract Gate

Status: `PASS / expected RED established`

Evidence class: `Ran`

Version 5 and `INV-SNOWENERGY-027` were authored before production edits. The
captured-fragment real-consumer regression then failed on the existing tree
with `SnowLayerAggregateMismatch`: reported depth `0.25 m`, expected depth
`0.25000000100777475 m`, and the exact omitted fragment retained in the typed
snapshot.

Ran:

`cargo nextest run --test snow_surface_eb03_contract --test snow_surface_eb03_runtime density_handoff_retains_captured_subnanometer_swe_fragments_and_state`

Result: expected failure, run `7ec7c95f-c4a4-4d27-bb92-fad0b3386614`.
