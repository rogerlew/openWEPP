# HPHYS0225 Worker Handoff

Status: completed  
Evidence mode: Static + Ran

## Immediate Next Actions

1. Continue HPHYS remediation with constitutive re-derivation package focused on
   the remaining coupled residual families (`Dp`, `latqcc`, `Total-Soil`,
   `SoilWaterTotal`, `ProfileFCStore`) under Correctness Authority Model gates.
2. Keep both required Level-4 WB19 suites in hard-fail lane for all follow-on
   WB19 work:
   - `cas_l4_subhyd_withdrawal_soilwater_cap_001`
   - `cas_l4_subhyd_layer_pool_withdrawal_cap_001`
3. Preserve explicit prohibition of WB19 legacy available-pool reconciliation
   (`max(layer_pool, legacy_term)`) as a standing contract/test guard.
