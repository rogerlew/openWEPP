# REFACTOR005 Public API Surface Parity Report

Status: complete
Evidence mode: static+ran
Date: 2026-05-27

## Static
- `hydrology/mod.rs` include ordering remains unchanged.
- `lib.rs` exports for `Wb11HydrologyKernel` and related types are unchanged.
- Split is intra-module only; symbol visibility is preserved through ordered
  `include!` composition.

## Ran
- `REFACTOR005_RECONCAT_EQUIVALENT=1` check:
  - `git show HEAD:.../03_kernel_support.rs`
  - compared to concatenation of
    `03_kernel_support_00_support_helpers.rs` +
    `03_kernel_support_01_kernel_phases.rs`.
- `wc -l` after split:
  - wrapper: `03_kernel_support.rs` = 2 lines
  - helper section: `3196` lines
  - kernel phase section: `4387` lines

## Conclusion
Mechanical split preserves source content ordering and effective module surface.
