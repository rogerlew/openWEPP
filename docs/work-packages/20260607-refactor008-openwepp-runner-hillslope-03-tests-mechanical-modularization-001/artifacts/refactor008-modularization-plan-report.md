# REFACTOR008 refactor008 modularization plan report

Status: complete  
Evidence mode: Static + Ran

## Scope
Mechanically split `03_tests.rs` into three test modules while preserving all behavior, assertions, and source provenance.

## Baseline (pre-refactor) [Static]
- Source file: `crates/openwepp-runner/src/hillslope/03_tests.rs`
- Test count: `68`
- Line count: `3942`
- File exceeded 3000-line threshold and required decomposition before closure.

## Execution strategy [Static]
- Preserve all `#[test]` functions exactly by moving bodies into:
  - `crates/openwepp-runner/src/hillslope/tests03/simimpl.rs`
  - `crates/openwepp-runner/src/hillslope/tests03/publication.rs`
  - `crates/openwepp-runner/src/hillslope/tests03/trace.rs`
- Keep shared test helpers in `crates/openwepp-runner/src/hillslope/03_tests.rs`.
- Keep helper visibility via `super::*` import in each split module.
- Add module declarations (`mod simimpl; mod publication; mod trace;`) inside `mod tests { ... }`.

## Planned seams [Static]
- simimpl: `simimpl`-prefixed and scheduler-runtime support tests.
- publication: WB13/WB13-guard/publication-oriented tests.
- trace: `hphys0245`, `hphys0259`, `hphys0260`, `hphys0261`, `hphys0262`, `hphys0268`, `wbval06_*`, `hphys0288`, `hphys0270`, `hphys0271`, `hphys0318`, and adjacent trace tests.

## Outcome summary [Static]
- No production source paths changed.
- No contract text changes.
- No behavior contract intent changes intended.
