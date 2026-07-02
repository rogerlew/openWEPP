# Consumer Path Evidence

Status: `passed`

Evidence mode: `Static:` source path inspection plus `Ran:` scaling outputs.

## Producer

- Producer object:
  `openwepp_watershed_orchestrator::WatershedPublicationFrame`.
- Producer method:
  `WatershedNetworkFrame::publish_typed_routing_report`.
- Runtime state:
  `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs` builds
  `WatershedNetworkFrame`, adds typed `HillslopeContribution` records, executes
  `execute_watershed_dispatch_with_frame`, then publishes the typed frame.

## Public CLI Handoff

The public watershed CLI calls:

```rust
write_typed_publication_parquet_outputs(&runfile.outputs, &[publication_frame])
```

This is the real downstream consumer path for W6 scaling runs:

- `tests/fixtures/watershed/onshore-xenophobia/runs/case.run`
- `/tmp/wshedw6_onshore_scaling_final/jobs1-full/out`
- `/tmp/wshedw6_onshore_scaling_final/jobs48-full/out`
- `/tmp/wshedw6_carnivorous_scaling_final/jobs1-full/out`
- `/tmp/wshedw6_carnivorous_scaling_final/jobs32-full/out`

All runs emitted the `14` required watershed parquet outputs.

## Writer Consumer

`crates/openwepp-watershed-output/src/writers.rs` exposes
`write_typed_publication_parquet_outputs(outputs, &[WatershedPublicationFrame])`.
The writer uses a shared internal `WatershedOutputRecord` projection trait so
schemas and field formulas stay identical for retained row-seed tests and the
new typed publication frame path.

## Negative Proof

Public CLI source guard:

- no `publication_frame_to_row_seed`;
- no `write_watershed_interchange_outputs`;
- no `WatershedInterchangeRowSeed` import or construction.

Retained row-seed writer helpers are not on the public W6 closure path; they
remain inside the output crate for existing non-public/edge writer coverage.

## Output Proof

`artifacts/scaling/onshore-xenophobia-scaling-summary.json` and
`artifacts/scaling/carnivorous-adobo-scaling-summary.json` record successful
public CLI runs and row/content identity across job counts for all required
watershed outputs.

The public CLI writes nulls for unavailable typed operands rather than routing
through a compatibility row seed or filling channel-balance fields from
unrelated routing scalars.
