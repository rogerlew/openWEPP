# CQR04 Function Length Before

Static baseline from pre-refactor `routing.rs`.

- Target file line count: 1934.
- Long target functions:
  - `derive_ws15_channel_sediment_scaffold`: 41 lines
  - `ws18_hydchn`: 176 lines
  - `ws18_trncap`: 140 lines
  - `ws26_dcap`: 190 lines
  - `ws23_detach_case4_iterative_closure`: 219 lines
  - `ws20_route_case12_segment_family`: 783 lines

The primary concentration risk was `ws20_route_case12_segment_family`, which
combined segment-profile loading, hydraulic calculation, case 1/2 routing,
case 3/4 continuation, detachment/deposition diagnostics, and outgoing mass
assembly.
