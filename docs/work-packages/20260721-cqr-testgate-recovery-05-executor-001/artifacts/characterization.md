# Characterization

Ran: test-first characterization passed before production decomposition.

- Direct stage-receipt reconstruction binds valid maps/vectors/sets, inventory
  deduplication/order, duplicate unavailable-item last-wins behavior, empty
  resume decisions, and sequential field/item error codes and messages.
- Public stage selection on the real isolated executor fixture binds LIGHT stage
  receipt shape/identity, FINAL_LIGHT ordinary receipt selection, unknown-stage
  rejection after ordinary admission, and HEAVY missing-audit failure.
- A real LIGHT -> constructed READY audit -> HEAVY path binds the unchanged
  LIGHT attempt prefix, plan-ordered HEAVY suffix, combined inventory, exact
  embedded audit, empty no-candidate resume disposition, and final PASS.
- Seven deterministic branch tests bind source-mutation attribution, checkout
  precedence, missing inventory behavior, exact JUnit inventory, artifact reset
  types, exact real-artifact paths, real versus synthetic publication bytes,
  the complete process-artifact envelope, and canonical-directory rejection.

Ran: focused stage-receipt test passed 1/1. The public stage test passed 1/1 in
104.704 seconds after its initial invalid empty-definition fixture was corrected
to use the existing PASS definition. No production behavior changed.

Static: characterization lives in `executor_coverage_tests.rs`; three existing
fixture surfaces are exposed only `pub(super)` for the child test module.

Ran: after the final production correction, the six-test floor matrix passed
6/6, the split path-selection test passed 1/1, the three stage/receipt tests
passed 3/3, and the three hardened artifact-oracle tests passed 3/3. The
authoritative instrumented traversal passed all 135 executed library tests with
two intentional ignores.
