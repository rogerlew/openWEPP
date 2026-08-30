# Line-count governance

Status: `PASS`

Evidence mode: `Static + Ran`

Touching the pre-existing 3,252-line `open_snow.rs` required a mechanical
split. The unchanged
`execute_precomputed_terminal_accepted_endpoint` implementation moved into
`open_snow_terminal_accepted_endpoint.rs` through `include!`; its pre-move and
post-move body digest was
`6da236cb20f9f83088d8ff120294ebc15ed8460b646747a31133e5912071fdd2`.

Terminal counts are 2,726 lines for `open_snow.rs` and 529 lines for the new
include file. Formatting, compilation, focused tests, integration contracts,
and the canonical one-day test pass after the split. The authority impact map
contains an exact critical SnowEnergy binding for the new file.
