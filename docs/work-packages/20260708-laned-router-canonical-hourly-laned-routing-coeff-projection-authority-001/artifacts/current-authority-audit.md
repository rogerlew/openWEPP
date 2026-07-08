# Current Authority Audit

Status: complete.
Evidence class: Static.

## Ordering

M-T2A is complete. Its final disposition is
`EXECUTED-COMPLETE-AUTHORITY`, and its handoff is M-T2B-ready. This package
therefore executed after the required groundwater/baseflow authority package.

## Current Coefficient Authority

`SC-OFEROUTE-001` rev 46 made Lane D default activation conditional on complete
native `routing_coefficients`; rev 47 retained that selector posture while
changing local numerics. The current rule is: all scheduled lanes with
coefficients run active by default, no-coefficient runs remain legacy/off, and
mixed authority fails closed.

The plant-file specification exposes the native `ow-lanuse-1` route block as
five explicit values:

- `k_o`;
- `form_C_d`;
- `D_r_m`;
- `lambda`;
- `vegetation_C_d`.

The parser stores the optional block as a five-field
`RoutingCoefficientExtension` in
`crates/openwepp-input-contract/src/parsers/management.rs:2038-2064`. Parser
tests cover native forest and native cropland examples in
`tests/integration/infile_management_parser_contract.rs:921-975`.

The production builder reads only projected `route_*` fields:
`crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs:818-873`.
It fails closed when some schedule crop slots have route authority and others do
not, and it validates all five fields at lines `876-1010`.

The default resolver counts only lanes with this authority:
`00_builders_and_authority.rs:24-47` and
`crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs:85-125`.

## Existing Fail-Closed Evidence

Integration tests already pin the current policy:

- `tests/integration/laned_shadow_h2637.rs:299-315` proves no-coefficient
  default does not attach active routing.
- `tests/integration/laned_shadow_h2637.rs:429-440` proves explicit active
  fails closed without route coefficients.
- `tests/integration/laned_shadow_h2637.rs:546-571` proves mixed and malformed
  routing coefficients fail closed.

## Impact

The current rule is safe but narrow. It keeps coefficient-absent legacy cropland
on the legacy/off path, which means single-OFE and MOFE groundwater/baseflow and
watershed consumers cannot assume one universal hourly Lane D production path.
The only safe broadening would be a ratified producer that supplies all five
static Lane D operands with provenance and bounds.
