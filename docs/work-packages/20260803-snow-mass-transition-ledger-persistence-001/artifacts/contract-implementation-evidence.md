# Contract Implementation Evidence

Status: `implemented / focused gates pass`

Evidence mode: `Static + Ran`

`SC-SNOWFREEZE-001` v124 adds `INV-SNOWFREEZE-091`,
`OBL-SNOWFREEZE-P-064`, and `TOL-SNOWFREEZE-016`.

| Authority | Implementation | Direct evidence |
|---|---|---|
| one physical authority | Existing CoE, density, Stage-3, and storage calculations remain in their original sequence in `runoff_reconciliation.rs`; no equation or guard was duplicated | scaffold/candidate schema-v4, WAT, and HBP identities are exact |
| upstream ledger | `DirectSnowSolidToLiquidLedger` owns raw signed melt, redistributed positive melt, bounded SWE loss, released rain, and liquid handoff copied from `SnowCouplingOutcome` | candidate JSONL independently closes with maximum error `1.3877787807814457e-17 m` |
| exact handoff | the same `routed_melt_m` local populates upstream `liquid_handoff_m` and is passed as Stage-3 `incoming_liquid_m` | all `8615` enabled Stage-3 rows link exactly within `1e-9 m` |
| downstream ledger | `DirectSnowLiquidDispositionLedger` owns incoming, routed, signed retained change, refrozen, and residual values returned by the authoritative Stage-3 solve | candidate maximum independent closure error `1.2271813339820303e-17 m` |
| production outcome | `DirectSnowStage3Outcome` retains enabled state, typed meltwater temperature, and sublimation on every capture path | direct-runtime publication consumes `stage3_outcome`, not verbose diagnostics |
| optional capture | `DirectSnowDiagnosticCapture` is resolved by the runner before the snow solve; `DirectSnowVerboseDiagnostics` is boxed only for selected rows | real trace writer calls `verbose_diagnostics.as_deref()` and fails closed if the selected payload is absent |

The contract-derived test ran RED against the scaffold (`1 PASS / 2 RED`) and
now passes `8/8`. The changed local-content dependency was adopted with typed
scientific-full transaction
`assurance/v2/transactions/25f857b712ee7a69738cfd71ddfc1e03a974bfb052ac5212723155dce46a1cef.json`.
It updates generated identity/review locks only; no active review authority was
invalidated.
