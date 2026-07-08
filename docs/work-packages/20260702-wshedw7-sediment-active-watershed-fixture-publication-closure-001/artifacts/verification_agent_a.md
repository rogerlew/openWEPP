# Verification Agent A

Status: `completed-local-substitution`

Evidence mode: `Ran:` local verification and `Static:` source review.

Verification result: W7R closure is supported.

Checks:

- Current-main p102 producer emits nonzero `tdet`, `tdep`, and all five
  `sedcon_*` sums.
- Accepted fixture is committed and manifest-validated.
- Release serial and parallel watershed runs pass.
- Public parquet decoded schema/row identity passes.
- Focused guard proves generated HBP detachment/deposition reaches public
  `totalwatsed3`.
- No surrogate sediment or manual pass edits are introduced.

Residual risk: the accepted fixture is intentionally small. Large onshore
channel dispatch remains outside W7R closure and should not be treated as closed
by this package.
