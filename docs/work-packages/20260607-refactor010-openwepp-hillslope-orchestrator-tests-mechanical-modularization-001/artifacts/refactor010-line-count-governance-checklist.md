# REFACTOR010 refactor010 line count governance checklist

Static:
- Baseline check required because source file exceeded 3000 lines.

Ran:
- `crates/openwepp-hillslope-orchestrator/src/tests.rs` (baseline): 3460 lines (pre): exceeds 3000.
- `crates/openwepp-hillslope-orchestrator/src/tests.rs` (current): 3 lines.
- Split modules:
  - `mod.rs`: 55 lines
  - `fixtures.rs`: 924 lines
  - `hydrology.rs`: 575 lines
  - `phase.rs`: 726 lines
  - `growth.rs`: 368 lines
  - `boundaries.rs`: 491 lines
  - `writeback.rs`: 260 lines
  - `schedule_export.rs`: 135 lines
- Files >=2000 lines after split: none.
- Files >=3000 lines after split: none.
- Decomposition rationale: extraction removed oversized file into cohesive modules and reduced review complexity while preserving test intent.
