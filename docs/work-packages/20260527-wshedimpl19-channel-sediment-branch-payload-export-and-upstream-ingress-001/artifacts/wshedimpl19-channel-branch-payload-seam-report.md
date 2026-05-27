# WSHEDIMPL19 Channel Branch Payload Seam Report

Status: complete
Evidence mode: static+ran
Date: 2026-05-27

## Static
- WS10 channel runtime now exports class payload families for downstream use:
  - `particle_class_count`
  - class-indexed `particle_flow_fraction`
  - class-indexed `particle_diameter_m`
- WS10 channel runtime now reads upstream channel dependency sediment payloads
  and folds them into class-aware aggregation for current-node `qsed`/`tc`
  publication continuity.
- WS11 vectors confirm:
  - export-family publication is present and normalized,
  - downstream channel `qsed` continuity uses upstream channel payload ingress.
- Residual parity blockers remain open:
  - `case12/case34/detach/dcap/enddet`
  - full `chnero/chnrt` inflow-partition parity

## Ran
- `cargo fmt --check` -> pass
- `cargo clippy --workspace --all-targets -- -D warnings` -> pass
- `cargo test --workspace` -> pass
- `cargo deny check` -> pass (warnings only)
