# Owned File Manifest

Status: executed-hold

Evidence mode: Static.

| Path | Class | Summary |
| --- | --- | --- |
| `Cargo.toml` | Test registry | Adds `snowfrost_fidelity_g0_pysnobal_bridge_contract`. |
| `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/06_simimpl28_hourly_forcing.rs` | Rust diagnostic helper | Adds complete-row SIMIMPL28 diagnostic export mode without changing production trigger behavior. |
| `crates/openwepp-runner/Cargo.toml` | Rust binary registry | Adds `openwepp-snowbench`. |
| `crates/openwepp-runner/src/bin/openwepp-snowbench.rs` | Rust exporter CLI | Adds `export-pysnobal` command. |
| `crates/openwepp-runner/src/hillslope/mod.rs` | Rust module wiring | Exposes snowbench exporter API. |
| `crates/openwepp-runner/src/hillslope/snowbench.rs` | Rust exporter | Emits PySnobal forcing/config/lineage/audit artifacts. |
| `crates/openwepp-runner/src/lib.rs` | Rust public API | Exports snowbench types and function for tests. |
| `tests/integration/snowfrost_fidelity_g0_pysnobal_bridge_contract.rs` | Test | Validates schema, lineages, lane config, and anti-alias gates. |
| `tools/snowfreeze_observed/pysnobal_compare.py` | Python harness | Runs PySnobal and writes per-lane/site summaries. |
| `tools/snowfreeze_observed/README.md` | Docs | Documents exporter and harness usage. |
| `docs/work-packages/README.md` | Docs | Records executed-HOLD state. |
| `docs/work-packages/20260625-snowfrost-fidelity-g0-pysnobal-input-bridge-001/package.md` | Package docs | Updates status and HOLD boundary. |
| `docs/work-packages/20260625-snowfrost-fidelity-g0-pysnobal-input-bridge-001/artifacts/*.md` | Package artifacts | Records execution, gates, review, disposition, and handoff. |
| `docs/work-packages/20260625-snowfrost-fidelity-g0-pysnobal-input-bridge-001/artifacts/pysnobal_site_summary*.json` | Package artifacts | Compact PySnobal run summaries. |
