# Implementation

Evidence mode: Static.

The package extracted the existing root mass/root depth candidate block from
`DirectGrowthInputs::compute_equation_growth_state` into the private
`compute_root_mass_and_depth_candidates` helper.

The extraction preserves:

- the SC-PLANT-001 `INV-PLANT-027` perennial cap check before incremental
  mass or depth computation;
- the exact comparison and arithmetic expressions;
- annual versus perennial branch conditions;
- tuple values and their order;
- the post-candidate validation and upper-bound operation in the caller;
- all public types, errors, and state publication.

No test, equation, threshold, contract, or adjudication changed. The helper is
private and receives only already-computed scalar operands plus the copied input
state and management class.

Source SHA-256 after extraction:
`1ce345e533159d7317f8c7d1a5f41b292a27896aa53d8e10d693d6366a6eb041`.
