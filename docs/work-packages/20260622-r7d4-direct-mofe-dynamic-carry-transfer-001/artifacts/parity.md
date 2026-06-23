# Parity Evidence

Status: executed-held.

## Focused Fixture

- Ran:
  `cargo test -p openwepp-hillslope-orchestrator r4j_runon_carry_consumes_dynamic_transfer_arrays_and_feeds_total_runon --lib`
  passed.
- Ran:
  `cargo test -p openwepp-hillslope-orchestrator r7d4_publication_capture_copies_mofe_carry_to_downstream_lane_before_r4j --lib`
  passed.
- Ran:
  `cargo test -p openwepp-hillslope-orchestrator r3c_lane_transfer_span_projects_multilane_topology --lib`
  passed.
- Static: the focused tests distinguish raw dynamic carry buffers from
  area-scaled downstream hydrology consumption. `DirectFrameExecutor`
  publishes raw `ui_SCrunf`/`ui_LfCrf` arrays into the downstream lane, and
  R3A/R4J apply `upstream_area_ratio` at consumption.

## H2637

- Ran:
  `target/release/openwepp-cli-hill --run-dir /tmp/r7d4-h2637-5day/run --run-file /tmp/r7d4-h2637-5day/default.run --output-dir /tmp/r7d4-h2637-5day/manifests/default-cleaned-r7d4`.
  Result: exit 0, `default elapsed=0.70 rss_kb=51088`.
- Ran:
  `target/release/openwepp-cli-hill --run-dir /tmp/r7d4-h2637-5day/run --run-file /tmp/r7d4-h2637-5day/direct.run --output-dir /tmp/r7d4-h2637-5day/manifests/direct-cleaned-r7d4 --direct-production-executor`.
  Result: exit 0, `direct elapsed=1.12 rss_kb=63312`.
- Ran: WAT and PASS value comparison with `.venv/bin/python` and `pyarrow`.
  `H2637.wat.parquet` had no residuals; `H2637.pass.parquet` had no
  residuals.
- Ran: byte comparison. `H2637.wat.parquet` and `H2637.pass.parquet` are
  byte-identical. `H2637.loss.json` differs only by `run_name`;
  `H2637.plot.parquet` is text in this harness and differs only on
  `run_name=...`.
- Static: direct manifest `direct_runtime_counters.compatibility_edge_invocations`
  is `0`.

## Residuals

- `H2637.hbp` still differs: both files are `5254` bytes; there are `34`
  differing bytes; first differing byte is 1-based offset `838`.
- Decoded HBP event-payload slots:
  - offset `928`: default `f64 3.63e-08`, direct `0.0`;
  - offset `936`: default `i64 600000000`, direct `0`;
  - offset `944`: default `i64 300000000`, direct `0`;
  - remaining differences are payload/checksum bytes.
- Static: these residuals map to HBP sediment concentration plus total
  detachment/deposition payload. Direct publication still builds
  `DirectPublicationErosionOperands` from zero authority or narrow runtime
  scalar bridge rather than a direct EROD14/EROD15 sediment producer.
- Disposition:
  `HOLD-R7D4-HBP-EROD14-SEDIMENT-PRODUCER-ABSENT`.
