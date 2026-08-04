# Contract-Test Implementation Evidence

Status: `PASS / pre-production RED recorded`

Evidence mode: `Static + Ran`

`snow_surface_eb04w_accumulation_melt_diagnostics_contract.rs` now binds
v123/INV-090/OBL-P-063/TOL-015, evaluates the exact Stage-3 identity with
deliberately distinct operands, and rejects four adjacent formulas:

1. retained change omitted;
2. top-level CoE routed melt substituted for Stage-3 routed liquid;
3. the CoE retained store substituted for the Stage-3 retained delta; and
4. refrozen liquid counted twice.

It also requires the v4 schema, exact Stage-3 field names, signed-hour typed
carrier fields, and real writer/formatter names. Thirty-four mechanical
contract-version pins were reconciled from v122 to v123, and existing current-
producer schema assertions were advanced from v3 to v4.

Ran before production edits:

    cargo nextest run --no-fail-fast --test snow_surface_eb04w_accumulation_melt_diagnostics_contract

Result: `7 passed, 2 failed`. Contract and anti-alias tests passed. Both
production-path tests failed first at the expected absent schema-v4 marker,
proving the implementation gate was RED rather than vacuously green.

Post-implementation result: `9/9` passed. The consumer test now also requires
duration-weighted active/lower thermal arrays and lower-volume present fraction,
and the real-file verifier requires a mixed-sign population with individually
nonzero Stage-3 operands plus at least one all-nonzero joint row.
