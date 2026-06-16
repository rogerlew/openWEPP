# CQR27 Quality Plan Report

Static: scoped quality target is CRAP/cyclomatic-complexity burn-down for the
current target function in
`crates/openwepp-input-contract/src/parsers/management.rs`.

Static: protected boundaries are parser grammar, strict/compatibility behavior,
public API, error IDs, diagnostics, defaults, parsed output shape, and runtime
projection compatibility.

Status: complete.

Ran: baseline metrics identified
`parse_yearly_annual_fallow` as the live CQR27 target at line `1113`, CC
`35.0`, coverage `40.67796610169492%`, and CRAP `290.7314769280208`.

Static: closure strategy was behavior-preserving helper extraction, with
additional characterization tests added before production refactor for branches
that were not sufficiently pinned by existing focused tests.

Ran: after metrics close the target and extracted helpers at CRAP `<= 30`.

Static: this package is parser-boundary-affecting but not kernel-affecting, so
science-contract kernel-profile artifacts were not required for CQR27.
