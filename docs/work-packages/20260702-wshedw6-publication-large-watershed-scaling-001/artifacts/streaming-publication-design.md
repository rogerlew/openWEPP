# Streaming Publication Design

Status: `passed`

Evidence mode: `Static`

Record the direct typed writer/streamer design, consumer path, schema
preservation strategy, and any retained non-production diagnostic adapters.

## Design

Static: implement a direct typed publication writer in
`crates/openwepp-watershed-output/src/writers.rs`:

- add `write_typed_publication_parquet_outputs(outputs, publication_frames)`;
- add a shared internal output-record trait used by both existing
  `WatershedInterchangeRowSeed` edge writers and the new typed frame writer;
- implement that trait for `WatershedPublicationFrame` so Arrow columns are
  built directly from typed publication-frame fields;
- keep all schema constructors, metadata, unit checks, compression, required
  output paths, and column names unchanged;
- emit nulls for unavailable typed publication operands instead of inventing
  zeroes on the public typed frame path;
- make `openwepp-cli-watershed` call the typed writer directly after
  `publish_typed_routing_report`.

This is direct typed writing rather than a new adapter. The public watershed CLI
will no longer construct `WatershedInterchangeRowSeed`, call
`publication_frame_to_row_seed`, or route publication through its local
`write_watershed_interchange_outputs` wrapper.

## Retained Edge Surface

`WatershedInterchangeRowSeed` and `write_interchange_parquet_outputs*` remain in
`openwepp-watershed-output` for current non-public aggregation callers:

- `openwepp-cli-totalwatsed3`;
- `openwepp-runner::watershed_wat` WAT aggregation helper;
- existing unit tests that exercise multi-row schema behavior.

They are not part of the W6 public CLI closure claim.

## Consumer Path Proof Target

The source-guard test will require:

- public CLI source contains `write_typed_publication_parquet_outputs`;
- public CLI source contains `publish_typed_routing_report`;
- public CLI source does not contain `publication_frame_to_row_seed`,
  `write_watershed_interchange_outputs`, or `WatershedInterchangeRowSeed`;
- the direct typed writer test emits all required parquet outputs from a
  `WatershedPublicationFrame` and preserves key public values.
