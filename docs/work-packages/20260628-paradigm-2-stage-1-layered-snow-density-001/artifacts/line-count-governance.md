# Line-Count Governance

Evidence class: `Static`

## Snapshot

Line counts after `cargo fmt`:

| File | Lines | Note |
|---|---:|---|
| `Cargo.toml` | 799 | Existing workspace manifest; one integration-test target added. |
| `crates/openwepp-hillslope-orchestrator/src/winter_column.rs` | 500 | Adds snow-layer state to the winter column. |
| `crates/openwepp-hillslope-orchestrator/src/direct_runtime/00_core_frames.rs` | 2499 | Existing large direct-runtime frame file; Stage 1 adds snow-layer carry plumbing and boxes optional snow mirrors to preserve frame-size guards. |
| `crates/openwepp-hillslope-orchestrator/src/direct_runtime/storage.rs` | 1688 | Existing R4G snow-coupling path; Stage 1 persists layer stacks. |
| `crates/openwepp-hillslope-orchestrator/src/hydrology/09_snow_density.rs` | 1173 | Existing snow-density module; Stage 1 adds the opt-in multilayer candidate. |
| `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs` | 2810 | Existing direct-publication authority builder; Stage 1 adds selector and diagnostic trace columns. |
| `tools/snowfreeze_observed/paradigm2_stage1_layered_density.py` | 554 | New observed-corpus diagnostic runner. |
| `tests/integration/paradigm2_stage1_layered_snow_density.rs` | 291 | New contract/selector/runtime integration test. |
| `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md` | 2957 | Existing governing contract; v108 Stage 1 amendment. |
| `docs/work-packages/20260628-paradigm-2-stage-1-layered-snow-density-001/package.md` | 131 | New package record. |

## Assessment

The largest touched files are pre-existing direct-runtime and contract surfaces.
Stage 1 edits are localized to the snow-density selector, winter-column snow
state, typed snow coupling, and diagnostic observed-run tooling. No root
`AGENTS.md` tutorial or long-form process material was added.

The size guard regression found during `cargo test --workspace` was corrected by
boxing optional constructor/lane snow runtime mirrors. The authoritative winter
column and day-frame snow state remain typed Rust values; only optional mirror
storage moved behind `Box` to keep fixed frame sizes bounded.
