# Unit Governance

Evidence mode: **Ran + Static**.

The canonical unit registry test passed (`21/21`). The science-contract unit
compliance scanner completed with the repository's retained inventory of `149`
findings; EB-04W did not create a new unregistered unit category. The raw-unit
conversion guard continued to report pre-existing integration-test literals
and Stage 3 constants and did not identify an EB-04W conversion bypass.

The new boundary uses only established units and canonical conversion helpers:

| Quantity | Unit | Boundary rule |
|---|---|---|
| rain / snowfall SWE / melt operands | `m` water equivalent | canonical legacy-inch conversion for CoE terms |
| physical snowfall depth | `m` snow | never aliased to SWE |
| rain and snow fractions | dimensionless | bounded `[0,1]` with wet/dry closure |
| hydrometeor temperature | `degC` | typed finite optional diagnostic |

Inventory output is observational debt, not a newly passed zero-finding gate.
