# Review A

Evidence mode: Static.

Scope reviewed:

- `SC-SNOWFREEZE-001`
- `08_snow_albedo.rs`
- `infiltration_reconciliation.rs`
- `snowbench_coe_melt.rs`
- direct-publication include split and source-guard test split
- `winter_thaw_melt_response_correction.py`
- `winter_thaw_melt_response_coupled_gate.py`
- `snowdensity10_3_7_winter_thaw_melt_response_correction.rs`
- package artifacts

Findings:

- No blocking correctness findings.
- The contract explicitly qualifies the legacy density gate instead of silently
  weakening `INV-SNOWFREEZE-002`.
- The runtime change is selector-gated and does not alter the `legacy_coe`
  low-density branch or `coe_shortwave_albedo_v1` albedo branch.
- The diagnostic comparison uses the same 10.3.6 surface/event-window logic for
  both models.
- v94 conservation review finding is resolved: emitted active-ledger rows close
  SWE balance and routed state-loss residuals to zero for the candidate.
- v94 coupled WAT review finding is resolved: real direct-production WAT
  improves paired snow-control failures from `1147` to `978` and no paired
  surface worsens.

Residual risk:

- The candidate improves thaw under-response but leaves 108 under-ablation
  windows and 978 coupled WAT snow-control failures. Treating this as activation
  or frost unblock would be premature.
