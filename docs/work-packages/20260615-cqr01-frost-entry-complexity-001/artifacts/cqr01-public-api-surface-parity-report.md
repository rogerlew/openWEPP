# CQR01 Public API Surface Parity Report

Status: complete

Evidence mode: static-and-ran

## Static

Expected unchanged public/internal-call surface:

- `Wb11HydrologyKernel::resolve_active_frost_coupling`
- `Wb11HydrologyKernel::compute_active_frost_coupling`

Observed after refactor:

- line `98`: `pub(crate) fn resolve_active_frost_coupling(`
- line `1453`: `pub(crate) fn compute_active_frost_coupling(`

No public API widening was introduced; extracted helpers are private methods and
private helper structs in `frost_entry.rs`.

## Ran

- `rg -n "pub\\(crate\\)|pub\\(super\\)|pub fn|pub\\(super\\) fn|fn compute_active_frost_coupling|fn resolve_active_frost_coupling" .../frost_entry.rs`
  - exit_code: 0
  - result: only the two expected `pub(crate)` methods are exposed from this file
