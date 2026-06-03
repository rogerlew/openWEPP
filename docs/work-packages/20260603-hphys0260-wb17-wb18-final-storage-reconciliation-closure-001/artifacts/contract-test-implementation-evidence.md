# Contract-Test Implementation Evidence

Status: completed

Evidence mode: ran

## Tests

- Static: added
  `hphys0260_trace_row_captures_wb17_wb18_storage_diagnostics` in
  `crates/openwepp-runner/src/hillslope/mod.rs`.
- Static: the test constructs a post-`plant_root_uptake` trace surface with
  WB17 aggregate/layer `UPi`/`Ui`, WB18 residual/depth/frozen aggregate
  components, `D`, `Pe`, final `Ep`, `Etp`, and `Ws`.
- Ran: focused test passed after implementation:

```text
cargo test -p openwepp-runner hphys0260_trace_row_captures_wb17_wb18_storage_diagnostics -- --nocapture
```

- Ran: trace writer JSON serialization coverage also passed:

```text
cargo test -p openwepp-runner hphys0245_trace_writer_serializes_jsonl_rows -- --nocapture
```
