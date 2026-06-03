# Pre-Implementation Contract Gate

Status: completed

Evidence mode: ran

## Red Gate

Ran before production trace fields were implemented:

```text
cargo test -p openwepp-runner hphys0260_trace_row_captures_wb17_wb18_storage_diagnostics -- --nocapture
```

Result:

- Ran: command failed before production trace propagation.
- Ran: failure class was compile-time missing trace fields on
  `Hphys0245TraceRow`.
- Ran: representative error was `error[E0609]: no field
  wb17_upi_layers_m on type hillslope::Hphys0245TraceRow`.
