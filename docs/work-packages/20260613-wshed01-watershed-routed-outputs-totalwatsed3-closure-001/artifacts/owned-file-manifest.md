# Owned File Manifest

Status: W-C executed

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
