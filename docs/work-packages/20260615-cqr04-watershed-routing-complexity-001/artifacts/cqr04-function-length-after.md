# CQR04 Function Length After

Static: after-refactor function spans from `routing.rs`.

- Target file line count: 2807.
- Largest remaining target functions:
  - `ws20_case12_class_update`: 103 lines
  - `ws20_segment_hydraulics`: 102 lines
  - `ws20_load_channel_profile`: 99 lines
  - `ws20_route_case4_segment`: 99 lines
  - `ws23_detach_case4_iterative_closure`: 97 lines
  - `ws18_hydchn`: 87 lines
  - `ws20_route_case12_segment_family`: 86 lines
  - `ws20_route_case34_segment`: 74 lines
  - `ws26_dcap_expanding_width_outcome`: 72 lines
  - `ws20_transport_snapshot`: 71 lines

Original long-function reductions:

- `ws18_hydchn`: 176 -> 87 lines plus private geometry helpers.
- `ws18_trncap`: 140 -> 16 lines plus private state/result helpers.
- `ws26_dcap`: 190 -> 62 lines plus private layer/outcome helpers.
- `ws23_detach_case4_iterative_closure`: 219 -> 97 lines plus private
  validation, iteration, sum, and outcome helpers.
- `ws20_route_case12_segment_family`: 783 -> 86 lines plus private profile,
  hydraulic, case routing, diagnostics, and outgoing-mass helpers.

WARN: target file line count increased above the 2000-line governance threshold.
The package explicitly excluded a module/file split, so this is dispositioned as
a scoped follow-on rather than a blocker for the CRAP closure objective.
