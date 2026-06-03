# Pre-Implementation Contract Gate

Status: completed

Evidence mode: ran

Ran:

- Command:
  `cargo test -p openwepp-runner hphys0262_trace_row_captures_pmet_demand_seeding_lineage -- --nocapture`
- Result: failed as expected before production code edits.
- Failure signal: `document["pmet_sidecar_present"]` was `Null` rather than
  `1.0`, proving the required PMET branch lineage fields were not yet emitted.
