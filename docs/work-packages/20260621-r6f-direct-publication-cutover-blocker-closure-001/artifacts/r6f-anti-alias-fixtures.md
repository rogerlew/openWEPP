# R6F Anti-Alias Fixtures

Status: scaffolded.

Anti-alias evidence must prove direct publication is not merely self-consistent
or compatibility-wrapped.

## Fixture Requirements

- At least one fixture must exercise HBP, WAT, PASS, loss, and manifest output
  families when the cutover gate is enabled.
- PASS fixture coverage must include Parquet output.
- Fixture assertions must fail if a direct output family reads compatibility
  rows or default/zero skeleton operands.
- Fixture assertions must fail if manifest provenance/checksum is sourced from
  compatibility output construction.

## Fixture Ledger

| Fixture/test | Output family | Alias rejected | Direct operand varied | Expected failure if aliased | Status |
|---|---|---|---|---|---|
| Pending | HBP | Pending | Pending | Pending | Pending |
| Pending | WAT | Pending | Pending | Pending | Pending |
| Pending | PASS | Pending | Pending | Pending | Pending |
| Pending | Loss | Pending | Pending | Pending | Pending |
| Pending | Manifest | Pending | Pending | Pending | Pending |

## Evidence

| Date | Command | Result | Notes |
|---|---|---|---|
| Pending | Pending | Pending |  |
