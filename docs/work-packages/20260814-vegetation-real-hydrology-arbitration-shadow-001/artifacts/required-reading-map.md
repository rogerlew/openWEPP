# Required Reading Map

Status: `complete before production edits`

Evidence class: `Static`

## Applicable instructions

Instruction discovery was run for every intended write surface. The binding
chain is:

| Surface | Instructions |
|---|---|
| workspace and Cargo files | `AGENTS.md` |
| `crates/openwepp-kernel-contract/**` | `AGENTS.md`; `crates/AGENTS.md` |
| `crates/openwepp-vegetation/**` | `AGENTS.md`; `crates/AGENTS.md` |
| `crates/openwepp-hillslope-orchestrator/**` | `AGENTS.md`; `crates/AGENTS.md` |
| `tests/integration/**` | `AGENTS.md`; `tests/AGENTS.md` |
| this package | `AGENTS.md`; `docs/work-packages/AGENTS.md` |

## Governance and authority read

The executor read the root and nested instructions, package procedure,
campaign coordinator, Child-1 terminal disposition, and the complete current
authority relevant to this boundary:

- `SC-WATBAL-001` same-snapshot ownership, OFE state and routing addenda;
- `SC-VEGETATION-001@12` V8 potential/final coupled transaction;
- `SC-VEGETATIONTRANSACTION-001@2` source-keyed D/A/F protocol;
- `SC-LANDSURFACEENERGY-001@3` beginning-store authorization ordering;
- completed V7 implementation state/owner/equation maps and terminal evidence;
- Child-1 V8/LSE definitions, schemas, transaction ordering and handoff.

## Production implementation read

- `direct_runtime/00_core_frames.rs`: lane, run and day-frame seed/commit state;
- `direct_runtime/02_state_reports.rs`: `DirectWaterState`;
- `direct_runtime/03_executor.rs`: production span order and commit boundary;
- `direct_runtime/subsurface.rs`: complete ordered layer state;
- `direct_runtime/evapotranspiration.rs`: native R4N surface and root ET;
- `direct_runtime/01_publication.rs` and `laned_active.rs`: OFE and area lineage;
- vegetation `transaction.rs`, `water_phase.rs`, occupancy request/capped-pass
  resources, and the current diagnostic proportional owner;
- dependency-neutral resource transaction types and proportional arbitration.

The exact production call graph, state projection, scheduler decision, and V8
identity delta are recorded in the adjacent package artifacts.
