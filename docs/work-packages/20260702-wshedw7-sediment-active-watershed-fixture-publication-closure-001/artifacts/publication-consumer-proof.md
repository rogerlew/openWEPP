# Publication Consumer Proof

Status: `passed`

Evidence mode: `Static:` source review plus `Ran:` focused test.

Public watershed CLI path:

1. `openwepp-cli-watershed` builds a typed `WatershedRunPlan`.
2. Generated hillslope jobs produce `H1.hbp`, `H1.loss.json`, and
   `H1.pass.parquet`.
3. `PassInventory::validate` parses the generated HBP with
   `parse_hbp_from_path_with_latest_event_payload`.
4. The CLI constructs `HillslopeContribution` from the HBP payload, including
   detachment, deposition, class fractions, hourly runoff volume, and hourly
   sediment mass.
5. `WatershedNetworkFrame::from_parsed_inputs` receives the contributions.
6. `execute_watershed_dispatch_with_frame` routes the typed frame.
7. `publish_typed_routing_report` emits a `WatershedPublicationFrame`.
8. `write_typed_publication_parquet_outputs` writes public parquet outputs.

Focused guard:

```sh
cargo test -p openwepp-runner --test watershed_cli_behavior_contract \
  wshedw7r_p102_sediment_active_fixture_publishes_nonzero_sediment_and_jobs_identity \
  -- --nocapture
```

Result: `1 passed`.

The test fails if public output no longer consumes generated p102 HBP
detachment/deposition, if hourly HBP sediment no longer closes to
`tdet - tdep`, if `sed_del` becomes zero, or if decoded serial/parallel rows
diverge.
