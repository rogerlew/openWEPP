# contract test evidence

Status: red-before / corrected target passed

Evidence mode: Ran

The existing registered EB-04W contract target now carries contract-marker,
warm typed-snow, mixed-event, and warm all-rain vectors. Its first pre-edit run
failed only the required activation assertion; see
`pre-implementation-contract-gate.md`.

Ran after terminal in-envelope correction:
`cargo nextest run --test snow_surface_eb04w_accumulation_melt_diagnostics_contract`
passed `6/6`, including exact/just-over snowfall activation and the retained
warm-snow, mixed-event, warm all-rain, contract-marker, and real-consumer
source obligations.
