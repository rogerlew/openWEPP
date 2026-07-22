# Characterization

Ran: aggregate admission passed before implementation. Commit `e3a05b35` added
`replace_string_preserves_recursive_value_only_mutation` before decomposition.
It proves root exact match and nonmatch, repeated nested array/object values,
exact-equality-only behavior, number/boolean/null no-ops, empty containers,
object-key preservation, and stable `old == new` idempotence.

Ran: direct Nextest passed 1/1 at run
`37ce16ae-19be-4607-bf77-8297fce225c2`. The unchanged real consumer
`ready_audit_verification_preserves_order_and_exact_verdict`, which exercises
`make_light_only` plan/receipt rewrites, passed 1/1 at run
`e3ce5d89-b78d-4766-93df-bb3ed164c895` in 209.652 seconds.
