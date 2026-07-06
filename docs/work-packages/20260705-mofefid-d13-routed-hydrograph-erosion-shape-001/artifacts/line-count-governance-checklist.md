# Line-Count Governance Checklist

Status: **COMPLETE** (Ran).

Command:
`wc -l crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs crates/openwepp-hillslope-orchestrator/src/direct_runtime/00_core_frames.rs crates/openwepp-hillslope-orchestrator/src/direct_runtime/01_publication.rs crates/openwepp-hillslope-orchestrator/src/direct_runtime/erosion.rs crates/openwepp-hillslope-orchestrator/src/lib.rs crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime_wave1_continuity.rs crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs`

| File | Lines | Disposition |
|---|---:|---|
| `crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs` | 813 | Below WARN. |
| `crates/openwepp-hillslope-orchestrator/src/direct_runtime/00_core_frames.rs` | 2601 | Pre-existing WARN-band core frame file; D13 only updated one comment. No split in package scope. |
| `crates/openwepp-hillslope-orchestrator/src/direct_runtime/01_publication.rs` | 668 | Below WARN. |
| `crates/openwepp-hillslope-orchestrator/src/direct_runtime/erosion.rs` | 1246 | Below WARN. |
| `crates/openwepp-hillslope-orchestrator/src/lib.rs` | 179 | Below WARN. |
| `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs` | 2990 | Pre-existing WARN-band test helper file; D13 added two initializer fields only. No behavior refactor in package scope. |
| `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime_wave1_continuity.rs` | 1235 | Below WARN. |
| `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs` | 2696 | Pre-existing WARN-band builder file; D13 added default constructor fields only. Split remains outside D13 scope. |
