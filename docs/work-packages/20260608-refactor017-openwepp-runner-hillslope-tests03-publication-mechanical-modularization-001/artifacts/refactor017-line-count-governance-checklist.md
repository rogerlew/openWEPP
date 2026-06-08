# REFACTOR017 Line-Count Governance Checklist

## Evidence mode
- Static: completed
- Ran: completed

## File line counts (post-refactor)

- `crates/openwepp-runner/src/hillslope/tests03/publication.rs`: 20
- `crates/openwepp-runner/src/hillslope/tests03/publication/publication_scheduler_pl_activation.rs`: 152
- `crates/openwepp-runner/src/hillslope/tests03/publication/publication_wb11_seed.rs`: 513
- `crates/openwepp-runner/src/hillslope/tests03/publication/publication_wb13.rs`: 592
- `crates/openwepp-runner/src/hillslope/tests03/publication/publication_wb13_guard.rs`: 350
- `crates/openwepp-runner/src/hillslope/tests03/publication/publication_wb19_wb12_wb16.rs`: 464

## Governance outcome

- No single `.rs` file remains above 2000-line warning threshold.
- No file exceeds 3000-line hard stop threshold.
- Baseline monolith (`publication.rs`) target module was reduced from 2079 lines to 20.
- Decision: `PARITY` (warn conditions resolved) for this module slice.
