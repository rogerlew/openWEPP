# ADR-0021 Coverage Closure

Tier: science. This parser forms typed inputs to the soil-domain contracts, so
the science-tier floor is at least 90% line and 90% region coverage, with no
eligible function below 75% region coverage without a documented exclusion.

Obligation-to-test binding: public `parse_soil` cases bind all seven datver
families to representative base, extended, and Rosetta rows. Additional direct
seam tests bind raw/alias status, every public `SOL-E-*` label and error
formatting, strict/compatibility record selection, quoted token/policy/header
behavior (including the quoted arity failure), header/policy ordering, layer
depth rules, restrictive-row/footer identity, and file-tail rejection to their
fail-closed typed errors.

Ran: final isolated coverage records `1085/1108` production-only lines
(97.924%) and `1434/1571` production-only regions (91.279%), above both
science-tier floors. All target functions meet or exceed 75% region coverage;
no exclusion is needed.

The pre-decomposition all-datver public parser oracle is recorded in
`characterization.md`, and the real public consumer plus exact error paths are
exercised after the private decomposition.
