# PL03 Parser to Runtime Integration Evidence

Status: `complete`
Evidence mode: `Ran`

Ran:
- Parsed canonical management fixture `canonical_cropland_nonzero_98_4.man` and projected PL runtime surfaces.
- Verified required projected controls and canonical seed aliases in merged runtime state surface.
- Verified typed negative paths for out-of-range initial reference, unsupported landuse branch, non-finite row width, and unsupported perennial option.

## Fixture and Test Anchors

1. Fixture import:
- `tests/fixtures/infile/management/canonical_cropland_nonzero_98_4.man`

2. Positive projection test:
- `management_runtime_surfaces_project_required_pl_controls_and_seeds`

3. Negative path tests:
- `management_runtime_projection_rejects_out_of_range_initial_reference`
- `management_runtime_projection_rejects_unsupported_pl_landuse`
- `management_runtime_projection_rejects_non_finite_row_width`
- `management_runtime_projection_rejects_unsupported_perennial_option`

## Executed Validation Commands

1. `cargo fmt --check` -> pass.
2. `cargo clippy --workspace --all-targets -- -D warnings` -> pass.
3. `cargo test --workspace` -> pass.
4. `cargo deny check` -> pass (warning-only unmatched allow-list entries in `deny.toml`; advisories/bans/licenses/sources all ok).

## Evidence Links

- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs:2017`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs:2222`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs:2310`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs:2332`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs:2353`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs:2381`
