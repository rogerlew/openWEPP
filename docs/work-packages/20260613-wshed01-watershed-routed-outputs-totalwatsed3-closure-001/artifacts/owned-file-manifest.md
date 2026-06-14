# Owned File Manifest

Status: T-B2-REDO2 executed

Evidence mode: Static

W-A write set:

- `docs/work-packages/20260613-wshed01-watershed-routed-outputs-totalwatsed3-closure-001/package.md`
- `docs/work-packages/20260613-wshed01-watershed-routed-outputs-totalwatsed3-closure-001/artifacts/*.md`
- `docs/work-packages/README.md`
- `docs/ROADMAP.md`

Production source files read only:

- `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs`
- `crates/openwepp-input-contract/src/parsers/watershed_impoundment.rs`
- `crates/openwepp-watershed-output/src/contracts.rs`
- `crates/openwepp-watershed-output/src/writers.rs`
- `crates/openwepp-watershed-orchestrator/src/lib.rs`
- `crates/openwepp-watershed-orchestrator/src/lib_mod/*`

Temporary evidence root:

- `/tmp/openwepp_wshed01_wa/`

W-B write set:

- `docs/contracts/openwepp-watershed-runfile-contract.md`
- `crates/openwepp-input-contract/src/parsers/watershed_impoundment.rs`
- `tests/integration/infile_watershed_impoundment_parser_contract.rs`
- `tests/fixtures/infile/watershed_impoundment/strict_zero_impoundments.imp`
- `crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs`
- `docs/work-packages/20260613-wshed01-watershed-routed-outputs-totalwatsed3-closure-001/package.md`
- `docs/work-packages/20260613-wshed01-watershed-routed-outputs-totalwatsed3-closure-001/artifacts/*.md`

W-B temporary evidence root:

- `/tmp/openwepp_wshed01_wb/`

W-C write set:

