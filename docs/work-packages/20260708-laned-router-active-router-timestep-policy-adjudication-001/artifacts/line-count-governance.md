# Line-Count Governance

Evidence mode: Static + Ran (`wc -l`).

## Touched Rust Files

| File | Lines | Disposition |
|---|---:|---|
| `crates/openwepp-runner/src/hillslope/laned_active.rs` | 305 | OK; focused selector/parser surface |
| `crates/openwepp-hillslope-orchestrator/src/direct_runtime/laned_active.rs` | 1515 | Existing large active-router module; package added focused metadata/config wiring and tests only |
| `crates/openwepp-hillslope-orchestrator/src/direct_runtime/03_executor.rs` | 1266 | Existing large executor; package added one config argument pass-through |
| `crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs` | 1509 | Existing large output module; package added three provenance fields |
| `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs` | 1327 | Existing large intake/provenance module; package added one manifest field |
| `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs` | 2755 | WARN-band existing file; package added `max_dt_s` builder projection only |

## Touched Contract / Package Docs

| File | Lines | Disposition |
|---|---:|---|
| `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md` | 608 | Existing canonical contract; rev-43 amendment is scoped |
| package `package.md` | 205 | OK for autonomous execution spec |
| package markdown artifacts | 8-120 | OK |

## Judgment

No decomposition/refactor was attempted because this package is a narrow
adjudication and authority/evidence increment. Existing large modules are
not made meaningfully worse by the focused additions. The WARN-band builder
file remains a standing refactor candidate outside this work package.
