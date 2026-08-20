# Constraint Arbitration.md

Status: authority candidate

Evidence mode: Static

Reduction key is `(end_ns, class_precedence, source_owner_id, digest)`.
Classes, in order, are hard, event, output, restart, and adaptive upper bound.
The clock validates/reduces constraints; adopter policy generates proposals and
owns its configuration/history digest. Behind-cursor, past-parent, zero-step
without event, irreconcilable equal-time, exhaustion, policy mismatch, and
direct-clock-advance inputs fail typed.
