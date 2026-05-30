# HPHYS0208 Contract Implementation Evidence

Status: completed  
Evidence mode: Static

## Canonical contract amendments
- Static: `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
  - Added `HPHYS0208 FC-Threshold Coupled Residual Closure Addendum` requiring
    WB11 seed coupling via `sat`, `por_####`, `cpm_####`, `thetfc_####`,
    `thetdr_####`, and `dg_####`.
  - Added explicit WB18 seed formulas and fail-closed posture for missing or
    domain-invalid lineage symbols.
- Static: `docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
  - Added HPHYS0208 addendum for WB11 threshold-lineage projection authority
    (`sat`/`por_####`/`cpm_####` publication continuity).
- Static: `docs/specifications/science-contracts/contracts/SC-PERC-001.md`
  - Added HPHYS0208 addendum binding WB18 threshold-consumer initialization to
    coupled seed lineage and surrogate prohibition.
- Static: `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
  - Added HPHYS0208 coupled residual closure addendum for WB13 `Dp`/`latqcc`
    lineage claims.
- Static: `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
  - Added HPHYS0208 system-boundary addendum for coupled WB13 publication
    lineage (`Total-Soil`/`SoilWaterTotal`/`Dp`/`latqcc`).
- Static: `docs/specifications/science-contracts/index.md`
  - Added HPHYS0208 registry notes for the touched canonical contracts.

## Contract versioning hygiene
- Static: updated `contract_version` and revision-history rows in each touched
  contract with HPHYS0208 provenance.
