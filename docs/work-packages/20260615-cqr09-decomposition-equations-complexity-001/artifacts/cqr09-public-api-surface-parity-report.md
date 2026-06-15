# CQR09 Public API Surface Parity Report

Static: planned production edits are private helper extraction in an included
hydrology module file. No public API change is authorized.

Ran: `git diff --unified=0 --
crates/openwepp-hillslope-orchestrator/src/hydrology/07_decomposition_equations.rs
crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/phase.rs | rg
'^[+-].*\bpub\b'` returned no matches.

Static: added production symbols are private:

- `AnnualDecompositionControlSymbols`
- `AnnualDecompositionControlValues`
- `AnnualDecompositionControlInputs`
- `require_annual_decomposition_control_inputs`
- `resolve_annual_decomposition_action`
- annual action helper functions for herbicide, burn, silage, cut, remove, and
  no-op branches.

Static: public scheduler-facing payload types, error enum variants, stable
reason strings, symbol names, and `HillslopeAnnualDecompositionControl` field
construction remain unchanged.

Status: no intentional public API delta.
