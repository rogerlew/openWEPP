# Behavior Equivalence

Evidence label: Static/Ran.

Status: `EXECUTED-PASS`

## Scope

Static:

- This package is parser CQR. It does not own kernel numeric formulas or
  conservation-sensitive output aggregation.
- The touched production code parses scalar text/YAML fields into existing data
  structures.

## Equivalence Claim

Static:

- Public parser APIs are unchanged.
- Public parser output models are unchanged.
- Error enum variants and contract IDs are unchanged.
- The refactor preserves field read order, short-circuit error behavior, and
  assignment order.
- No floating-point expression grouping or accumulation order changed.

## Evidence

Ran:

| Command | Result |
|---|---|
| `cargo nextest run --test infile_management_parser_contract` | PASS, exit `0`; `45` tests passed |
| `cargo nextest run --test infile_management_yaml_contract` | PASS, exit `0`; `2` tests passed |
| `cargo nextest run --workspace --profile full` | PASS, exit `0`; post-review delegated run, `1566` tests passed, `3` skipped |

Behavior oracles exercised:

- Canonical strict parser fixtures still parse with expected schedules and
  scenario registries.
- New operation, contour, initial, drain, and YAML fixtures exercise the
  extracted helper paths.
- Stable error IDs are asserted for every management parser error variant.

Disposition:

- Behavior/API/output identity is preserved for package scope.
- No comparator delta review is applicable because this package does not change
  simulation math, route ownership, serialization, or public output surfaces.
