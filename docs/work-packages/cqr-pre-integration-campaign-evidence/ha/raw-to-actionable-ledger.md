# High-A Raw-To-Actionable Ledger

Evidence class: **Ran** for metrics and source identity; **Static** for semantic
classification.

All 13 fixed-cohort rows are hand-authored eligible behavior. There are no
`R-*` exceptions, `X-*` exclusions, or no-action modules.

| ID | Function | Line | CC | Coverage | CRAP | Class / tier | Treatment |
| --- | --- | ---: | ---: | ---: | ---: | --- | --- |
| HA-01 | `cascade.rs::interpolate_unit_discharge` | 90 | 7 | 0.000% | 56.000 | `E-SCIENCE` / science | Actionable |
| HA-02 | `kinematic_wave.rs::CellParameters::alpha_q_celerity` | 405 | 33 | 86.047% | 35.959 | `E-SCIENCE` / science | Actionable |
| HA-02 | `kinematic_wave.rs::KinematicWaveSolver::step` | 1091 | 53 | 91.031% | 55.026 | `E-SCIENCE` / science | Actionable |
| HA-02 | `kinematic_wave.rs::KinematicWaveSolver::run_with_options_and_step_trace` | 1512 | 37 | 92.754% | 37.521 | `E-SCIENCE` / science | Actionable |
| HA-03 | `laned_active.rs::laned_active_route_lane` | 810 | 30 | 83.843% | 33.796 | `E-SCIENCE` / science | Actionable |
| HA-04 | `projection.rs::DirectDayFrame::validate_r4pqz_hydrology_projection_domain` | 214 | 20 | 61.333% | 43.124 | `E-SCIENCE` / science | Actionable |
| HA-05 | `03_executor.rs::DirectFrameExecutor::run_laned_active_publication_stream` | 427 | 43 | 79.835% | 58.160 | `E-SCIENCE` / science | Actionable |
| HA-06 | `01_publication.rs::DirectPublicationDayRow::from_day_frame` | 307 | 30 | 84.810% | 33.154 | `E-SCIENCE` / science | Actionable |
| HA-07 | `00_builders_and_authority.rs::direct_production_typed_growth_crop_authority` | 1875 | 27 | 80.000% | 32.832 | `E-SCIENCE` / science | Actionable |
| HA-08 | `05_runner_execution_and_outputs.rs::execute_direct_publication_stream` | 118 | 17 | 54.545% | 44.141 | `E-PRODUCTION` / science | Actionable |
| HA-08 | `05_runner_execution_and_outputs.rs::write_laned_active_trace_output` | 989 | 8 | 5.344% | 62.279 | `E-PRODUCTION` / science | Actionable |
| HA-09 | `openwepp-cli-hill.rs::run` | 16 | 19 | 60.417% | 41.389 | `E-PRODUCTION` / glue | Actionable |
| HA-10 | `open_wepp_runner.rs::run_hillslope_command` | 41 | 11 | 0.000% | 132.000 | `E-PRODUCTION` / glue | Actionable |

Review A confirmed the real chain from the kinematic-wave solver through the
lane router, executor, publication row, runner sink, and CLI launch surfaces.
HA-01's point sampler is currently bypassed when the conservative integral
handoff exists; ADR-0021 has no dead-code exclusion, so it remains eligible.
Potential semantic removal/exposure is a separately scoped follow-up, not a CQR
hold or an exclusion.

Review B independently reproduced 67 rows/45 modules overall and 13 rows/10
modules for High A using the exact six-field deduplication key. It matched every
current symbol/line and proved all ten source files byte-identical to the metric
source state.
