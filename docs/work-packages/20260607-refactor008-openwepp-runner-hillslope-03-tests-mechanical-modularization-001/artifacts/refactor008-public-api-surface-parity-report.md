# REFACTOR008 refactor008 public api surface parity report

Status: complete  
Evidence mode: Static + Ran

## Scope
Track surface impact of test-file modularization.

## Public API inventory [Static]
- No `pub` signatures were added, removed, or modified.
- No production modules were added under crate `lib.rs`.
- No CLI/runtime API files were modified.

## Test API surface [Static]
- Test functions and helpers remain in the same crate module tree under `mod tests`.
- Shared helper functions remain in `03_tests.rs` and continue to be private to tests.
- Moved tests are now nested under `tests::simimpl`, `tests::publication`, `tests::trace`.

## Equivalence note [Static]
- Any direct symbol path references requiring `mod tests` internals remain valid because movement is inside test-only modules and not exported outside crate APIs.
