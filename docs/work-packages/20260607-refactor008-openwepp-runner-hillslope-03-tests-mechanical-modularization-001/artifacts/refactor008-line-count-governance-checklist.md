# REFACTOR008 refactor008 line count governance checklist

Status: complete  
Evidence mode: Static

## Scope
Line-count governance under the mechanical-refactor policy.

## Static
- Pre-refactor file counts:
- `crates/openwepp-runner/src/hillslope/03_tests.rs`: `3942` lines
- Post-refactor file counts:
- `crates/openwepp-runner/src/hillslope/03_tests.rs`: `518` lines
- `crates/openwepp-runner/src/hillslope/tests03/simimpl.rs`: `130` lines
- `crates/openwepp-runner/src/hillslope/tests03/publication.rs`: `2079` lines
- `crates/openwepp-runner/src/hillslope/tests03/trace.rs`: `1228` lines
- Files >= 2000 lines:
  - `crates/openwepp-runner/src/hillslope/tests03/publication.rs` (`2079`)
- Files >= 3000 lines:
  - none
- Decomposition rationale:
  - `publication` segment was extracted from monolith to satisfy `.rs < 3000` requirement and keep related test concern cohesive.

## Ran
- No additional decomposition was required after split because all post files are < 3000 lines.
