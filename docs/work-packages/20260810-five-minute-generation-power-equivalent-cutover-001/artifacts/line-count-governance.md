# Line Count Governance

Status: `PASS with three existing-file WARNs`

Evidence mode: `Ran`

Exact changed-Rust line counts:

| File | Lines | Disposition |
|---|---:|---|
| runner `03_tests.rs` | 2,905 | WARN; split intent: move domain fixtures into existing `tests03/` modules by authority surface |
| orchestrator `runoff.rs` | 2,869 | WARN; split intent: extract WB14 projection/binning helpers by hydrologic responsibility |
| orchestrator `00_core_frames.rs` | 2,713 | WARN; split intent: move phase-owned frame types into focused direct-runtime modules |
| runner `05_runner_execution_and_outputs.rs` | 1,912 | PASS |
| orchestrator `03_executor.rs` | 1,769 | PASS |
| sim-contract `output_catalog.rs` | 1,551 | PASS |
| runner `00_runner_intake_and_lane_setup.rs` | 1,388 | PASS |
| orchestrator `01_publication.rs` | 1,259 | PASS |
| runner `04_direct_publication.rs` | 1,215 | PASS |
| `sim_contract_boundary_unit_registry.rs` | 1,155 | PASS |
| orchestrator `direct_runtime.rs` | 877 | PASS |
| new `subhourly_generation.rs` | 644 | PASS |
| new `hillslope_wat_subhourly.rs` | 596 | PASS |
| runner `runfile_helpers.rs` | 321 | PASS |
| output `contracts.rs` | 299 | PASS |
| orchestrator `lib.rs` | 201 | PASS |
| `subhourly_water_output_roundtrip.rs` | 143 | PASS |
| `hbp_subhourly_exclusion_contract.rs` | 87 | PASS |
| `subhourly_generation_contract.rs` | 80 | PASS |
| output `writers.rs` | 65 | PASS |
| `subhourly_generation_properties.rs` | 41 | PASS |
| output `lib.rs` | 9 | PASS |

No changed Rust file reaches the 3,000-line blocker. WAT5 production logic and
the typed writer remain in focused sub-650-line modules; the three WARNs are
pre-existing multi-authority owners with explicit split directions.
