# Pre-Implementation Gate

Evidence class: **Ran + Static**

`parse_structure_preamble` parses the first token with unconstrained
`parse_f64` and branches immediately on `first_numeric > 10`. Positive infinity
enters and passes the explicit-header minimum check; NaN and negative infinity
fall into strict/compat no-datver handling. `G-STR-001` assigns invalid version
policy to `UnsupportedDatver` / `STR-E-003`, so all three are misclassified.

The correction is a single finite check before discrimination. Finite future
versions remain accepted. All DC conversion criteria pass subject to a
contract-derived red regression before production correction.

The committed strict/compat non-finite test then ran before correction and
failed exit `100`: strict NaN returned `LegacyNoDatverDisallowed` rather than
`UnsupportedDatver`. The red log identity is recorded in `validation.md`.
Disposition: `PASS` to the bounded production correction.
