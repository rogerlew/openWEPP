# Source Map

Status: `QUEUED`
Evidence mode: `Static scaffold placeholder`

The executing agent must replace this placeholder with a current source map.

Initial surfaces to inspect:

| Surface | Candidate file/function | Closure question |
| --- | --- | --- |
| Active HBP producer | `crates/openwepp-runner/src/hillslope/04_direct_publication.rs` | Does the active routed outlet shape source `hourly_runoff_volume_m3` and `hourly_sediment_mass_kg`? |
| HBP parser/intake | HBP parser plus `crates/openwepp-runner/src/watershed_supervisor.rs` | Are minor-1 hourly arrays parsed and validated as a pair? |
| Typed handoff | `HillslopeContribution` in `crates/openwepp-watershed-orchestrator/src/lib_mod/network_frame.rs` | Are arrays carried into the production watershed frame? |
| Route consumer | `assemble_direct_incoming_peak_partition` in `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/direct.rs` | Does production dispatch consume hourly runoff distribution? |
| Sediment time-base consumer | watershed kernel sediment-rate calculation | Does production dispatch consume hourly sediment timing? |
| Publication/output | `crates/openwepp-watershed-output/src/writers.rs` if touched | Which output or diagnostic proves hourly distribution affects the real consumer? |

The final artifact must also answer: what still reads the old daily scalar path,
and why it cannot carry the closure claim.
