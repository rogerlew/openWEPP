# HPHYS0232 Contract Implementation Evidence

Status: completed  
Evidence mode: Static

## Static

Updated canonical contract authority in:
- `docs/specifications/science-contracts/contracts/SC-PERC-001.md`

Amendments applied:
1. `contract_version` advanced `16 -> 17`.
2. Added WB18 lane attenuation authority for legacy hourly seepage lineage
   (`ui_LFtstp`) via runtime control symbol `wb18_perc_lane_substeps`.
3. Codified algorithm semantics:
   - default daily divisor when symbol absent (`1`),
   - strict finite/positive/integral domain when symbol is present,
   - hourly lineage divisor `24` from `watbal_hourly.for`.
4. Added/updated constants, invariant language, branch table scope, test-vector
   obligations, and new HPHYS0232 addendum.

Authority anchors cited in-contract:
- `/workdir/wepp-forest_260430_baseline/src/purk.for`
- `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for`
