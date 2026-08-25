# Terminal bounded observation seam V2 exact write set

Status: `CANDIDATE / NO SOURCE AUTHORITY`

Base is `31cb590576fa421e0754ec4dddf2971df007a19c`. The governed V3--V6 and
first bounded-seam HOLD artifacts are immutable.

After, and only after, two GO-to-evidence reviews of the same candidate hash,
the following crate-private edits are permitted:

1. `crates/openwepp-hillslope-orchestrator/src/snow_stage3_v11_terminal_execution.rs`
   retains the physical result with test evidence, counts the single provider
   closure entry, captures explicit before/after owner locations, and owns the
   focused `interior_terminal_event_runs_covered_event_and_snow_free_remainder`
   capture variant.
2. `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation.rs`
   defines the sealed mode and primitive-only test DTOs.
3. `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver.rs`
   forwards the private generic mode and state.
4. `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver/support.rs`
   forwards the private generic mode and state.
5. `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver/evaluation.rs`
   captures borrowed coupling operands and the rain-derived
   `TerminalFluxIntegral::external_liquid_kg_m2` provenance.
6. `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver/terminal_event.rs`
   captures all three beginning/ending states, ledgers and joints before the
   coarse joint is discarded; captures pair/floor chronology; owns validators.
7. `crates/openwepp-hillslope-orchestrator/src/v11_covered/carrier_phase.rs`
   projects named result leaves into owned primitive DTOs.
8. `crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/covered_derived_ingress.rs`
   supplies a `cfg(test)` private projection at construction of
   `DirectSurfaceLiquidIngressInput`, enumerating every typed parcel kind and
   every `TerminalReceiver` match without serializing the input.
9. `crates/openwepp-hillslope-orchestrator/src/direct_runtime/surface_liquid_ingress.rs`
   supplies the private test projection type and exact parcel-kind walker used
   by item 8. It does not change ingress behavior.

Current source sha256 values, in that order, are:
`8a09365471874c50ea5123e307b1e3dcb81fad6c960339da8ad345012d438523`,
`b15687bcf8fb612fb3bd116e0c79de9bd3dcae47d1360c5c567f8374cd9a7f12`,
`98d170805e041c6931b82619a1c3221a1c2fbb026a705be3fe4064aaa55b58bf`,
`015f73b58e717bbcf959cbba052882cb5cfa19a5d21b313384b86d2f14687ec1`,
`ff016348c4edb9ee959970d6c93628caf6f37669cec35b506fca140e92b82dd6`,
`4e0ae87d9f1964e7dc6941a8dcc733018c04929b5451c35a1861f47d56af50ed`,
`af8d00b3135447d2c8f7a40d13194075a444e56f45b75e46d45c662016fa3ffe`,
`6c4a4ce711c5be6a80f524cf2c3692ce6e7785875dc4c0b6305912f5a3e0dd29`,
and `3c555f9f395b56a1ac92dcba70d66a714d45959d13ff96ff33dab5f0aaa3104f`.

No public signature, workspace manifest, feature, runtime selector, production
wire, restart surface, receiver, runner or temporal operator is in scope.
