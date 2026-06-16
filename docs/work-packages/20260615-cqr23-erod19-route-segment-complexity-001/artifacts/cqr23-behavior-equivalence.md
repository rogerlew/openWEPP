# CQR23 Behavior Equivalence

Status: complete.

Static: production edits are private helper extraction in
`hydrology_phase_erod19.rs`. The public target signature remains:
`pub(crate) fn run_erod19_route_segment_migration(request:
&HillslopeKernelRequest<'_>, erod13_state_updates: &[WritebackField]) ->
Result<Vec<WritebackField>, Wb11HydrologyKernelGuardError>`.

Static: protected behavior preserved:

- EROD14 wave-2 disabled gate returns an empty update vector.
- Route topology, segment symbol names, writeback order, minimum and maximum
  bounds, and state update symbols remain unchanged.
- EROD13 update precedence over stale state for `theta`, `phi`, and `taucn`
  remains unchanged.
- Legacy fallback inputs for `theta`, `phi`, and `taucn` remain available when
  direct state/update values are absent.
- Formula expressions were moved into helpers without changing operand order
  inside each expression.
- No typed guard error variant, stable symbol string, unit, parser behavior,
  dispatch phase, or science-contract behavior was intentionally changed.

Ran: focused characterization was added before production decomposition and
passed before the production refactor:
`cargo test -p openwepp-hillslope-orchestrator cqr23_erod19_route_segment`.

Ran: focused characterization passed after production decomposition and after
adding fallback coverage:
`cargo test -p openwepp-hillslope-orchestrator cqr23_erod19_route_segment`.

Ran: full closure gates passed after production edits:
`cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
`cargo test --workspace`, and `cargo deny check`.
