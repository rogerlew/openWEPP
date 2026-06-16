# CQR23 Line-Count Governance Checklist

Status: complete.

Ran: before target-file line count:

- `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_storage_erosion/hydrology_phase_erod19.rs`:
  `784`
- `docs/work-packages/README.md`: `614`
- `docs/work-packages/cqr-burndown-execplan.md`: `703`

Ran: after line counts:

- `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_storage_erosion/hydrology_phase_erod19.rs`:
  `1143`
- `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/hydrology.rs`:
  `1083`
- `docs/work-packages/README.md`: `614`
- `docs/work-packages/cqr-burndown-execplan.md`: `703`

Static: no touched non-exempt Rust file is at or above `3000` lines.

Ran: before suppression census in the target file:

- Line `319`: `#[allow(clippy::too_many_arguments, clippy::too_many_lines)]`
  on `erod19_depend`
- Line `445`: `#[allow(clippy::similar_names, clippy::too_many_lines)]` on
  `run_erod19_route_segment_migration`

Ran: after suppression census:

- Existing `erod19_depend` suppression remains out of scope.
- `run_erod19_route_segment_migration` no longer carries a `too_many_lines`
  suppression.
- No new broad CQR23 suppression was added.

Warning: target-file line count increased by helper extraction. The increase is
accepted for CQR23 because the production target was reduced from CRAP
`351.9234211799049` to `9.00460855712335`, all new helpers are below CRAP
`30`, and the file remains below the `3000`-line hard stop.
