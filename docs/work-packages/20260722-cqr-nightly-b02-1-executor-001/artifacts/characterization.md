# Characterization

Ran: aggregate admission passed before implementation. Commit `d15f7b5a`
then added the direct characterization
`affected_quality_scope_preserves_success_and_error_precedence` before the
production decomposition.

The final test at commit `0bd56dc3` proves:

- the valid package/node/inventory path succeeds;
- one stacked-invalid fixture selects exact precedence in this order:
  `GATE-EXEC-QUALITY-INCOMPLETE`, `GATE-EXEC-QUALITY-PACKAGES`,
  `GATE-EXEC-QUALITY-COVERING-NODES`, then
  `GATE-EXEC-QUALITY-INVENTORY` as each earlier fault is repaired;
- every seam asserts both the exact stable code and exact message; and
- a covering node with the wrong gate definition fails with the exact
  covering-node error.

Ran: the exact focused Nextest command passed one test and skipped 152 at
terminal source `0bd56dc3` (run ID
`82f91617-da31-4c62-8b91-93740f0b814f`).
