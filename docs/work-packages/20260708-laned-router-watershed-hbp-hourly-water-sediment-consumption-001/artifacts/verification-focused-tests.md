# Focused Verification

Status: `COMPLETE`
Evidence mode: `Ran`

Commands:

- `cargo fmt --check`: PASS.
- `cargo nextest run --test wshedw5_typed_watershed_runtime_contract`:
  PASS, 18/18.
- `cargo nextest run -p openwepp-runner --test mt3_hbp_hourly_consumer_contract`:
  PASS, 1/1.
- `cargo nextest run --test infile_hbp_parser_contract`: PASS, 25/25.
- `cargo nextest run -p openwepp-watershed-orchestrator`: PASS, 9/9.
- `cargo nextest run -p openwepp-runner --test watershed_cli_behavior_contract wshedw7r_p102_sediment_active_fixture_publishes_nonzero_sediment_and_jobs_identity`:
  PASS, 1 passed, 28 skipped.

Verification scope:

- Contract-derived WS10 all-hourly/no-hourly authority.
- Equal-total/different-hourly-distribution water and sediment sensitivity.
- Mixed, malformed, and dependency-node fail-closed branches.
- Production CLI HBP schema-1.1 hourly pair handoff into watershed output.
- Existing HBP parser and orchestrator package tests.
