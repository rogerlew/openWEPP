# Contract-test implementation

Status: green after production correction
Evidence mode: Ran

Added `every_real_field_rejects_nan_and_infinities_in_both_modes`, a
table-driven public-parser contract test covering all eight real symbols,
`NaN`, positive infinity, negative infinity, and strict/compatibility modes.
It requires `FieldRangeError`, canonical field identity, a non-finite captured
value, and contract error `FDIR-E-005`; it also requires no typed output.

Ran: `cargo nextest run --test infile_irrigation_fixeddate_parser_contract`
failed as intended with 14 pass / 1 fail. First counterexample: `NaN` datver
returned `FDIR-E-003`, proving current production does not satisfy the amended
contract. Production parser source had an empty git diff when this evidence was
recorded.

Pre-implementation test hash:
`744e243626c5abdace7989746118daa78817e0e7b7059fcc3072e12a72e87dae`.

After the pre-implementation gate, the public integration suite expanded from
15 to 27 tests to cover warning/error identity and formatting, physical-line
normalization, every header/date/event domain and arity, datver threshold
branches, OFE cycling, 20-surge cardinality, deterministic parses, and closure
failures. Ran: the final focused suite passed `27/27` after the production
finite-value correction. The final test hash is recorded at disposition.

Final test hash:
`473d2ba682562122cf16bbc2ea6f83a43cd2a8352d47c7f72ff1a30167b3d87e`.
The final amendment adds exhaustive typed-structure expectations for canonical
strict sprinkler/furrow fixtures and the compatibility no-datver/nozzle fixture.