- `Cargo.lock`
- `crates/openwepp-runner/Cargo.toml`
- `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs`
- `crates/openwepp-runner/src/lib.rs`
- `crates/openwepp-runner/src/watershed_wat.rs`
- `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/diagnostics.rs`
- `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/helpers.rs`
- `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/validation.rs`
- `crates/openwepp-watershed-output/src/writers.rs`
- `docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `tests/integration/ws11_channel_routing_physics_equivalence_contract.rs`
- `docs/work-packages/20260613-wshed01-watershed-routed-outputs-totalwatsed3-closure-001/package.md`
- `docs/work-packages/20260613-wshed01-watershed-routed-outputs-totalwatsed3-closure-001/artifacts/*.md`

W-C temporary evidence roots:

- `/tmp/openwepp_wshed01_wc_final_configured/`
- `/tmp/openwepp_wshed01_wc_final_legacy/`

Cross-repo files read only:

- `/home/workdir/wepppy/wepppy/wepp/interchange/totalwatsed3.py`
- `/home/workdir/wepppy/tools/totalwatsed3_daily_closure_audit.py`

W-D write set:

- `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs`
- `crates/openwepp-runner/src/watershed_wat.rs`
- `crates/openwepp-watershed-output/src/writers.rs`
- `crates/openwepp-sim-contract/src/units_mod/output_catalog.rs`
- `docs/work-packages/20260613-wshed01-watershed-routed-outputs-totalwatsed3-closure-001/package.md`
- `docs/work-packages/20260613-wshed01-watershed-routed-outputs-totalwatsed3-closure-001/artifacts/*.md`

W-D temporary evidence roots:

- `/tmp/openwepp_wshed01_wd_configured/`
- `/tmp/openwepp_wshed01_wd_legacy/`

W-D cross-repo files read only:

- `/home/workdir/wepppy/wepppy/wepp/interchange/totalwatsed3.py`
- `/home/workdir/wepppy/tools/totalwatsed3_daily_closure_audit.py`

T-A write set:

- `docs/work-packages/20260613-wshed01-watershed-routed-outputs-totalwatsed3-closure-001/package.md`
- `docs/work-packages/20260613-wshed01-watershed-routed-outputs-totalwatsed3-closure-001/artifacts/*.md`

T-A production source read only:

- `crates/openwepp-hillslope-output/src/contracts.rs`
- `crates/openwepp-runner/src/hillslope/02_output_and_climate_helpers.rs`
- `crates/openwepp-input-contract/src/parsers/hbp/payload_validator.rs`
- `crates/openwepp-runner/src/watershed_wat.rs`
- `crates/openwepp-watershed-output/src/writers.rs`

T-A cross-repo files read only:

- `/home/workdir/wepppy/wepppy/wepp/interchange/totalwatsed3.py`
- `/home/workdir/wepppy/tools/totalwatsed3_daily_closure_audit.py`

T-A external substrate sampled read only:

- `/wc1/runs/ar/arboreal-dendrite/wepp/output/interchange/H.pass.parquet`
- `/wc1/runs/ar/arboreal-dendrite/wepp/output/interchange/H.wat.parquet`
- `/wc1/runs/ar/arboreal-dendrite/wepp/output/interchange/H.soil.parquet`
- `/wc1/runs/ar/arboreal-dendrite/wepp/output/interchange/H.element.parquet`

T-B write set:

- `Cargo.lock`
- `crates/openwepp-runner/Cargo.toml`
- `crates/openwepp-runner/src/bin/openwepp-cli-totalwatsed3.rs`
- `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs`
- `crates/openwepp-runner/src/lib.rs`
- `crates/openwepp-runner/src/totalwatsed3.rs`
- `crates/openwepp-runner/tests/totalwatsed3_cli_contract.rs`
- `crates/openwepp-watershed-output/src/writers.rs`
- `crates/openwepp-sim-contract/src/units_mod/output_catalog.rs`
- `tests/integration/sim_contract_boundary_unit_registry.rs`
- `docs/work-packages/20260613-wshed01-watershed-routed-outputs-totalwatsed3-closure-001/package.md`
- `docs/work-packages/20260613-wshed01-watershed-routed-outputs-totalwatsed3-closure-001/artifacts/*.md`

T-B temporary evidence root:

- `/tmp/openwepp_wshed01_tb/`

T-B cross-repo files read only:

- `/home/workdir/wepppy/tools/totalwatsed3_daily_closure_audit.py`

T-B external substrate read only:

- `/wc1/runs/ar/arboreal-dendrite/wepp/output/interchange/H.pass.parquet`
- `/wc1/runs/ar/arboreal-dendrite/wepp/output/interchange/H.wat.parquet`
- `/wc1/runs/ar/arboreal-dendrite/wepp/output/interchange/H.soil.parquet`
- `/wc1/runs/ar/arboreal-dendrite/wepp/output/interchange/H.element.parquet`

T-B2 write set:

- `crates/openwepp-hillslope-output/src/lib.rs`
- `crates/openwepp-hillslope-output/src/contracts.rs`
- `crates/openwepp-hillslope-output/src/writers.rs`
- `crates/openwepp-hillslope-output/src/hillslope_pass.rs`
- `crates/openwepp-runner/src/bin/openwepp-cli-totalwatsed3.rs`
- `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs`
- `crates/openwepp-runner/src/hillslope/02_output_and_climate_helpers.rs`
- `crates/openwepp-runner/src/hillslope/intake_lane_setup/runfile_helpers.rs`
- `crates/openwepp-runner/src/hillslope/scheduler_trace/per_ofe_internal_wb13.rs`
- `crates/openwepp-runner/src/hillslope/tests03/per_ofe_state.rs`
- `crates/openwepp-runner/src/totalwatsed3.rs`
- `crates/openwepp-runner/tests/totalwatsed3_cli_contract.rs`
- `crates/openwepp-sim-contract/src/units_mod/output_catalog.rs`
- `tests/integration/sim_contract_boundary_unit_registry.rs`
- `docs/work-packages/20260613-wshed01-watershed-routed-outputs-totalwatsed3-closure-001/package.md`
- `docs/work-packages/20260613-wshed01-watershed-routed-outputs-totalwatsed3-closure-001/artifacts/*.md`

T-B2 temporary evidence root:

- `/tmp/openwepp_wshed01_tb2/`

T-B2 read-only comparison substrate:

- `/tmp/openwepp_mofe01_mh_final/runs/`
- `/tmp/openwepp_mofe01_mi_final/output/`

T-B2-REDO write set:

- `crates/openwepp-runner/src/hillslope/02_output_and_climate_helpers.rs`
- `crates/openwepp-runner/src/hillslope/scheduler_trace/per_ofe_internal_wb13.rs`
- `crates/openwepp-runner/src/hillslope/tests03/per_ofe_state.rs`
- `docs/work-packages/20260613-wshed01-watershed-routed-outputs-totalwatsed3-closure-001/package.md`
- `docs/work-packages/20260613-wshed01-watershed-routed-outputs-totalwatsed3-closure-001/artifacts/*.md`

T-B2-REDO temporary evidence roots:

- `/tmp/openwepp_wshed01_tb2_redo/` (intermediate rejected
  `QOFE * record.area` attempt)
- `/tmp/openwepp_wshed01_tb2_redo_qarea/` (superseded REDO output using
  under-scaled `Q * outlet Area`)

T-B2-REDO read-only comparison substrate:

- `/tmp/openwepp_wshed01_tb2/runs/`
- `/tmp/openwepp_mofe01_mi_final/output/`
- `/home/workdir/wepppy/tools/totalwatsed3_daily_closure_audit.py`

T-B2-REDO2 write set:

- `crates/openwepp-runner/src/hillslope/02_output_and_climate_helpers.rs`
- `crates/openwepp-runner/src/hillslope/tests03/per_ofe_state.rs`
- `crates/openwepp-sim-contract/src/units_mod/output_catalog.rs`
- `docs/work-packages/20260613-wshed01-watershed-routed-outputs-totalwatsed3-closure-001/package.md`
- `docs/work-packages/20260613-wshed01-watershed-routed-outputs-totalwatsed3-closure-001/artifacts/*.md`

T-B2-REDO2 temporary evidence root:

- `/tmp/openwepp_wshed01_tb2_redo2_qofearea_20260614T213618Z/`

T-B2-REDO2 read-only comparison substrate:

- `/tmp/openwepp_wshed01_tb2_redo_qarea/runs/`
- `/tmp/openwepp_mofe01_mi_final/output/`
- `/home/workdir/wepppy/tools/totalwatsed3_daily_closure_audit.py`
