# Test Plan

Status: `EXECUTED-HOLD-GWDSV-CHANNEL-CONSUMER`

Evidence: `Static + ran`

| Contract vector | Coverage | Status |
|---|---|---|
| `TV-GWBASEFLOW-001` disabled branch | Missing `gwcoeff.txt` parses through existing `GwcoeffFile` missing branch and configures disabled direct groundwater authority; disabled runs publish zero `Base`. | implemented by parser handoff and WAT fixtures |
| `TV-GWBASEFLOW-002` one-hillslope recurrence | `gwbaseflow_linear_reservoir_recurrence_uses_prior_day_exports` verifies two-day recurrence, prior-day outflow debit order, and generated `Qb`/`Qs`. | PASS |
| `TV-GWBASEFLOW-003` domain failures | `gwbaseflow_exports_over_accepted_storage_fail_closed` verifies coefficient sums that export more than accepted storage fail closed. Parser tests already cover malformed present `gwcoeff` domain/shape failures. | PASS |
| `TV-GWBASEFLOW-004` consumer proof | `r6a_direct_projection_consumers_read_publication_frame_operands` proves `groundwater_baseflow_mm` maps to WAT `Base`; existing `watershed_wat.rs` reads optional `Base` into `baseflow_mm` and `channel_baseflow_m3`. | PASS for `gwbfv`; HOLD for `gwdsv` |
| `TV-GWBASEFLOW-005` `bftharea` threshold | Parser state is carried into `DirectGroundwaterAuthority`, but watershed/channel threshold consumption is not in this hillslope package. | HOLD |
| `TV-GWBASEFLOW-006` namespace separation | Runtime uses `hydrology_projection.deep_percolation_m` only as recharge, keeps `latqcc` as lateral export, maps generated baseflow only to WAT `Base`, and never reads `cbase`. | PASS for implemented surfaces; channel branch HOLD |
| `TV-GWBASEFLOW-007` active MOFE ledger | `gwbaseflow_mofe_recharge_aggregates_lane_deep_percolation` verifies lane-area weighted MOFE recharge. `laned_active_lane_source` remains unchanged and reads no groundwater output. | PASS |
| `TV-GWBASEFLOW-008` publication anti-alias | WAT `Base` is nullable and unit-registered; manifest active summary reports generated groundwater totals. Disabled versus generated state is distinguishable from manifest/process authority, but `gwdsv` has no real public consumer yet. | HOLD for `gwdsv` consumer |

Focused commands run:

- `cargo check -p openwepp-hillslope-orchestrator -p openwepp-runner -p openwepp-hillslope-output -p openwepp-sim-contract`
- `cargo test -p openwepp-hillslope-orchestrator gwbaseflow -- --nocapture`
- `cargo test -p openwepp-runner r6a_direct_projection_consumers_read_publication_frame_operands -- --nocapture`
- `cargo test -p openwepp-hillslope-output hillslope_wat -- --nocapture`
- `cargo test -p openwepp-sim-contract units -- --nocapture`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo nextest run --workspace --profile quick`
- `cargo nextest run --workspace --profile full`
- `cargo deny check`
