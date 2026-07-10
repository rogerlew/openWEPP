# Numeric / API Equivalence

Static:

- The target module contains topology parser/validation glue and typed status
  construction. It does not perform floating-point science calculations or
  conservation output publication.
- Numeric operations are unchanged: existing count/range checks still pass the
  same values to `check_min`, `check_max`, and `check_equal_count` in the same
  validation order.
- Declared channel/impoundment count conversion preserves the previous
  fail-closed overflow behavior for unsupported narrow targets while executing
  the same equality checks on supported targets.
- Public APIs, enum variants, message IDs, fixture grammar, graph sorting,
  edge construction, and fail-closed validation status are unchanged.

Ran:

- Test-first detached proof: new characterization tests passed on the
  pre-refactor source.
- Focused post-refactor run:
  `cargo nextest run --test topology_graph_validation_gate --profile quick`;
  13/13 passed.

Disposition: behavior identity is preserved for parser errors, display strings,
graph construction, validation message IDs, and validation status classes.
