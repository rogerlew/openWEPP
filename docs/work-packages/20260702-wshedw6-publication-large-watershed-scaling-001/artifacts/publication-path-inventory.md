# Publication Path Inventory

Status: `passed`

Evidence mode: `Static`

Record the pre-edit and post-edit path from `WatershedPublicationFrame` or
typed projection state to the required watershed parquet outputs. Classify each
stage as typed production, compatibility-shaped staging, diagnostic, or
out-of-scope.

## Pre-Edit Inventory

Static: inspected
`crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs`,
`crates/openwepp-watershed-orchestrator/src/lib_mod/network_frame.rs`, and
`crates/openwepp-watershed-output/src/writers.rs` before production edits.

| Stage | File / symbol | Classification | Disposition |
| --- | --- | --- | --- |
| Public CLI entrypoint | `openwepp-cli-watershed::run` | typed production consumer | Retain. It parses the runfile, supervises hillslope jobs, validates pass inventory, builds `WatershedNetworkFrame`, dispatches typed routing, and writes public watershed outputs. |
| Typed routing frame | `WatershedNetworkFrame::from_parsed_inputs` | typed production state | Retain. It replaces the deleted W5 writeback surface. |
| Typed contribution ingestion | `network_frame.add_hillslope_contribution` | typed production state | Retain. It consumes validated latest-event pass payloads into `HillslopeContribution`. |
| Typed dispatch | `execute_watershed_dispatch_with_frame` | typed production execution | Retain. W5 source guards prohibit the deleted watershed request/writeback runtime. |
| Typed publication projection | `WatershedNetworkFrame::publish_typed_routing_report` and `WatershedPublicationFrame` | typed production projection | Retain. It is the W6 source object for public output writing. |
| CLI publication staging | `publication_frame_to_row_seed` | compatibility-shaped staging | Remove from the public watershed CLI path. It maps typed `WatershedPublicationFrame` into `WatershedInterchangeRowSeed` before writing. |
| CLI writer wrapper | `write_watershed_interchange_outputs` | compatibility-shaped staging | Remove from the public watershed CLI path. It forwards row-seed slices to the watershed-output crate. |
| Output writer schema | `write_interchange_parquet_outputs*` | retained edge writer for existing tools | Retain for `totalwatsed3` and WAT aggregation callers that still build explicit `WatershedInterchangeRowSeed` records. Do not use it for the public W6 watershed CLI closure claim. |
| Required outputs | `ebe_pw0`, `chan.out`, `chanwb`, `chnwb`, `soil_pw0`, `totalwatsed3`, `loss_*` parquet files | public output schema | Preserve the file set, column names, metadata, and Arrow/Parquet schema constructors. W6 removes fake publication defaults by emitting null for unavailable typed operands and by using source-slope area when available. |

## Post-Edit Path

Static: public `openwepp-cli-watershed` calls
`write_typed_publication_parquet_outputs` from `openwepp-watershed-output` with
`&[WatershedPublicationFrame]`. The writer preserves the existing
Arrow/Parquet schema constructors and column formulas while reading typed
publication-frame fields directly. The row-seed writer remains an edge helper
for non-public watershed aggregation tools but cannot carry the W6 public CLI
closure claim.
