# Implementation And Test Evidence

Status: passing focused implementation evidence

Evidence mode: Ran

- `cargo test -p openwepp-hillslope-orchestrator terminal`:
  8 passed. This covers analytical pure-melt, pure-sublimation, joint
  melt/sublimation, cooling plus deposition/refreeze, request/state mismatch,
  the real hourly carrier seam, localized exhaustion, censored remaining time,
  and exact empty terminal layer state.
- `cargo nextest run -p openwepp-hillslope-orchestrator -p openwepp-runner`:
  697 passed. This re-ran both touched crates, including historical
  Stage 3 and runner behavior.
- `cargo nextest run -p openwepp-runner terminal`: 4 passed, including two
  parsed schema-v8 reconstruction/poison tests.
- The runner consumer in `00j_snow_terminal_event_trace.rs` independently
  reconstructs solid, liquid, complete-component energy, and phase energy,
  adaptive LTE, interval chronology, resolved-state liquid custody, and phase
  energy, and rejects post-event snow flux before trace-file creation.
