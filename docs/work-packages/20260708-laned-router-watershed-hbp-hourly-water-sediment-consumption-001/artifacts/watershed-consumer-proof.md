# Watershed Consumer Proof

Status: `EXECUTED`
Evidence mode: `Static` plus `Ran`

Real consumer path:

1. Producer:
   `04_direct_publication.rs` writes the HBP minor-1 `V_h`/`S_h` pair.
2. Parser:
   `payload_validator.rs` parses and validates both arrays as a 24-slot pair.
3. Run-level inventory:
   `watershed_supervisor.rs` validates latest EVENT vectors and hourly
   sediment closure.
4. Supervisor handoff:
   `openwepp-cli-watershed.rs` clones the parsed arrays into
   `HillslopeContribution`.
5. Typed frame:
   `WatershedNetworkFrame` stores the contribution.
6. Kernel consumer:
   `Ws10ChannelImpoundmentKernel::assemble_direct_incoming_peak_partition`
   consumes `V_h` for inlet peak/volume/duration and
   `assemble_direct_incoming_sediment_load_and_capacity` consumes `S_h` for
   sediment mass/time base.

Ran:

- `cargo nextest run --test wshedw5_typed_watershed_runtime_contract`:
  18 passed.
- `cargo nextest run -p openwepp-runner --test mt3_hbp_hourly_consumer_contract`:
  1 passed. This test writes two schema-1.1 HBP EVENT fixtures with identical
  scalar peak/duration and identical daily hourly runoff/sediment totals, runs
  the production watershed CLI in reuse mode, and proves `ebe_pw0` peak runoff
  and sediment yield change when only the hourly distribution changes.
- `cargo nextest run -p openwepp-runner --test watershed_cli_behavior_contract wshedw7r_p102_sediment_active_fixture_publishes_nonzero_sediment_and_jobs_identity`:
  1 passed, 28 skipped.
- Release CLI fixture under `/tmp/mt3_p102_release`: success, with watershed
  outputs and generated HBP present.

Negative proof:

- Sidecars and reports do not carry the closure claim; the CLI handoff reads
  arrays only from `HbpLatestEventState::EventPayload`.
- All-no-hourly contributors retain the contract-authorized fallback.
- Mixed/malformed hourly authority now fails before any routed channel state is
  applied.
