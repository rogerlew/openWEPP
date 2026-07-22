# Characterization

Ran: test-first characterization passed before production decomposition.

- Direct stage-receipt reconstruction binds valid maps/vectors/sets, inventory
  deduplication/order, duplicate unavailable-item last-wins behavior, empty
  resume decisions, and sequential field/item error codes and messages.
- Public stage selection on the real isolated executor fixture binds LIGHT stage
  receipt shape/identity, FINAL_LIGHT ordinary receipt selection, unknown-stage
  rejection after ordinary admission, and HEAVY missing-audit failure.

Ran: focused stage-receipt test passed 1/1. The public stage test passed 1/1 in
104.704 seconds after its initial invalid empty-definition fixture was corrected
to use the existing PASS definition. No production behavior changed.

Static: characterization lives in `executor_coverage_tests.rs`; three existing
fixture surfaces are exposed only `pub(super)` for the child test module.
