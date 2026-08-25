# Terminal bounded observation seam V3 exact write set

Status: `CANDIDATE / NO SOURCE AUTHORITY`

Base: `bda290573a17fd9be5b07542a55b67688500cf5a`.

After two GO-to-evidence reviews only, edits are limited to:

1. `snow_stage3_v11_terminal_execution.rs`: private generic evidence entry,
   provider closure ordinal, explicit physical snapshots, post-return handback.
2. `hydrology/support_helpers_mod/runoff_reconciliation.rs`: sealed mode,
   literal hook and test DTO declarations.
3. `hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver.rs`:
   private mode/state forwarding.
4. `hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver/support.rs`:
   private mode/state forwarding.
5. `hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver/evaluation.rs`:
   coupling hooks and rain-derived external-liquid operand/provenance.
6. `hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver/terminal_event.rs`:
   retained coarse joint, selected trials, pair/floor hooks and validators.
7. `v11_covered/carrier_phase.rs`: named primitive projection including two
   distinct lists from the retained surface candidate: `open_ingress_parcels()`
   and `receipts()`.
8. `v9_real_consumer_shadow_wb14_tests.rs`: capture assertion inside the exact
   fixture owner; it calls the existing private helper directly.

Paths are relative to `crates/openwepp-hillslope-orchestrator/src/`. No other
source, Cargo file, frozen artifact, API, runtime protocol or physical branch is
in scope. Requiring a ninth source file returns the intent to review.
