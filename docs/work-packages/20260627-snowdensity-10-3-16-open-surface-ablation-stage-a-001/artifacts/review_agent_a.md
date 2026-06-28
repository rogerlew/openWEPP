# Review A

Evidence mode: Static/Ran.

Scope: contract authority, selector isolation, and production-boundary review.

## Findings

No blocking implementation findings remain.

The contract amendment is in the right order and bounds the candidate tightly:
`SC-SNOWFREEZE-001` v102 adds `INV-SNOWFREEZE-073` and
`OBL-SNOWFREEZE-P-048`, keeps the activated default and rollback selectors, and
forbids default activation, output-schema drift, fixture edits, density-cap
changes, Qwet/frzftp changes, and frost attribution from Stage A evidence.

The selector is opt-in only. Absent/empty
`OPENWEPP_SNOWDENSITY1038_MELT_MODEL` still selects
`coe_liquid_holding_capacity_v1`; `legacy_coe` remains explicit rollback; the
new Stage A id is accepted only when explicitly selected.

The PySnobal boundary is respected. The package checked local metadata and did
not read C source because no local permissive license declaration was found.

## Residual Risk

The Stage A candidate uses a neutral bulk-aerodynamic approximation and a
provisional magnitude envelope. That is acceptable for opt-in non-promotion
evidence, but not sufficient for activation or a Stage B/two-layer package.
