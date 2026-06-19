# Owned File Manifest

Status: complete.
Evidence mode: Static/Ran.

## Final Retained Write Set

- `docs/work-packages/20260619-perfdeep08-disabled-path-hard-isolation-001/**`
- `docs/work-packages/README.md`
- `docs/ROADMAP.md`

## Temporary Candidate Write Set

The rejected candidate temporarily edited and then reverted:

- `crates/openwepp-runner/src/hillslope/indexed_shadow_surface.rs`
- `crates/openwepp-runner/src/hillslope/scheduler_trace/perfdeep02_frame_roundtrip.rs`
- `crates/openwepp-hillslope-orchestrator/src/scheduler.rs`

`scheduler.rs` was reverted before timing to avoid retaining a touched 3000+
line file without a split/closure plan. The two runner hook-cache edits were
timed, measured slower, and then reverted. No production Rust edit remains.
