# Parse and Numeric Equivalence

Static: this is a structural extraction only. Every original numeric parse,
validation, comparison, unit-bearing field assignment, and typed error literal
was moved intact into the helper representing its prior branch. The dispatcher
continues to select the same seven datvers; no arithmetic expression, operation
order, tolerance, conversion, or default value changed.

Ran: the pre-decomposition detached-worktree oracle passed through the real
public `parse_soil` consumer for all seven datver families. Current tests add
field-level checks for base, extended, and Rosetta shapes, datver raw/alias
semantics, quoted rows, footer identity/conflict, and exact error code, line,
field, and message behavior. This proves the external parse result/error
contract rather than a helper-only substitute.
