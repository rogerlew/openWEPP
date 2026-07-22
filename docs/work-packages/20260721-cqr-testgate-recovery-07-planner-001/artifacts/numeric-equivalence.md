# Numeric Equivalence

Static: no process physics. Preserve exact plan JSON, identities, selected
nodes, ordering, errors, and reconciliation behavior.

Static: `load_source_graph` is a whole-block extraction. It preserves the exact
base-before-head load order, workspace branch selection, `?` short-circuit
points, and final `base_graph.union(&head_graph)` expression. No numerical
operation exists and no observable ordering or error mapping changed. The
unchanged characterization plus bound-context and canonical reconstruction
tests are the equivalence oracle.
