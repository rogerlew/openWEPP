# WSHEDPLAN01 Current Surface Inventory

Status: complete
Evidence mode: static+ran
Date: 2026-05-27

## Static

### Watershed execution and routing surfaces

| Surface | Current state | Evidence | Notes |
|---|---|---|---|
| Deterministic topology-gated scheduler and writeback orchestration (`schedule_watershed_dispatch`, `execute_watershed_dispatch_with_kernel`) | implemented | `crates/openwepp-watershed-orchestrator/src/lib.rs:1234-1364` | Typed precondition/dispatch statuses, stable node order, orchestrator-owned writeback policy are present. |
| WS10 node kernel entry (`Ws10ChannelImpoundmentKernel`) | implemented | `crates/openwepp-watershed-orchestrator/src/lib.rs:1078-1084` | Dispatches typed `channel` / `impoundment` nodes with guard-family continuity. |
| Channel node physics (`run_channel_node`) | partial | `crates/openwepp-watershed-orchestrator/src/lib.rs:652-875` | Uses simplified gain/storage expressions and branch-specific algebra, but does not implement legacy wave-array routing state (`q1`, `qin`, `qlat`, segment loops). |
| Impoundment node physics (`run_impoundment_node`) | partial | `crates/openwepp-watershed-orchestrator/src/lib.rs:879-1074` | Uses simplified area/outflow equations and single-step stage update; does not implement WS12 RK4/adaptive retry/regime-transition lineage. |
| `chan.inp` runtime seam (`build_watershed_runtime_surface_from_chaninp`) | partial | `crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs:499-573` | Projects `ipeak`, `dtchr`, `ntchr`, `nchnum`, `cbase`, but downstream kernel currently consumes only a subset of routing controls. |
| Watershed channel runtime symbol seeding | partial | `crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs:586-625` | Projects only `chnn`, `ctlslp`, `chnk` per node; broader channel routing/sediment state families are not projected. |
| Watershed impoundment runtime symbol seeding | partial | `crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs:640-682` | Projects only `h`, `hfull`, `deltat`, `qinf`; production kernel additionally requires coefficient family fields that are not parser-projected here. |
| Watershed CLI contributor intake from HBP latest-event payload | partial-plus | `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs:296-417` | Includes typed `npart>0` guard and MOFE manifest validation; still seeds zeros for missing latest-event payload instead of full hydrograph/event-family transport surfaces. |
| Missing `chan.inp` behavior in watershed CLI | partial-risk | `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs:267-272`, `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs:1340-1370` | Explicit fallback defaults (`dtchr=3600`, `ntchr=24`, `nchnum=0`, `cbase=0`) are applied with warning; this is deterministic but non-authoritative for parity closure. |

### Output publication surfaces

| Surface | Current state | Evidence | Notes |
|---|---|---|---|
| Watershed output path contract (`validate_output_contract`) | implemented | `crates/openwepp-watershed-output/src/contracts.rs:62-109` | All required parquet outputs are path/extension validated. |
| Watershed schema definitions and metadata scaffolding | implemented (schema-only) | `crates/openwepp-watershed-output/src/writers.rs:15-29`, `crates/openwepp-watershed-output/src/writers.rs:61-79` | Schema objects and dataset metadata keys are built. |
| Watershed parquet emission | stubbed/blocked | `crates/openwepp-watershed-output/src/writers.rs:12-35`, `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs:1373-1375` | Writer hard-fails with `OWSOUT-E-004`; no output files are emitted. |

### Test and validation surfaces

| Surface | Current state | Evidence | Notes |
|---|---|---|---|
| WS10/WS11/WS12 integration tests | partial | `tests/integration/ws10_watershed_kernel_contract.rs:172-277`, `tests/integration/ws11_channel_routing_physics_equivalence_contract.rs:195-348`, `tests/integration/ws12_impoundment_physics_equivalence_contract.rs:173-251` | Coverage is largely guard behavior + finite/non-negative closure on synthetic fixtures. |
| WS12 coefficient ingestion in tests | synthetic/manual | `tests/integration/ws10_watershed_kernel_contract.rs:37-60`, `tests/integration/ws12_impoundment_physics_equivalence_contract.rs:37-60` | Required coefficient symbols are manually injected in tests, indicating parser/runtime projection gap. |
| Watershed CLI integration and watershed parquet emission tests | missing | `tests/integration/cli03_runner_contract_derived_tests.rs:1-155`, `tests/integration/cli04_runner_wat_parquet_contract_derived_tests.rs:1-188` | CLI03/CLI04 focus on contract text and hillslope `wat` parquet; no end-to-end watershed CLI execution-to-parquet test lane exists. |

## Ran
- `rg`/`sed`/`nl` evidence extraction over:
  - `crates/openwepp-watershed-orchestrator/src/lib.rs`
  - `crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs`
  - `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs`
  - `crates/openwepp-watershed-output/src/{contracts.rs,writers.rs}`
  - `tests/integration/ws10_watershed_kernel_contract.rs`
  - `tests/integration/ws11_channel_routing_physics_equivalence_contract.rs`
  - `tests/integration/ws12_impoundment_physics_equivalence_contract.rs`
  - `tests/integration/cli03_runner_contract_derived_tests.rs`
  - `tests/integration/cli04_runner_wat_parquet_contract_derived_tests.rs`
