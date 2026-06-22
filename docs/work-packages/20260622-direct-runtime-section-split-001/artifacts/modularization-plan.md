# Modularization Plan

Evidence class: Static and Ran.

## Pre-Refactor Source

- `crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs`: `2922`
  lines at `HEAD`.
- Root module responsibilities before the split: constants, direct process
  submodule declarations and re-exports, frame definitions, publication DTOs,
  state/report DTOs, executor implementation, audit counters, runtime error,
  and validation helpers.

## Mechanical Split

The package retained lines `1..205` of `direct_runtime.rs` as the root module
header, constants, submodule declarations, and direct process re-exports. The
remaining contiguous source ranges were moved without semantic edits:

- `206..1207` -> `direct_runtime/00_core_frames.rs`
- `1208..1662` -> `direct_runtime/01_publication.rs`
- `1663..2096` -> `direct_runtime/02_state_reports.rs`
- `2097..2488` -> `direct_runtime/03_executor.rs`
- `2489..end` -> `direct_runtime/04_audit_error_helpers.rs`

Range parity was checked with `git show HEAD:... | sed -n '<range>' | cmp -s`
before formatting:

- Core frames: `cmp` exit `0`.
- Publication: `cmp` exit `0`.
- State/reports: `cmp` exit `0`.
- Executor: `cmp` exit `0`.
- Audit/error/helpers: `cmp` exit `0`.

The only formatting fallout during Rust formatting was one blank line before
the first `include!` in the retained root module. `cargo fmt` removed that blank
line. The final staged whitespace check also removed one trailing blank line at
EOF from `00_core_frames.rs`, `01_publication.rs`, `02_state_reports.rs`, and
`03_executor.rs`.

## Included Section Order

`direct_runtime.rs` now wires the section files in dependency order:

```rust
include!("direct_runtime/00_core_frames.rs");
include!("direct_runtime/01_publication.rs");
include!("direct_runtime/02_state_reports.rs");
include!("direct_runtime/03_executor.rs");
include!("direct_runtime/04_audit_error_helpers.rs");
```

The split intentionally uses textual inclusion, not child modules, so private
item access, module namespace, item paths, and public re-export behavior remain
unchanged.

## Section Ownership

- `00_core_frames.rs`: `DirectExecutorMode`, `DirectPhaseKind`,
  `DirectPhaseLifecycleStatus`, `DirectPhaseStatusCount`,
  `DirectRunIdentity`, `DirectRunFrame`, `DirectLaneFrame`, `DirectDayFrame`,
  and `DirectPhaseView`.
- `01_publication.rs`: `DirectPublicationFrame`, publication calendar/day
  inputs, PMET/frost carry projection, `DirectRunPublicationFrame`,
  `DirectPublicationDayRow`, direct publication operands, and peak-runoff
  publication helper.
- `02_state_reports.rs`: `DirectPhasePlan`, direct state/downstream/shadow
  DTOs, span reports, `DirectExecutionReport`, and
  `DirectPublicationExecution`.
- `03_executor.rs`: direct execution counters, span accounting macro, and
  `DirectFrameExecutor`.
- `04_audit_error_helpers.rs`: `DirectRuntimeAuditSnapshot`,
  audit reset/snapshot helpers, `DirectRuntimeError`, display/source
  implementations, and validation helpers.

## Source-Scan Maintenance

The direct-runtime compatibility-token source scan now iterates a compact
`direct_source_paths` list that includes:

- The retained root module.
- All five new included section files.
- Existing direct process modules: decomposition, storage, runoff, subsurface,
  evapotranspiration, normalization, growth, and projection.

The focused scan test passed:

`cargo test -p openwepp-hillslope-orchestrator r2a_direct_runtime_source_excludes_compatibility_storage_tokens -- --nocapture`
