# HPHYS0216 Contract Implementation Evidence

Status: completed
Evidence mode: Static + Ran

## Canonical authority intake
Read and applied:
- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `/workdir/wepp-forest_260430_baseline/src/watbal.for`
- `/workdir/wepp-forest_260430_baseline/src/watbalprint.for`

## Contract amendments implemented
1. `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
   - added `### HPHYS0216 ProfileFC Layer-Authority Realignment`.
   - realigned WB13 `ProfileFCStore` publication authority to
     `Σ(thetfc_i * dg_i) * 1000`.
   - retained `wb13_profile_fc_store_mm` as diagnostic carry surface.
2. `docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
   - added `## HPHYS0216 ProfileFC Layer-Authority Realignment Addendum`.
   - clarified corrected-layer lineage and FC publication authority split.
3. `docs/specifications/science-contracts/contracts/SC-PERC-001.md`
   - updated WB13 coupling addendum to match FC layer authority and WP
     projected-storage authority.
4. `docs/specifications/science-contracts/index.md`
   - updated contract index `Last updated` stamp and SC row notes.

## Authority decision recorded
- Baseline-authoritative `ProfileFCStore` publication is layer-aggregated
  (`thetfc_####`, `dg_####`, `nsl`) at WB13 publish boundary.
- `ProfileWPStore` authority remains `wb13_profile_wp_store_mm`.
- `wb13_profile_fc_store_mm` remains available for diagnostics/provenance only.
